use core::fmt;
use core::time::Duration;

use jiff::{SignedDuration, Timestamp, Zoned, civil::Date, tz::TimeZone};

use core_maths::*;

use crate::{
    Location, Zone,
    astronomy::{
        GEOMETRIC_ZENITH, SolarEvent, adjusted_local_date, apparent_solar_radius, meeus::Meeus,
        rise_set_on_adjusted_date,
    },
    zmanim::{ZmanimError, minus, offset, plus},
};

use super::{
    DAWN_72_DEGREES, DAWN_90_DEGREES, DEFAULT_CANDLE_LIGHTING, DIP_DEGREES_PER_ROOT_METER,
    SEA_LEVEL_REFRACTION_ARCMINUTES,
};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

const FIXED_72_MINUTES: i64 = 72;
const MINCHA_30_MINUTES: i64 = 30;
const NANOS_PER_SECOND: i128 = 1_000_000_000;

/// Declares `enum MyZman` with its `ALL` and `name` from one list of rows, in the table's order.
macro_rules! my_zmanim {
    ($(
        $(#[doc = $doc:literal])*
        $variant:ident => $name:literal,
    )*) => {
        /// One row of MyZmanim's "All zmanim" table, in the table's order.
        ///
        /// Calculate one with [`MyZmanim::get`]. Every variant returns an error where
        /// the website shows no time. The error is [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`]
        /// where the sun never reaches the angle that time, or a time it is built from, needs.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        #[non_exhaustive]
        pub enum MyZman {
            $(
                $(#[doc = $doc])*
                $variant,
            )*
        }

        impl MyZman {
            /// Every row, in the table's order.
            pub const ALL: [MyZman; [$(stringify!($variant)),*].len()] = [$(Self::$variant),*];

            /// The row's label on the website.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }
        }
    };
}

my_zmanim! {
    /// "Dawn, 90 minutes as degrees before sunrise": the sun [`DAWN_90_DEGREES`] below the horizon.
    Dawn90Degrees => "Dawn, 90 minutes as degrees",
    /// "Dawn, fixed 72 minutes before sunrise": before [level sunrise](Self::SunriseLevel).
    Dawn72Fixed => "Dawn, fixed 72 minutes",
    /// "Dawn, 72 minutes as 16.1 degrees".
    Dawn72Degrees => "Dawn, 72 minutes as 16.1 degrees",
    /// "Earliest talis & tefillin, sun is 11.5 degrees below horizon".
    Talis11Point5Degrees => "Earliest talis & tefillin, 11.5 degrees",
    /// "Earliest talis & tefillin, sun is 11 degrees below horizon".
    Talis11Degrees => "Earliest talis & tefillin, 11 degrees",
    /// "Earliest talis & tefillin, sun is 10.2 degrees below horizon".
    Talis10Point2Degrees => "Earliest talis & tefillin, 10.2 degrees",
    /// "Sunrise, level region": sunrise with no dip of the horizon, and with refraction for the
    /// air at the observer's height.
    SunriseLevel => "Sunrise, level region",
    /// "Sunrise, elevated observer/horizon": sunrise with the observer's height above the
    /// eastern horizon. Where the horizon is level with the observer it is the level time.
    SunriseElevated => "Sunrise, elevated observer/horizon",
    /// "Latest shema Magen Avraham, using 90 minutes as degrees": three hours into the 19.75° day.
    ShemaMA90Degrees => "Latest shema Magen Avraham, 90 minutes as degrees",
    /// "Latest shema Magen Avraham, using fixed 72 minutes": three hours into the day from 72
    /// minutes before level sunrise to 72 minutes after level sunset.
    ShemaMA72Fixed => "Latest shema Magen Avraham, fixed 72 minutes",
    /// "Latest shema Magen Avraham, using 72 minutes as 16.1 degrees": three hours into the 16.1° day.
    ShemaMA72Degrees => "Latest shema Magen Avraham, 72 minutes as 16.1 degrees",
    /// "Latest shema Gra & Baal HaTanya": three hours into the day from level sunrise to level sunset.
    ShemaGra => "Latest shema Gra & Baal HaTanya",
    /// "Latest shacharis Magen Avraham, using 90 minutes as degrees": four hours into the 19.75° day.
    ShacharisMA90Degrees => "Latest shacharis Magen Avraham, 90 minutes as degrees",
    /// "Latest shacharis Magen Avraham, using fixed 72 minutes": four hours into the fixed-72-minute day.
    ShacharisMA72Fixed => "Latest shacharis Magen Avraham, fixed 72 minutes",
    /// "Latest shacharis Magen Avraham, using 72 minutes as 16.1 degrees": four hours into the 16.1° day.
    ShacharisMA72Degrees => "Latest shacharis Magen Avraham, 72 minutes as 16.1 degrees",
    /// "Latest shacharis Gra & Baal HaTanya": four hours into the Gra day.
    ShacharisGra => "Latest shacharis Gra & Baal HaTanya",
    /// "Midday": the midpoint of level sunrise and sunset.
    Midday => "Midday",
    /// "Earliest mincha, 30 fixed minutes after midday".
    Mincha30Fixed => "Earliest mincha, 30 fixed minutes after midday",
    /// "Earliest mincha, Gra & Baal HaTanya": six and a half hours into the Gra day.
    MinchaGra => "Earliest mincha, Gra & Baal HaTanya",
    /// "Earliest mincha, lechumra": the later of [`Self::Mincha30Fixed`] and [`Self::MinchaGra`].
    MinchaLechumra => "Earliest mincha, lechumra",
    /// "Earliest mincha, Magen Avraham using fixed 72 minutes": six and a half hours into the fixed-72-minute day.
    MinchaMA72Fixed => "Earliest mincha, Magen Avraham fixed 72 minutes",
    /// "Mincha ketana, Gra & Baal HaTanya": nine and a half hours into the Gra day.
    KetanaGra => "Mincha ketana, Gra & Baal HaTanya",
    /// "Mincha ketana, Magen Avraham using fixed 72 minutes": nine and a half hours into the fixed-72-minute day.
    KetanaMA72Fixed => "Mincha ketana, Magen Avraham fixed 72 minutes",
    /// "Plag hamincha, Gra & Baal HaTanya": ten and three quarter hours into the Gra day.
    PlagGra => "Plag hamincha, Gra & Baal HaTanya",
    /// "Plag hamincha, Magen Avraham using fixed 72 minutes": ten and three quarter hours into the fixed-72-minute day.
    PlagMA72Fixed => "Plag hamincha, Magen Avraham fixed 72 minutes",
    /// "Candle lighting": [`MyZmanim::candle_lighting_offset`] before the sunset with heights.
    CandleLighting => "Candle lighting",
    /// "Sunset, level region": sunset with no dip of the horizon, and with refraction for the air
    /// at the observer's height.
    SunsetLevel => "Sunset, level region",
    /// "Sunset, elevated observer/horizon": sunset with the observer's height above the western horizon.
    SunsetElevated => "Sunset, elevated observer/horizon",
    /// "Gra's nightfall, 3/4 mil, 13 1/2 minutes as degrees, based on 1 mil = 18 minutes": 3.65°.
    NightGra18Minutes => "Gra's nightfall, 13 1/2 minutes as degrees",
    /// "Gra's nightfall, 3/4 mil, 16 7/8 minutes as degrees, based on 1 mil = 22.5 minutes": 4.37°.
    NightGra22Point5Minutes => "Gra's nightfall, 16 7/8 minutes as degrees",
    /// "Gra's nightfall, 3/4 mil, 18 minutes as degrees, based on 1 mil = 24 minutes": 4.61°.
    NightGra24Minutes => "Gra's nightfall, 18 minutes as degrees",
    /// "Gra's nightfall, 24 minutes as degrees, Siddur of R' Schneur Zalman": 5.95°.
    NightZalman => "Gra's nightfall, 24 minutes as degrees, R' Schneur Zalman",
    /// "Nightfall, end of ordained fasts, R' Tukaccinsky 27 minutes as degrees": 6.45°.
    NightFastTukaccinsky => "End of ordained fasts, R' Tukaccinsky",
    /// "Nightfall, end of ordained fasts, R' Moshe Feinstein for one who finds fasting difficult": 7.12°.
    NightFastFeinstein => "End of ordained fasts, R' Moshe Feinstein",
    /// "Nightfall, 3 stars emerge, 34 minutes as degrees, Sefer Moed Moadim": 8°.
    NightMoed => "3 stars emerge, Sefer Moed Moadim",
    /// "Nightfall, 3 stars emerge, 36 minutes as degrees, end of Shabbos and Yom Tov": 8.5°.
    NightShabbos => "3 stars emerge, end of Shabbos and Yom Tov",
    /// "Nightfall, 3 stars emerge, 40 minutes as degrees, Chazon Ish": 9.28°.
    NightChazonIsh => "3 stars emerge, Chazon Ish",
    /// "R' Tam's nightfall, 4 milin, fixed 72 minutes after sunset": after the sunset with heights.
    NightRTam72Fixed => "R' Tam's nightfall, fixed 72 minutes",
    /// "R' Tam's nightfall, 4 milin, 72 minutes as 16.1 degrees".
    NightRTam72Degrees => "R' Tam's nightfall, 72 minutes as 16.1 degrees",
    /// "Midnight": the midpoint of level sunset and the next day's level sunrise.
    Midnight => "Midnight",
}

impl MyZman {
    /// The variant whose [`name`](Self::name) is exactly `name`. Returns `None` when no
    /// variant has that name.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|candidate| candidate.name() == name)
    }
}

/// The row's label on the website.
impl fmt::Display for MyZman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// MyZmanim's *zmanim* for one place on one day.
///
/// Build one with [`MyZmanim::new`], then [`get`](Self::get) any [`MyZman`]. The location's
/// [elevation](Location::with_elevation) and [horizons](Location::with_horizons) matter here:
/// MyZmanim reports both "level region" and "elevated" sunrise and sunset.
///
/// # Time zones
///
/// When calculating *zmanim*, pass a time zone to [`MyZmanim::new`]. If you pass `None`, you will
/// get [`ZmanimError::TimeZoneRequired`] when you call [`get_zoned`](Self::get_zoned) and for any
/// *zman* if the location is more than 150° east or west of Greenwich.
///
/// The time zone is a [`jiff::tz::TimeZone`] unless you build the *zmanim* with
/// [`with_zone`](Self::with_zone) and a [`Zone`] of your own.
#[derive(Debug, Clone, PartialEq)]
pub struct MyZmanim<Z = TimeZone> {
    location: Location,
    date: Date,
    candle_lighting: Duration,
    timezone: Option<Z>,
}

impl MyZmanim {
    /// The *zmanim* at `location` on `date` in `timezone`. Candles are lit
    /// [`DEFAULT_CANDLE_LIGHTING`] before sunset.
    ///
    /// Pass a time zone if you have one. Every time works with a time zone. Without one a few
    /// things do not work. The section on [time zones](MyZmanim#time-zones) lists them.
    #[must_use]
    pub fn new(location: Location, date: Date, timezone: Option<TimeZone>) -> Self {
        Self::with_zone(location, date, timezone)
    }

    /// The time of `zman` in the time zone of these *zmanim*.
    ///
    /// # Errors
    ///
    /// It returns the same errors as [`get`](Self::get). It also returns
    /// [`ZmanimError::TimeZoneRequired`] when the *zmanim* have no time zone.
    pub fn get_zoned(&self, zman: MyZman) -> Result<Zoned, ZmanimError> {
        let timezone = self.timezone.as_ref().ok_or(ZmanimError::TimeZoneRequired)?;
        Ok(self.get(zman)?.to_zoned(timezone.clone()))
    }
}

impl<Z: Zone> MyZmanim<Z> {
    /// The *zmanim* at `location` on `date` in `zone`, any [`Zone`]. Candles are lit
    /// [`DEFAULT_CANDLE_LIGHTING`] before sunset.
    #[must_use]
    pub fn with_zone(location: Location, date: Date, zone: Option<Z>) -> Self {
        Self {
            location,
            date,
            candle_lighting: DEFAULT_CANDLE_LIGHTING,
            timezone: zone,
        }
    }

    /// The same, lighting candles `offset` before sunset, for example 40 minutes in Jerusalem.
    /// An offset of a day or more makes [`MyZman::CandleLighting`] return
    /// [`ZmanimError::InvalidOffset`].
    #[must_use]
    pub fn candle_lighting(mut self, offset: Duration) -> Self {
        self.candle_lighting = offset;
        self
    }

    /// The location, with its observer and horizon heights.
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// The time zone, if one was given.
    #[must_use]
    pub fn timezone(&self) -> Option<&Z> {
        self.timezone.as_ref()
    }

    /// The time zone as the calculations read it.
    fn zone(&self) -> Option<&dyn Zone> {
        self.timezone.as_ref().map(|zone| zone as &dyn Zone)
    }

    /// The date the *zmanim* are for.
    #[must_use]
    pub fn date(&self) -> Date {
        self.date
    }

    /// How long before sunset [`MyZman::CandleLighting`] is.
    #[must_use]
    pub fn candle_lighting_offset(&self) -> Duration {
        self.candle_lighting
    }

    /// The time of `zman` on this day.
    ///
    /// # Errors
    ///
    /// - [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] where the website shows no time:
    ///   the sun never reaches the angle this time, or one it is built from, needs.
    /// - [`ZmanimError::TimeZoneRequired`] when there is no time zone and the location is more
    ///   than 150° from Greenwich.
    /// - [`ZmanimError::InvalidOffset`] for candle lighting when its offset is a day or more.
    /// - [`ZmanimError::TimeOutOfRange`] on a date within a day or two of the years -9999 and
    ///   9999, when the time falls past the end of the range.
    pub fn get(&self, zman: MyZman) -> Result<Timestamp, ZmanimError> {
        use MyZman::*;
        match zman {
            Dawn90Degrees => self.rise_set_by_degrees(SolarEvent::Sunrise, DAWN_90_DEGREES),
            Dawn72Fixed => minus(self.get(SunriseLevel)?, minutes(FIXED_72_MINUTES)),
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
            Midday => midpoint(self.get(SunriseLevel)?, self.get(SunsetLevel)?),
            Mincha30Fixed => plus(self.get(Midday)?, minutes(MINCHA_30_MINUTES)),
            MinchaGra => self.proportional(SunriseLevel, SunsetLevel, 6.5),
            MinchaLechumra => {
                let fixed = self.get(Mincha30Fixed)?;
                let gra = self.get(MinchaGra)?;
                Ok(if fixed > gra { fixed } else { gra })
            }
            MinchaMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 6.5),
            KetanaGra => self.proportional(SunriseLevel, SunsetLevel, 9.5),
            KetanaMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 9.5),
            PlagGra => self.proportional(SunriseLevel, SunsetLevel, 10.75),
            PlagMA72Fixed => self.proportional(Dawn72Fixed, NIGHT_72_FIXED_LEVEL, 10.75),
            CandleLighting => minus(self.get(SunsetElevated)?, offset(self.candle_lighting)?),
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
            NightRTam72Fixed => plus(self.get(SunsetElevated)?, minutes(FIXED_72_MINUTES)),
            NightRTam72Degrees => self.rise_set_by_degrees(SolarEvent::Sunset, DAWN_72_DEGREES),
            Midnight => {
                let sunset = self.get(SunsetLevel)?;
                let tomorrow = self.date.tomorrow()?;
                let next_sunrise = self.rise_set_on(tomorrow, SolarEvent::Sunrise, false)?;
                midpoint(sunset, next_sunrise)
            }
        }
    }

    /// "Proportional hour, Gra & Baal HaTanya": a twelfth of the day from level sunrise to
    /// level sunset.
    ///
    /// It is not rounded. The proportional rows, such as [`MyZman::ShemaGra`], are cut to a whole
    /// second, so adding these hours to sunrise can differ from them by up to a second.
    ///
    /// # Errors
    ///
    /// [`ZmanimError::AllDay`] or [`ZmanimError::AllNight`] when the sun doesn't rise or set.
    pub fn proportional_hour_gra(&self) -> Result<SignedDuration, ZmanimError> {
        let sunrise = self.get(MyZman::SunriseLevel)?;
        let sunset = self.get(MyZman::SunsetLevel)?;
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
            DayBound::Zman(zman) => self.get(zman),
            // The end of the 19.75° Magen Avraham day; the website shows no row for it.
            DayBound::Night90Degrees => self.rise_set_by_degrees(SolarEvent::Sunset, DAWN_90_DEGREES),
            // The end of the fixed-72-minute Magen Avraham day: 72 minutes after level sunset.
            DayBound::Night72FixedLevel => plus(self.get(MyZman::SunsetLevel)?, minutes(FIXED_72_MINUTES)),
        }
    }

    // ---- The sun ----

    /// True sunrise or sunset, with MyZmanim's refraction and dip, cut to a whole second as the
    /// website shows it. Refraction happens where
    /// the light grazes the horizon, so it is scaled for the air at that horizon's height. That is
    /// the observer's own for the level times, and the eastern or western horizon's for the
    /// elevated.
    fn rise_set(&self, event: SolarEvent, elevated: bool) -> Result<Timestamp, ZmanimError> {
        self.rise_set_on(self.date, event, elevated)
    }

    /// [`rise_set`](Self::rise_set) on another `date` at the same location.
    fn rise_set_on(&self, date: Date, event: SolarEvent, elevated: bool) -> Result<Timestamp, ZmanimError> {
        let adjusted_date = adjusted_local_date(date, &self.location, self.zone())?;
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
        floor_second(rise_set_on_adjusted_date(
            &calculator(),
            adjusted_date,
            &self.location,
            zenith,
            event,
        )?)
    }

    /// The time the sun is `degrees` below the geometric horizon, with no refraction, radius or
    /// dip, cut to a whole second as the website shows it.
    fn rise_set_by_degrees(&self, event: SolarEvent, degrees: f64) -> Result<Timestamp, ZmanimError> {
        let adjusted_date = adjusted_local_date(self.date, &self.location, self.zone())?;
        floor_second(rise_set_on_adjusted_date(
            &calculator(),
            adjusted_date,
            &self.location,
            GEOMETRIC_ZENITH + degrees,
            event,
        )?)
    }
}

