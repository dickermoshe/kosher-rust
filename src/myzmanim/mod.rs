//! The *zmanim* of [MyZmanim](https://www.myzmanim.com/)'s website, reproduced to the second.
//!
//! This module is self-contained: it does not use the [`crate::zmanim`] presets or
//! [`CalculatorConfig`](crate::zmanim::types::config::CalculatorConfig). Give it a [`Location`] (latitude, longitude, the
//! observer's elevation and the elevations of the eastern and western horizons), a date and
//! optionally the candle-lighting minutes, and every row of the website's "All zmanim" table
//! is a [`MyZman`].
//!
//! MyZmanim does not publish its methods. Everything here was read off that table at 284
//! places over a year and reproduces it within a second 92% of the time and within two
//! seconds 99.8% of the time. The rules, as measured:
//!
//! - The sun's position is Jean Meeus's VSOP87 algorithm with ΔT, but with nutation off and
//!   the Earth's monthly motion around the Earth–Moon barycenter left out.
//! - Refraction at the horizon is [`SEA_LEVEL_REFRACTION_ARCMINUTES`] (not 34′), scaled by
//!   the air pressure at the height of the horizon the sun is seen over: the observer's own
//!   height for the "level region" times, the eastern or western horizon's height for the
//!   "elevated" times.
//! - The dip of the horizon for a raised observer is [`DIP_DEGREES_PER_ROOT_METER`]·√h
//!   (*Ma'aglay Tzedek*), not the geometric `acos(R / (R + h))`.
//! - "90 minutes as degrees" is [`DAWN_90_DEGREES`] (19.75°, not 19.8°); the Gra's nightfalls
//!   are 3.65°, 4.37° and 4.61°; R' Moshe Feinstein's end of fasts 7.12°, Sefer Moed Moadim
//!   8°, the Chazon Ish 9.28°.
//! - Candle lighting and R' Tam's fixed 72 minutes count from the sunset with heights; the
//!   72-minute Magen Avraham day still ends 72 minutes after sea-level sunset.
//! - Proportional *zmanim*, midday and midnight are built from the start and end of the day
//!   rounded to the nearest second, as the website displays them, and cut to a whole second.
//!
//! # Quick start
//!
//! ```
//! use jiff::{civil::Date, tz::TimeZone};
//! use kosher_rust::myzmanim::prelude::*;
//!
//! let location = Location::with_horizon_elevations(
//!     31.778,  // latitude
//!     35.235,  // longitude
//!     754.0,   // observer elevation in meters
//!     754.0,   // eastern horizon elevation in meters
//!     0.0,     // western horizon elevation in meters
//!     Some(TimeZone::get("Asia/Jerusalem").unwrap()),
//! )
//! .unwrap();
//! let calendar = MyZmanimCalendar::new(location, Date::new(2026, 11, 6).unwrap());
//! let sunset = calendar.calculate(MyZman::SunsetElevated).unwrap();
//! let candles = calendar.calculate(MyZman::CandleLighting).unwrap();
//! assert!(candles < sunset);
//! ```

use jiff::{SignedDuration, Timestamp, civil::Date};

#[allow(unused_imports)]
use core_maths::*;

use crate::zmanim::{
    astronomy::{
        GEOMETRIC_ZENITH, SolarEvent, adjusted_local_date, apparent_solar_radius, meeus::Meeus,
        rise_set_on_adjusted_date,
    },
    types::{error::ZmanimError, location::Location},
};

/// Common MyZmanim imports: the calendar, its *zmanim*, and the location and error types.
pub mod prelude {
    pub use super::{MyZman, MyZmanimCalendar};
    pub use crate::zmanim::types::{error::ZmanimError, location::Location};
}

#[cfg(test)]
mod tests;

/// The sun's depression for "90 minutes as degrees": 19.75°, measured from the website's times.
pub const DAWN_90_DEGREES: f64 = 19.75;

/// The sun's depression for "72 minutes as 16.1 degrees", dawn and R' Tam's nightfall.
pub const DAWN_72_DEGREES: f64 = 16.1;

