from __future__ import annotations

import argparse
import asyncio
import json
import os
import random
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

import tree_sitter_java
from dotenv import load_dotenv
from openai import AsyncOpenAI
from tree_sitter import Language, Node, Parser

SCRIPT_DIR = Path(__file__).parent
DOCS_PY_OUTPUT = SCRIPT_DIR / "docs.py"
DEPRECATED_PY_OUTPUT = SCRIPT_DIR / "deprecated.py"
ENV_FILE = SCRIPT_DIR / ".env"
DEFAULT_SOURCE = SCRIPT_DIR.parent / "java/src/main/java"
INCLUDED_CLASSES = {
    "ComprehensiveZmanimCalendar",
    "ZmanimCalendar",
    "AstronomicalCalendar",
}
SKIPPED_METHODS = set[str]()

MODEL = "gpt-5.4-mini"
REASONING_EFFORT = "low"
CONCURRENCY = 10
REQUEST_TIMEOUT_SECONDS = 120
MAX_GENERATION_ATTEMPTS = 3
DEFAULT_DEV_COUNT = 20
DEFAULT_SEED = 613

JAVA_LANGUAGE = Language(tree_sitter_java.language())
JAVA_PARSER = Parser()
JAVA_PARSER.language = JAVA_LANGUAGE


@dataclass(frozen=True)
class ExtractedMethod:
    name: str
    qualified_name: str
    package: str
    class_name: str
    qualified_class: str
    file: str
    line: int
    return_type: str
    parameters: list[str]
    annotations: list[str]
    is_deprecated: bool
    docs: str


@dataclass(frozen=True)
class UserDocs:
    meaning: str
    calculation: str
    notes: list[str]
    deprecated_note: str


class ZmanGenerationError(RuntimeError):
    pass


SYSTEM_PROMPT = """
You turn raw KosherJava zmanim Javadocs into concise, user-facing documentation.
Use only facts directly supported by the source docs.
Write for people trying to understand zmanim, not for API users.
Do not mention Javadoc, source code, Java, APIs, methods, return values, or null.
If the source says null is returned, say the zman may not be available or cannot
be calculated in that situation.
Preserve useful links as Markdown when using linked text, but do not invent links.
Use ASCII only in generated user docs. Write "degrees" instead of the degree
symbol, plain apostrophes instead of curly quotes, and hyphens instead of dashes.

Ignore source text about alternate astronomical calculators, fallback
calculations, and calculator-specific caveats. Never mention NOAA, SunTimes,
USNO, or any calculator implementation in user-facing fields.

Always assume astronomical chatzos is used. Treat isUseAstronomicalChatzos as
always true and never mention it, half-day chatzos fallbacks, or any choice
between astronomical chatzos and sunrise-to-sunset midpoint. Document only the
astronomical-chatzos behavior when the source describes both options.

Allowed runtime placeholders:
- {uses_elevation}
- {sea_level}
- {candel_lighting_offset}
- {ateret_torah_offset}
- {use_astronomical_chatzos_for_other_zmanim}

Use placeholders only when directly supported by the source docs. The elevation
and {use_astronomical_chatzos_for_other_zmanim} placeholders are full sentences;
place them as standalone notes. The candle-lighting and Ateret Torah placeholders
are time-like values; use them inside normal prose. Return valid JSON only.
"""

REVIEW_PROMPT = """
Review generated user-facing zman documentation against the source docs.
Be practical, not strict. Approve concise docs when they are source-backed,
read clearly, use the required schema, avoid API/source-code language, and do
not misuse placeholders.

The allowed placeholders are intentional runtime tokens and will be replaced in
post-processing. Do not reject documentation merely because it includes one of
these placeholders: {uses_elevation}, {sea_level}, {candel_lighting_offset},
{ateret_torah_offset}, {use_astronomical_chatzos_for_other_zmanim}. The
elevation and {use_astronomical_chatzos_for_other_zmanim} placeholders are
valid as standalone notes. The candle-lighting and Ateret Torah placeholders are
valid inside normal prose.
Reject placeholders only when the source docs do not support them or when they
are placed in a grammatically broken way.
If a supported placeholder covers a setting or caveat, treat that setting or
caveat as documented. Do not ask for both the placeholder and expanded wording.

Ignore source text about alternate astronomical calculators and
calculator-specific caveats. Do not require caveats that apply only to those
alternate calculators. Never mention NOAA, SunTimes, USNO, or any calculator
implementation in user-facing fields.

Always assume astronomical chatzos is used. Treat isUseAstronomicalChatzos as
always true and never mention it, half-day chatzos fallbacks, or any choice
between astronomical chatzos and sunrise-to-sunset midpoint. Do not reject docs
for omitting those Java-only configuration branches.

Reject only for hallucinated facts, unsupported links, materially wrong
calculations, broken prose, forbidden implementation language, or placeholder
misuse as defined above. Reject any generated user doc containing non-ASCII
characters. Return valid JSON only.
"""


