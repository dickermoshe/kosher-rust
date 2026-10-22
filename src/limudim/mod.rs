//! Learning schedules (*limudim*): what Daf Yomi, Mishna Yomis, Tehillim and the others assign
//! on a given day.
//!
//! Every schedule implements [`Schedule`]: call [`on`](Schedule::on) with a date and get that
//! day's portion, or `None` when nothing is scheduled (before the program began, on Shabbos for a
//! weekly program, outside the Pirkei Avos season).
//!
//! ```
//! use jiff::civil::date;
//! use kosher_rust::limudim::{DafYomiBavli, MishnaYomis, Schedule};
//!
//! let day = date(2024, 4, 23);
//! assert_eq!(DafYomiBavli.on(day).unwrap().to_string(), "Bava Metzia 55");
//! assert_eq!(MishnaYomis.on(day).unwrap().to_string(), "Nazir 5:5-6");
//! ```
//!
//! | Schedule | Gives |
//! |---|---|
//! | [`DafYomiBavli`] | [`Daf`] |
//! | [`DafHashavuaBavli`] | [`Daf`] |
//! | [`AmudYomiDirshu`] | [`Amud`] |
//! | [`DafYomiYerushalmi`] | [`Daf`] |
//! | [`MishnaYomis`] | [`MishnaPair`] |
//! | [`TehillimMonthly`] | [`TehillimUnit`] |
//! | [`PirkeiAvos`] | [`PirkeiAvosUnit`] |

mod amud_yomi_dirshu;
mod daf_hashavua_bavli;
mod daf_yomi_bavli;
mod daf_yomi_yerushalmi;
mod engine;
mod mishna_yomis;
mod pirkei_avos;
mod tehillim_monthly;
mod units;

pub use amud_yomi_dirshu::AmudYomiDirshu;
pub use daf_hashavua_bavli::DafHashavuaBavli;
pub use daf_yomi_bavli::DafYomiBavli;
pub use daf_yomi_yerushalmi::DafYomiYerushalmi;
pub use mishna_yomis::{MishnaPair, MishnaYomis};
pub use pirkei_avos::{PirkeiAvos, PirkeiAvosUnit};
pub use tehillim_monthly::{TehillimMonthly, TehillimUnit};
pub use units::{Amud, Daf, Mishna, Side, Tractate};

use crate::calendar::HebrewDate;

/// A learning program: maps a date to that day's portion.
///
/// Implemented by the schedules in this module, and only by them.
pub trait Schedule: sealed::Sealed {
    /// What one day's portion is: a [`Daf`], an [`Amud`], a [`MishnaPair`], and so on.
    type Unit;

    /// The portion scheduled for `date`, which may be a [`jiff::civil::Date`], an ICU date or a
    /// [`HebrewDate`].
    ///
    /// Returns `None` when nothing is scheduled that day.
    fn on(&self, date: impl Into<HebrewDate>) -> Option<Self::Unit>;
}

mod sealed {
    pub trait Sealed {}
}

/// Implements [`Schedule`] for a type that implements the private cycle engine.
macro_rules! schedule {
    ($schedule:ty => $unit:ty) => {
        impl $crate::limudim::sealed::Sealed for $schedule {}

        impl $crate::limudim::Schedule for $schedule {
            type Unit = $unit;

            fn on(&self, date: impl Into<$crate::calendar::HebrewDate>) -> Option<$unit> {
                $crate::limudim::engine::InternalLimud::limud(self, date.into().icu())
            }
        }
    };
}
use schedule;