/// A start or end of a proportional day. It is a table row, or one of the two ends the website
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

/// Meeus with ΔT, without nutation, on the Earth–Moon barycenter's orbit, timing noon with the
/// Astronomical Almanac's equation of time.
const fn calculator() -> Meeus {
    Meeus {
        apply_delta_t: true,
        apply_nutation: false,
        use_earth_moon_barycenter: true,
        use_almanac_equation_of_time: true,
    }
}

/// Refraction at a horizon `horizon_elevation` meters above sea level, in degrees. It is the
/// sea-level value scaled by the barometric pressure ratio at that height.
fn refraction(horizon_elevation: f64) -> f64 {
    // Above about 44 km the formula's pressure ratio would go negative; there is no air to bend light.
    SEA_LEVEL_REFRACTION_ARCMINUTES / 60.0 * CoreFloat::powf((1.0 - 2.25577e-5 * horizon_elevation).max(0.0), 5.25588)
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

/// `hours` proportional hours into a day whose start and end are already whole seconds, cut to a
/// second.
fn proportional(start_of_day: Timestamp, end_of_day: Timestamp, hours: f64) -> Result<Timestamp, ZmanimError> {
    let start = start_of_day;
    let day_nanos = end_of_day.as_nanosecond() - start.as_nanosecond();
    // Java: `start.plusNanos((long) (dayNanos / 12 * hours))` — integer twelfth, then scaled.
    let offset = ((day_nanos / 12) as f64 * hours) as i64;
    floor_second(timestamp_from_nanos(start.as_nanosecond() + i128::from(offset))?)
}

/// The midpoint of two whole seconds, cut to a second.
fn midpoint(begin: Timestamp, end: Timestamp) -> Result<Timestamp, ZmanimError> {
    let nanos = end.as_nanosecond() - begin.as_nanosecond();
    floor_second(timestamp_from_nanos(begin.as_nanosecond() + nanos / 2)?)
}

fn floor_second(time: Timestamp) -> Result<Timestamp, ZmanimError> {
    let seconds = time.as_nanosecond().div_euclid(NANOS_PER_SECOND);
    Ok(Timestamp::from_second(seconds as i64)?)
}

fn timestamp_from_nanos(nanos: i128) -> Result<Timestamp, ZmanimError> {
    Ok(Timestamp::from_nanosecond(nanos)?)
}
