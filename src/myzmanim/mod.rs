//! The *zmanim* shown on [MyZmanim](https://www.myzmanim.com/)'s website. Almost all of the
//! calculated times are within one second of the times on the site.
//!
//! Use it when your users compare your times with that site. It is separate from
//! [`zmanim`](crate::zmanim) and differs from it slightly, because the site uses its own
//! refraction, horizon dip and rounding. Give [`MyZmanim`] a [`Location`](crate::Location) with
//! its elevation and horizons and a date; every row of the site's "All zmanim" table is a
//! [`MyZman`].
//!
//! MyZmanim does not publish its methods. Everything here was read off that table. At 245 places
//! on 135 days from September 2026 to August 2027 it reproduces the table within a second 99.6%
//! of the time and within two seconds 99.9% of the time. The rules, as measured:
//!
//! - The sun's declination is Jean Meeus's VSOP87 algorithm with ΔT, but with nutation off and
//!   the Earth's monthly motion around the Earth–Moon barycenter left out.
//! - Noon comes from the 2002 Astronomical Almanac's low-precision equation of time. The Almanac
//!   rates its formulas from 1950 to 2050, so expect these times to drift from the true sun
//!   outside those years.
//! - Refraction at the horizon is [`SEA_LEVEL_REFRACTION_ARCMINUTES`] (not 34′), scaled by
//!   the air pressure at the height of the horizon the sun is seen over. That is the observer's
//!   own height for the "level region" times, and the eastern or western horizon's height for
//!   the "elevated" times.
//! - The dip of the horizon for a raised observer is [`DIP_DEGREES_PER_ROOT_METER`]·√h
//!   (*Ma'aglay Tzedek*), not the geometric `acos(R / (R + h))`.
//! - "90 minutes as degrees" is [`DAWN_90_DEGREES`] (19.75°, not 19.8°); the Gra's nightfalls
//!   are 3.65°, 4.37° and 4.61°; R' Moshe Feinstein's end of fasts 7.12°, Sefer Moed Moadim
//!   8°, the Chazon Ish 9.28°.
//! - Candle lighting and R' Tam's fixed 72 minutes count from the sunset with heights; the
//!   72-minute Magen Avraham day still ends 72 minutes after the level sunset.
//! - Every time from the sun is cut, not rounded, to a whole second. Proportional *zmanim*,
//!   midday and midnight are built from those whole seconds and cut to a whole second again.
//!
//! ```
//! use core::time::Duration;
//! use jiff::{civil::date, tz::TimeZone};
//! use kosher_rust::Location;
//! use kosher_rust::myzmanim::{MyZman, MyZmanim};
//!
//! let jerusalem = Location::new(31.778, 35.235)?
//!     .with_elevation(754.0)?
//!     .with_horizons(754.0, 0.0)?;
//! let times = MyZmanim::new(jerusalem, date(2026, 11, 6), Some(TimeZone::get("Asia/Jerusalem")?))
//!     .candle_lighting(Duration::from_mins(40));
//! assert!(times.get(MyZman::CandleLighting)? < times.get(MyZman::SunsetElevated)?);
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

/// The dip of the horizon below an observer `h` meters above it, in degrees per root meter, measured.
pub const DIP_DEGREES_PER_ROOT_METER: f64 = 0.0347;

/// MyZmanim's default time before sunset for candle lighting, 18 minutes.
pub const DEFAULT_CANDLE_LIGHTING: Duration = Duration::from_mins(18);
