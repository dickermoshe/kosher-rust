//! Conversions between the exported types in [`crate::api`] and kosher-rust's.
//!
//! Every enum conversion is an exhaustive `match` in both directions, so kosher-rust gaining or
//! losing a variant is a compile error here rather than a silent gap in the bindings.

pub(crate) mod calendar;
pub(crate) mod error;
pub(crate) mod limudim;
pub(crate) mod myzmanim;
pub(crate) mod zmanim;

use std::{
    cell::Cell,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use jiff::{Timestamp, civil, tz::Offset};
use kosher_rust::TimeZoneLike;

use crate::api::kosher::{GregorianDate, KosherError, Location, zmanim::TimeZoneOffset};

thread_local! {
    /// Whether a time zone callback failed during the calculation running on this thread.
    static CALLBACK_FAILED: Cell<bool> = const { Cell::new(false) };
}

/// How many answers a [`CallbackTimeZone`] remembers.
const ANSWERS_REMEMBERED: usize = 32;

/// A caller's [`TimeZoneOffset`] callback as a kosher-rust time zone. A failed call answers UTC
/// and is recorded for [`watching_callbacks`] to report; successful answers are remembered.
pub(crate) struct CallbackTimeZone {
    callback: Arc<dyn TimeZoneOffset>,
    /// Offsets the callback gave, by the Unix second it was asked about.
    answers: Mutex<Vec<(i64, Offset)>>,
}

impl CallbackTimeZone {
    pub(crate) fn new(callback: Arc<dyn TimeZoneOffset>) -> Self {
        Self {
            callback,
            answers: Mutex::new(Vec::with_capacity(ANSWERS_REMEMBERED)),
        }
    }

    fn answers(&self) -> MutexGuard<'_, Vec<(i64, Offset)>> {
        // Nothing panics while holding the lock, so a poisoned one still holds whole answers.
        self.answers.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl TimeZoneLike for CallbackTimeZone {
    fn offset_at(&self, timestamp: Timestamp) -> Offset {
        let second = timestamp.as_second();
        if let Some(&(_, offset)) = self.answers().iter().find(|(asked, _)| *asked == second) {
            return offset;
        }
        // The lock is released: the callback may calculate something on this object itself.
        let offset = self
            .callback
            .offset_seconds(second)
            .ok()
            .filter(|seconds| seconds.unsigned_abs() < 86_400)
            .and_then(|seconds| Offset::from_seconds(seconds).ok());
        let Some(offset) = offset else {
            CALLBACK_FAILED.set(true);
            return Offset::UTC;
        };
        let mut answers = self.answers();
        if answers.len() == ANSWERS_REMEMBERED {
            answers.clear();
        }
        answers.push((second, offset));
        offset
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
