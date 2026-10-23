//! Conversions between the exported types in [`crate::api`] and kosher-rust's.
//!
//! Every enum conversion is an exhaustive `match` in both directions, so kosher-rust gaining or
//! losing a variant is a compile error here rather than a silent gap in the bindings.

pub(crate) mod calendar;
pub(crate) mod error;
pub(crate) mod limudim;
pub(crate) mod myzmanim;
pub(crate) mod zmanim;

use std::ops::RangeInclusive;

use jiff::{SignedDuration, Timestamp, civil, tz::Offset};
use kosher_rust::TimeZoneLike;

use crate::api::{
    calendar::GregorianDate,
    times::{LocalDay, Location, ZmanimError},
};

/// A caller's [`LocalDay`] as a kosher-rust time zone. The calculations for a date only ask when
/// its local midnight and the next one are, and the offset at each, so that is all it answers.
/// Any other local time has no answer, which the calculation reports as an internal error.
pub(crate) struct DayTimeZone {
    date: civil::Date,
    start: Timestamp,
    start_offset: Offset,
    end: Timestamp,
    end_offset: Offset,
}

impl DayTimeZone {
    /// The time zone for `date` that `day` describes, if its values are possible: the end at or
    /// after the start and at most two days later, and offsets under a day.
    pub(crate) fn new(date: civil::Date, day: &LocalDay) -> Option<Self> {
        let start = Timestamp::from_millisecond(day.start_ms).ok()?;
        let end = Timestamp::from_millisecond(day.end_ms).ok()?;
        let length = end.duration_since(start);
        if length.is_negative() || length > SignedDuration::from_hours(48) {
            return None;
        }
        let offset = |seconds: i32| {
            if seconds.unsigned_abs() < 86_400 {
                Offset::from_seconds(seconds).ok()
            } else {
                None
            }
        };
        Some(Self {
            date,
            start,
            start_offset: offset(day.start_offset_seconds)?,
            end,
            end_offset: offset(day.end_offset_seconds)?,
        })
    }
}

impl TimeZoneLike for DayTimeZone {
    fn offset_at(&self, timestamp: Timestamp) -> Offset {
        if timestamp < self.end {
            self.start_offset
        } else {
            self.end_offset
        }
    }

    fn local_to_timestamp(&self, datetime: civil::DateTime) -> Option<Timestamp> {
        if datetime == self.date.at(0, 0, 0, 0) {
            Some(self.start)
        } else if datetime == self.date.tomorrow().ok()?.at(0, 0, 0, 0) {
            Some(self.end)
        } else {
            None
        }
    }
}

/// The time zone for `date` that `day` describes, or none without a `day`.
pub(crate) fn time_zone(date: civil::Date, day: Option<&LocalDay>) -> Result<Option<DayTimeZone>, ZmanimError> {
    day.map(|day| DayTimeZone::new(date, day).ok_or(ZmanimError::InvalidLocalDay))
        .transpose()
}

/// The kosher-rust location, if it is valid and, more than 150 degrees from Greenwich, has a time
/// zone.
pub(crate) fn location(location: &Location, has_time_zone: bool) -> Result<kosher_rust::Location, ZmanimError> {
    let valid = kosher_rust::Location::new(location.latitude, location.longitude)?
        .with_elevation(location.elevation)?
        .with_horizons(location.eastern_horizon_elevation, location.western_horizon_elevation)?;
    if !has_time_zone && location.longitude.abs() > 150.0 {
        return Err(ZmanimError::TimeZoneRequired);
    }
    Ok(valid)
}

pub(crate) fn validate_location(location: &Location, has_time_zone: bool) -> Result<(), ZmanimError> {
    self::location(location, has_time_zone).map(|_| ())
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
