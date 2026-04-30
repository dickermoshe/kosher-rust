//! Java parity tests for the Rust preset catalog.

mod assertions;
mod fixtures;
pub(crate) mod java_reference;
mod policy;
mod random;
mod rust_reference;
mod types;

use std::error::Error;

use rand::{rngs::StdRng, SeedableRng};

use crate::presets::ZmanPreset;

use self::{
    assertions::run_test_case,
    fixtures::REGRESSION_CASES,
    random::random_test_case,
    types::{CaseRun, TestCase},
};

/// Runs the randomized Java parity check for one preset.
pub(crate) fn test_preset(preset: &'static ZmanPreset<'static>) -> Result<(), Box<dyn Error>> {
    let seed = policy::test_seed();
    let iterations = policy::test_iterations();
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let case = random_test_case(&mut rng, preset.name);
        run_test_case(case, preset, Some(CaseRun { seed, iteration }))?;
    }

    Ok(())
}

/// Runs the fixed regression cases that belong to one preset.
#[cfg(test)]
pub(crate) fn test_regressions(preset: &'static ZmanPreset<'static>) {
    for case in REGRESSION_CASES
        .iter()
        .copied()
        .filter(|case| case.preset_name == preset.name)
    {
        run_test_case(case, preset, None).unwrap();
    }
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
