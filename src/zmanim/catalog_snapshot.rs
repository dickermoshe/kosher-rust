// Pins every preset's definition so the 0.3 catalog rewrite can be checked byte for byte.
//
// The snapshot records, per preset: KosherJava name, display name, category, deprecated flag,
// the formula tree (Debug, which prints variant names only, so renaming the enum is free), and
// the alloc description under the default config plus a config with every option flipped.
//
// Run with `UPDATE_SNAPSHOT=1 cargo test catalog_snapshot` to rewrite the file after an
// intended change, then review the diff.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

extern crate std;

use std::{env, fmt::Write as _, fs, path::PathBuf, string::String, vec::Vec};

use jiff::{SignedDuration, civil::Date, tz::TimeZone};

use crate::{
    Location,
    zmanim::{AstronomicalCalculator, CalculatorConfig, ZmanimCalculator, presets::ALL_ZMANIM},
};

const SNAPSHOT_PATH: &str = "tests/fixtures/zman_catalog.snap";

fn calculator(config: CalculatorConfig) -> ZmanimCalculator {
    let location = Location::new(0.0, 0.0, Some(TimeZone::UTC)).unwrap();
    ZmanimCalculator::new(location, Date::new(2024, 1, 1).unwrap(), config)
}

fn flipped_config() -> CalculatorConfig {
    let default = CalculatorConfig::default();
    CalculatorConfig {
        candle_lighting_offset: SignedDuration::from_mins(40),
        use_astronomical_chatzos_for_other_zmanim: !default.use_astronomical_chatzos_for_other_zmanim,
        use_elevation: !default.use_elevation,
        ateret_torah_sunset_offset: SignedDuration::from_mins(25),
        use_astronomical_chatzos: !default.use_astronomical_chatzos,
        astronomical_calculator: AstronomicalCalculator::Meeus { apply_delta_t: true },
    }
}

fn render() -> String {
    let default_calc = calculator(CalculatorConfig::default());
    let flipped_calc = calculator(flipped_config());

    let mut presets: Vec<_> = ALL_ZMANIM.iter().collect();
    presets.sort_by_key(|preset| preset.method_name);

    let mut out = String::new();
    for preset in presets {
        let description = preset.description(&default_calc);
        let flipped_description = preset.description(&flipped_calc);

        writeln!(out, "{}", preset.method_name).unwrap();
        writeln!(out, "  name: {}", preset.name).unwrap();
        writeln!(out, "  category: {:?}", preset.zman_type).unwrap();
        writeln!(out, "  deprecated: {}", preset.deprecated).unwrap();
        writeln!(out, "  formula: {:?}", preset.event).unwrap();
        writeln!(out, "  description: {description:?}").unwrap();
        if flipped_description != description {
            writeln!(out, "  description (flipped config): {flipped_description:?}").unwrap();
        }
        writeln!(out).unwrap();
    }
    out
}

#[test]
fn catalog_snapshot() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT_PATH);
    let actual = render();

    if env::var_os("UPDATE_SNAPSHOT").is_some() {
        fs::write(&path, &actual).unwrap();
        return;
    }

    let expected = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("{SNAPSHOT_PATH} is missing; run with UPDATE_SNAPSHOT=1 to create it"));
    // Normalise line endings so a Windows checkout with autocrlf still matches.
    let expected = expected.replace("\r\n", "\n");

    if actual != expected {
        let first_diff = actual
            .lines()
            .zip(expected.lines())
            .position(|(a, e)| a != e)
            .unwrap_or_else(|| actual.lines().count().min(expected.lines().count()));
        panic!(
            "zman catalog changed (first difference at line {}).\n  expected: {:?}\n  actual:   {:?}\n\
             If the change is intended, rerun with UPDATE_SNAPSHOT=1 and review the diff.",
            first_diff + 1,
            expected.lines().nth(first_diff),
            actual.lines().nth(first_diff),
        );
    }
}
