//! Conversions between the exported types in [`crate::api`] and kosher-rust's.
//!
//! Every enum conversion is an exhaustive `match` in both directions, so kosher-rust gaining or
//! losing a variant is a compile error here rather than a silent gap in the bindings.

pub(crate) mod calendar;
pub(crate) mod error;
pub(crate) mod limudim;
pub(crate) mod myzmanim;
pub(crate) mod zmanim;

use std::{cell::Cell, ops::RangeInclusive, sync::Arc};

use jiff::{
    Timestamp, civil,
    tz::{Offset, TimeZone},
};
use kosher_rust::TimeZoneLike;

use crate::api::{
    calendar::GregorianDate,
    times::{Location, TimeZoneOffset, ZmanimError},
};

thread_local! {
    /// Whether a time zone callback failed during the calculation running on this thread.
    static CALLBACK_FAILED: Cell<bool> = const { Cell::new(false) };
}

/// A caller's [`TimeZoneOffset`] callback as a kosher-rust time zone. A failed call is recorded
/// for [`watching_callbacks`] to report. A failed offset answers UTC in the meantime.
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

    fn local_to_timestamp(&self, datetime: civil::DateTime) -> Option<Timestamp> {
        let as_utc = datetime.to_zoned(TimeZone::UTC).ok()?.timestamp().as_second();
        let instant = self
            .callback
            .local_to_unix_seconds(
                i32::from(datetime.year()),
                i32::from(datetime.month()),
                i32::from(datetime.day()),
                i32::from(datetime.hour()),
                i32::from(datetime.minute()),
                i32::from(datetime.second()),
            )
            .ok()
            // The instant is the local time minus an offset, and offsets are under a day.
            .filter(|seconds| seconds.abs_diff(as_utc) < 86_400)
            .and_then(|seconds| Timestamp::from_second(seconds).ok());
        if instant.is_none() {
            CALLBACK_FAILED.set(true);
        }
        instant
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

/// The kosher-rust location, if it is valid and, more than 150 degrees from Greenwich, has a time
/// zone.
pub(crate) fn location(location: &Location, has_time_zone: bool) -> Result<kosher_rust::Location, ZmanimError> {
    if !has_time_zone && location.longitude.abs() > 150.0 {
        return Err(ZmanimError::TimeZoneRequired);
    }
    Ok(kosher_rust::Location::new(location.latitude, location.longitude)?
        .with_elevation(location.elevation)?
        .with_horizons(location.eastern_horizon_elevation, location.western_horizon_elevation)?)
}

pub(crate) fn validate_location(location: &Location, has_time_zone: bool) -> Result<(), ZmanimError> {
    self::location(location, has_time_zone).map(|_| ())
}

/// The caller's time zone callback, as a kosher-rust time zone.
pub(crate) fn time_zone(time_zone: Option<Arc<dyn TimeZoneOffset>>) -> Option<CallbackTimeZone> {
    time_zone.map(CallbackTimeZone::new)
}

impl From<&kosher_rust::Location> for Location {
    fn from(location: &kosher_rust::Location) -> Self {
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
pub(crate) fn valid_minutes(minutes: f64) -> Result<f64, ZmanimError> {
    if minutes.is_finite() && minutes.abs() <= 1440.0 {
        Ok(minutes)
    } else {
        Err(ZmanimError::InvalidMinutes)
    }
}

/// `value` as kosher-rust's narrower type, if it is in `range`.
pub(crate) fn valid_number<T: TryFrom<i32>>(value: i32, range: RangeInclusive<i32>) -> Option<T> {
    range.contains(&value).then(|| T::try_from(value).ok()).flatten()
}

/// The date, if it exists.
pub(crate) fn civil_date(date: &GregorianDate) -> Option<civil::Date> {
    let year = i16::try_from(date.year).ok()?;
    let month = i8::try_from(date.month).ok()?;
    let day = i8::try_from(date.day).ok()?;
    civil::Date::new(year, month, day).ok()
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
