use core::time::Duration;

use jiff::{SignedDuration, Timestamp, Zoned, civil::Date, tz::TimeZone};

use super::{Config, Zman, ZmanimError, formula::Formula};
use crate::{Location, Zone};

/// The *zmanim* for one place on one day.
///
/// Create one with [`Zmanim::new`]. Then call [`get`](Self::get) for any [`Zman`]. It gives the
/// time as a [`Timestamp`]. To show the time to someone call [`get_zoned`](Self::get_zoned)
/// instead. It gives the time in the time zone of these *zmanim*.
///
/// ```
/// use jiff::{civil::date, tz::TimeZone};
/// use kosher_rust::{Location, Zman, Zmanim};
///
/// let tz = TimeZone::get("America/New_York")?;
/// let lakewood = Location::new(40.09, -74.22)?;
/// let zmanim = Zmanim::new(lakewood, date(2024, 4, 23), Some(tz));
///
/// let shma = zmanim.get_zoned(Zman::SofZmanShmaGra)?;
/// assert_eq!(shma.strftime("%H:%M").to_string(), "09:31");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Time zones
///
/// When calculating *zmanim*, pass a time zone to [`Zmanim::new`]. If you pass `None`, you will
/// get [`ZmanimError::TimeZoneRequired`] when you call [`get_zoned`](Self::get_zoned), for any
/// *zman* if the location is more than 150° east or west of Greenwich, and when you calculate one
/// of the four Kiddush Levana *zmanim* ([`Zman::TchilasZmanKidushLevana3Days`],
/// [`Zman::TchilasZmanKidushLevana7Days`], [`Zman::SofZmanKidushLevanaBetweenMoldos`] or
/// [`Zman::SofZmanKidushLevana15Days`]).
///
/// The time zone is a [`jiff::tz::TimeZone`] unless you build the *zmanim* with
/// [`with_zone`](Self::with_zone) and a [`Zone`] of your own.
#[derive(Clone, Debug, PartialEq)]
pub struct Zmanim<Z = TimeZone> {
    pub(crate) location: Location,
    pub(crate) date: Date,
    pub(crate) config: Config,
    pub(crate) timezone: Option<Z>,
}

impl Zmanim {
    /// The *zmanim* at `location` on `date` in `timezone`, with the default [`Config`].
    ///
    /// Pass a time zone if you have one. Every time works with a time zone. Without one a few
    /// times do not work. The section on [time zones](Zmanim#time-zones) lists them.
    pub fn new(location: Location, date: Date, timezone: Option<TimeZone>) -> Self {
        Self::with_config(location, date, timezone, Config::default())
    }

    /// The *zmanim* at `location` on `date` in `timezone`, calculated with `config`. The config
    /// chooses the solar algorithm, whether to use elevation, how to find *chatzos*, and the
    /// candle-lighting offset.
    ///
    /// Pass a time zone if you have one. Every time works with a time zone. Without one a few
    /// times do not work. The section on [time zones](Zmanim#time-zones) lists them.
    pub fn with_config(location: Location, date: Date, timezone: Option<TimeZone>, config: Config) -> Self {
        Self::with_zone(location, date, timezone, config)
    }

    /// The time of `zman` in the time zone of these *zmanim*.
    ///
    /// # Errors
    ///
    /// It returns the same errors as [`get`](Self::get). It also returns
    /// [`ZmanimError::TimeZoneRequired`] when the *zmanim* have no time zone.
    pub fn get_zoned(&self, zman: Zman) -> Result<Zoned, ZmanimError> {
        let timezone = self.timezone.as_ref().ok_or(ZmanimError::TimeZoneRequired)?;
        Ok(self.get(zman)?.to_zoned(timezone.clone()))
    }
}

impl<Z: Zone> Zmanim<Z> {
    /// The *zmanim* at `location` on `date` in `zone`, any [`Zone`], calculated with `config`.
    ///
    /// Pass a time zone if you have one. Every time works with a time zone. Without one a few
    /// times do not work. The section on [time zones](Zmanim#time-zones) lists them.
    pub fn with_zone(location: Location, date: Date, zone: Option<Z>, config: Config) -> Self {
        Self {
            location,
            date,
            config,
            timezone: zone,
        }
    }

