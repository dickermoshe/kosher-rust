#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::ToString;

use crate::presets::ZmanPreset;
use crate::primitive_zman::ZmanPrimitive;

pub static SUNRISE_WITH_ELEVATION: ZmanPreset = ZmanPreset {
    event: ZmanPrimitive::ElevationAdjustedSunrise,
    #[cfg(test)]
    method_name: "getSunriseWithElevation",
    name: "Sunrise (elevation-adjusted)",
    #[cfg(feature = "alloc")]
    description: |_calculator| "Sunrise at the configured location/date.".to_string(),
};
