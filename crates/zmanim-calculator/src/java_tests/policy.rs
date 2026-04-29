//! Test policy for Java parity checks.
//!
//! This module is the place for choices that are not part of the calculator's
//! behavior: how broad randomized coverage should be, which inputs are too noisy
//! to generate by default, and which known Java/Rust differences are intentional.
//! Keeping those decisions here makes the comparison code read as a mechanical
//! "call both implementations and compare" flow.

use std::{env, sync::OnceLock};

pub(crate) const DEFAULT_RANDOM_PARITY_ITERATIONS: u64 = 10_000;
pub(crate) const DEFAULT_RANDOM_YEAR_START: i32 = 2000;
pub(crate) const DEFAULT_RANDOM_YEAR_END: i32 = 2200;
pub(crate) const MOLAD_RANDOM_YEAR_START: i32 = 1990;
pub(crate) const MOLAD_RANDOM_YEAR_END: i32 = 2030;
pub(crate) const MAX_TIMEZONE_ATTEMPTS: u32 = 1_000;
pub(crate) const MAX_RANDOM_ELEVATION_METERS: f64 = 4000.0;
pub(crate) const DEFAULT_MAX_DIFF_MS: i64 = 10_000;

/// Number of randomized cases to run for each preset.
pub(crate) fn test_iterations() -> u64 {
    read_env_u64(
        "ZMANIM_JAVA_PARITY_ITERATIONS",
        DEFAULT_RANDOM_PARITY_ITERATIONS,
    )
}

/// Seed used by randomized test cases.
///
/// Set `ZMANIM_JAVA_PARITY_SEED` to replay a failure reported by
/// [`crate::java_tests::parity`].
pub(crate) fn test_seed() -> u64 {
    static SEED: OnceLock<u64> = OnceLock::new();

    *SEED.get_or_init(|| match env::var("ZMANIM_JAVA_PARITY_SEED") {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid ZMANIM_JAVA_PARITY_SEED value: {value}")),
        Err(env::VarError::NotPresent) => rand::random::<u64>(),
        Err(err) => panic!("failed to read ZMANIM_JAVA_PARITY_SEED: {err}"),
    })
}

/// Maximum absolute latitude generated for randomized cases for one preset.
///
/// Many dawn/dusk-derived zmanim are undefined or numerically noisy at extreme
/// latitudes.  The limits here bias random coverage toward places where the
/// corresponding zman is normally meaningful, while still letting regression
/// cases pin specific edge behavior.
pub(crate) fn max_latitude_for_preset(preset_name: &str) -> f64 {
    match preset_name {
        "getChatzos" => 85.0,
        "getSunriseWithElevation"
        | "getSeaLevelSunrise"
        | "getSunsetWithElevation"
        | "getSeaLevelSunset"
        | "getChatzosAsHalfDay"
        | "getFixedLocalChatzos" => 60.0,
        _ => 40.0,
    }
}

/// Inclusive year range generated for randomized cases for one preset.
///
/// Molad-derived zmanim are sensitive to local-day boundaries.  Keeping them in
/// a modern range avoids false Java/Rust parity failures caused by diverging
/// far-future timezone rules rather than calculator logic.
pub(crate) fn random_year_range_for_preset(preset_name: &str) -> (i32, i32) {
    match preset_name {
        "getSofZmanKidushLevana15Days"
        | "getSofZmanKidushLevanaBetweenMoldos"
        | "getTchilasZmanKidushLevana3Days"
        | "getTchilasZmanKidushLevana7Days"
        | "getZmanMolad" => (MOLAD_RANDOM_YEAR_START, MOLAD_RANDOM_YEAR_END),
        _ => (DEFAULT_RANDOM_YEAR_START, DEFAULT_RANDOM_YEAR_END),
    }
}

/// Maximum tolerated Java/Rust timestamp difference for one preset.
pub(crate) fn max_diff_ms_for_preset(preset_name: &str) -> i64 {
    match preset_name {
        _ => DEFAULT_MAX_DIFF_MS,
    }
}

/// Returns true when Java and Rust intentionally disagree about nullability.
pub(crate) fn allows_intentional_null_mismatch(preset_name: &str) -> bool {
    matches!(
        preset_name,
        // Rust exposes the raw Chametz calculation for any date, including
        // dates that are not Erev Pesach; KosherJava returns null outside the
        // holiday-specific context.
        "getSofZmanAchilasChametzGRA"
            | "getSofZmanAchilasChametzMGA72Minutes"
            | "getSofZmanAchilasChametzMGA16Point1Degrees"
            | "getSofZmanAchilasChametzBaalHatanya"
            | "getSofZmanBiurChametzGRA"
            | "getSofZmanBiurChametzMGA72Minutes"
            | "getSofZmanBiurChametzMGA16Point1Degrees"
            | "getSofZmanBiurChametzBaalHatanya"
    )
}

/// Reads an unsigned integer from the environment, or returns the default.
pub(crate) fn read_env_u64(var_name: &str, default: u64) -> u64 {
    match env::var(var_name) {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid {var_name} value: {value}")),
        Err(env::VarError::NotPresent) => default,
        Err(err) => panic!("failed to read {var_name}: {err}"),
    }
}