/// Refraction at the horizon at sea level, in arc-minutes: 34′ scaled from 1010 mb to 1000 mb, measured.
pub const SEA_LEVEL_REFRACTION_ARCMINUTES: f64 = 33.66;

/// The dip of the horizon below an observer `h` meters above it, in degrees per root meter, measured.
pub const DIP_DEGREES_PER_ROOT_METER: f64 = 0.0347;

/// MyZmanim's default minutes before sunset for candle lighting.
pub const DEFAULT_CANDLE_LIGHTING_MINUTES: f64 = 18.0;

const FIXED_72_MINUTES: i64 = 72;
const MINCHA_30_MINUTES: i64 = 30;
const NANOS_PER_SECOND: i128 = 1_000_000_000;

/// One row of MyZmanim's "All zmanim" table, in the table's order.
///
/// Calculate one with [`MyZmanimCalendar::calculate`]. Every variant returns an error where
/// the website shows no time: [`ZmanimError::AllDay`] / [`ZmanimError::AllNight`] where the
/// sun never reaches the angle, and [`ZmanimError::CalculationError`] where a derived time
/// has no inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MyZman {
    /// "Dawn, 90 minutes as degrees before sunrise": the sun [`DAWN_90_DEGREES`] below the horizon.
    Dawn90Degrees,
    /// "Dawn, fixed 72 minutes before sunrise": before sea-level sunrise.
    Dawn72Fixed,
    /// "Dawn, 72 minutes as 16.1 degrees".
    Dawn72Degrees,
    /// "Earliest talis & tefillin, sun is 11.5 degrees below horizon".
    Talis11Point5Degrees,
    /// "Earliest talis & tefillin, sun is 11 degrees below horizon".
    Talis11Degrees,
    /// "Earliest talis & tefillin, sun is 10.2 degrees below horizon".
    Talis10Point2Degrees,
    /// "Sunrise, level region": sea-level sunrise.
    SunriseLevel,
    /// "Sunrise, elevated observer/horizon": sunrise with the observer's height above the
    /// eastern horizon. Where the horizon is level with the observer it is the level time.
    SunriseElevated,
    /// "Latest shema Magen Avraham, using 90 minutes as degrees": three hours into the 19.75° day.
    ShemaMA90Degrees,
    /// "Latest shema Magen Avraham, using fixed 72 minutes": three hours into the day from 72
    /// minutes before sea-level sunrise to 72 minutes after sea-level sunset.
    ShemaMA72Fixed,
    /// "Latest shema Magen Avraham, using 72 minutes as 16.1 degrees": three hours into the 16.1° day.
    ShemaMA72Degrees,
    /// "Latest shema Gra & Baal HaTanya": three hours into the day from sea-level sunrise to sea-level sunset.
    ShemaGra,
    /// "Latest shacharis Magen Avraham, using 90 minutes as degrees": four hours into the 19.75° day.
    ShacharisMA90Degrees,
    /// "Latest shacharis Magen Avraham, using fixed 72 minutes": four hours into the fixed-72-minute day.
    ShacharisMA72Fixed,
    /// "Latest shacharis Magen Avraham, using 72 minutes as 16.1 degrees": four hours into the 16.1° day.
    ShacharisMA72Degrees,
    /// "Latest shacharis Gra & Baal HaTanya": four hours into the Gra day.
    ShacharisGra,
    /// "Midday": the midpoint of sea-level sunrise and sunset.
    Midday,
    /// "Earliest mincha, 30 fixed minutes after midday".
    Mincha30Fixed,
    /// "Earliest mincha, Gra & Baal HaTanya": six and a half hours into the Gra day.
    MinchaGra,
    /// "Earliest mincha, lechumra": the later of [`Self::Mincha30Fixed`] and [`Self::MinchaGra`].
    MinchaLechumra,
    /// "Earliest mincha, Magen Avraham using fixed 72 minutes": six and a half hours into the fixed-72-minute day.
    MinchaMA72Fixed,
    /// "Mincha ketana, Gra & Baal HaTanya": nine and a half hours into the Gra day.
    KetanaGra,
    /// "Mincha ketana, Magen Avraham using fixed 72 minutes": nine and a half hours into the fixed-72-minute day.
    KetanaMA72Fixed,
    /// "Plag hamincha, Gra & Baal HaTanya": ten and three quarter hours into the Gra day.
    PlagGra,
    /// "Plag hamincha, Magen Avraham using fixed 72 minutes": ten and three quarter hours into the fixed-72-minute day.
    PlagMA72Fixed,
    /// "Candle lighting": [`MyZmanimCalendar::candle_lighting_minutes`] before the sunset with heights.
    CandleLighting,
    /// "Sunset, level region": sea-level sunset.
    SunsetLevel,
    /// "Sunset, elevated observer/horizon": sunset with the observer's height above the western horizon.
    SunsetElevated,
    /// "Gra's nightfall, 3/4 mil, 13 1/2 minutes as degrees, based on 1 mil = 18 minutes": 3.65°.
    NightGra18Minutes,
    /// "Gra's nightfall, 3/4 mil, 16 7/8 minutes as degrees, based on 1 mil = 22.5 minutes": 4.37°.
    NightGra22Point5Minutes,
    /// "Gra's nightfall, 3/4 mil, 18 minutes as degrees, based on 1 mil = 24 minutes": 4.61°.
    NightGra24Minutes,
    /// "Gra's nightfall, 24 minutes as degrees, Siddur of R' Schneur Zalman": 5.95°.
    NightZalman,
    /// "Nightfall, end of ordained fasts, R' Tukaccinsky 27 minutes as degrees": 6.45°.
    NightFastTukaccinsky,
    /// "Nightfall, end of ordained fasts, R' Moshe Feinstein for one who finds fasting difficult": 7.12°.
    NightFastFeinstein,
    /// "Nightfall, 3 stars emerge, 34 minutes as degrees, Sefer Moed Moadim": 8°.
    NightMoed,
    /// "Nightfall, 3 stars emerge, 36 minutes as degrees, end of Shabbos and Yom Tov": 8.5°.
    NightShabbos,
    /// "Nightfall, 3 stars emerge, 40 minutes as degrees, Chazon Ish": 9.28°.
    NightChazonIsh,
    /// "R' Tam's nightfall, 4 milin, fixed 72 minutes after sunset": after the sunset with heights.
    NightRTam72Fixed,
    /// "R' Tam's nightfall, 4 milin, 72 minutes as 16.1 degrees".
    NightRTam72Degrees,
    /// "Midnight": the midpoint of sea-level sunset and the next day's sea-level sunrise.
    Midnight,
}

