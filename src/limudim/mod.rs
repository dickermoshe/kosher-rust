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
//! use jiff::civil;
//! use kosher_rust::limudim::prelude::*;
//!
//! let date = civil::date(2017, 12, 28);
//! let daf = date.limud(DafYomiBavli::default()).unwrap();
//! assert_eq!(daf.tractate, Tractate::Shevuos);
//! assert_eq!(daf.page, 30);
//! ```
//!
//! Import [`prelude`] (or [`crate::prelude`]) for calculators,
//! unit types, and [`LimudCalendar`].

mod amud_yomi_bavli_dirshu;
mod daf_hashavua_bavli;
mod daf_yomi_bavli;
mod daf_yomi_yerushalmi;
mod engine;
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

/// A learning schedule: the calculators in this module, passed to [`LimudCalendar::limud`].
///
/// Sealed: only this crate's schedules implement it.
#[allow(private_bounds)]
pub trait Limud<T>: engine::InternalLimud<T> {}

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
