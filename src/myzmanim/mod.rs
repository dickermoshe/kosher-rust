//! The *zmanim* shown by [MyZmanim](https://www.myzmanim.com/), on its original website or in
//! its web app. Almost all of the calculated times are within one second of MyZmanim's.
//!
//! Use it when your users compare your times with MyZmanim. It is separate from
//! [`zmanim`](crate::zmanim) and differs from it slightly, because MyZmanim uses its own
//! refraction, horizon dip and rounding. Give [`MyZmanim`] a [`Location`](crate::Location) with
//! its elevation and horizons and a date; every row of MyZmanim's "All zmanim" table is a
//! [`MyZman`]. By default the times match the web app. Call
//! [`matching(Source::LegacySite)`](MyZmanim::matching) to match the original website, which uses
//! slightly different angles and rounding, as [`Source`] lists.
//!
//! MyZmanim does not publish its methods. Everything here was read off its times. At 245 places
//! on 135 days from September 2026 to August 2027 it reproduces the website within a second
//! 99.89% of the time and within two seconds 99.999% of the time. At 746 places on every day from
//! November 2026 to November 2027 it reproduces the web app within a second 99.991% of the time
//! and within two seconds 99.998% of the time. The rules, as measured:
//!
//! - The sun's declination is Jean Meeus's VSOP87 algorithm with ΔT, but with nutation off and
//!   the Earth's monthly motion around the Earth–Moon barycenter left out.
//! - Noon comes from the 2002 Astronomical Almanac's low-precision equation of time. The Almanac
//!   rates its formulas from 1950 to 2050, so expect these times to drift from the true sun
//!   outside those years.
//! - Refraction at the horizon is [`SEA_LEVEL_REFRACTION_ARCMINUTES`] (not 34′), scaled by
//!   p^[`REFRACTION_PRESSURE_EXPONENT`], where p is the air pressure at the height of the horizon
//!   the sun is seen over, as a fraction of the pressure at sea level. That is the observer's
//!   own height for the "level region" times, and the eastern or western horizon's height for
//!   the "elevated" times.
//! - The dip of the horizon for a raised observer `h` meters above it is
//!   [`DIP_DEGREES_PER_ROOT_METER`]·√h·p^[`DIP_PRESSURE_EXPONENT`], where p is the air pressure
//!   at the observer's height as a fraction of the pressure at sea level. The dip shrinks
//!   slightly as the observer gets higher. Both numbers were fitted to MyZmanim's web app at 10
//!   Israeli places with surveyed horizons, observers 12 m to 800 m high, and then checked at 9
//!   other such places. There every elevated sunset was within a second on 184 days. Observers
//!   above 800 m were not checked.
//! - On the website, "90 minutes as degrees" is [`DAWN_90_DEGREES`] (19.75°, not 19.8°); the
//!   Gra's nightfalls are 3.65°, 4.37° and 4.61°; R' Moshe Feinstein's end of fasts 7.12°, Sefer
//!   Moed Moadim 8°, the Chazon Ish 9.28°. The web app's angles differ slightly; [`Source`]
//!   lists them.
//! - Candle lighting and R' Tam's fixed 72 minutes count from the sunset with heights; the
//!   72-minute Magen Avraham day still ends 72 minutes after the level sunset.
//! - Every time from the sun is cut, not rounded, to a whole second. On the website,
//!   proportional *zmanim*, midday and midnight are built from those whole seconds and cut to a
//!   whole second again. The web app builds them from the exact times and cuts only the result.
//!
//! ```
//! use core::time::Duration;
//! use jiff::{civil::date, tz::TimeZone};
//! use kosher_rust::Location;
//! use kosher_rust::myzmanim::{MyZman, MyZmanim, Source};
//!
//! let jerusalem = Location::new(31.778, 35.235)?
//!     .with_elevation(754.0)?
//!     .with_horizons(754.0, 0.0)?;
//! let times = MyZmanim::new(jerusalem, date(2026, 11, 6), Some(TimeZone::get("Asia/Jerusalem")?))
//!     .candle_lighting(Duration::from_mins(40));
//! assert!(times.get(MyZman::CandleLighting)? < times.get(MyZman::SunsetElevated)?);
//!
//! // The same place on the original website, whose 16.1° dawn is earlier than the web app's
//! // 16.053°.
//! let site = times.clone().matching(Source::LegacySite);
//! assert!(site.get(MyZman::Dawn72Degrees)? < times.get(MyZman::Dawn72Degrees)?);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use core::time::Duration;