impl MyZman {
    /// Every row, in the table's order.
    pub const ALL: [MyZman; 40] = [
        Self::Dawn90Degrees,
        Self::Dawn72Fixed,
        Self::Dawn72Degrees,
        Self::Talis11Point5Degrees,
        Self::Talis11Degrees,
        Self::Talis10Point2Degrees,
        Self::SunriseLevel,
        Self::SunriseElevated,
        Self::ShemaMA90Degrees,
        Self::ShemaMA72Fixed,
        Self::ShemaMA72Degrees,
        Self::ShemaGra,
        Self::ShacharisMA90Degrees,
        Self::ShacharisMA72Fixed,
        Self::ShacharisMA72Degrees,
        Self::ShacharisGra,
        Self::Midday,
        Self::Mincha30Fixed,
        Self::MinchaGra,
        Self::MinchaLechumra,
        Self::MinchaMA72Fixed,
        Self::KetanaGra,
        Self::KetanaMA72Fixed,
        Self::PlagGra,
        Self::PlagMA72Fixed,
        Self::CandleLighting,
        Self::SunsetLevel,
        Self::SunsetElevated,
        Self::NightGra18Minutes,
        Self::NightGra22Point5Minutes,
        Self::NightGra24Minutes,
        Self::NightZalman,
        Self::NightFastTukaccinsky,
        Self::NightFastFeinstein,
        Self::NightMoed,
        Self::NightShabbos,
        Self::NightChazonIsh,
        Self::NightRTam72Fixed,
        Self::NightRTam72Degrees,
        Self::Midnight,
    ];

