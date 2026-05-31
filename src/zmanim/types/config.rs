use jiff::SignedDuration;

/// Parameters that control how zmanim are calculated.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CalculatorConfig {
    /// Offset subtracted from sea-level sunset to produce [`crate::zmanim::presets::CANDLE_LIGHTING`]. Default: 18 min.
    pub candle_lighting_offset: SignedDuration,
    /// When `true`, derived zmanim (sof zman shma, mincha gedola, etc.) are computed
    /// relative to solar noon rather than as a fraction of the sunrise–sunset interval. Default: `false`.
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    /// Is elevation above sea level calculated for times besides sunrise and sunset. Default: `false`.
    pub use_elevation: bool,
    /// Offset added to elevation-adjusted sunset for the Ateret Torah opinion (see [`crate::zmanim::presets::TZAIS_ATERET_TORAH`]). Default: 40 min.
    pub ateret_torah_sunset_offset: SignedDuration,
    /// When true, [`crate::zmanim::presets::SUN_TRANSIT`] is used for [`crate::zmanim::presets::CHATZOS_HAYOM`] and [`crate::zmanim::presets::CHATZOS_HALAYLA`] for enhanced accuracy. Default: `true`.
    pub use_astronomical_chatzos: bool,
}

impl Default for CalculatorConfig {
    fn default() -> Self {
        Self {
            candle_lighting_offset: SignedDuration::from_mins(18),
            use_astronomical_chatzos_for_other_zmanim: false,
            use_elevation: false,
            ateret_torah_sunset_offset: SignedDuration::from_mins(40),
            use_astronomical_chatzos: true,
        }
    }
}

#[cfg(feature = "defmt")]
// TODO: Can derive when jiff/defmt integration is available upstream
impl defmt::Format for CalculatorConfig {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "CalculatorConfig {{ candle_lighting_offset: {}, use_astronomical_chatzos_for_other_zmanim: {}, ateret_torah_sunset_offset: {}, use_astronomical_chatzos: {}, use_elevation: {} }}",
            self.candle_lighting_offset.as_secs_f64(),
            self.use_astronomical_chatzos_for_other_zmanim,
            self.ateret_torah_sunset_offset.as_secs_f64(),
            self.use_astronomical_chatzos,
            self.use_elevation
        )
    }
}
