//! Settings for Java comparison tests.
//!
//! These values keep the random tests useful without spending most failures on
//! known edge cases.

use std::{env, sync::OnceLock};

pub(crate) const DEFAULT_RANDOM_PARITY_ITERATIONS: u64 = 10_000;
pub(crate) const RANDOM_YEAR_START: i32 = 2000; // TODO: Change this to 1900
pub(crate) const RANDOM_YEAR_END: i32 = 2100;
pub(crate) const MAX_TIMEZONE_ATTEMPTS: u32 = 1_000;
pub(crate) const MAX_RANDOM_ELEVATION_METERS: f64 = 4000.0;
pub(crate) const DEFAULT_MAX_DIFF_MS: i64 = 10_000;
const SOLAR_TRANSIT_MAX_DIFF_MS: i64 = 10_000; // TODO

/// How many random cases to run for each latitude group.
pub(crate) fn test_iterations() -> u64 {
    read_env_u64(
        "ZMANIM_JAVA_PARITY_ITERATIONS",
        DEFAULT_RANDOM_PARITY_ITERATIONS,
    )
}

/// Seed for random test cases.
///
/// Set `ZMANIM_JAVA_PARITY_SEED` to replay a failure.
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

/// How far north or south random cases may go for this preset.
pub(crate) fn max_latitude_for_preset(preset_name: &str) -> f64 {
    // Some zmanim get noisy near the poles, so test them closer to normal use.
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

/// Maximum allowed Java/Rust difference for a preset.
pub(crate) fn max_diff_ms_for_preset(preset_name: &str) -> i64 {
    match preset_name {
        // Solar transit depends on the calculator's equation-of-time model. Java's
        // NOAA implementation and Rust's transit calculation can differ slightly
        // more than the general parity threshold on historical or edge cases.
        // "getChatzos" => SOLAR_TRANSIT_MAX_DIFF_MS, TODO
        _ => DEFAULT_MAX_DIFF_MS,
    }
}

/// Cases where Java and Rust are allowed to disagree about whether a zman exists.
pub(crate) fn allows_intentional_null_mismatch(preset_name: &str) -> bool {
    matches!(
        preset_name,
        // Rust will return Some(..) on any date, including dates that are not Erev Pesach.
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
/// Reads a u64 from an environment variable, or returns a default value if the variable is not set.
fn read_env_u64(var_name: &str, default: u64) -> u64 {
    match env::var(var_name) {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid {var_name} value: {value}")),
        Err(env::VarError::NotPresent) => default,
        Err(err) => panic!("failed to read {var_name}: {err}"),
    }
}
