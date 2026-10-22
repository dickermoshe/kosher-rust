use jiff::{SignedDuration, Timestamp, civil::Date, tz::TimeZone};

use super::{Config, Zman, ZmanimError, formula::Formula};
use crate::{Location, TimeZoneLike};

/// The *zmanim* for one place on one day.
///
/// Build one with [`Zmanim::new`], then [`get`](Self::get) any [`Zman`]. Times are
/// [`Timestamp`]s; convert them to the location's time zone to display them.
///
/// ```
/// use jiff::{civil::date, tz::TimeZone};
/// use kosher_rust::{Location, Zman, Zmanim};
///
/// let tz = TimeZone::get("America/New_York")?;
/// let lakewood = Location::new(40.09, -74.22, Some(tz.clone()))?;
/// let zmanim = Zmanim::new(lakewood, date(2024, 4, 23));
///
/// let shma = zmanim.get(Zman::SofZmanShmaGra)?;
/// assert_eq!(shma.to_zoned(tz).strftime("%H:%M").to_string(), "09:31");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug)]
pub struct Zmanim<Tz = TimeZone> {
    pub(crate) location: Location<Tz>,
    pub(crate) date: Date,
    pub(crate) config: Config,
}

impl<Tz: TimeZoneLike> Zmanim<Tz> {
    /// The *zmanim* at `location` on `date`, with the default [`Config`].
    pub fn new(location: Location<Tz>, date: Date) -> Self {
        Self::with_config(location, date, Config::default())
    }

    /// The *zmanim* at `location` on `date`, calculated with `config`: which solar algorithm,
    /// whether to use elevation, how to find *chatzos*, and the candle-lighting offset.
    pub fn with_config(location: Location<Tz>, date: Date, config: Config) -> Self {
        Self { location, date, config }
    }

    /// The time of `zman` on this day.
    ///
    /// # Errors
    ///
    /// - [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] where the sun never reaches the
    ///   angle the time needs, as in the far north or south.
    /// - [`ZmanimError::NotApplicable`] for a time that only exists on some days, such as the
    ///   latest time to eat chametz outside Erev Pesach.
    /// - [`ZmanimError::TimeZoneRequired`] for the Kiddush Levana times at a location without a
    ///   time zone.
    pub fn get(&self, zman: Zman) -> Result<Timestamp, ZmanimError> {
        zman.formula().evaluate(self)
    }

    /// When the sun is `degrees` below the eastern horizon before sunrise, for an opinion the
    /// catalog doesn't have. Measured from the geometric horizon, without elevation.
    ///
    /// # Errors
    ///
    /// [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] when the sun doesn't reach that
    /// depression on this day.
    pub fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimError> {
        Formula::SunriseOffsetByDegrees(degrees).evaluate(self)
    }

    /// When the sun is `degrees` below the western horizon after sunset, for example 8.5 for a
    /// custom *tzais*. Measured from the geometric horizon, without elevation.
    ///
    /// # Errors
    ///
    /// [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] when the sun doesn't reach that
    /// depression on this day.
    pub fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimError> {
        Formula::SunsetOffsetByDegrees(degrees).evaluate(self)
    }

    /// The location.
    pub fn location(&self) -> &Location<Tz> {
        &self.location
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

/// `time` plus `duration`, or [`ZmanimError::Internal`] past the last instant a [`Timestamp`]
/// holds.
pub(crate) fn plus(time: Timestamp, duration: SignedDuration) -> Result<Timestamp, ZmanimError> {
    time.checked_add(duration).map_err(|_| ZmanimError::Internal)
}

/// `time` minus `duration`, or [`ZmanimError::Internal`] before the first instant a
/// [`Timestamp`] holds.
pub(crate) fn minus(time: Timestamp, duration: SignedDuration) -> Result<Timestamp, ZmanimError> {
    time.checked_sub(duration).map_err(|_| ZmanimError::Internal)
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
