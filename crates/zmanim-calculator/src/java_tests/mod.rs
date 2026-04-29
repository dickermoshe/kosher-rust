#![allow(clippy::unwrap_used)]

//! Java parity tests for the Rust preset catalog.
//!
//! These tests treat KosherJava as the behavioral reference for each
//! [`ZmanPreset`].  Every preset gets:
//!
//! - a deterministic Jerusalem smoke case,
//! - any hand-curated regression cases recorded for that preset, and
//! - a seeded randomized run across dates, locations, elevations, and relevant
//!   calculator configuration flags.
//!
//! Random failures print the seed, iteration, and a ready-to-paste
//! [`TestCase`] literal.  Add that literal to [`REGRESSION_CASES`] before
//! changing implementation code so the edge case remains covered.
//!
//! Policy decisions that intentionally narrow the random input space or allow a
//! known Java/Rust behavioral difference live in [`policy`].

mod cases;
mod hebrew_dates;
mod jni;
mod parity;
mod policy;

use std::error::Error;

use rand::{rngs::StdRng, SeedableRng};

use crate::presets::ZmanPreset;

use self::{
    cases::{TestCase, REGRESSION_CASES},
    parity::{run_test_case, CaseRun},
    policy::{test_iterations, test_seed},
};

/// Runs the randomized Java parity check for one preset.
pub(crate) fn test_preset(preset: &'static ZmanPreset<'static>) -> Result<(), Box<dyn Error>> {
    let seed = test_seed();
    let iterations = test_iterations();
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let case = TestCase::random(&mut rng, preset.name);
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

#[test]
fn test_hebrew_date_parity() -> Result<(), Box<dyn Error>> {
    hebrew_dates::test_hebrew_date_parity()
}
