//! The Hebrew calendar: dates, holidays, the weekly *parsha* and the molad.
//!
//! Convert any date to a [`HebrewDate`] and ask it questions. Methods that depend on whether
//! Yom Tov has one day or two take a [`Region`].
//!
//! ```
//! use jiff::civil::date;
//! use kosher_rust::calendar::{HebrewDate, Holiday, Parsha, Region};
//!
//! let yom_kippur = HebrewDate::from(date(2023, 9, 25));
//! assert!(yom_kippur.holidays(Region::Diaspora).any(|h| h == Holiday::YomKippur));
//! assert!(yom_kippur.is_assur_bemelacha(Region::Diaspora));
//!
//! let shabbos = HebrewDate::from(date(2024, 5, 4));
//! assert_eq!(shabbos.parsha(Region::Israel), Some(Parsha::AchreiMos));
//! ```
//!
//! - [`HebrewDate`]: a day, and everything the calendar says about it.
//! - [`HebrewYear`] and [`HebrewMonth`]: leap years, year and month lengths.
//! - [`Holiday`]: what [`HebrewDate::holidays`] yields.
//! - [`Parsha`]: what [`HebrewDate::parsha`] returns.
//! - [`Molad`]: what [`HebrewDate::molad`] returns.

mod date;
mod holiday;
pub(crate) mod molad;
pub(crate) mod month;
mod parsha;
mod parsha_tables;
mod year;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests;

pub use date::{HebrewDate, Region};
pub use holiday::Holiday;
pub use molad::Molad;
pub use month::HebrewMonth;
pub use parsha::Parsha;
pub use year::{HebrewYear, YearLength};
