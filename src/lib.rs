//! Rust port of [KosherJava](https://github.com/KosherJava/zmanim) for Jewish holidays,
//! halachic times (*zmanim*), and Torah/Talmud learning schedules (e.g. Daf Yomi,
//! Pirkei Avos). A `no_std` crate (optional `alloc`) for Hebrew calendar extensions,
//! location-based *zmanim*, and *limudim* calculators.
//!
//! # Modules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`calendar`] | Hebrew dates, holidays, the weekly *parsha*, and the molad |
//! | [`zmanim`] | Sunrise, candle lighting, *alos*, *tzeis*, and other halachic times |
//! | [`limudim`] | Daf Yomi, Mishna Yomis, Tehillim, Pirkei Avos, and related daily units |
//! | [`myzmanim`] | The *zmanim* of MyZmanim's website, reproduced to the second, as a self-contained calendar |
//!
//! The types you need most are at the crate root: [`HebrewDate`], [`Region`], [`Location`],
//! [`Zmanim`] and [`Zman`]. Each module's page has a short example.
//!
//! # Features
//!
//! | Feature | Default | Effect |
//! |---------|---------|--------|
//! | `alloc` | yes | `Zman::description`, a paragraph explaining each zman to your users |
//! | `defmt` | no | `defmt::Format` on calculator, config, location, and error types |
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
pub use location::{Location, LocationError};
pub use zmanim::{Zman, Zmanim};