def strip_java_noise(line: str) -> str:
    """Remove the comment delimiters and leading stars from one Javadoc line."""
    stripped = line.strip()
    if stripped.startswith("/**"):
        stripped = stripped[3:]
    if stripped.endswith("*/"):
        stripped = stripped[:-2]
    if stripped.startswith("*"):
        stripped = stripped[1:]
        if stripped.startswith(" "):
            stripped = stripped[1:]
    return stripped.rstrip()


def relative_path(path: Path) -> str:
    try:
        return str(path.relative_to(Path.cwd()))
    except ValueError:
        return str(path)


def node_text(node: Node | None, source: bytes) -> str:
    if node is None:
        return ""
    return source[node.start_byte : node.end_byte].decode("utf-8")


def walk(node: Node) -> list[Node]:
    """Return a depth-first list of nodes because tree-sitter has no built-in walker."""
    nodes = [node]
    for child in node.children:
        nodes.extend(walk(child))
    return nodes


def find_child(node: Node, node_type: str) -> Node | None:
    return next((child for child in node.children if child.type == node_type), None)


def package_name(root: Node, source: bytes) -> str:
    """Extract the declared Java package from a parsed compilation unit."""
    package = find_child(root, "package_declaration")
    if package is None:
        return ""

    for child in package.children:
        if child.type in {"identifier", "scoped_identifier"}:
            return node_text(child, source)
    return ""


def enclosing_class(node: Node, source: bytes) -> str:
    """Build the dotted class path for methods inside nested Java types."""
    class_names: list[str] = []
    current = node.parent
    while current is not None:
        if current.type in {
            "class_declaration",
            "interface_declaration",
            "record_declaration",
            "enum_declaration",
        }:
            name = node_text(current.child_by_field_name("name"), source)
            if name:
                class_names.append(name)
        current = current.parent
    return ".".join(reversed(class_names))


def has_public_modifier(method: Node) -> bool:
    modifiers = find_child(method, "modifiers")
    return modifiers is not None and any(
        child.type == "public" for child in modifiers.children
    )


def method_annotations(method: Node, source: bytes) -> list[str]:
    modifiers = find_child(method, "modifiers")
    if modifiers is None:
        return []
    return [
        node_text(child, source)
        for child in modifiers.children
        if "annotation" in child.type
    ]


def is_no_arg_method(method: Node) -> bool:
    parameters = method.child_by_field_name("parameters")
    if parameters is None:
        return False
    return not any("parameter" in child.type for child in parameters.children)


def returns_instant(method: Node, source: bytes) -> bool:
    return_type = method.child_by_field_name("type")
    return node_text(return_type, source) in {"Instant", "java.time.Instant"}


def preceding_javadoc(method: Node, source: bytes) -> list[str]:
    """Find the Javadoc block attached immediately before a method declaration."""
    sibling = method.prev_named_sibling
    if sibling is None or sibling.type != "block_comment":
        return []

    comment = node_text(sibling, source)
    if not comment.lstrip().startswith("/**"):
        return []
    return comment.splitlines()


def parse_javadoc(lines: list[str]) -> str:
    """Normalize a raw Javadoc block while preserving inline tags and HTML."""
    cleaned_lines = [strip_java_noise(line) for line in lines]
    raw_text = "\n".join(cleaned_lines).strip()
    return raw_text


def parse_java_file(path: Path) -> list[ExtractedMethod]:
    source = path.read_bytes()
    tree = JAVA_PARSER.parse(source)
    root = tree.root_node
    package = package_name(root, source)
    methods: list[ExtractedMethod] = []

    for method in walk(root):
        if method.type != "method_declaration":
            continue
        if not has_public_modifier(method) or not returns_instant(method, source):
            continue
        if not is_no_arg_method(method):
            continue

        class_name = enclosing_class(method, source)
        if class_name not in INCLUDED_CLASSES:
            continue

        qualified_class = ".".join(part for part in (package, class_name) if part)
        method_name = node_text(method.child_by_field_name("name"), source)
        if method_name in SKIPPED_METHODS:
            continue

        annotations = method_annotations(method, source)
        javadocs = preceding_javadoc(method, source)
        docs = parse_javadoc(javadocs)
        if not docs:
            raise ValueError(f"No docs found for {method_name} in {path}")

        modifiers = find_child(method, "modifiers")
        public = None
        if modifiers is not None:
            public = next(
                (child for child in modifiers.children if child.type == "public"), None
            )

        methods.append(
            ExtractedMethod(
                name=method_name,
                qualified_name=".".join(
                    part for part in (qualified_class, method_name) if part
                ),
                package=package,
                class_name=class_name,
                qualified_class=qualified_class,
                file=relative_path(path),
                line=(public or method).start_point[0] + 1,
                return_type=node_text(method.child_by_field_name("type"), source),
                parameters=[],
                annotations=annotations,
                is_deprecated=any(
                    annotation.startswith("@Deprecated") for annotation in annotations
                ),
                docs=docs,
            )
        )

    return methods


