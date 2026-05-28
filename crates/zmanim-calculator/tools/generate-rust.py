from __future__ import annotations

import importlib.util
import json
import re
from pathlib import Path

from dsl import (
    ZMAN,
    HalfDayBasedOffset,
    LocalMeanTime,
    MinchaGedola,
    MinchaKetana,
    Offset,
    PlagHamincha,
    SamuchLeMinchaKetana,
    ShaahZmanisBasedOffset,
    Shema,
    SofZmanBiurChametz,
    SunriseOffsetByDegrees,
    SunsetOffsetByDegrees,
    Tefila,
    Zman,
    ZmanisOffset,
)

SCRIPT_DIR = Path(__file__).parent
INPUT = SCRIPT_DIR / "docs.py"
DEPRECATED_INPUT = SCRIPT_DIR / "deprecated.py"
OUTPUT = SCRIPT_DIR.parent / "src" / "presets_gen.rs"

NAMES = {
    "getSunrise": "ELEVATION_ADJUSTED_SUNRISE",
    "getSunset": "ELEVATION_ADJUSTED_SUNSET",
}


def method_to_const(method_name: str) -> str:
    if not method_name.startswith("get"):
        raise ValueError(f"Expected Java getter name, got {method_name!r}")

    name = method_name[3:]
    with_underscores = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    with_underscores = re.sub(r"([a-zA-Z])(\d)", r"\1_\2", with_underscores)
    with_underscores = re.sub(r"(\d)([a-zA-Z])", r"\1_\2", with_underscores)
    return with_underscores.upper()


def load_docs(path: Path) -> dict[str, str]:
    spec = importlib.util.spec_from_file_location("generated_docs", path)
    if spec is None or spec.loader is None:
        raise ValueError(f"Unable to load Python docs from {path}")

    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    docs = getattr(module, "DOCS", None)
    if not isinstance(docs, dict):
        raise ValueError(f"{path} must define a DOCS dictionary")
    if not docs:
        raise ValueError(f"No docs found in {path}")

    parsed: dict[str, str] = {}
    for method_name, text in docs.items():
        if not isinstance(method_name, str) or not isinstance(text, str):
            raise ValueError(f"{path} DOCS keys and values must be strings")
        if not text.strip():
            raise ValueError(f"{method_name} has empty docs")
        parsed[method_name] = text
    return parsed


def load_deprecated_methods(path: Path) -> set[str]:
    spec = importlib.util.spec_from_file_location("generated_deprecated", path)
    if spec is None or spec.loader is None:
        raise ValueError(f"Unable to load Python deprecated methods from {path}")

    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    deprecated_methods = getattr(module, "DEPRECATED_METHODS", None)
    if not isinstance(deprecated_methods, list):
        raise ValueError(f"{path} must define a DEPRECATED_METHODS list")

    parsed: set[str] = set()
    for method_name in deprecated_methods:
        if not isinstance(method_name, str):
            raise ValueError(f"{path} DEPRECATED_METHODS values must be strings")
        if method_name in parsed:
            raise ValueError(
                f"{path} contains duplicate deprecated method {method_name}"
            )
        parsed.add(method_name)
    return parsed


def snake_to_pascal(value: str) -> str:
    return "".join(part.capitalize() for part in value.split("_"))


def rust_float(value: float) -> str:
    return repr(float(value))


def rust_bool(value: bool) -> str:
    return "true" if value else "false"


def rust_duration(seconds: float) -> str:
    millis = round(seconds * 1000)
    if abs(seconds * 1000 - millis) > 1e-9:
        raise ValueError(
            f"Duration must be representable in whole milliseconds: {seconds}"
        )
    if millis % 60_000 == 0:
        return f"Duration::from_mins({millis // 60_000})"
    return f"Duration::from_millis({millis})"


def rust_ref(primitive: object) -> str:
    return f"&{rust_primitive(primitive)}"


def rust_primitive(primitive: object) -> str:
    type_name = getattr(primitive, "type_", None)
    if not isinstance(type_name, str):
        raise TypeError(f"Expected DSL primitive with type_, got {primitive!r}")

    if isinstance(primitive, (SunriseOffsetByDegrees, SunsetOffsetByDegrees)):
        return (
            f"ZmanPrimitive::{snake_to_pascal(type_name)}"
            f"({rust_float(primitive.degrees)})"
        )
    if isinstance(primitive, LocalMeanTime):
        return f"ZmanPrimitive::LocalMeanTime({rust_float(primitive.hour)})"
    if isinstance(primitive, Offset):
        return (
            "ZmanPrimitive::Offset("
            f"{rust_ref(primitive.base)}, {rust_duration(primitive.duration_secs)}"
            ")"
        )
    if isinstance(primitive, ZmanisOffset):
        return (
            "ZmanPrimitive::ZmanisOffset("
            f"{rust_ref(primitive.base)}, {rust_float(primitive.hours)}"
            ")"
        )
    if isinstance(primitive, ShaahZmanisBasedOffset):
        return (
            "ZmanPrimitive::ShaahZmanisBasedOffset("
            f"{rust_ref(primitive.start)}, {rust_ref(primitive.end)}, "
            f"{rust_float(primitive.hours)}"
            ")"
        )
    if isinstance(primitive, HalfDayBasedOffset):
        return (
            "ZmanPrimitive::HalfDayBasedOffset("
            f"{rust_ref(primitive.start)}, {rust_ref(primitive.end)}, "
            f"{rust_float(primitive.fraction)}"
            ")"
        )
    if isinstance(
        primitive,
        (
            Shema,
            MinchaGedola,
            SamuchLeMinchaKetana,
            MinchaKetana,
            Tefila,
            PlagHamincha,
            SofZmanBiurChametz,
        ),
    ):
        return (
            f"ZmanPrimitive::{snake_to_pascal(type_name)}("
            f"{rust_ref(primitive.start)}, {rust_ref(primitive.end)}, "
            f"{rust_bool(primitive.synchronous)}"
            ")"
        )
    return f"ZmanPrimitive::{snake_to_pascal(type_name)}"


