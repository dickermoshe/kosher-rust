//! Halachic time calculations (*zmanim*) for a geographic location and civil date.
//!
//! This module computes Jewish prayer and ritual times — sunrise, sunset, *alos*,
//! *tzeis*, *chatzos*, candle lighting, and dozens of other presets aligned with
//! [KosherJava](https://github.com/KosherJava/zmanim) method names. Results are
//! [`jiff::Timestamp`] values in UTC; convert to local time with the location's
//! timezone when presenting them to users.
//!
//! # How it fits together
//!
//! 1. Build a [`Location`]: latitude, longitude, optional time zone, then
//!    elevation and horizon heights.
//! 2. Create a [`ZmanimCalculator`] for that location, a [`jiff::civil::Date`], and
//!    a [`CalculatorConfig`] that selects elevation and *chatzos* behavior and the
//!    [`AstronomicalCalculator`] algorithm (NOAA by default, or Meeus / SPA).
//! 3. Pass any [`ZmanLike`] value to [`ZmanimCalculator::calculate`].
//!
//! Most callers use the ready-made constants in [`presets`] (for example
//! [`presets::ELEVATION_ADJUSTED_SUNRISE`]). Each preset is a [`ZmanPreset`] backed
//! by a low-level [`ZmanPrimitive`] expression and tagged with a [`ZmanType`].
//!
//! Failures return [`ZmanimError`] — unlike calendar or limud lookups, which use
//! `Option` when a value simply does not apply on a given date. Some zmanim cannot
//! be calculated at extreme latitudes or on polar days; others require a timezone
//! on the location (for example the Kiddush Levana times).
//!
//! # Submodules
//!
//! - [`presets`] — named zman constants (`ELEVATION_ADJUSTED_SUNRISE`, `CANDLE_LIGHTING`, …)
//! - [`CalculatorConfig`] options (elevation, *chatzos*, offsets) and the
//!   [`AstronomicalCalculator`] solar algorithm selector (NOAA, Meeus, SPA)
//! - [`ZmanimError`], returned when a time does not exist for that place and date
//! - [`primitives`] — [`ZmanPrimitive`] building blocks for custom zman definitions
//!
//! # Quick start
//!
//! ```
//! use jiff::{civil::Date, tz::TimeZone};
//! use kosher_rust::zmanim::prelude::*;
//!
//! let location = Location::new(40.09, -74.22, Some(TimeZone::get("America/New_York").unwrap())).unwrap();
//! let calc = ZmanimCalculator::new(
//!     location,
//!     Date::new(2017, 10, 17).unwrap(),
//!     CalculatorConfig::default(),
//! );
//! calc.calculate(&presets::ELEVATION_ADJUSTED_SUNRISE).unwrap();
//! ```
//!
//! Import [`prelude`] (or [`crate::prelude`]) for calculator types, the [`presets`]
//! module, and [`ZmanPrimitive`] building blocks.

use jiff::{Timestamp, civil::Date};

mod config;
mod error;
pub mod presets;

pub use config::{AstronomicalCalculator, CalculatorConfig};
pub use error::ZmanimError;

pub mod primitives;

/// Common zmanim imports.
///
/// Import this module to bring the calculator types, location and config types,
/// preset constants, and primitive building blocks into scope.
pub mod prelude {
    pub use super::presets;
    pub use super::primitives::ZmanPrimitive;
    pub use super::{AstronomicalCalculator, CalculatorConfig, ZmanimError};
    pub use super::{ZmanLike, ZmanPreset, ZmanType, ZmanimCalculator};
    pub use crate::Location;
}

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "alloc"))]
mod catalog_snapshot;

use crate::{Location, zmanim::primitives::ZmanPrimitive};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Calculates zmanim for a [`Location`] on a specific [`Date`].
///
/// Create one calculator for a location and date, then pass any [`ZmanLike`]
/// value to [`ZmanimCalculator::calculate`].
///
/// Most callers should use the ready-made definitions in [`presets`],
/// such as `presets::ELEVATION_ADJUSTED_SUNRISE`, instead of writing custom zman logic.
#[derive(Clone, Debug)]
pub struct ZmanimCalculator {
    /// The location to calculate for.
    pub(crate) location: Location,
    /// The civil date at the configured location.
    pub(crate) date: Date,
    /// Options that control calculation behavior.
    pub(crate) config: CalculatorConfig,
}

impl ZmanimCalculator {
    /// Creates a calculator for the given `location`, `date`, and `config`.
    ///
    /// Use this before calculating zmanim for that location and date.
    pub fn new(location: Location, date: Date, config: CalculatorConfig) -> Self {
        Self { location, date, config }
    }

    /// Calculates a single zman.
    pub fn calculate(&self, zman: &impl ZmanLike) -> Result<Timestamp, ZmanimError> {
        zman.calculate(self)
    }
}