    /// The row's label on the website.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Dawn90Degrees => "Dawn, 90 minutes as degrees",
            Self::Dawn72Fixed => "Dawn, fixed 72 minutes",
            Self::Dawn72Degrees => "Dawn, 72 minutes as 16.1 degrees",
            Self::Talis11Point5Degrees => "Earliest talis & tefillin, 11.5 degrees",
            Self::Talis11Degrees => "Earliest talis & tefillin, 11 degrees",
            Self::Talis10Point2Degrees => "Earliest talis & tefillin, 10.2 degrees",
            Self::SunriseLevel => "Sunrise, level region",
            Self::SunriseElevated => "Sunrise, elevated observer/horizon",
            Self::ShemaMA90Degrees => "Latest shema Magen Avraham, 90 minutes as degrees",
            Self::ShemaMA72Fixed => "Latest shema Magen Avraham, fixed 72 minutes",
            Self::ShemaMA72Degrees => "Latest shema Magen Avraham, 72 minutes as 16.1 degrees",
            Self::ShemaGra => "Latest shema Gra & Baal HaTanya",
            Self::ShacharisMA90Degrees => "Latest shacharis Magen Avraham, 90 minutes as degrees",
            Self::ShacharisMA72Fixed => "Latest shacharis Magen Avraham, fixed 72 minutes",
            Self::ShacharisMA72Degrees => "Latest shacharis Magen Avraham, 72 minutes as 16.1 degrees",
            Self::ShacharisGra => "Latest shacharis Gra & Baal HaTanya",
            Self::Midday => "Midday",
            Self::Mincha30Fixed => "Earliest mincha, 30 fixed minutes after midday",
            Self::MinchaGra => "Earliest mincha, Gra & Baal HaTanya",
            Self::MinchaLechumra => "Earliest mincha, lechumra",
            Self::MinchaMA72Fixed => "Earliest mincha, Magen Avraham fixed 72 minutes",
            Self::KetanaGra => "Mincha ketana, Gra & Baal HaTanya",
            Self::KetanaMA72Fixed => "Mincha ketana, Magen Avraham fixed 72 minutes",
            Self::PlagGra => "Plag hamincha, Gra & Baal HaTanya",
            Self::PlagMA72Fixed => "Plag hamincha, Magen Avraham fixed 72 minutes",
            Self::CandleLighting => "Candle lighting",
            Self::SunsetLevel => "Sunset, level region",
            Self::SunsetElevated => "Sunset, elevated observer/horizon",
            Self::NightGra18Minutes => "Gra's nightfall, 13 1/2 minutes as degrees",
            Self::NightGra22Point5Minutes => "Gra's nightfall, 16 7/8 minutes as degrees",
            Self::NightGra24Minutes => "Gra's nightfall, 18 minutes as degrees",
            Self::NightZalman => "Gra's nightfall, 24 minutes as degrees, R' Schneur Zalman",
            Self::NightFastTukaccinsky => "End of ordained fasts, R' Tukaccinsky",
            Self::NightFastFeinstein => "End of ordained fasts, R' Moshe Feinstein",
            Self::NightMoed => "3 stars emerge, Sefer Moed Moadim",
            Self::NightShabbos => "3 stars emerge, end of Shabbos and Yom Tov",
            Self::NightChazonIsh => "3 stars emerge, Chazon Ish",
            Self::NightRTam72Fixed => "R' Tam's nightfall, fixed 72 minutes",
            Self::NightRTam72Degrees => "R' Tam's nightfall, 72 minutes as 16.1 degrees",
            Self::Midnight => "Midnight",
        }
    }

    /// Calculates this *zman* with `calendar`; the same as [`MyZmanimCalendar::calculate`].
    pub fn calculate(self, calendar: &MyZmanimCalendar) -> Result<Timestamp, ZmanimError> {
        calendar.calculate(self)
    }
}

/// MyZmanim's *zmanim* for a [`Location`] on a civil [`Date`].
#[derive(Debug, Clone, PartialEq)]
pub struct MyZmanimCalendar {
    location: Location,
    date: Date,
    candle_lighting_minutes: f64,
}

