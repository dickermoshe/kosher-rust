use jiff::{SignedDuration, Timestamp, Zoned, civil::Date, tz::TimeZone};

use super::{Config, Zman, ZmanimError, formula::Formula};
use crate::{Location, TimeZoneLike};

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
/// You can give the *zmanim* a time zone when you create them. With a time zone every time
/// works. You can also read every time in that time zone with [`get_zoned`](Self::get_zoned).
///
/// You can also leave the time zone out by passing `None`. Most times still work without one.
/// These do not work without a time zone:
///
/// - The four Kiddush Levana times return [`ZmanimError::TimeZoneRequired`]. These are the four
///   times:
///   - [`Zman::TchilasZmanKidushLevana3Days`]
///   - [`Zman::TchilasZmanKidushLevana7Days`]
///   - [`Zman::SofZmanKidushLevanaBetweenMoldos`]
///   - [`Zman::SofZmanKidushLevana15Days`]
/// - [`get_zoned`](Self::get_zoned) returns [`ZmanimError::TimeZoneRequired`]. You can still
///   read the time as a [`Timestamp`] with [`get`](Self::get).
/// - A location more than 150° from Greenwich needs a time zone for every time. Without one
///   every time returns [`ZmanimError::TimeZoneRequired`].
#[derive(Clone, Debug, PartialEq)]
pub struct Zmanim<Tz = TimeZone> {
    pub(crate) location: Location,
    pub(crate) date: Date,
    pub(crate) config: Config,
    pub(crate) timezone: Option<Tz>,
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
        Self::with_custom_timezone(location, date, timezone, config)
    }
}

impl<Tz: TimeZoneLike> Zmanim<Tz> {
    /// The same as [`Zmanim::with_config`] with any [`TimeZoneLike`] as the time zone. The
    /// section on [time zones](Zmanim#time-zones) lists the times that need one.
    pub fn with_custom_timezone(location: Location, date: Date, timezone: Option<Tz>, config: Config) -> Self {
        Self {
            location,
            date,
            config,
            timezone,
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
    /// - [`ZmanimError::TimeZoneRequired`] when the *zmanim* have no time zone and the time
    ///   needs one. The section on [time zones](Zmanim#time-zones) lists those times.
    pub fn get(&self, zman: Zman) -> Result<Timestamp, ZmanimError> {
        zman.formula().evaluate(self)
    }

    /// The time of `zman` in the time zone of these *zmanim*. With a [`TimeZone`] the result is in
    /// that zone. With another [`TimeZoneLike`] it has the fixed offset in effect at that time.
    ///
    /// # Errors
    ///
    /// It returns the same errors as [`get`](Self::get). It also returns
    /// [`ZmanimError::TimeZoneRequired`] when the *zmanim* have no time zone.
    pub fn get_zoned(&self, zman: Zman) -> Result<Zoned, ZmanimError> {
        let timezone = self.timezone.as_ref().ok_or(ZmanimError::TimeZoneRequired)?;
        Ok(timezone.to_zoned(self.get(zman)?))
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
    pub fn timezone(&self) -> Option<&Tz> {
        self.timezone.as_ref()
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

/// A *shaah zmanis*: a twelfth of the day from `start` to `end`.
///
/// Combine it with any two times to build a proportional *zman*: the latest time for Shema by a
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

/// `time` plus `duration`, or [`ZmanimError::Internal`] past the last instant a [`Timestamp`]
/// holds.
pub(crate) fn plus(time: Timestamp, duration: SignedDuration) -> Result<Timestamp, ZmanimError> {
    Ok(time.checked_add(duration)?)
}

/// `time` minus `duration`, or [`ZmanimError::Internal`] before the first instant a
/// [`Timestamp`] holds.
pub(crate) fn minus(time: Timestamp, duration: SignedDuration) -> Result<Timestamp, ZmanimError> {
    Ok(time.checked_sub(duration)?)
}

#[cfg(feature = "defmt")]
impl<Tz> defmt::Format for Zmanim<Tz> {
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
