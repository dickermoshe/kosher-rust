//! Hebrew calendar and daily limud schedules.
#![cfg_attr(not(test), no_std)]

/// Hebrew calendar extensions for [`icu_calendar`] and [`jiff`] dates.
pub mod calendar;
/// Daily *limud* (Torah study) schedules extensions for [`icu_calendar`] and [`jiff`] dates.
pub mod limudim;
/// Halachic time (*zmanim*) calculations for a location and date.
pub mod zmanim;

/// Common imports for the library.
pub mod prelude {
    pub use crate::calendar::prelude::*;
    pub use crate::limudim::prelude::*;
    pub use crate::zmanim::prelude::*;
}
