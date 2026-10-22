//! Conversions between the exported types in [`crate::api`] and kosher-rust's.
//!
//! Every enum conversion is an exhaustive `match` in both directions, so kosher-rust gaining or
//! losing a variant is a compile error here rather than a silent gap in the bindings.

pub(crate) mod calendar;
pub(crate) mod error;
pub(crate) mod limudim;
pub(crate) mod myzmanim;
pub(crate) mod zmanim;

use std::{cell::Cell, sync::Arc};

use jiff::{Timestamp, civil, tz::Offset};
use kosher_rust::TimeZoneLike;

use crate::api::kosher::{GregorianDate, KosherError, Location, zmanim::TimeZoneOffset};

thread_local! {
    /// Whether a time zone callback failed during the calculation running on this thread.
    static CALLBACK_FAILED: Cell<bool> = const { Cell::new(false) };
}

/// A caller's [`TimeZoneOffset`] callback as a kosher-rust time zone. A failed call answers UTC
/// and is recorded for [`watching_callbacks`] to report.
pub(crate) struct CallbackTimeZone {
    callback: Arc<dyn TimeZoneOffset>,
}

impl CallbackTimeZone {
    pub(crate) fn new(callback: Arc<dyn TimeZoneOffset>) -> Self {
        Self { callback }
    }
}

impl TimeZoneLike for CallbackTimeZone {
    fn offset_at(&self, timestamp: Timestamp) -> Offset {
        let offset = self
            .callback
            .offset_seconds(timestamp.as_second())
            .ok()
            .filter(|seconds| seconds.unsigned_abs() < 86_400)
            .and_then(|seconds| Offset::from_seconds(seconds).ok());
        offset.unwrap_or_else(|| {
            CALLBACK_FAILED.set(true);
            Offset::UTC
        })
    }
}

/// Runs `calculate`, and says whether a time zone callback failed during it. The record is per
/// thread, and saved and restored around `calculate`, so concurrent and nested calculations each
/// see only their own failures.
pub(crate) fn watching_callbacks<T>(calculate: impl FnOnce() -> T) -> (T, bool) {
    let outer = CALLBACK_FAILED.replace(false);
    let result = calculate();
    (result, CALLBACK_FAILED.replace(outer))
}

/// The kosher-rust location, with the caller's time zone.
pub(crate) fn location(
    location: &Location,
    time_zone: Option<Arc<dyn TimeZoneOffset>>,
) -> Result<kosher_rust::Location<CallbackTimeZone>, KosherError> {
    let time_zone = time_zone.map(CallbackTimeZone::new);
    Ok(
        kosher_rust::Location::new_with_timezone(location.latitude, location.longitude, time_zone)?
            .with_elevation(location.elevation)?
            .with_horizons(location.eastern_horizon_elevation, location.western_horizon_elevation)?,
    )
}

pub(crate) fn validate_location(location: &Location, has_time_zone: bool) -> Result<(), KosherError> {
    let time_zone = has_time_zone.then_some(jiff::tz::TimeZone::UTC);
    kosher_rust::Location::new(location.latitude, location.longitude, time_zone)?
        .with_elevation(location.elevation)?
        .with_horizons(location.eastern_horizon_elevation, location.western_horizon_elevation)?;
    Ok(())
}

impl<Tz> From<&kosher_rust::Location<Tz>> for Location {
    fn from(location: &kosher_rust::Location<Tz>) -> Self {
        Self {
            latitude: location.latitude(),
            longitude: location.longitude(),
            elevation: location.elevation(),
            eastern_horizon_elevation: location.eastern_horizon_elevation(),
            western_horizon_elevation: location.western_horizon_elevation(),
        }
    }
}

/// `minutes`, if it is a finite number within a day either way.
pub(crate) fn valid_minutes(minutes: f64) -> Result<f64, KosherError> {
    if minutes.is_finite() && minutes.abs() <= 1440.0 {
        Ok(minutes)
    } else {
        Err(KosherError::InvalidMinutes)
    }
}

pub(crate) fn civil_date(date: &GregorianDate) -> Result<civil::Date, KosherError> {
    let year = i16::try_from(date.year).map_err(|_| KosherError::InvalidDate)?;
    let month = i8::try_from(date.month).map_err(|_| KosherError::InvalidDate)?;
    let day = i8::try_from(date.day).map_err(|_| KosherError::InvalidDate)?;
    civil::Date::new(year, month, day).map_err(|_| KosherError::InvalidDate)
}

impl From<civil::Date> for GregorianDate {
    fn from(date: civil::Date) -> Self {
        Self {
            year: i32::from(date.year()),
            month: i32::from(date.month()),
            day: i32::from(date.day()),
        }
    }
}
