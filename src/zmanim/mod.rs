use jiff::{Timestamp, civil::Date};

mod astronomy;

pub mod presets;
/// Core zmanim types: configuration, errors, and location.
pub mod types {
    /// Calculator configuration options.
    pub mod config;
    /// Error types returned by zmanim calculations.
    pub mod error;
    /// Geographic location with optional timezone.
    pub mod location;
}
/// Molad and Kiddush Levana time calculations.
pub mod molad;

pub mod primitives;

/// Common zmanim imports.
///
/// Import this module to bring the calculator types, location and config types,
/// preset constants, and primitive building blocks into scope.
pub mod prelude {
    pub use super::presets;
    pub use super::primitives::ZmanPrimitive;
    pub use super::types::config::CalculatorConfig;
    pub use super::types::error::ZmanimError;
    pub use super::types::location::Location;
    pub use super::{ZmanLike, ZmanPreset, ZmanimCalculator};
}

#[cfg(test)]
mod tests;

use crate::zmanim::{
    primitives::ZmanPrimitive,
    types::{config::CalculatorConfig, error::ZmanimError, location::Location},
};

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
/// such as `presets::SUNRISE`, instead of writing custom zman logic.
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
/// A named zman preset backed by a low-level [`ZmanPrimitive`].
///
/// Most callers should use presets directly instead of constructing
/// [`ZmanPrimitive`] values themselves.
#[derive(Debug, Clone)]
pub struct ZmanPreset {
    /// The primitive calculation used by this preset.
    pub event: ZmanPrimitive,
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
        if self.event.uses_astronomical_chatzos_from_config()
            || self.event.uses_astronomical_chatzos_for_other_zmanim_from_config()
        {
            if calculator.config.use_astronomical_chatzos {
                desc.push_str("\n\nThis zman uses astronomical chatzos as the midpoint of the day.");
            } else {
                desc.push_str("\n\nThis zman uses the midpoint between sunrise and sunset for chatzos.");
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
        defmt::write!(fmt, "ZmanPreset {{ event: {}, name: {} }}", self.event, self.name)
    }
}