def collect_methods() -> list[ExtractedMethod]:
    source = DEFAULT_SOURCE.resolve()
    if not source.exists():
        raise FileNotFoundError(f"Source directory does not exist: {source}")
    methods: list[ExtractedMethod] = []
    for java_file in sorted(source.rglob("*.java")):
        methods.extend(parse_java_file(java_file))
    if not methods:
        raise RuntimeError(
            "No public no-argument Instant-returning methods were found."
        )
    return methods


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Parse Java Javadocs and generate user-facing zman documentation."
    )
    parser.add_argument(
        "--dev",
        action="store_true",
        help=f"Generate {DEFAULT_DEV_COUNT} random zmanim.",
    )
    parser.add_argument(
        "--count",
        type=int,
        help="Generate this many random zmanim. Implies dev-style sampling.",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=DEFAULT_SEED,
        help="Seed for random sampling.",
    )
    args = parser.parse_args()
    if args.count is not None and args.count < 1:
        parser.error("--count must be at least 1")
    return args


def choose_methods(
    methods: list[ExtractedMethod], args: argparse.Namespace
) -> list[ExtractedMethod]:
    count = args.count
    if count is None and args.dev:
        count = DEFAULT_DEV_COUNT
    if count is None:
        return methods

    sample_count = min(count, len(methods))
    selected = {
        method.qualified_name
        for method in random.Random(args.seed).sample(methods, sample_count)
    }
    return [method for method in methods if method.qualified_name in selected]


def generation_prompt(
    method: ExtractedMethod,
    review_issues: list[str] | None = None,
) -> str:
    payload: dict[str, Any] = {
        "requirements": [
            "Return exactly one JSON object, not an array and not an object with an items key.",
            "The top-level object must use the output_schema exactly.",
            "qualified_name must exactly match the input method qualified_name.",
            "Ignore any source text about alternate astronomical calculators.",
            "Never mention NOAA, SunTimes, USNO, or any calculator implementation.",
            "Always assume astronomical chatzos is used; never mention isUseAstronomicalChatzos, half-day chatzos, or choosing between chatzos modes.",
            "Use {use_astronomical_chatzos_for_other_zmanim} only when the source says other zmanim depend on isUseAstronomicalChatzosForOtherZmanim.",
        ],
        "output_schema": {
            "qualified_name": "same qualified_name as input",
            "meaning": "plain-English meaning, source-backed",
            "calculation": "how it is calculated, source-backed or empty",
            "notes": ["source-backed caveats or settings"],
            "deprecated_note": "source-backed deprecation warning or empty",
        },
        "method": method_payload(method),
    }
    if review_issues:
        payload["previous_review_issues"] = review_issues
    return json.dumps(payload, ensure_ascii=False)


def review_user_prompt(
    method: ExtractedMethod,
    generated: UserDocs,
) -> str:
    payload = {
        "requirements": [
            "Return approved=true if the documentation is acceptable.",
            "Return approved=false only for material problems.",
            "Use the output_schema exactly.",
        ],
        "output_schema": {
            "approved": "boolean",
            "issues": [
                {
                    "issue": "short actionable issue",
                }
            ],
        },
        "source_method": method_payload(method),
        "generated_docs": {
            "qualified_name": method.qualified_name,
            **asdict(generated),
        },
    }
    return json.dumps(payload, ensure_ascii=False)


def method_payload(method: ExtractedMethod) -> dict[str, Any]:
    return {
        "qualified_name": method.qualified_name,
        "name": method.name,
        "class_name": method.class_name,
        "is_deprecated": method.is_deprecated,
        "docs": method.docs,
    }


