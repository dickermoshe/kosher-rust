//! Test policy for zmanim Java parity checks.

use std::{env, sync::OnceLock};

pub(super) const DEFAULT_RANDOM_PARITY_ITERATIONS: u64 = 1_000;
pub(super) const DEFAULT_RANDOM_YEAR_START: i32 = 2000;
pub(super) const DEFAULT_RANDOM_YEAR_END: i32 = 2200;
pub(super) const MOLAD_RANDOM_YEAR_START: i32 = 1990;
pub(super) const MOLAD_RANDOM_YEAR_END: i32 = 2030;
pub(super) const MAX_TIMEZONE_ATTEMPTS: u32 = 1_000;
pub(super) const MAX_RANDOM_ELEVATION_METERS: f64 = 4000.0;

// SPA parity policy
//
// The default Java parity tests force Rust into NOAA-style sunrise/sunset
// refraction, matching KosherJava closely. The private `__test-spa-refraction`
// feature instead runs Rust with its production SPA/Bennett refraction model
// while Java remains NOAA-backed.
//
// That comparison is intentionally not exact. The model differences are most
// visible near sunrise/sunset, grow at higher latitudes where the sun crosses
// the horizon more slowly, and grow again when elevation is used. Zmanis
// alos/tzais offsets amplify the same difference because they use
// sunrise/sunset both as the anchor and as the day-length source for the
// temporal-hour offset. The 120-zmanis variants amplify it the most.
//
// Keep the time tolerances broad enough for normal SPA-vs-NOAA drift, but cap
// randomized latitude ranges for the presets where that drift dominates the
// test signal. These are test-generation limits only; they do not change the
// calculator's supported input range.
#[cfg(not(feature = "__test-spa-refraction"))]
const DEFAULT_MAX_DIFF_MS: i64 = 10_000;
#[cfg(feature = "__test-spa-refraction")]
const DEFAULT_MAX_DIFF_MS: i64 = 60_000;

const CHATZOS_HALAYLA_MAX_DIFF_MS: i64 = 20_000;

/// Number of randomized cases to run for each preset.
pub(super) fn test_iterations() -> u64 {
    read_env_u64(
        "ZMANIM_JAVA_PARITY_ITERATIONS",
        DEFAULT_RANDOM_PARITY_ITERATIONS,
    )
}

pub(super) fn max_diff_ms_for_preset(preset_name: &str) -> i64 {
    match preset_name {
        "getChatzosHalayla" => CHATZOS_HALAYLA_MAX_DIFF_MS,
        _ => DEFAULT_MAX_DIFF_MS,
    }
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
    // Most presets use the conservative 40-degree range. Chatzos can be tested
    // much farther poleward because it does not depend on a horizon crossing.
    // Sunrise/sunset-style events normally get 60 degrees, but SPA parity caps
    // them at 40 degrees because high-latitude horizon crossings magnify the
    // SPA-vs-NOAA model difference. SPA alos/tzais zmanis offsets get stricter
    // caps because they use the differing sunrise/sunset both as the anchor and
    // as the temporal-hour source.
    match preset_name {
        "getChatzos" => 85.0,
        #[cfg(feature = "__test-spa-refraction")]
        "getAlos120Zmanis" | "getTzais120Zmanis" => 25.0,
        #[cfg(feature = "__test-spa-refraction")]
        "getAlos72Zmanis" | "getAlos90Zmanis" | "getAlos96Zmanis" | "getTzais72Zmanis"
        | "getTzais90Zmanis" | "getTzais96Zmanis" => 30.0,
        "getSunriseWithElevation"
        | "getSeaLevelSunrise"
        | "getSunsetWithElevation"
        | "getSeaLevelSunset"
        | "getChatzosAsHalfDay"
        | "getFixedLocalChatzos" => {
            #[cfg(feature = "__test-spa-refraction")]
            {
                40.0
            }
            #[cfg(not(feature = "__test-spa-refraction"))]
            {
                60.0
            }
        }
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
