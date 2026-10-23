//! Rust-side reference path for zmanim Java parity tests.

use jiff::{SignedDuration as Duration, civil::Date, tz::TimeZone};

use kosher_rust::{Location, zmanim::*};

use super::types::{TestCase, ZmanResult};

/// Calculates one case with the Rust implementation.
pub(super) fn calculate_rust_zman(case: TestCase, preset: Zman) -> Result<Option<ZmanResult>, ZmanimError> {
    let timezone = TimeZone::get(case.timezone).unwrap();
    let location = Location::new(case.latitude, case.longitude)
        .and_then(|l| l.with_elevation(case.elevation))
        .and_then(|l| l.with_horizons(case.eastern_horizon_elevation, case.western_horizon_elevation))
        .expect("generated test locations are valid");
    let date = Date::new(case.year as i16, case.month as i8, case.day as i8).unwrap();
    let config = Config {
        candle_lighting_offset: std::time::Duration::from_secs(case.candle_lighting_offset_minutes.unsigned_abs() * 60),
        use_astronomical_chatzos_for_other_zmanim: case.use_astronomical_chatzos_for_other_zmanim,
        use_elevation: case.use_elevation,
        ateret_torah_sunset_offset: Duration::from_mins(case.ateret_torah_sunset_offset_minutes),
        use_astronomical_chatzos: case.use_astronomical_chatzos,
        astronomical_calculator: case.calculator,
    };
    let zmanim = Zmanim::with_config(location, date, Some(timezone.clone()), config);

    let dt = match zmanim.get(preset) {
        Ok(dt) => dt,
        Err(_) => return Ok(None),
    };
    let formatted = dt.to_zoned(timezone).to_string();
    Ok(Some(ZmanResult {
        formatted,
        timestamp_ms: dt.as_millisecond(),
    }))
}
