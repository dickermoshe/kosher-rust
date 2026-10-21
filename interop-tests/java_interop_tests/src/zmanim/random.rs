//! Randomized input generation for zmanim Java parity tests.

use std::{collections::HashSet, sync::OnceLock};

use jiff::{civil::Date, tz::TimeZone};
use rand::{RngExt, rngs::StdRng};
use tzf_rs::DefaultFinder;

use kosher_rust::prelude::*;

use super::{java_reference::java_supported_timezones, policy, types::TestCase};

static TIMEZONE_FINDER: OnceLock<DefaultFinder> = OnceLock::new();
static SHARED_TIMEZONES: OnceLock<HashSet<String>> = OnceLock::new();

/// Generates one randomized input shared by the Java and Rust calculators.
pub(super) fn random_test_case(rng: &mut StdRng, preset_name: &'static str) -> TestCase {
    let max_latitude = 90.0;
    let (year_start, year_end) = policy::random_year_range_for_preset(preset_name);

    for _ in 0..policy::MAX_TIMEZONE_ATTEMPTS {
        let year = rng.random_range(year_start..=year_end);
        let month: u32 = rng.random_range(1..=12);
        let days_in_month = Date::new(year as i16, month as i8, 1).unwrap().days_in_month() as u32;
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
            eastern_horizon_elevation: random_horizon_elevation(rng),
            western_horizon_elevation: random_horizon_elevation(rng),
            timezone,
            preset_name,
            ateret_torah_sunset_offset_minutes: rng.random_range(0..60),
            candle_lighting_offset_minutes: rng.random_range(0..60),
            use_astronomical_chatzos_for_other_zmanim: rng.random_bool(0.5),
            use_elevation: rng.random_bool(0.5),
            use_astronomical_chatzos: rng.random_bool(0.5),
            calculator: random_calculator(rng),
        };
    }

    panic!(
        "failed to find a supported timezone after {} attempts",
        policy::MAX_TIMEZONE_ATTEMPTS
    )
}

/// Picks one of the three KosherJava solar algorithms with equal probability, and a random
/// ΔT setting for the two that support it.
fn random_calculator(rng: &mut StdRng) -> AstronomicalCalculator {
    match rng.random_range(0..3) {
        0 => AstronomicalCalculator::Noaa,
        1 => AstronomicalCalculator::Meeus {
            apply_delta_t: rng.random_bool(0.5),
        },
        _ => AstronomicalCalculator::Spa {
            apply_delta_t: rng.random_bool(0.5),
        },
    }
}

/// Picks a horizon elevation: sea level half the time, otherwise anywhere up to the
/// maximum random observer elevation so horizons above the observer are exercised too.
fn random_horizon_elevation(rng: &mut StdRng) -> f64 {
    if rng.random_bool(0.5) {
        0.0
    } else {
        rng.random_range(0.0..=policy::MAX_RANDOM_ELEVATION_METERS)
    }
}

/// Returns a shared Java/Rust timezone for the given coordinates.
fn timezone_for_coordinates(longitude: f64, latitude: f64) -> Option<&'static str> {
    let timezone_name = TIMEZONE_FINDER
        .get_or_init(DefaultFinder::new)
        .get_tz_name(longitude, latitude);
    let timezone = TimeZone::get(timezone_name).ok()?;
    let timezone_name = timezone.iana_name()?;
    let shared_timezones = SHARED_TIMEZONES.get_or_init(|| {
        let java_timezones = java_supported_timezones();

        java_timezones
            .iter()
            .filter(|timezone| TimeZone::get(timezone).is_ok())
            .cloned()
            .collect()
    });
    shared_timezones.get(timezone_name).map(String::as_str)
}
