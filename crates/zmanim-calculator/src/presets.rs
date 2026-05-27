//! Predefined zmanim calculations built from reusable primitives.
//!
//! Prefer these presets for standard zmanim usage. Reach for `primitive_zman` only when
//! you need to compose a custom calculation that is not already provided here.
use crate::prelude::ZmanimCalculator;
use crate::presets_gen::*;

use crate::types::error::ZmanimError;

use crate::{calculator::ZmanLike, primitive_zman::ZmanPrimitive};
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
use jiff::{SignedDuration as Duration, Timestamp};

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
    pub name: &'static str,
    #[cfg(feature = "alloc")]
    pub description: fn(&ZmanimCalculator) -> String,
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
