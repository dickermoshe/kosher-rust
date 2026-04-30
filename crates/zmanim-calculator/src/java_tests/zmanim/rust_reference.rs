//! Rust-side reference path for zmanim Java parity tests.

use std::str::FromStr;

use chrono::{Duration, NaiveDate};
use chrono_tz::Tz;

use crate::{
    calculator::ZmanLike,
    prelude::ZmanimError,
    presets::ZmanPreset,
    types::{config::CalculatorConfig, location::Location},
};

use super::types::{TestCase, ZmanResult};

/// Calculates one case with the Rust implementation.
pub(super) fn calculate_rust_zman(
    case: TestCase,
    preset: &'static ZmanPreset<'static>,
) -> Result<Option<ZmanResult>, ZmanimError> {
    let timezone = Tz::from_str(case.timezone).unwrap();
    let location = Location::new(
        case.latitude,
        case.longitude,
        case.elevation,
        Some(timezone),
    )?;
    let date = NaiveDate::from_ymd_opt(case.year, case.month, case.day).unwrap();
    let config = CalculatorConfig {
        candle_lighting_offset: Duration::minutes(case.candle_lighting_offset_minutes),
        use_astronomical_chatzos_for_other_zmanim: case.use_astronomical_chatzos_for_other_zmanim,
        use_elevation: case.use_elevation,
        ateret_torah_sunset_offset: Duration::minutes(case.ateret_torah_sunset_offset_minutes),
    };
    let mut calculator = crate::calculator::ZmanimCalculator::new(location, date, config)?;

    let dt = match preset.calculate(&mut calculator) {
        Ok(dt) => dt,
        Err(_) => return Ok(None),
    };
    let formatted = dt.with_timezone(&timezone).to_rfc3339();
    Ok(Some(ZmanResult {
        formatted,
        timestamp_ms: dt.timestamp_millis(),
    }))
}
