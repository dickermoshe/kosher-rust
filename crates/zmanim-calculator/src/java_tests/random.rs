//! Build random test cases.

use std::{collections::HashSet, str::FromStr, sync::OnceLock};

use chrono::NaiveDate;
use chrono_tz::Tz;
use rand::{rngs::StdRng, RngExt};
use tzf_rs::DefaultFinder;

use super::{cases::RegressionCase, jni::java_supported_timezones, policy};

static TIMEZONE_FINDER: OnceLock<DefaultFinder> = OnceLock::new();
static SHARED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Builds one random case whose timezone works in both Java and Rust.
///
/// The caller fills in the preset name later.
pub(crate) fn random_case(rng: &mut StdRng, max_latitude: f64) -> RegressionCase {
    for _ in 0..policy::MAX_TIMEZONE_ATTEMPTS {
        let year = rng.random_range(policy::RANDOM_YEAR_START..=policy::RANDOM_YEAR_END);
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
            elevation: rng.random_range(0.0..=policy::MAX_RANDOM_ELEVATION_METERS),
            timezone,
            preset_name: "",
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

/// Returns a timezone only if Java and Rust both know it.
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
