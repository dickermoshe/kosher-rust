from __future__ import annotations

import argparse
from dataclasses import dataclass
from pathlib import Path
from typing import cast

import tree_sitter_java
from tree_sitter import Language, Node, Parser

from dsl import ZMAN_NAMES, Zman

SCRIPT_DIR = Path(__file__).parent
DEFAULT_SOURCE = SCRIPT_DIR.parent / "java/src/main/java"
DEFAULT_OUTPUT_DIR = SCRIPT_DIR / "docs"
INCLUDED_CLASSES = {
    "ComprehensiveZmanimCalendar",
    "ZmanimCalendar",
    "AstronomicalCalendar",
}

JAVA_LANGUAGE = Language(tree_sitter_java.language())
JAVA_PARSER = Parser()
JAVA_PARSER.language = JAVA_LANGUAGE


@dataclass(frozen=True)
class ExtractedMethod:
    name: str
    qualified_name: str
    class_name: str
    file: str
    line: int
    docs: str


def strip_java_noise(line: str) -> str:
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
    nodes = [node]
    for child in node.children:
        nodes.extend(walk(child))
    return nodes


def find_child(node: Node, node_type: str) -> Node | None:
    return next((child for child in node.children if child.type == node_type), None)


def package_name(root: Node, source: bytes) -> str:
    package = find_child(root, "package_declaration")
    if package is None:
        return ""

    for child in package.children:
        if child.type in {"identifier", "scoped_identifier"}:
            return node_text(child, source)
    return ""


def enclosing_class(node: Node, source: bytes) -> str:
    class_names: list[str] = []
    current = node.parent
    while current is not None:
        if current.type in {"class_declaration", "interface_declaration"}:
            class_names.append(node_text(current.child_by_field_name("name"), source))
        current = current.parent
    return ".".join(reversed(class_names))


def has_public_modifier(method: Node) -> bool:
    modifiers = find_child(method, "modifiers")
    return modifiers is not None and any(
        child.type == "public" for child in modifiers.children
    )


def is_no_arg_method(method: Node) -> bool:
    parameters = method.child_by_field_name("parameters")
    if parameters is None:
        return False
    return not any("parameter" in child.type for child in parameters.children)


def returns_instant(method: Node, source: bytes) -> bool:
    return_type = method.child_by_field_name("type")
    return node_text(return_type, source) in {"Instant", "java.time.Instant"}


def preceding_javadoc(method: Node, source: bytes) -> list[str]:
    sibling = method.prev_named_sibling
    if sibling is None or sibling.type != "block_comment":
        return []

    comment = node_text(sibling, source)
    if not comment.lstrip().startswith("/**"):
        return []
    return comment.splitlines()


def parse_javadoc(lines: list[str]) -> str:
    cleaned_lines = [strip_java_noise(line) for line in lines]
    return "\n".join(cleaned_lines).strip()


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

        method_name = node_text(method.child_by_field_name("name"), source)
        qualified_class = ".".join(part for part in (package, class_name) if part)
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
                class_name=class_name,
                file=relative_path(path),
                line=(public or method).start_point[0] + 1,
                docs=docs,
            )
        )

    return methods


def collect_methods(source: Path) -> list[ExtractedMethod]:
    source = source.resolve()
    if not source.exists():
        raise FileNotFoundError(f"Source directory does not exist: {source}")

    supported_names = {
        method_name for method_name, value in ZMAN_NAMES.items() if isinstance(value, Zman)
    }
    methods: list[ExtractedMethod] = []
    for java_file in sorted(source.rglob("*.java")):
        methods.extend(
            method
            for method in parse_java_file(java_file)
            if method.name in supported_names
        )

    missing_docs = sorted(supported_names - {method.name for method in methods})
    if missing_docs:
        joined = ", ".join(cast(list[str], missing_docs))
        raise RuntimeError(f"Missing Javadocs for supported methods: {joined}")
    return methods


def markdown_for(method: ExtractedMethod) -> str:
    return f"""# {method.name}

Source: `{method.qualified_name}` ({method.file}:{method.line})

```javadoc
{method.docs}
```

# Human docs

```markdown
```
"""


def write_stub(method: ExtractedMethod, output_dir: Path) -> Path:
    output_dir.mkdir(parents=True, exist_ok=True)
    target = output_dir / f"{method.name}.md"
    with target.open("x", encoding="utf-8", newline="\n") as file:
        file.write(markdown_for(method))
    return target


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Create one non-overwriting raw-Javadoc markdown stub per supported zman."
    )
    parser.add_argument(
        "--source",
        type=Path,
        default=DEFAULT_SOURCE,
        help="Java source root to scan.",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=DEFAULT_OUTPUT_DIR,
        help="Directory where markdown stubs will be created.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    methods = collect_methods(args.source)
    existing = [args.output_dir / f"{method.name}.md" for method in methods]
    existing = [path for path in existing if path.exists()]
    if existing:
        joined = "\n".join(str(path) for path in existing)
        raise FileExistsError(
            "Refusing to overwrite existing documentation stub(s):\n" + joined
        )

    for method in methods:
        write_stub(method, args.output_dir)
    print(f"Created {len(methods)} documentation stubs in {args.output_dir}")


if __name__ == "__main__":
    main()