impl MyZmanimCalendar {
    /// A calendar for `location`, with its observer and horizon heights, on `date`, with
    /// MyZmanim's default [`DEFAULT_CANDLE_LIGHTING_MINUTES`].
    #[must_use]
    pub fn new(location: Location, date: Date) -> Self {
        Self {
            location,
            date,
            candle_lighting_minutes: DEFAULT_CANDLE_LIGHTING_MINUTES,
        }
    }

    /// The same calendar with `minutes` before sunset for [`MyZman::CandleLighting`].
    #[must_use]
    pub fn with_candle_lighting_minutes(mut self, minutes: f64) -> Self {
        self.candle_lighting_minutes = minutes;
        self
    }

    /// The location, with its observer and horizon heights.
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// The date the *zmanim* are for.
    #[must_use]
    pub fn date(&self) -> Date {
        self.date
    }

    /// Sets the date the *zmanim* are for.
    pub fn set_date(&mut self, date: Date) {
        self.date = date;
    }

    /// The minutes before sunset of [`MyZman::CandleLighting`].
    #[must_use]
    pub fn candle_lighting_minutes(&self) -> f64 {
        self.candle_lighting_minutes
    }

    /// Sets the minutes before sunset of [`MyZman::CandleLighting`].
    pub fn set_candle_lighting_minutes(&mut self, minutes: f64) {
        self.candle_lighting_minutes = minutes;
    }

