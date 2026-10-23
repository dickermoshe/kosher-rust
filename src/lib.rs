#![doc = include_str!("../README.md")]
//!
//! ## Modules
//!
//! | Module | For |
//! |---|---|
//! | [`calendar`] | [`HebrewDate`]: holidays, the weekly *parsha*, year and month lengths, the molad |
//! | [`zmanim`] | [`Zmanim`] and [`Zman`]: halachic times for a [`Location`] |
//! | [`limudim`] | Daf Yomi and the other learning schedules, through [`limudim::Schedule`] |
//! | [`myzmanim`] | The times shown on the MyZmanim website. Almost all of them are within one second of the site |
#![cfg_attr(not(test), no_std)]
// Float functions are called as `CoreFloat::sin(x)`, not `x.sin()`. Where std is linked, as in
// unit tests, `x.sin()` resolves to std's version instead of the one the crate ships with.

#[cfg(feature = "alloc")]
extern crate alloc;

mod astronomy;
pub mod calendar;
pub mod limudim;
mod location;
pub mod myzmanim;
pub mod zmanim;

pub use calendar::{HebrewDate, Region};
pub use location::{Location, LocationError};
pub use zmanim::{Zman, Zmanim};
