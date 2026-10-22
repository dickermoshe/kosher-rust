#![doc = include_str!("../README.md")]
//!
//! ## Modules
//!
//! | Module | For |
//! |---|---|
//! | [`calendar`] | [`HebrewDate`]: holidays, the weekly *parsha*, year and month lengths, the molad |
//! | [`zmanim`] | [`Zmanim`] and [`Zman`]: halachic times for a [`Location`] |
//! | [`limudim`] | Daf Yomi and the other learning schedules, through [`limudim::Schedule`] |
//! | [`myzmanim`] | The times of the MyZmanim website, reproduced to the second |
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod astronomy;
pub mod calendar;
pub mod limudim;
mod location;
pub mod myzmanim;
pub mod zmanim;

pub use calendar::{HebrewDate, Region};
pub use location::{Location, LocationError, TimeZoneLike};
pub use zmanim::{Zman, Zmanim};