def parse_docs(data: Any, method: ExtractedMethod) -> UserDocs:
    if not isinstance(data, dict):
        raise ValueError("Model response must be an object")
    if isinstance(data.get("items"), list) and len(data["items"]) == 1:
        data = data["items"][0]
        if not isinstance(data, dict):
            raise ValueError("Model response item must be an object")
    if "qualified_name" not in data:
        data = {"qualified_name": method.qualified_name, **data}
    qualified_name = data.get("qualified_name")
    if qualified_name != method.qualified_name:
        raise ValueError(f"Unexpected qualified_name: {qualified_name!r}")

    notes = data.get("notes")
    if not isinstance(notes, list) or not all(isinstance(note, str) for note in notes):
        raise ValueError(f"{method.qualified_name} has invalid notes")
    docs = UserDocs(
        meaning=require_string(data, "meaning", method.qualified_name),
        calculation=require_string(data, "calculation", method.qualified_name),
        notes=notes,
        deprecated_note=require_string(data, "deprecated_note", method.qualified_name),
    )
    validate_ascii_docs(method.qualified_name, docs)
    return docs


def require_string(item: dict[str, Any], field: str, qualified_name: str) -> str:
    value = item.get(field)
    if not isinstance(value, str):
        raise ValueError(f"{qualified_name} has invalid {field}")
    return value


def validate_ascii_docs(qualified_name: str, docs: UserDocs) -> None:
    validate_ascii(f"{qualified_name}.meaning", docs.meaning)
    validate_ascii(f"{qualified_name}.calculation", docs.calculation)
    validate_ascii(f"{qualified_name}.deprecated_note", docs.deprecated_note)
    for index, note in enumerate(docs.notes, start=1):
        validate_ascii(f"{qualified_name}.notes[{index}]", note)


def validate_ascii(field: str, value: str) -> None:
    try:
        value.encode("ascii")
    except UnicodeEncodeError as error:
        character = value[error.start]
        raise ValueError(
            f"{field} contains non-ASCII character {character!r}; "
            "use ASCII only, for example 'degrees' instead of the degree symbol"
        ) from None


def parse_review(data: Any) -> tuple[bool, list[str]]:
    if not isinstance(data, dict):
        raise ValueError("Review response must be an object")
    approved = data.get("approved")
    issues = data.get("issues")
    if not isinstance(approved, bool) or not isinstance(issues, list):
        raise ValueError("Review response must include approved and issues")

    parsed_issues: list[str] = []
    for item in issues:
        if not isinstance(item, dict):
            raise ValueError("Review issue must be an object")
        issue = item.get("issue")
        if not isinstance(issue, str) or not issue.strip():
            raise ValueError("Review issue is malformed")
        parsed_issues.append(issue.strip())

    if approved:
        return True, []
    if not parsed_issues:
        raise ValueError("Review rejected the documentation without issues")
    return False, parsed_issues


async def json_chat(
    client: AsyncOpenAI,
    system: str,
    user: str,
    seed: int,
) -> Any:
    response = await asyncio.wait_for(
        client.chat.completions.create(
            model=os.environ.get("OPENAI_MODEL", MODEL),
            seed=seed,
            reasoning_effort=REASONING_EFFORT,
            response_format={"type": "json_object"},
            messages=[
                {"role": "system", "content": system},
                {"role": "user", "content": user},
            ],
        ),
        timeout=REQUEST_TIMEOUT_SECONDS,
    )
    content = response.choices[0].message.content
    if content is None:
        raise ValueError("Model returned no content")
    return json.loads(content)


async def generate_one(
    client: AsyncOpenAI,
    method: ExtractedMethod,
    item_number: int,
) -> tuple[str, UserDocs]:
    review_issues: list[str] | None = None
    for attempt in range(1, MAX_GENERATION_ATTEMPTS + 1):
        generated = parse_docs(
            await json_chat(
                client,
                SYSTEM_PROMPT,
                generation_prompt(method, review_issues),
                DEFAULT_SEED + item_number * 100 + attempt,
            ),
            method,
        )
        approved, issues = parse_review(
            await json_chat(
                client,
                REVIEW_PROMPT,
                review_user_prompt(method, generated),
                DEFAULT_SEED + item_number * 100 + 50 + attempt,
            )
        )
        if approved:
            print(
                f"{item_number}: {method.name} review approved attempt {attempt}.",
                flush=True,
            )
            return method.qualified_name, generated

        review_issues = issues
        print(
            f"{item_number}: {method.name} review rejected attempt {attempt}/"
            f"{MAX_GENERATION_ATTEMPTS}: {'; '.join(issues)}",
            flush=True,
        )

    raise RuntimeError(
        f"{method.qualified_name} failed review after {MAX_GENERATION_ATTEMPTS} attempts: "
        f"{'; '.join(review_issues or [])}"
    )