    /// The time of `zman` on this day.
    ///
    /// # Errors
    ///
    /// - [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] where the sun never reaches the
    ///   angle the time needs, as in the far north or south.
    /// - [`ZmanimError::NotApplicable`] for a time that only exists on some days, such as the
    ///   latest time to eat chametz outside Erev Pesach.
    /// - [`ZmanimError::DateOutOfRange`] for the chametz and Kiddush Levana times on a date after
    ///   29 Elul 9999 or before 1 January of year 1, where there is no Hebrew date.
    /// - [`ZmanimError::TimeZoneRequired`] when the *zmanim* have no time zone and the time
    ///   needs one. The section on [time zones](Zmanim#time-zones) lists those times.
    /// - [`ZmanimError::InvalidOffset`] for candle lighting and the Ateret Torah times when their
    ///   offset in the [`Config`] is a day or more.
    /// - [`ZmanimError::TimeOutOfRange`] on a date within a day or two of the years -9999 and
    ///   9999, when the time falls past the end of the range.
    pub fn get(&self, zman: Zman) -> Result<Timestamp, ZmanimError> {
        zman.formula().evaluate(self)
    }

    /// When the sun is `degrees` below the eastern horizon before sunrise, for an opinion the
    /// catalog doesn't have. The angle is measured from the geometric horizon, without elevation.
    /// A negative angle is above the horizon. At exactly 0 the result is sea-level sunrise, which
    /// also allows for refraction and the size of the sun.
    ///
    /// # Errors
    ///
    /// - [`ZmanimError::InvalidDegrees`] when `degrees` is not a number from -90 to 90.
    /// - [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] when the sun doesn't reach that
    ///   angle on this day.
    pub fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimError> {
        Formula::SunriseOffsetByDegrees(valid_degrees(degrees)?).evaluate(self)
    }

    /// When the sun is `degrees` below the western horizon after sunset, for example 8.5 for a
    /// custom *tzais*. The angle is measured from the geometric horizon, without elevation. A
    /// negative angle is above the horizon. At exactly 0 the result is sea-level sunset, which
    /// also allows for refraction and the size of the sun.
    ///
    /// # Errors
    ///
    /// - [`ZmanimError::InvalidDegrees`] when `degrees` is not a number from -90 to 90.
    /// - [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] when the sun doesn't reach that
    ///   angle on this day.
    pub fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimError> {
        Formula::SunsetOffsetByDegrees(valid_degrees(degrees)?).evaluate(self)
    }

    /// The location.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// The time zone, if one was given.
    pub fn timezone(&self) -> Option<&Z> {
        self.timezone.as_ref()
    }

    /// The time zone as the calculations read it.
    pub(crate) fn zone(&self) -> Option<&dyn Zone> {
        self.timezone.as_ref().map(|zone| zone as &dyn Zone)
    }

    /// The date.
    pub fn date(&self) -> Date {
        self.date
    }

    /// The config.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// A *shaah zmanis*, a twelfth of the day from `start` to `end`.
///
/// Combine it with any two times to build a proportional *zman*. The latest time for Shema by a
/// day from dawn to nightfall is `start + shaah_zmanis(start, end) * 3`.
pub fn shaah_zmanis(start: Timestamp, end: Timestamp) -> SignedDuration {
    end.duration_since(start) / 12
}

/// `degrees`, if it is an angle from -90 to 90.
fn valid_degrees(degrees: f64) -> Result<f64, ZmanimError> {
    if (-90.0..=90.0).contains(&degrees) {
        Ok(degrees)
    } else {
        Err(ZmanimError::InvalidDegrees)
    }
}

/// `time` plus `duration`, or [`ZmanimError::TimeOutOfRange`] past the last instant a
/// [`Timestamp`] holds.
pub(crate) fn plus(time: Timestamp, duration: SignedDuration) -> Result<Timestamp, ZmanimError> {
    Ok(time.checked_add(duration)?)
}

/// `time` minus `duration`, or [`ZmanimError::TimeOutOfRange`] before the first instant a
/// [`Timestamp`] holds.
pub(crate) fn minus(time: Timestamp, duration: SignedDuration) -> Result<Timestamp, ZmanimError> {
    Ok(time.checked_sub(duration)?)
}

/// A configured offset, or [`ZmanimError::InvalidOffset`] when it is a day or more.
pub(crate) fn offset(duration: Duration) -> Result<SignedDuration, ZmanimError> {
    if duration >= Duration::from_hours(24) {
        return Err(ZmanimError::InvalidOffset);
    }
    Ok(SignedDuration::try_from(duration)?)
}

#[cfg(feature = "defmt")]
impl<Z> defmt::Format for Zmanim<Z> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "Zmanim {{ location: {}, date: {=i16}-{=i8}-{=i8}, config: {} }}",
            self.location,
            self.date.year(),
            self.date.month(),
            self.date.day(),
            self.config
        )
    }
}
