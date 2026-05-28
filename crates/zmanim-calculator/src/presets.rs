//! Predefined zmanim calculations built from reusable primitives.
//!
//! Prefer these presets for standard zmanim usage. Reach for `primitive_zman` only when
//! you need to compose a custom calculation that is not already provided here.
use crate::prelude::ZmanimCalculator;
pub use crate::presets_gen::*;

use crate::types::error::ZmanimError;

use crate::{calculator::ZmanLike, primitive_zman::ZmanPrimitive};
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
#[cfg(feature = "alloc")]
use jiff::SignedDuration;
use jiff::Timestamp;

#[cfg(feature = "alloc")]
pub(crate) const USES_ELEVATION_DESCRIPTION: &str =
    "This zman takes the location's elevation into account when calculating the zman.";
#[cfg(feature = "alloc")]
pub(crate) const USES_SEA_LEVEL_DESCRIPTION: &str =
    "This zman is calculated at sea level, without adjusting for elevation.";

/// A zman preset built from a low-level [`ZmanPrimitive`] definition.
///
/// Most users should consume these predefined presets directly rather than constructing
/// [`ZmanPrimitive`] values by hand.
#[derive(Debug, Clone)]
pub struct ZmanPreset {
    /// The underlying low-level computation definition for this preset.
    pub(crate) event: ZmanPrimitive<'static>,
    #[cfg(test)]
    /// The KosherJava-style preset name used by parity tests.
    /// Regression tests use this name to identify the preset.
    pub method_name: &'static str,
    pub(crate) name: &'static str,
    #[cfg(feature = "alloc")]
    pub(crate) description: fn(&ZmanimCalculator) -> String,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ZmanPreset {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ZmanPreset {{ event: {}, name: {} }}",
            self.event,
            self.name
        )
    }
}

impl ZmanLike for ZmanPreset {
    fn calculate(&self, calculator: &mut ZmanimCalculator) -> Result<Timestamp, ZmanimError> {
        self.event.calculate(calculator)
    }
}

impl ZmanPreset {
    /// Returns a user-facing description of this preset.
    ///
    /// The description may reflect the active [`ZmanimCalculator`] configuration, such as
    /// elevation mode, when the preset's wording depends on those settings.
    ///
    /// Requires the `alloc` feature.
    #[cfg(feature = "alloc")]
    pub fn description(&self, calculator: &ZmanimCalculator) -> String {
        (self.description)(calculator)
    }

    /// Returns a short, user-facing name for this preset.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(feature = "alloc")]
pub(crate) fn format_minutes(offset: SignedDuration) -> String {
    let total_secs = offset.as_secs().unsigned_abs();
    let mins = total_secs / 60;
    let remainder = total_secs % 60;
    if remainder == 0 {
        if mins == 1 {
            "1 minute".to_string()
        } else {
            format!("{mins} minutes")
        }
    } else {
        format!("{:.1} minutes", total_secs as f64 / 60.0)
    }
}
