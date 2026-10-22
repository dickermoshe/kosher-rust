//! Conversions between the exported types in [`crate::api`] and kosher-rust's.
//!
//! Every enum conversion is an exhaustive `match` in both directions, so kosher-rust gaining or
//! losing a variant is a compile error here rather than a silent gap in the bindings.

pub(crate) mod calendar;
pub(crate) mod error;
pub(crate) mod limudim;
pub(crate) mod myzmanim;
pub(crate) mod zmanim;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use jiff::{Timestamp, civil, tz::Offset};
use kosher_rust::TimeZoneLike;

use crate::api::kosher::{GregorianDate, KosherError, Location, zmanim::TimeZoneOffset};

/// A caller's [`TimeZoneOffset`] callback as a kosher-rust time zone.
///
/// kosher-rust asks a time zone for offsets without expecting failure, so a failed call answers
/// UTC and sets `failed`; [`callback_failed`] reads and clears it after each calculation.
pub(crate) struct CallbackTimeZone {
    callback: Arc<dyn TimeZoneOffset>,
    failed: AtomicBool,
}

impl CallbackTimeZone {
    pub(crate) fn new(callback: Arc<dyn TimeZoneOffset>) -> Self {
        Self {
            callback,
            failed: AtomicBool::new(false),
        }
    }
}

impl TimeZoneLike for CallbackTimeZone {
    fn offset_at(&self, timestamp: Timestamp) -> Offset {
        let offset = self
            .callback
            .offset_seconds(timestamp.as_second())
            .ok()
            .filter(|seconds| seconds.abs() < 86_400)
            .and_then(|seconds| Offset::from_seconds(seconds).ok());
        offset.unwrap_or_else(|| {
            self.failed.store(true, Ordering::Relaxed);
            Offset::UTC
        })
    }
}

/// Whether the location's callback failed since the last check, clearing the flag.
///
/// Calls on one object from several threads at once share the flag, so a failure may be reported
/// by a concurrent call instead; either way it is reported.
pub(crate) fn callback_failed(location: &kosher_rust::Location<CallbackTimeZone>) -> bool {
    location
        .timezone()
        .is_some_and(|timezone| timezone.failed.swap(false, Ordering::Relaxed))
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
