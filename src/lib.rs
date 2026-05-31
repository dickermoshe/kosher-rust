//! Rust port of [KosherJava](https://github.com/KosherJava/zmanim) for Jewish holidays,
//! halachic times (*zmanim*), and Torah/Talmud learning schedules (e.g. Daf Yomi,
//! Pirkei Avos). A `no_std` crate (optional `alloc`) for Hebrew calendar extensions
//! on [ICU4X](https://github.com/unicode-org/icu4x), location-based *zmanim*, and
//! *limudim* daily units. Uses [`icu_calendar`] and [`jiff`] throughout so results
//! compose with the wider Rust ecosystem.
//!
//! # Modules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`calendar`] | Hebrew dates, holidays, *parshiyot*, month constants, and calendar traits |
//! | [`zmanim`] | Sunrise, candle lighting, *alos*, *tzeis*, and other halachic times |
//! | [`limudim`] | Daf Yomi, Mishna Yomis, Tehillim, Pirkei Avos, and related daily units |
//!
//! Each module has its own [`calendar::prelude`], [`zmanim::prelude`], or
//! [`limudim::prelude`]. Use [`prelude`] at the crate root when an application
//! needs more than one area.
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

pub mod calendar;
pub mod limudim;
pub mod zmanim;

/// Common imports across [`calendar`], [`zmanim`], and [`limudim`].
///
/// Re-exports each submodule's prelude. Prefer a submodule prelude when you only
/// need one area — for example [`zmanim::prelude`] alone avoids pulling limudim
/// names into scope.
///
/// ```
/// use kosher_rust::prelude::*;
/// ```
pub mod prelude {
    pub use crate::calendar::prelude::*;
    pub use crate::limudim::prelude::*;
    pub use crate::zmanim::prelude::*;
}
