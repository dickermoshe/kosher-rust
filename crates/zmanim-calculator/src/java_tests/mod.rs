#![allow(clippy::unwrap_used)]

//! The suite has three layers:
//! - a cheap deterministic smoke pass that exercises every preset name
//! - a curated `REGRESSION_CASES` list for known historical edge cases
//! - a seeded random campaign that can promote failures into regressions
//!
//! Random campaign behavior can be tuned with:
//! - `ZMANIM_JAVA_PARITY_ITERATIONS`
//! - `ZMANIM_JAVA_PARITY_SEED`

mod cases;
mod parity;
mod random;

use rand::{rngs::StdRng, SeedableRng};

use crate::presets::ALL;

use self::{
    cases::{RegressionCase, REGRESSION_CASES},
    parity::{assert_case_matches_java, TestResult},
    random::{random_case, random_parity_iterations, random_parity_seed},
};

#[test]
fn randomized_presets_match_java_bindings() -> TestResult {
    let seed = random_parity_seed();
    let iterations = random_parity_iterations();
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        for preset in ALL {
            let base_case = random_case(&mut rng, preset.name);
            assert_case_matches_java(RegressionCase { ..base_case }, Some((seed, iteration)))?;
        }
    }
    Ok(())
}

#[test]
fn regression_cases_match_java_bindings() -> TestResult {
    for case in REGRESSION_CASES {
        assert_case_matches_java(*case, None)?;
    }
    Ok(())
}

#[test]
fn all_presets_are_invocable_against_java_for_standard_case() -> TestResult {
    // Keep one cheap deterministic smoke pass for JNI wiring and preset-name dispatch.
    for preset in ALL {
        assert_case_matches_java(
            RegressionCase {
                year: 2024,
                month: 4,
                day: 22,
                latitude: 31.778,
                longitude: 35.235,
                elevation: 754.0,
                timezone: "Asia/Jerusalem",
                preset_name: preset.name,
                ateret_torah_sunset_offset_minutes: 0,
                candle_lighting_offset_minutes: 18,
                use_astronomical_chatzos_for_other_zmanim: false,
                use_elevation: false,
            },
            None,
        )?;
    }
    Ok(())
}
