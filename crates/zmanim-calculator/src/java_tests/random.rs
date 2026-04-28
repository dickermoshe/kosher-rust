//! Randomized parity campaign configuration and input generation.

use std::{collections::HashSet, env, str::FromStr, sync::OnceLock};

use chrono::NaiveDate;
use chrono_tz::Tz;
use rand::{rngs::StdRng, RngExt};
use tzf_rs::DefaultFinder;

use super::{cases::RegressionCase, parity::java_supported_timezones};

const DEFAULT_RANDOM_PARITY_ITERATIONS: u64 = 10_000;
const RANDOM_YEAR_START: i32 = 1900;
const RANDOM_YEAR_END: i32 = 2100;
const MAX_TIMEZONE_ATTEMPTS: u32 = 1_000;
const MAX_RANDOM_ELEVATION_METERS: f64 = 4_000.0;

static TIMEZONE_FINDER: OnceLock<DefaultFinder> = OnceLock::new();
static SHARED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Returns the number of randomized base cases to generate before fanning out
/// across every preset.
pub(crate) fn random_parity_iterations() -> u64 {
    read_env_u64(
        "ZMANIM_JAVA_PARITY_ITERATIONS",
        DEFAULT_RANDOM_PARITY_ITERATIONS,
    )
}

/// Returns the seed for the randomized parity campaign.
///
/// When unset, the suite uses fresh randomness and reports the chosen seed in
/// any failure output so the exact run can be replayed locally or in CI.
pub(crate) fn random_parity_seed() -> u64 {
    match env::var("ZMANIM_JAVA_PARITY_SEED") {
        Ok(value) => value
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid ZMANIM_JAVA_PARITY_SEED value: {value}")),
        // Default to fresh randomness; failures print the seed for replay.
        Err(env::VarError::NotPresent) => rand::random::<u64>(),
        Err(err) => panic!("failed to read ZMANIM_JAVA_PARITY_SEED: {err}"),
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

pub(crate) fn random_case(rng: &mut StdRng, preset_name: &'static str) -> RegressionCase {
    let max_latitude = max_latitude_for_preset(preset_name);

    for _ in 0..MAX_TIMEZONE_ATTEMPTS {
        let year = rng.random_range(RANDOM_YEAR_START..=RANDOM_YEAR_END);
        let month = rng.random_range(1..=12);
        let day = rng.random_range(1..=days_in_month(year, month));
        let latitude = rng.random_range(-max_latitude..=max_latitude);
        let longitude = rng.random_range(-179.999_999..=179.999_999);
        let Some(timezone) = timezone_for_coordinates(longitude, latitude) else {
            continue;
        };

        return RegressionCase {
            year,
            month,
            day,
            latitude,
            longitude,
            elevation: rng.random_range(0.0..=MAX_RANDOM_ELEVATION_METERS),
            timezone,
            preset_name,
            ateret_torah_sunset_offset_minutes: rng.random_range(0..60),
            candle_lighting_offset_minutes: rng.random_range(0..60),
            use_astronomical_chatzos_for_other_zmanim: rng.random_bool(0.5),
            use_elevation: rng.random_bool(0.5),
        };
    }

    panic!("failed to find a supported timezone after {MAX_TIMEZONE_ATTEMPTS} attempts")
}

fn timezone_for_coordinates(longitude: f64, latitude: f64) -> Option<&'static str> {
    let timezone_name = timezone_finder().get_tz_name(longitude, latitude);
    let timezone = Tz::from_str(timezone_name).ok()?;
    let timezone_name = timezone.name();
    shared_timezones()
        .contains(timezone_name)
        .then_some(timezone_name)
}

fn timezone_finder() -> &'static DefaultFinder {
    TIMEZONE_FINDER.get_or_init(DefaultFinder::new)
}

fn max_latitude_for_preset(preset_name: &str) -> f64 {
    // The Dart harness narrowed the latitude range for edge-sensitive zmanim so
    // random failures stay focused on parity issues instead of polar-boundary
    // noise where tiny astronomical differences can produce large deltas.
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

fn shared_timezones() -> &'static HashSet<String> {
    SHARED_TIMEZONES.get_or_init(|| {
        let java_timezones = java_supported_timezones();
        chrono_tz::TZ_VARIANTS
            .iter()
            .map(|timezone| timezone.name().to_string())
            .filter(|timezone| java_timezones.contains(timezone))
            .collect()
    })
}

fn days_in_month(year: i32, month: u32) -> u32 {
    for day in (28..=31).rev() {
        if NaiveDate::from_ymd_opt(year, month, day).is_some() {
            return day;
        }
    }
    unreachable!("month/day calculation should always find a valid day")
}
