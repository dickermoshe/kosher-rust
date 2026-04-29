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

use std::error::Error;

use rand::{rngs::StdRng, SeedableRng};

use crate::presets::ZmanPreset;

use self::{
    cases::{TestCase, REGRESSION_CASES},
    parity::{run_test_case, CaseRun},
    policy::{max_latitude_for_preset, test_iterations, test_seed},
};

/// Runs the randomized Java parity check for one preset.
pub(crate) fn test_preset(preset: &'static ZmanPreset<'static>) -> Result<(), Box<dyn Error>> {
    let seed = test_seed();
    let iterations = test_iterations();
    let mut rng = StdRng::seed_from_u64(seed);
    let max_latitude = max_latitude_for_preset(preset.name);

    for iteration in 0..iterations {
        let case = TestCase::random(&mut rng, max_latitude);
        run_test_case(case, preset, Some(CaseRun { seed, iteration }))?;
    }

    Ok(())
}

/// Runs the fixed regression cases that belong to one preset.
pub(crate) fn test_regressions(preset: &'static ZmanPreset<'static>) -> Result<(), Box<dyn Error>> {
    for case in REGRESSION_CASES
        .iter()
        .copied()
        .filter(|case| case.preset_name == preset.name)
    {
        run_test_case(case, preset, None)?;
    }
    Ok(())
}

/// Checks one preset against Java for a standard Jerusalem case.
pub(crate) fn test_preset_in_jerusalem(
    preset: &'static ZmanPreset<'static>,
) -> Result<(), Box<dyn Error>> {
    run_test_case(
        TestCase {
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
        preset,
        None,
    )
}