async def generate_all(methods: list[ExtractedMethod]) -> dict[str, UserDocs]:
    client = AsyncOpenAI(
        api_key=os.environ["OPENAI_API_KEY"],
        timeout=REQUEST_TIMEOUT_SECONDS,
        max_retries=0,
    )
    semaphore = asyncio.Semaphore(CONCURRENCY)

    async def run_one(
        item_number: int, method: ExtractedMethod
    ) -> tuple[str, UserDocs]:
        async with semaphore:
            print(
                f"{item_number}/{len(methods)}: generating {method.name}",
                flush=True,
            )
            try:
                return await generate_one(client, method, item_number)
            except Exception as error:
                raise ZmanGenerationError(
                    f"{item_number}/{len(methods)} {method.qualified_name}: "
                    f"{type(error).__name__}: {error}"
                ) from error

    try:
        tasks = [
            asyncio.create_task(run_one(item_number, method))
            for item_number, method in enumerate(methods, start=1)
        ]
        docs: dict[str, UserDocs] = {}
        completed = 0
        try:
            for task in asyncio.as_completed(tasks):
                qualified_name, user_docs = await task
                docs[qualified_name] = user_docs
                completed += 1
                print(
                    f"Completed {completed}/{len(methods)}.",
                    flush=True,
                )
        except Exception:
            for task in tasks:
                if not task.done():
                    task.cancel()
            await asyncio.gather(*tasks, return_exceptions=True)
            raise
        return docs
    finally:
        await client.close()


def docs_paragraph(docs: UserDocs) -> str:
    pieces = [
        docs.meaning,
        docs.calculation,
        *docs.notes,
        docs.deprecated_note,
    ]
    return " ".join(" ".join(piece.split()) for piece in pieces if piece.strip())


def write_python_docs(
    methods: list[ExtractedMethod], docs: dict[str, UserDocs]
) -> None:
    docs_by_method: dict[str, str] = {}
    for method in methods:
        if method.name in docs_by_method:
            raise RuntimeError(f"Duplicate Java method name for DOCS: {method.name}")
        docs_by_method[method.name] = docs_paragraph(docs[method.qualified_name])

    lines = [
        "# Generated by tools/generate-docs.py. Do not edit by hand.",
        "",
        "DOCS = {",
    ]
    for method_name, text in docs_by_method.items():
        lines.append(f"    {method_name!r}: {text!r},")
    lines.append("}")

    DOCS_PY_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    DOCS_PY_OUTPUT.write_text(
        "\n".join(lines) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def write_deprecated_methods(methods: list[ExtractedMethod]) -> None:
    seen: set[str] = set()
    deprecated_methods: list[str] = []
    for method in methods:
        if method.name in seen:
            raise RuntimeError(
                f"Duplicate Java method name for DEPRECATED_METHODS: {method.name}"
            )
        seen.add(method.name)
        if method.is_deprecated:
            deprecated_methods.append(method.name)

    lines = [
        "# Generated by tools/generate-docs.py. Do not edit by hand.",
        "",
        "DEPRECATED_METHODS = [",
    ]
    for method_name in sorted(deprecated_methods):
        lines.append(f"    {method_name!r},")
    lines.append("]")

    DEPRECATED_PY_OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    DEPRECATED_PY_OUTPUT.write_text(
        "\n".join(lines) + "\n",
        encoding="utf-8",
        newline="\n",
    )


def write_docs_output(
    methods: list[ExtractedMethod], docs: dict[str, UserDocs]
) -> None:
    missing = [
        method.qualified_name for method in methods if method.qualified_name not in docs
    ]
    if missing:
        raise RuntimeError(f"Missing docs for: {', '.join(missing)}")

    write_python_docs(methods, docs)


def main() -> None:
    load_dotenv(ENV_FILE)
    args = parse_args()
    if not os.environ.get("OPENAI_API_KEY"):
        raise RuntimeError("OPENAI_API_KEY is required")

    all_methods = collect_methods()
    methods = choose_methods(all_methods, args)
    print(
        f"Generating {len(methods)} methods with {CONCURRENCY} running at a time.",
        flush=True,
    )
    docs = asyncio.run(generate_all(methods))
    write_docs_output(methods, docs)
    write_deprecated_methods(all_methods)
    print(
        f"Wrote {len(methods)} documented methods to {DOCS_PY_OUTPUT} and "
        f"deprecated method list to {DEPRECATED_PY_OUTPUT}.",
        flush=True,
    )


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Stopped.", file=sys.stderr)
        raise SystemExit(130)
    except Exception as error:
        print(f"ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