mod myzman;

pub use myzman::{MyZman, MyZmanim};

/// The sun's depression for "90 minutes as degrees": 19.75°, measured from the website's times.
pub const DAWN_90_DEGREES: f64 = 19.75;

/// The sun's depression for "72 minutes as 16.1 degrees", dawn and R' Tam's nightfall.
pub const DAWN_72_DEGREES: f64 = 16.1;

/// Refraction at the horizon at sea level, in arc-minutes: 34′ scaled from 1010 mb to 1000 mb, measured.
pub const SEA_LEVEL_REFRACTION_ARCMINUTES: f64 = 33.66;

/// The power of the air-pressure ratio at the horizon's height that scales the refraction there.
/// Measured.
pub const REFRACTION_PRESSURE_EXPONENT: f64 = 1.10243;

/// The dip of the horizon below an observer `h` meters above it, in degrees per root meter, for
/// an observer at sea level. Measured.
pub const DIP_DEGREES_PER_ROOT_METER: f64 = 0.035130;

/// The power of the observer's air-pressure ratio that scales the dip. Measured.
pub const DIP_PRESSURE_EXPONENT: f64 = 0.28890;

/// MyZmanim's default time before sunset for candle lighting, 18 minutes.
pub const DEFAULT_CANDLE_LIGHTING: Duration = Duration::from_mins(18);

/// Which of MyZmanim's front ends the times match.
///
/// MyZmanim shows its times in two places: the original website and the newer web app. Both
/// calculate the sun the same way, with the same refraction, dip, heights, candle lighting and
/// fixed-minute times. They differ in two things.
///
/// The rows measured in degrees have the same names in both, but the web app uses slightly
/// different angles. They were measured from the web app's own times: for each row, place and day
/// we solved for the angle that gives its time exactly, and each row came out as one fixed angle
/// at places from Reykjavík to Melbourne.
///
/// | Row | Website | Web app |
/// |---|---|---|
/// | Dawn, 90 minutes as degrees | 19.75° | 19.860° |
/// | The end of that Magen Avraham day | 19.75° | 19.885° |
/// | Dawn, 72 minutes as 16.1 degrees | 16.1° | 16.053° |
/// | R' Tam's nightfall, 72 minutes as 16.1 degrees | 16.1° | 16.073° |
/// | Talis and tefillin, 11.5 degrees | 11.5° | 11.496° |
/// | Talis and tefillin, 10.2 degrees | 10.2° | 10.129° |
/// | The Gra's nightfalls | 3.65°, 4.37°, 4.61° | 3.647°, 4.365°, 4.604° |
/// | The end of ordained fasts, R' Tukaccinsky | 6.45° | 6.443° |
/// | The end of ordained fasts, R' Moshe Feinstein | 7.12° | 7.108° |
/// | Three stars emerge, the Chazon Ish | 9.28° | 9.281° |
///
/// Talis at 11°, R' Schneur Zalman's 5.95°, Sefer Moed Moadim's 8° and the end of Shabbos at 8.5°
/// are the same in both.
///
/// The other difference is how times built from other times are rounded. The proportional times,
/// midday and midnight are built from sunrise, sunset and dawn. The website first cuts those to
/// whole seconds, and the web app uses the exact times. Either way the result is cut to a whole
/// second, so the two differ by at most a second there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Source {
    /// The times in MyZmanim's web app. This is the default.
    #[default]
    WebApp,
    /// The times on MyZmanim's original website, [www.myzmanim.com](https://www.myzmanim.com/).
    LegacySite,
}

