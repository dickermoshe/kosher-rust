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
//! [`zmanim`], [`limudim`] and [`myzmanim`] each have a prelude. Use [`prelude`] at the crate
//! root when an application needs more than one area.
//!
//!
//! See each module's documentation for focused examples: [`calendar`] for holidays
//! and parsha, [`zmanim`] for location-based times, [`limudim`] for learning schedules.
//!
//! # Features
//!
//! | Feature | Default | Effect |
//! |---------|---------|--------|
//! | `alloc` | yes | Zman preset descriptions; without it the crate stays `no_std` and calculation APIs are unchanged |
//! | `defmt` | no | `defmt::Format` on calculator, config, location, and error types |
//!
//! Disable default features in `Cargo.toml` when targeting embedded platforms that
//! cannot use the allocator:
//!
#![cfg_attr(not(test), no_std)]

mod astronomy;
pub mod calendar;
pub mod limudim;
mod location;
pub mod myzmanim;
pub mod zmanim;

pub use calendar::{HebrewDate, Region};
pub use location::{Location, LocationError};

/// Common imports across [`zmanim`] and [`limudim`].
///
/// Re-exports each submodule's prelude. Prefer a submodule prelude when you only
/// need one area — for example [`zmanim::prelude`] alone avoids pulling limudim
/// names into scope.
///
/// ```
/// use kosher_rust::prelude::*;
/// ```
pub mod prelude {
    pub use crate::limudim::prelude::*;
    pub use crate::zmanim::prelude::*;
}
