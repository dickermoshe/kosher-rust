//! The Hebrew calendar, with its dates, holidays, weekly *parsha* and molad.
//!
//! Convert a date to a [`HebrewDate`] and ask it questions. Methods that depend on whether Yom Tov
//! has one day or two take a [`Region`]. A `HebrewDate` covers 1 January of year 1 through the end
//! of Hebrew year 9999.
//!
//! ```
//! use jiff::civil::date;
//! use kosher_rust::calendar::{HebrewDate, SpecialDay, Parsha, Region};
//!
//! let yom_kippur = HebrewDate::try_from(date(2023, 9, 25))?;
//! assert!(yom_kippur.special_days(Region::Diaspora).any(|h| h == SpecialDay::YomKippur));
//! assert!(yom_kippur.is_assur_bemelacha(Region::Diaspora));
//!
//! let shabbos = HebrewDate::try_from(date(2024, 5, 4))?;
//! assert_eq!(shabbos.parsha(Region::Israel), Some(Parsha::AchreiMos));
//! # Ok::<(), kosher_rust::calendar::DateOutOfRange>(())
//! ```
//!
//! - [`HebrewDate`]: a day, and everything the calendar says about it.
//! - [`DateOutOfRange`]: why a date has no `HebrewDate`.
//! - [`HebrewYear`] and [`HebrewMonth`]: leap years, year and month lengths.
//! - [`SpecialDay`]: what [`HebrewDate::special_days`] yields.
//! - [`Parsha`]: what [`HebrewDate::parsha`] returns.
//! - [`Molad`]: what [`HebrewDate::molad`] returns.

mod date;
pub(crate) mod molad;
pub(crate) mod month;
mod parsha;
mod parsha_tables;
mod special_day;
pub(crate) mod year;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests;

pub use date::{DateOutOfRange, HebrewDate, Region};
pub use molad::Molad;
pub use month::HebrewMonth;
pub use parsha::Parsha;
pub use special_day::SpecialDay;
pub use year::{HebrewYear, YearLength};