    /// Calculates one *zman*.
    pub fn calculate(&self, zman: MyZman) -> Result<Timestamp, ZmanimError> {
        use MyZman::*;
        match zman {
            Dawn90Degrees => self.rise_set_by_degrees(SolarEvent::Sunrise, DAWN_90_DEGREES),
            Dawn72Fixed => Ok(self.calculate(SunriseLevel)? - minutes(FIXED_72_MINUTES)),
            Dawn72Degrees => self.rise_set_by_degrees(SolarEvent::Sunrise, DAWN_72_DEGREES),
            Talis11Point5Degrees => self.rise_set_by_degrees(SolarEvent::Sunrise, 11.5),
            Talis11Degrees => self.rise_set_by_degrees(SolarEvent::Sunrise, 11.0),
            Talis10Point2Degrees => self.rise_set_by_degrees(SolarEvent::Sunrise, 10.2),
            SunriseLevel => self.rise_set(SolarEvent::Sunrise, false),
            SunriseElevated => self.rise_set(SolarEvent::Sunrise, true),
            ShemaMA90Degrees => self.proportional(Dawn90Degrees, NIGHT_90_DEGREES, 3.0),
            ShemaMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 3.0),
            ShemaMA72Degrees => self.proportional(Dawn72Degrees, NightRTam72Degrees, 3.0),
            ShemaGra => self.proportional(SunriseLevel, SunsetLevel, 3.0),
            ShacharisMA90Degrees => self.proportional(Dawn90Degrees, NIGHT_90_DEGREES, 4.0),
            ShacharisMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 4.0),
            ShacharisMA72Degrees => self.proportional(Dawn72Degrees, NightRTam72Degrees, 4.0),
            ShacharisGra => self.proportional(SunriseLevel, SunsetLevel, 4.0),
            Midday => midpoint(self.calculate(SunriseLevel)?, self.calculate(SunsetLevel)?),
            Mincha30Fixed => Ok(self.calculate(Midday)? + minutes(MINCHA_30_MINUTES)),
            MinchaGra => self.proportional(SunriseLevel, SunsetLevel, 6.5),
            MinchaLechumra => {
                let fixed = self.calculate(Mincha30Fixed)?;
                let gra = self.calculate(MinchaGra)?;
                Ok(if fixed > gra { fixed } else { gra })
            }
            MinchaMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 6.5),
            KetanaGra => self.proportional(SunriseLevel, SunsetLevel, 9.5),
            KetanaMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 9.5),
            PlagGra => self.proportional(SunriseLevel, SunsetLevel, 10.75),
            PlagMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 10.75),
            CandleLighting => {
                let sunset = self.calculate(SunsetElevated)?;
                let millis = (self.candle_lighting_minutes * 60_000.0).round() as i64;
                Ok(sunset - SignedDuration::from_millis(millis))
            }
            SunsetLevel => self.rise_set(SolarEvent::Sunset, false),
            SunsetElevated => self.rise_set(SolarEvent::Sunset, true),
            NightGra18Minutes => self.rise_set_by_degrees(SolarEvent::Sunset, 3.65),
            NightGra22Point5Minutes => self.rise_set_by_degrees(SolarEvent::Sunset, 4.37),
            NightGra24Minutes => self.rise_set_by_degrees(SolarEvent::Sunset, 4.61),
            NightZalman => self.rise_set_by_degrees(SolarEvent::Sunset, 5.95),
            NightFastTukaccinsky => self.rise_set_by_degrees(SolarEvent::Sunset, 6.45),
            NightFastFeinstein => self.rise_set_by_degrees(SolarEvent::Sunset, 7.12),
            NightMoed => self.rise_set_by_degrees(SolarEvent::Sunset, 8.0),
            NightShabbos => self.rise_set_by_degrees(SolarEvent::Sunset, 8.5),
            NightChazonIsh => self.rise_set_by_degrees(SolarEvent::Sunset, 9.28),
            NightRTam72Fixed => Ok(self.calculate(SunsetElevated)? + minutes(FIXED_72_MINUTES)),
            NightRTam72Degrees => self.rise_set_by_degrees(SolarEvent::Sunset, DAWN_72_DEGREES),
            Midnight => {
                let sunset = self.calculate(SunsetLevel)?;
                let tomorrow = self.date.tomorrow().map_err(|_| ZmanimError::TimeConversionError)?;
                let next_sunrise = Self::new(self.location.clone(), tomorrow).calculate(SunriseLevel)?;
                midpoint(sunset, next_sunrise)
            }
        }
    }

    /// "Proportional hour, Gra & Baal HaTanya": a twelfth of the day from sea-level sunrise
    /// to sea-level sunset.
    pub fn proportional_hour_gra(&self) -> Result<SignedDuration, ZmanimError> {
        let sunrise = self.calculate(MyZman::SunriseLevel)?;
        let sunset = self.calculate(MyZman::SunsetLevel)?;
        Ok(sunset.duration_since(sunrise) / 12)
    }

    // ---- The website's arithmetic ----

    /// `hours` proportional hours into the day bounded by two *zmanim*.
    fn proportional(
        &self,
        start: impl Into<DayBound>,
        end: impl Into<DayBound>,
        hours: f64,
    ) -> Result<Timestamp, ZmanimError> {
        proportional(self.bound(start.into())?, self.bound(end.into())?, hours)
    }

    fn bound(&self, bound: DayBound) -> Result<Timestamp, ZmanimError> {
        match bound {
            DayBound::Zman(zman) => self.calculate(zman),
            // The end of the 19.75° Magen Avraham day; the website shows no row for it.
            DayBound::Night90Degrees => self.rise_set_by_degrees(SolarEvent::Sunset, DAWN_90_DEGREES),
            // The end of the fixed-72-minute Magen Avraham day: 72 minutes after sea-level sunset.
            DayBound::Night72FixedLevel => Ok(self.calculate(MyZman::SunsetLevel)? + minutes(FIXED_72_MINUTES)),
        }
    }

    // ---- The sun ----

    /// True sunrise or sunset, with MyZmanim's refraction and dip. Refraction happens where
    /// the light grazes the horizon, so it is scaled for the air at that horizon's height: the
    /// observer's own for the level times, the eastern or western horizon's for the elevated.
    fn rise_set(&self, event: SolarEvent, elevated: bool) -> Result<Timestamp, ZmanimError> {
        let adjusted_date = adjusted_local_date(self.date, &self.location)?;
        let (horizon_elevation, height_above_horizon) = match (event, elevated) {
            (SolarEvent::Sunrise, true) => (
                self.location.eastern_horizon_elevation,
                self.location.sunrise_elevation(),
            ),
            (SolarEvent::Sunset, true) => (
                self.location.western_horizon_elevation,
                self.location.sunset_elevation(),
            ),
            _ => (self.location.elevation, 0.0),
        };
        let zenith = GEOMETRIC_ZENITH
            + (apparent_solar_radius(adjusted_date) + refraction(horizon_elevation) + dip(height_above_horizon));
        rise_set_on_adjusted_date(&calculator(), adjusted_date, &self.location, zenith, event)
    }

    /// The time the sun is `degrees` below the geometric horizon: no refraction, radius or dip.
    fn rise_set_by_degrees(&self, event: SolarEvent, degrees: f64) -> Result<Timestamp, ZmanimError> {
        let adjusted_date = adjusted_local_date(self.date, &self.location)?;
        rise_set_on_adjusted_date(
            &calculator(),
            adjusted_date,
            &self.location,
            GEOMETRIC_ZENITH + degrees,
            event,
        )
    }
}

