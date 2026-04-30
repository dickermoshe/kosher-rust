//! Randomized input generation for zmanim Java parity tests.

use std::{collections::HashSet, str::FromStr, sync::OnceLock};

use chrono::Month;
use chrono_tz::Tz;
use rand::{rngs::StdRng, RngExt};
use tzf_rs::DefaultFinder;

use super::{java_reference::java_supported_timezones, policy, types::TestCase};

static TIMEZONE_FINDER: OnceLock<DefaultFinder> = OnceLock::new();
static SHARED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Generates one randomized input shared by the Java and Rust calculators.
pub(super) fn random_test_case(rng: &mut StdRng, preset_name: &'static str) -> TestCase {
    let max_latitude = policy::max_latitude_for_preset(preset_name);
    let (year_start, year_end) = policy::random_year_range_for_preset(preset_name);

    for _ in 0..policy::MAX_TIMEZONE_ATTEMPTS {
        let year = rng.random_range(year_start..=year_end);
        let month: u32 = rng.random_range(1..=12);
        let days_in_month = Month::try_from(month as u8)
            .unwrap()
            .num_days(year)
            .unwrap() as u32;
        let day: u32 = rng.random_range(1..=days_in_month);
        let latitude = rng.random_range(-max_latitude..=max_latitude);
        let longitude = rng.random_range(-179.999_999..=179.999_999);
        let Some(timezone) = timezone_for_coordinates(longitude, latitude) else {
            continue;
        };

        return TestCase {
            year,
            month,
            day,
            latitude,
            longitude,
            elevation: rng.random_range(0.0..=policy::MAX_RANDOM_ELEVATION_METERS),
            timezone,
            preset_name,
            ateret_torah_sunset_offset_minutes: rng.random_range(0..60),
            candle_lighting_offset_minutes: rng.random_range(0..60),
            use_astronomical_chatzos_for_other_zmanim: rng.random_bool(0.5),
            use_elevation: rng.random_bool(0.5),
        };
    }

    panic!(
        "failed to find a supported timezone after {} attempts",
        policy::MAX_TIMEZONE_ATTEMPTS
    )
}

/// Returns a shared Java/Rust timezone for the given coordinates.
fn timezone_for_coordinates(longitude: f64, latitude: f64) -> Option<&'static str> {
    let timezone_name = TIMEZONE_FINDER
        .get_or_init(DefaultFinder::new)
        .get_tz_name(longitude, latitude);
    let timezone = Tz::from_str(timezone_name).ok()?;
    let timezone_name = timezone.name();
    let shared_timezones = SHARED_TIMEZONES.get_or_init(|| {
        let java_timezones = java_supported_timezones();

        chrono_tz::TZ_VARIANTS
            .iter()
            .map(|timezone| timezone.name().to_string())
            .filter(|timezone| java_timezones.contains(timezone))
            .collect()
    });
    shared_timezones.get(timezone_name).map(String::as_str)
}
