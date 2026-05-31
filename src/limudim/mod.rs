//! Daily Torah and Talmud learning schedules (*limudim*).
//!
//! This module maps a Hebrew calendar date to the learning unit assigned by a
//! particular program: Daf Yomi, Mishna Yomis, Pirkei Avos, monthly Tehillim,
//! and others. Each schedule is a zero-sized calculator type implementing
//! [`Limud`]; call [`LimudCalendar::limud`] on any [`HebrewCalendarDate`] to
//! look up that day's assignment.
//!
//! # Schedules
//!
//! | Calculator | Unit type | Notes |
//! |------------|-----------|-------|
//! | [`DafYomiBavli`] | [`Daf`] | Daily Bavli daf; cycle 1 began 1923-09-11 |
//! | [`DafHashavuaBavli`] | [`Daf`] | Weekly Bavli daf (Sunday–Friday); began 2005-03-06 |
//! | [`AmudYomiBavliDirshu`] | [`Amud`] | Daily Bavli amud (Dirshu); began 2023-10-16 |
//! | [`DafYomiYerushalmiVilna`] | [`Daf`] | Daily Yerushalmi daf (Vilna edition); began 1980-02-02 |
//! | [`MishnaYomis`] | [`Mishnas`] | Two mishnayos per day; began 1947-05-20 |
//! | [`PirkeiAvos`] | [`PirkeiAvosUnit`] | Summer Shabbos afternoons; Israel vs. diaspora |
//! | [`TehillimMonthly`] | [`TehillimUnit`] | Psalms divided across the Hebrew month |
//!
//! Returns `None` when no learning is scheduled (for example, before a cycle
//! starts, on Shabbat for weekly programs, or outside Pirkei Avos season).
//!
//! # Quick start
//!
//! ```
//! use icu_calendar::{Date, cal::Hebrew};
//! use kosher_rust::limudim::prelude::*;
//!
//! let date = Date::try_new_gregorian(2017, 12, 28).unwrap();
//! let daf = date.limud(DafYomiBavli::default()).unwrap();
//! assert_eq!(daf.tractate, Tractate::Shevuos);
//! assert_eq!(daf.page, 30);
//! ```
//!
//! Import [`prelude`] (or [`crate::prelude`]) for calculators,
//! unit types, and [`LimudCalendar`].

use icu_calendar::{
    Date,
    cal::Hebrew,
    options::{DateAddOptions, DateDifferenceOptions},
    types::DateDuration,
};

mod amud_yomi_bavli_dirshu;
mod cycle;
mod daf_hashavua_bavli;
mod daf_yomi_bavli;
mod daf_yomi_yerushalmi;
mod interval;
mod limud;
mod mishna_yomis;
mod pirkei_avos;
mod tehillim_monthly;
mod units;

/// Looks up the scheduled daily limud for a calendar date.
///
/// Implemented for any type that can be viewed as a Hebrew date (ICU, Jiff, etc.).
/// Pass a [`Limud`] such as [`DafYomiBavli`] to select the schedule.
pub trait LimudCalendar {
    /// Calculate the limud (learning unit) for this date using the given calculator.
    ///
    /// # Arguments
    /// * `limud_calculator` - A calculator implementing the [`Limud`] trait
    ///
    /// # Returns
    /// The learning unit for this date, or `None` if no learning is scheduled
    fn limud<U>(&self, limud: impl Limud<U>) -> Option<U>;
}
impl<T> LimudCalendar for T
where
    T: HebrewCalendarDate,
{
    fn limud<U>(&self, limud: impl Limud<U>) -> Option<U> {
        limud.limud(self.hebrew_date())
    }
}
pub use amud_yomi_bavli_dirshu::AmudYomiBavliDirshu;
pub use daf_hashavua_bavli::DafHashavuaBavli;
pub use daf_yomi_bavli::DafYomiBavli;
pub use daf_yomi_yerushalmi::DafYomiYerushalmiVilna;
pub use mishna_yomis::{MishnaYomis, Mishnas};
pub use pirkei_avos::{PirkeiAvos, PirkeiAvosUnit};
pub use tehillim_monthly::{TehillimMonthly, TehillimUnit};

pub use units::{Amud, Daf, Mishna, Side, Tractate};

pub use limud::Limud;

use crate::calendar::HebrewCalendarDate;

/// Common limudim imports.
///
/// Import this module to bring the limud extension trait, calculators, unit
/// types, and public schedule constants into scope.
pub mod prelude {
    pub use super::{
        Amud, AmudYomiBavliDirshu, Daf, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmiVilna, Limud, LimudCalendar,
        Mishna, MishnaYomis, Mishnas, PirkeiAvos, PirkeiAvosUnit, Side, TehillimMonthly, TehillimUnit, Tractate,
    };
}

pub(crate) trait HebrewDateExt {
    /// Return a copy of this date with `days` added.
    fn add_days(&self, days: i32) -> Option<Date<Hebrew>>;
    /// Calculate the number of days between two Hebrew dates, inclusive of partial spans.
    ///
    /// Returns `None` when `self > end`, or when ICU cannot compute the difference
    /// (e.g. dates outside the supported Hebrew calendar range).
    fn days_until(&self, end: &Date<Hebrew>) -> Option<u32>;
}

impl HebrewDateExt for Date<Hebrew> {
    fn add_days(&self, days: i32) -> Option<Date<Hebrew>> {
        self.try_added_with_options(DateDuration::for_days(days), DateAddOptions::default())
            .ok()
    }

    fn days_until(&self, end: &Date<Hebrew>) -> Option<u32> {
        if self > end {
            return None;
        }
        match self.try_until_with_options(end, DateDifferenceOptions::default()) {
            Ok(duration) => Some(duration.days),
            Err(error) => match error {},
        }
    }
}

/// Total number of amudim (half-pages) in the Babylonian Talmud for Dirshu
pub(crate) const BAVLI_TOTAL_AMUDIM: i32 = 5407;

/// Number of dafim in Daf Yomi Bavli cycles 1-7 (before Shekalim expansion)
pub(crate) const BAVLI_DAF_COUNT_EARLY: i32 = 2702;

/// Number of dafim in Daf Yomi Bavli cycles 8+ (after Shekalim expansion)
pub(crate) const BAVLI_DAF_COUNT_MODERN: i32 = 2711;

/// Number of dafim in the Yerushalmi Talmud
pub(crate) const YERUSHALMI_DAF_COUNT: i32 = 1554;

/// Number of days in a Mishna Yomis cycle.
///
/// This is the zero-based offset from cycle start to the last inclusive day
/// (4192 mishnas at 2 per day → 2096 days, so last day = start + 2095).
pub(crate) const MISHNA_YOMIS_CYCLE_DAYS: i32 = 2095;

/// Cycle number at which Shekalim expanded from 13 to 22 pages
pub(crate) const SHEKALIM_EXPANSION_CYCLE: i32 = 8;

#[cfg(test)]
#[allow(clippy::expect_used)]
/// A helper function to convert a Gregorian date to a Hebrew date.
/// This is only allowed in test code.
pub(crate) fn from_gregorian_date(year: i32, month: u8, day: u8) -> Date<Hebrew> {
    Date::try_new_gregorian(year, month, day)
        .expect("hard-coded Gregorian date should be valid")
        .to_calendar(Hebrew)
}
