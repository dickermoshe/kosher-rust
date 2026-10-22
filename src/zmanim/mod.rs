//! Halachic times (*zmanim*) for a place and a day: sunrise, the latest time for Shema, candle
//! lighting, nightfall, and the rest of KosherJava's catalog.
//!
//! ```
//! use jiff::{civil::date, tz::TimeZone};
//! use kosher_rust::{Location, Zman, Zmanim};
//!
//! let tz = TimeZone::get("America/New_York")?;
//! let zmanim = Zmanim::new(Location::new(40.09, -74.22)?, date(2024, 4, 23), Some(tz));
//!
//! for zman in Zman::ALL {
//!     if let Ok(time) = zmanim.get_zoned(zman) {
//!         println!("{:<45} {}", zman.name(), time.strftime("%H:%M:%S"));
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! - [`Zmanim`]: the times for a [`Location`](crate::Location) and a date.
//! - [`Zman`]: every time it can calculate, named after KosherJava's methods.
//! - [`Config`]: the solar algorithm, elevation, *chatzos* and candle-lighting settings.
//! - [`ZmanimError`]: why a time doesn't exist that day.

mod calculator;
mod catalog;
mod config;
mod error;
mod formula;
mod zman;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "alloc"))]
mod catalog_snapshot;

pub use calculator::{Zmanim, shaah_zmanis};
pub(crate) use calculator::{minus, plus};
pub use catalog::Zman;
pub use config::{AstronomicalCalculator, Config};
pub use error::ZmanimError;
pub use zman::Category;
