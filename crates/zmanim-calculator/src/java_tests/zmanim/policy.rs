//! Test policy for zmanim Java parity checks.

use std::{env, sync::OnceLock};

pub(super) const DEFAULT_RANDOM_PARITY_ITERATIONS: u64 = 10_000;
pub(super) const DEFAULT_RANDOM_YEAR_START: i32 = 2000;
pub(super) const DEFAULT_RANDOM_YEAR_END: i32 = 2200;
pub(super) const MOLAD_RANDOM_YEAR_START: i32 = 1990;
pub(super) const MOLAD_RANDOM_YEAR_END: i32 = 2030;
pub(super) const MAX_TIMEZONE_ATTEMPTS: u32 = 1_000;
pub(super) const MAX_RANDOM_ELEVATION_METERS: f64 = 4000.0;
pub(super) const DEFAULT_MAX_DIFF_MS: i64 = 10_000;
pub(super) const CHATZOS_HALAYLA_MAX_DIFF_MS: i64 = 20_000;

/// Number of randomized cases to run for each preset.
pub(super) fn test_iterations() -> u64 {
    read_env_u64(
        "ZMANIM_JAVA_PARITY_ITERATIONS",
        DEFAULT_RANDOM_PARITY_ITERATIONS,
    )
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

/// Maximum absolute latitude generated for randomized cases for one preset.
pub(super) fn max_latitude_for_preset(preset_name: &str) -> f64 {
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
pub(super) fn random_year_range_for_preset(preset_name: &str) -> (i32, i32) {
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
pub(super) fn max_diff_ms_for_preset(preset_name: &str) -> i64 {
    match preset_name {
        "getChatzosHalayla" => CHATZOS_HALAYLA_MAX_DIFF_MS,
        _ => DEFAULT_MAX_DIFF_MS,
    }
}

/// Returns true when Java and Rust intentionally disagree about nullability.
pub(super) fn allows_intentional_null_mismatch(preset_name: &str) -> bool {
    matches!(
        preset_name,
        // Rust exposes the raw Chametz calculation for any date, including
        // dates that are not Erev Pesach; KosherJava returns null outside the
        // holiday-specific context.
        "getSofZmanAchilasChametzGRA"
            | "getSofZmanAchilasChametzMGA72Minutes"
            | "getSofZmanAchilasChametzMGA72MinutesZmanis"
            | "getSofZmanAchilasChametzMGA16Point1Degrees"
            | "getSofZmanAchilasChametzBaalHatanya"
            | "getSofZmanBiurChametzGRA"
            | "getSofZmanBiurChametzMGA72Minutes"
            | "getSofZmanBiurChametzMGA72MinutesZmanis"
            | "getSofZmanBiurChametzMGA16Point1Degrees"
            | "getSofZmanBiurChametzBaalHatanya"
    )
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