def rust_doc_comment(text: str) -> str:
    """Format user-facing text as Rust line doc comments."""
    lines = text.strip().splitlines() or ["Generated zman preset."]
    return "\n".join(f"/// {line}" if line else "///" for line in lines)


def preset_block(
    const_name: str,
    method_name: str,
    event: str,
    preset_name: str,
    description: str,
    deprecated: bool,
) -> str:
    name_literal = json.dumps(preset_name, ensure_ascii=False)
    description_literal = json.dumps(description, ensure_ascii=False)
    doc = rust_doc_comment(description)
    test_cfg = "#[cfg(test)]\n" if deprecated else ""
    return f"""#[cfg(test)]
java_parity_test!({const_name});

{test_cfg}{doc}
pub static {const_name}: ZmanPreset = ZmanPreset {{
    event: {event},
    #[cfg(test)]
    method_name: {json.dumps(method_name)},
    name: {name_literal},
    #[cfg(feature = "alloc")]
    description: |_| {description_literal}.to_string(),
}};
"""


def all_presets_array(presets: list[tuple[str, bool]]) -> str:
    entries = ",\n".join(
        (f"    #[cfg(test)]\n    &{const_name}" if deprecated else f"    &{const_name}")
        for const_name, deprecated in presets
    )
    return f"""/// Every generated zman preset.
pub static ALL: &[&ZmanPreset] = &[
{entries},
];
"""


def java_parity_test_macro() -> str:
    return """#[cfg(test)]
macro_rules! java_parity_test {
    ($name:ident) => {
        #[allow(deprecated)]
        #[allow(non_snake_case)]
        mod $name {
            #[test]
            fn standard() -> Result<(), Box<dyn std::error::Error>> {
                crate::java_tests::zmanim::test_preset_in_jerusalem(&super::$name)
            }

            #[test]
            fn regressions() {
                crate::java_tests::zmanim::test_regressions(&super::$name);
            }

            #[test]
            fn random() -> Result<(), Box<dyn std::error::Error>> {
                crate::java_tests::zmanim::test_preset(&super::$name)
            }
        }
    };
}
"""


def generate(docs: dict[str, str], deprecated_methods: set[str]) -> str:
    presets: list[tuple[str, bool, str]] = []
    seen_consts: set[str] = set()
    zman_by_id = {zman.id: zman for zman in ZMAN}

    if len(zman_by_id) != len(ZMAN):
        raise ValueError("ZMAN contains duplicate ids")

    missing_docs = [zman.id for zman in ZMAN if zman.id not in docs]
    if missing_docs:
        raise ValueError(f"Missing docs for DSL methods: {missing_docs}")

    unknown_docs = [
        method_name for method_name in docs if method_name not in zman_by_id
    ]
    if unknown_docs:
        raise ValueError(f"Docs exist for methods missing from DSL: {unknown_docs}")

    unknown_deprecated = [
        method_name
        for method_name in deprecated_methods
        if method_name not in zman_by_id
    ]
    if unknown_deprecated:
        raise ValueError(
            f"Deprecated methods exist for methods missing from DSL: {unknown_deprecated}"
        )

    for zman in ZMAN:
        method_name = zman.id
        if not isinstance(zman, Zman):
            raise TypeError(f"{method_name} is not a DSL Zman")
        if zman.zman is None:
            raise ValueError(f"{method_name} has no DSL primitive")

        if method_name in NAMES:
            const_name = NAMES[method_name]
        else:
            const_name = method_to_const(method_name)
        if const_name in seen_consts:
            raise ValueError(f"Duplicate preset constant {const_name}")
        seen_consts.add(const_name)

        presets.append(
            (
                const_name,
                method_name in deprecated_methods,
                preset_block(
                    const_name,
                    method_name,
                    rust_primitive(zman.zman),
                    zman.name,
                    docs[method_name],
                    method_name in deprecated_methods,
                ),
            )
        )

    if not presets:
        raise RuntimeError("No presets were generated; add methods to the DSL.")

    presets.sort(key=lambda preset: preset[0])
    const_names = [(const_name, deprecated) for const_name, deprecated, _ in presets]
    blocks = [block for _, _, block in presets]

    header = """//! Generated by tools/generate-rust.py. Do not edit by hand.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::ToString;

use crate::presets::ZmanPreset;
use crate::primitive_zman::ZmanPrimitive;
use jiff::SignedDuration as Duration;

"""
    return (
        header
        + java_parity_test_macro()
        + "\n"
        + "\n".join(blocks)
        + "\n"
        + all_presets_array(const_names)
        + "\n"
    )


def main() -> None:
    docs = load_docs(INPUT)
    deprecated_methods = load_deprecated_methods(DEPRECATED_INPUT)
    OUTPUT.write_text(
        generate(docs, deprecated_methods), encoding="utf-8", newline="\n"
    )
    print(f"Wrote presets to {OUTPUT}.")


if __name__ == "__main__":
    main()