/// A zman definition that can be calculated by a [`ZmanimCalculator`].
///
/// Prefer the predefined values in [`presets`]. Implement this trait
/// only when you need a custom zman definition.
pub trait ZmanLike {
    /// Calculates this zman with the given calculator.
    ///
    /// Custom zman definitions should put their calculation logic here.
    fn calculate(&self, calculator: &ZmanimCalculator) -> Result<Timestamp, ZmanimError>;
}
/// Broad category of a zman preset (alos, tzais, sof zman shma, …).
///
/// Every generated [`ZmanPreset`] carries one of these values so callers can
/// group or filter presets by kind without parsing method names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmanType {
    /// Start and end of astronomical, civil, and nautical twilight.
    Twilight,
    /// Dawn (*alos*).
    Alos,
    /// Earliest time for tallis and tefillin (*misheyakir*).
    Misheyakir,
    /// Sunrise (*netz*).
    Netz,
    /// Latest time for the morning Shema (*sof zman shma*).
    SofZmanShma,
    /// Latest time for morning prayer (*sof zman tefila*).
    SofZmanTefila,
    /// Latest time to eat chametz on Erev Pesach.
    SofZmanAchilasChametz,
    /// Latest time to burn chametz on Erev Pesach.
    SofZmanBiurChametz,
    /// Midday (*chatzos hayom*).
    ChatzosHayom,
    /// Earliest Mincha (*mincha gedola*).
    MinchaGedola,
    /// Plag hamincha.
    PlagHamincha,
    /// Immediately before *mincha ketana* (*samuch le-mincha ketana*).
    SamuchLeMinchaKetana,
    /// Preferred Mincha time (*mincha ketana*).
    MinchaKetana,
    /// Twilight between sunset and nightfall (*bein hashmashos*).
    BeinHashmashos,
    /// Candle lighting before Shabbat or Yom Tov.
    CandleLighting,
    /// Sunset (*shkiya*).
    Shkiya,
    /// Nightfall (*tzais*).
    Tzais,
    /// Kiddush Levana window bounds.
    KidushLevana,
    /// Midnight (*chatzos halayla*).
    ChatzosHalayla,
}

/// A named zman preset backed by a low-level [`ZmanPrimitive`].
///
/// Most callers should use presets directly instead of constructing
/// [`ZmanPrimitive`] values themselves.
#[derive(Debug, Clone)]
pub struct ZmanPreset {
    /// The primitive calculation used by this preset.
    pub event: ZmanPrimitive,
    /// The broad category this preset belongs to.
    pub zman_type: ZmanType,
    /// The KosherJava getter name (for example `getSunrise`).
    pub method_name: &'static str,
    /// The user-facing preset name.
    pub name: &'static str,
    #[cfg(feature = "alloc")]
    pub(crate) description: fn(&ZmanimCalculator) -> String,
    /// Whether KosherJava marks this preset as deprecated.
    pub deprecated: bool,
}

impl ZmanLike for ZmanPreset {
    fn calculate(&self, calculator: &ZmanimCalculator) -> Result<Timestamp, ZmanimError> {
        self.event.calculate(calculator)
    }
}

impl ZmanPreset {
    /// Returns a user-facing preset description.
    ///
    /// The description includes active [`ZmanimCalculator`] settings when they
    /// affect how this preset is calculated.
    ///
    /// Requires the `alloc` feature.
    #[cfg(feature = "alloc")]
    pub fn description(&self, calculator: &ZmanimCalculator) -> String {
        let mut desc = (self.description)(calculator);

        if self.event.uses_elevation_from_config() {
            if calculator.config.use_elevation {
                desc.push_str(
                    "\n\nThis zman takes the configured location's elevation into account when it is calculated.",
                );
            } else {
                desc.push_str("\n\nThis zman is calculated as though the configured location were at sea level.");
            }
        }
        if self.event.uses_astronomical_chatzos_from_config() {
            if calculator.config.use_astronomical_chatzos {
                desc.push_str("\n\nThis zman uses astronomical chatzos (solar transit) as the midpoint of the day.");
            } else {
                desc.push_str("\n\nThis zman uses the midpoint between sunrise and sunset for chatzos.");
            }
        }
        if self.event.uses_astronomical_chatzos_for_other_zmanim_from_config() {
            if calculator.config.use_astronomical_chatzos_for_other_zmanim {
                desc.push_str(
                    "\n\nThis zman divides the afternoon from astronomical chatzos when calculating shaos zmaniyos.",
                );
            } else {
                desc.push_str(
                    "\n\nThis zman uses a fixed fraction of the sunrise-to-sunset interval for shaos zmaniyos, without dividing the day at astronomical chatzos.",
                );
            }
        }
        desc
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ZmanimCalculator {
    fn format(&self, fmt: defmt::Formatter) {
        let y = self.date.year();
        let m = self.date.month();
        let d = self.date.day();
        defmt::write!(
            fmt,
            "ZmanimCalculator {{ location: {}, date: {=i16}-{=i8}-{=i8}, config: {} }}",
            self.location,
            y,
            m,
            d,
            self.config
        )
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ZmanPreset {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ZmanPreset {{ event: {}, zman_type: {}, name: {} }}",
            self.event,
            self.zman_type,
            self.name
        )
    }
}