impl Source {
    /// The angles and arithmetic of this front end.
    const fn rules(self) -> &'static Rules {
        match self {
            Self::LegacySite => &WEBSITE_RULES,
            Self::WebApp => &WEB_APP_RULES,
        }
    }
}

/// What differs between the website's calculation and the web app's. One thing is the sun's
/// depression, in degrees, for each row measured in degrees; the two give these rows the same
/// names. The other is whether times built from other times start from whole seconds.
#[derive(Debug, PartialEq)]
struct Rules {
    /// True when proportional times, midday and midnight are built from sunrise, sunset and dawn
    /// already cut to whole seconds, as the website does. The web app builds them from the exact
    /// times. Either way the result is cut to a whole second.
    whole_second_bounds: bool,
    /// Dawn, "90 minutes as degrees", and the start of that Magen Avraham day.
    dawn_90: f64,
    /// The end of the "90 minutes as degrees" Magen Avraham day.
    night_90: f64,
    /// Dawn, "72 minutes as 16.1 degrees", and the start of that Magen Avraham day.
    dawn_72: f64,
    /// R' Tam's nightfall, "72 minutes as 16.1 degrees", and the end of that Magen Avraham day.
    night_72: f64,
    /// Earliest talis and tefillin, "11.5 degrees".
    talis_11_5: f64,
    /// Earliest talis and tefillin, "11 degrees".
    talis_11: f64,
    /// Earliest talis and tefillin, "10.2 degrees".
    talis_10_2: f64,
    /// The Gra's nightfall, 13 1/2 minutes as degrees.
    night_gra_18: f64,
    /// The Gra's nightfall, 16 7/8 minutes as degrees.
    night_gra_22_5: f64,
    /// The Gra's nightfall, 18 minutes as degrees.
    night_gra_24: f64,
    /// The Gra's nightfall, 24 minutes as degrees, R' Schneur Zalman.
    night_zalman: f64,
    /// The end of ordained fasts, R' Tukaccinsky.
    night_fast_tukaccinsky: f64,
    /// The end of ordained fasts, R' Moshe Feinstein.
    night_fast_feinstein: f64,
    /// Three stars emerge, Sefer Moed Moadim.
    night_moed: f64,
    /// Three stars emerge, the end of Shabbos and Yom Tov.
    night_shabbos: f64,
    /// Three stars emerge, the Chazon Ish.
    night_chazon_ish: f64,
}

/// The website's rules, measured from its "All zmanim" table.
const WEBSITE_RULES: Rules = Rules {
    whole_second_bounds: true,
    dawn_90: DAWN_90_DEGREES,
    night_90: DAWN_90_DEGREES,
    dawn_72: DAWN_72_DEGREES,
    night_72: DAWN_72_DEGREES,
    talis_11_5: 11.5,
    talis_11: 11.0,
    talis_10_2: 10.2,
    night_gra_18: 3.65,
    night_gra_22_5: 4.37,
    night_gra_24: 4.61,
    night_zalman: 5.95,
    night_fast_tukaccinsky: 6.45,
    night_fast_feinstein: 7.12,
    night_moed: 8.0,
    night_shabbos: 8.5,
    night_chazon_ish: 9.28,
};

/// The web app's rules, measured from its times.
const WEB_APP_RULES: Rules = Rules {
    whole_second_bounds: false,
    dawn_90: 19.860,
    night_90: 19.885,
    dawn_72: 16.053,
    night_72: 16.073,
    talis_11_5: 11.496,
    talis_11: 11.0,
    talis_10_2: 10.129,
    night_gra_18: 3.647,
    night_gra_22_5: 4.365,
    night_gra_24: 4.604,
    night_zalman: 5.95,
    night_fast_tukaccinsky: 6.443,
    night_fast_feinstein: 7.108,
    night_moed: 8.0,
    night_shabbos: 8.5,
    night_chazon_ish: 9.281,
};
