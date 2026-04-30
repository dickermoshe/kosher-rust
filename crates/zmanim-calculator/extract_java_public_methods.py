#!/usr/bin/env python3
"""Find public KosherJava zmanim methods not implemented as Rust presets.

The default output is a sorted, class-qualified list of missing methods:

    com.kosherjava.zmanim.ComprehensiveZmanimCalendar#getShaahZmanis90Minutes
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


CRATE_ROOT = Path(__file__).resolve().parent
DEFAULT_SOURCE_ROOT = CRATE_ROOT / "java" / "src" / "main" / "java"
DEFAULT_PRESETS = CRATE_ROOT / "src" / "presets.rs"
EXCLUDED_METHOD_NAMES = {"clone", "equals", "hashCode", "toString","getBeginAstronomicalTwilight"}
DEFAULT_INCLUDED_CLASSES = {
    "AstronomicalCalendar",
    "ComprehensiveZmanimCalendar",
    "ZmanimCalendar",
}
JAVA_CLASS_BY_CALC = {
    "AstronomicalCalendar": "com.kosherjava.zmanim.AstronomicalCalendar",
    "ComprehensiveCalendar": "com.kosherjava.zmanim.ComprehensiveZmanimCalendar",
    "ZmanimCalendar": "com.kosherjava.zmanim.ZmanimCalendar",
}
ZMAN_EXCLUDED_METHODS = {
    "com.kosherjava.zmanim.AstronomicalCalendar#getBeginAstronomicalTwilight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getBeginCivilTwilight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getBeginNauticalTwilight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getEndAstronomicalTwilight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getEndCivilTwilight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getEndNauticalTwilight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getSolarMidnight",
    "com.kosherjava.zmanim.AstronomicalCalendar#getSunTransit",
    "com.kosherjava.zmanim.AstronomicalCalendar#getSunrise",
    "com.kosherjava.zmanim.AstronomicalCalendar#getSunset",
}
IGNORED_METHOD_NAMES = {
    "clone",
    "equals",
    "hashCode",
    "toString",
}

PACKAGE_RE = re.compile(r"\bpackage\s+([\w.]+)\s*;")
TYPE_RE = re.compile(r"\bpublic\s+(?:abstract\s+|final\s+|sealed\s+|non-sealed\s+)*"
                     r"(?:class|interface|enum|record)\s+([A-Za-z_$][\w$]*)")
PUBLIC_CALLABLE_RE = re.compile(
    r"""
    \bpublic\s+
    (?:(?:static|final|synchronized|native|abstract|strictfp|default)\s+)*
    (?:<[^;{}()]+>\s+)?
    (?:
        (?P<return_type>[A-Za-z_$][\w$<>\[\].?,\s]*?)\s+
    )?
    (?P<name>[A-Za-z_$][\w$]*)\s*
    \(
    (?P<params>[^)]*)
    \)
    """,
    re.VERBOSE | re.MULTILINE,
)
RUST_PRESET_START_RE = re.compile(r"^\s*([A-Z][A-Z0-9_]*)\s*\{\s*$", re.MULTILINE)
RUST_PRESET_NAME_RE = re.compile(r'\bname:\s*"([^"]+)"')
RUST_PRESET_JAVA_CALC_RE = re.compile(r"\bjava:\s*JavaCalc::([A-Za-z]+)\s*\(")


def strip_comments_and_literals(source: str) -> str:
    """Remove comments/string bodies while preserving offsets and newlines."""
    chars = list(source)
    i = 0
    while i < len(chars):
        if chars[i] == "/" and i + 1 < len(chars) and chars[i + 1] == "/":
            chars[i] = chars[i + 1] = " "
            i += 2
            while i < len(chars) and chars[i] != "\n":
                chars[i] = " "
                i += 1
            continue

        if chars[i] == "/" and i + 1 < len(chars) and chars[i + 1] == "*":
            chars[i] = chars[i + 1] = " "
            i += 2
            while i + 1 < len(chars) and not (chars[i] == "*" and chars[i + 1] == "/"):
                if chars[i] != "\n":
                    chars[i] = " "
                i += 1
            if i + 1 < len(chars):
                chars[i] = chars[i + 1] = " "
                i += 2
            continue

        if chars[i] in {'"', "'"}:
            quote = chars[i]
            chars[i] = " "
            i += 1
            while i < len(chars):
                if chars[i] == "\\":
                    chars[i] = " "
                    if i + 1 < len(chars) and chars[i + 1] != "\n":
                        chars[i + 1] = " "
                    i += 2
                    continue
                if chars[i] == quote:
                    chars[i] = " "
                    i += 1
                    break
                if chars[i] != "\n":
                    chars[i] = " "
                i += 1
            continue

        i += 1
    return "".join(chars)


def package_name(clean_source: str) -> str:
    match = PACKAGE_RE.search(clean_source)
    return match.group(1) if match else ""


def public_type_name(clean_source: str, java_file: Path) -> str:
    match = TYPE_RE.search(clean_source)
    return match.group(1) if match else java_file.stem


def should_include_method(name: str) -> bool:
    if name in EXCLUDED_METHOD_NAMES:
        return False
    if name.lower() == "tojson":
        return False
    return not (name.startswith("set") or name.startswith("is"))


def returns_instant(return_type: str | None) -> bool:
    if return_type is None:
        return False
    normalized = return_type.strip().split()[-1].removesuffix("[]")
    return normalized == "Instant" or normalized.endswith(".Instant")


def extract_public_methods(
    java_file: Path,
    include_constructors: bool,
    included_classes: set[str] | None,
    zman_only: bool,
) -> set[str]:
    clean_source = strip_comments_and_literals(java_file.read_text(encoding="utf-8"))
    package = package_name(clean_source)
    class_name = public_type_name(clean_source, java_file)
    if included_classes is not None and class_name not in included_classes:
        return set()

    qualified_class = f"{package}.{class_name}" if package else class_name
    methods: set[str] = set()

    for match in PUBLIC_CALLABLE_RE.finditer(clean_source):
        name = match.group("name")
        if zman_only:
            has_args = bool(match.group("params").strip())
            if has_args or not returns_instant(match.group("return_type")):
                continue

        is_constructor = name == class_name and match.group("return_type") is None
        if is_constructor:
            if include_constructors:
                methods.add(f"{qualified_class}#<init>")
            continue
        if not should_include_method(name):
            continue
        method = f"{qualified_class}#{name}"
        if zman_only and method in ZMAN_EXCLUDED_METHODS:
            continue
        methods.add(method)

    return methods


def rust_preset_blocks(source: str) -> list[tuple[str, str]]:
    starts = list(RUST_PRESET_START_RE.finditer(source))
    blocks: list[tuple[str, str]] = []

    for index, start in enumerate(starts):
        end = starts[index + 1].start() if index + 1 < len(starts) else len(source)
        blocks.append((start.group(1), source[start.start():end]))

    return blocks


def extract_rust_presets(presets_file: Path) -> set[str]:
    source = presets_file.read_text(encoding="utf-8")
    presets: set[str] = set()

    for preset_name, block in rust_preset_blocks(source):
        name_match = RUST_PRESET_NAME_RE.search(block)
        calc_match = RUST_PRESET_JAVA_CALC_RE.search(block)
        if not name_match or not calc_match:
            continue

        java_class = JAVA_CLASS_BY_CALC.get(calc_match.group(1))
        if java_class is None:
            raise ValueError(f"Unknown JavaCalc variant for {preset_name}: {calc_match.group(1)}")

        presets.add(f"{java_class}#{name_match.group(1)}")

    return presets


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--source-root",
        type=Path,
        default=DEFAULT_SOURCE_ROOT,
        help=f"Java source root to scan. Default: {DEFAULT_SOURCE_ROOT}",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="Optional file to write instead of printing to stdout.",
    )
    parser.add_argument(
        "--presets",
        type=Path,
        default=DEFAULT_PRESETS,
        help=f"Rust presets file to scan. Default: {DEFAULT_PRESETS}",
    )
    parser.add_argument(
        "--names-only",
        action="store_true",
        help="Write only unqualified missing method names, one per line.",
    )
    parser.add_argument(
        "--zman",
        action="store_true",
        help="Only include no-argument Java methods that return Instant.",
    )
    parser.add_argument(
        "--all-classes",
        action="store_true",
        help="Include public methods from all Java classes instead of only zmanim calendar classes.",
    )
    parser.add_argument(
        "--class",
        dest="classes",
        action="append",
        help=(
            "Restrict extraction to a public Java class name. Can be passed multiple times. "
            "Defaults to AstronomicalCalendar, ZmanimCalendar, and ComprehensiveZmanimCalendar."
        ),
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    source_root = args.source_root.resolve()
    java_files = sorted(source_root.rglob("*.java"))
    included_classes = (
        None
        if args.all_classes
        else set(args.classes or DEFAULT_INCLUDED_CLASSES)
    )

    java_methods: set[str] = set()
    for java_file in java_files:
        java_methods.update(
            extract_public_methods(
                java_file,
                False,
                included_classes,
                args.zman,
            )
        )

    rust_presets = extract_rust_presets(args.presets.resolve())
    missing_methods = java_methods - rust_presets
    output_values = sorted(
        method.rsplit("#", 1)[1] if args.names_only else method for method in missing_methods
    )
    output = "\n".join(output_values)
    if args.output is None:
        print(output)
    else:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(output + "\n", encoding="utf-8")

    destination = "stdout" if args.output is None else str(args.output)
    print(
        f"Found {len(output_values)} missing Rust zmanim methods to {destination} "
        f"({len(java_methods)} Java methods, {len(rust_presets)} Rust presets)",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
