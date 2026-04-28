#![allow(clippy::unwrap_used)]

//! Tests that compare this crate with KosherJava.
//!
//! There are fixed regression cases, one smoke test for every preset, and a
//! random test that prints the seed when it fails.
//!
//! See `policy.rs` for random test limits and allowed Java/Rust differences.

mod cases;
mod jni;
mod parity;
mod policy;
mod random;

use rand::{rngs::StdRng, SeedableRng};

use crate::presets::ALL;

use self::{
    cases::{RegressionCase, REGRESSION_CASES},
    parity::{assert_case_matches_java, assert_presets_match_java, RandomRun, TestResult},
    policy::{max_latitude_for_preset, random_parity_iterations, random_parity_seed},
    random::random_case,
};

#[test]
fn randomized_presets_match_java_bindings() -> TestResult {
    let seed = random_parity_seed();
    let iterations = random_parity_iterations();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut presets_up_to_40 = Vec::new();
    let mut presets_up_to_60 = Vec::new();
    let mut presets_up_to_85 = Vec::new();

    for preset in ALL {
        let max_latitude = max_latitude_for_preset(preset.name);
        if max_latitude == 40.0 {
            presets_up_to_40.push(preset.name);
        } else if max_latitude == 60.0 {
            presets_up_to_60.push(preset.name);
        } else if max_latitude == 85.0 {
            presets_up_to_85.push(preset.name);
        } else {
            panic!("unexpected random latitude limit: {max_latitude}");
        }
    }

    for iteration in 0..iterations {
        check_random_case(&mut rng, &presets_up_to_40, 40.0, seed, iteration)?;
        check_random_case(&mut rng, &presets_up_to_60, 60.0, seed, iteration)?;
        check_random_case(&mut rng, &presets_up_to_85, 85.0, seed, iteration)?;
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
    // Quick check that every preset name can be called through JNI.
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

fn check_random_case(
    rng: &mut StdRng,
    preset_names: &[&'static str],
    max_latitude: f64,
    seed: u64,
    iteration: u64,
) -> TestResult {
    if preset_names.is_empty() {
        return Ok(());
    }

    let base_case = random_case(rng, max_latitude);
    assert_presets_match_java(base_case, preset_names, Some(RandomRun { seed, iteration }))
}
