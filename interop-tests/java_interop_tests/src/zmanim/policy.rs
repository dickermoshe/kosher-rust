//! Test policy for zmanim Java parity checks.

use std::{env, sync::OnceLock};

use kosher_rust::zmanim::AstronomicalCalculator;

pub(super) const DEFAULT_RANDOM_PARITY_ITERATIONS: u64 = 1_000;

pub(super) const MAX_TIMEZONE_ATTEMPTS: u32 = 1_000;
pub(super) const MAX_RANDOM_ELEVATION_METERS: f64 = 4000.0;

// Rust ports the same KosherJava NOAA, Meeus and SPA equations as the Java
// reference, and each random case runs both sides on the same algorithm. The
// tolerance leaves room for Java/Rust floating-point and timestamp formatting
// differences, not for a different astronomy model.
const DEFAULT_MAX_DIFF_MS: i64 = 1_000;

/// Number of randomized cases to run for each preset.
pub(super) fn test_iterations() -> u64 {
    read_env_u64("ZMANIM_JAVA_PARITY_ITERATIONS", DEFAULT_RANDOM_PARITY_ITERATIONS)
}

pub(super) fn max_diff_ms_for_preset(_preset_name: &str) -> i64 {
    DEFAULT_MAX_DIFF_MS
}

/// Whether a case where Java returns a time and Rust reports no event is accepted.
///
/// KosherJava's `SPACalculator.solveRiseSet` returns the last secant iterate even when the
/// iteration has not converged. Near the poles its closed-form first guess can report an
/// event the topocentric sun never reaches; the secant then wanders off and Java returns an
/// arbitrary time. Rust runs the same solve and reports no event when the final zenith
/// residual shows the iteration failed (see `src/astronomy/spa.rs`). The closed-form
/// rejection is the same on both sides, so for SPA this is the only way Rust can be empty
/// where Java is not.
pub(super) fn allow_java_only_result(calculator: AstronomicalCalculator) -> bool {
    matches!(calculator, AstronomicalCalculator::Spa { .. })
}

/// Seed used by randomized test cases.
///
/// Set `ZMANIM_JAVA_PARITY_SEED` to replay a failure reported by the parity harness.
pub(super) fn test_seed() -> u64 {
    static SEED: OnceLock<u64> = OnceLock::new();

    *SEED.get_or_init(|| match env::var("ZMANIM_JAVA_PARITY_SEED") {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid ZMANIM_JAVA_PARITY_SEED value: {value}")),
        Err(env::VarError::NotPresent) => rand::random::<u64>(),
        Err(err) => panic!("failed to read ZMANIM_JAVA_PARITY_SEED: {err}"),
    })
}

/// Inclusive year range generated for randomized cases for one preset.
pub(super) fn random_year_range_for_preset(preset_name: &str) -> (i32, i32) {
    match preset_name {
        // Limit the year range to avoid timezone implementation differences.
        "getSofZmanKidushLevana15Days"
        | "getSofZmanKidushLevanaBetweenMoldos"
        | "getTchilasZmanKidushLevana3Days"
        | "getTchilasZmanKidushLevana7Days"
        | "getZmanMolad" => (1990, 2030),
        _ => (1900, 2300),
    }
}

fn read_env_u64(var_name: &str, default: u64) -> u64 {
    match env::var(var_name) {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid {var_name} value: {value}")),
        Err(env::VarError::NotPresent) => default,
        Err(err) => panic!("failed to read {var_name}: {err}"),
    }
}
