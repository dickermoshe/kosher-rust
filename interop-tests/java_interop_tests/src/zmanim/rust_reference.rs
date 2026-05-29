//! Rust-side reference path for zmanim Java parity tests.

use jiff::{SignedDuration as Duration, civil::Date, tz::TimeZone};

use kosher_rust::prelude::*;

use super::types::{TestCase, ZmanResult};

/// Calculates one case with the Rust implementation.
pub(super) fn calculate_rust_zman(
    case: TestCase,
    preset: &'static ZmanPreset,
) -> Result<Option<ZmanResult>, ZmanimError> {
    let timezone = TimeZone::get(case.timezone).unwrap();
    let location = Location::new(case.latitude, case.longitude, case.elevation, Some(timezone.clone()))?;
    let date = Date::new(case.year as i16, case.month as i8, case.day as i8).unwrap();
    let config = CalculatorConfig {
        candle_lighting_offset: Duration::from_mins(case.candle_lighting_offset_minutes),
        use_astronomical_chatzos_for_other_zmanim: case.use_astronomical_chatzos_for_other_zmanim,
        use_elevation: case.use_elevation,
        ateret_torah_sunset_offset: Duration::from_mins(case.ateret_torah_sunset_offset_minutes),
        use_astronomical_chatzos: case.use_astronomical_chatzos,
    };
    let mut calculator = ZmanimCalculator::new(location, date, config);

    let dt = match preset.calculate(&mut calculator) {
        Ok(dt) => dt,
        Err(_) => return Ok(None),
    };
    let formatted = dt.to_zoned(timezone).to_string();
    Ok(Some(ZmanResult {
        formatted,
        timestamp_ms: dt.as_millisecond(),
    }))
}