/// A start or end of a proportional day: a table row, or one of the two ends the website
/// uses but does not show.
#[derive(Clone, Copy)]
enum DayBound {
    Zman(MyZman),
    Night90Degrees,
    Night72FixedLevel,
}

impl From<MyZman> for DayBound {
    fn from(zman: MyZman) -> Self {
        Self::Zman(zman)
    }
}

/// The end of the 19.75° Magen Avraham day.
const NIGHT_90_DEGREES: DayBound = DayBound::Night90Degrees;
/// The end of the fixed-72-minute Magen Avraham day.
const NIGHT_72_FIXED_LEVEL: DayBound = DayBound::Night72FixedLevel;

/// Meeus with ΔT, without nutation, on the Earth–Moon barycenter's orbit.
const fn calculator() -> Meeus {
    Meeus {
        apply_delta_t: true,
        apply_nutation: false,
        use_earth_moon_barycenter: true,
    }
}

/// Refraction at a horizon `horizon_elevation` meters above sea level, in degrees: the sea-level
/// value scaled by the barometric pressure ratio at that height.
fn refraction(horizon_elevation: f64) -> f64 {
    SEA_LEVEL_REFRACTION_ARCMINUTES / 60.0 * (1.0 - 2.25577e-5 * horizon_elevation).powf(5.25588)
}

/// The dip of the horizon below an observer `height` meters above it, in degrees.
fn dip(height: f64) -> f64 {
    if height <= 0.0 {
        0.0
    } else {
        DIP_DEGREES_PER_ROOT_METER * height.sqrt()
    }
}

fn minutes(count: i64) -> SignedDuration {
    SignedDuration::from_mins(count)
}

/// `hours` proportional hours into a day, from its start and end rounded to the nearest
/// second, cut to a second.
fn proportional(start_of_day: Timestamp, end_of_day: Timestamp, hours: f64) -> Result<Timestamp, ZmanimError> {
    let start = nearest_second(start_of_day);
    let day_nanos = nearest_second(end_of_day).as_nanosecond() - start.as_nanosecond();
    // Java: `start.plusNanos((long) (dayNanos / 12 * hours))` — integer twelfth, then scaled.
    let offset = ((day_nanos / 12) as f64 * hours) as i64;
    floor_second(timestamp_from_nanos(start.as_nanosecond() + i128::from(offset))?)
}

/// The midpoint of two times rounded to the nearest second, cut to a second.
fn midpoint(begin: Timestamp, end: Timestamp) -> Result<Timestamp, ZmanimError> {
    let start = nearest_second(begin);
    let nanos = nearest_second(end).as_nanosecond() - start.as_nanosecond();
    floor_second(timestamp_from_nanos(start.as_nanosecond() + nanos / 2)?)
}

fn nearest_second(time: Timestamp) -> Timestamp {
    let seconds = (time.as_nanosecond() + NANOS_PER_SECOND / 2).div_euclid(NANOS_PER_SECOND);
    Timestamp::from_second(seconds as i64).unwrap_or(time)
}

fn floor_second(time: Timestamp) -> Result<Timestamp, ZmanimError> {
    let seconds = time.as_nanosecond().div_euclid(NANOS_PER_SECOND);
    Timestamp::from_second(seconds as i64).map_err(|_| ZmanimError::TimeConversionError)
}

fn timestamp_from_nanos(nanos: i128) -> Result<Timestamp, ZmanimError> {
    Timestamp::from_nanosecond(nanos).map_err(|_| ZmanimError::TimeConversionError)
}
