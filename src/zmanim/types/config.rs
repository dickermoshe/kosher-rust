use jiff::SignedDuration;

/// The solar position algorithm used for sunrise, sunset, solar noon / midnight and every
/// degree-based zman.
///
/// Mirrors the `AstronomicalCalculator` implementations in KosherJava. All three agree to
/// within a few seconds for ordinary sunrise and sunset; the differences grow at high
/// latitudes and for solar noon. Set it through
/// [`CalculatorConfig::astronomical_calculator`].
///
/// ```
/// use kosher_rust::zmanim::prelude::*;
///
/// let config = CalculatorConfig {
///     astronomical_calculator: AstronomicalCalculator::Meeus { apply_delta_t: true },
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AstronomicalCalculator {
    /// The US National Oceanic and Atmospheric Administration algorithm (KosherJava's
    /// `NOAACalculator`, its default). Uses the low-accuracy Meeus formulas, good to
    /// roughly one minute of time, with no ΔT correction.
    #[default]
    Noaa,
    /// The Jean Meeus higher-accuracy VSOP87 algorithm (KosherJava's `MeeusCalculator`).
    /// Solar position good to roughly an arc-second. Positions are evaluated in
    /// Terrestrial Time, so an estimate of ΔT (TT − UT, currently about a minute) is
    /// applied when `apply_delta_t` is `true`; KosherJava defaults it to `true`, and
    /// `false` reproduces NOAA-style UT = TT behaviour for direct comparison.
    Meeus {
        /// Whether to apply the ΔT (TT − UT) correction.
        apply_delta_t: bool,
    },
    /// The NREL Solar Position Algorithm (KosherJava's `SPACalculator`, Reda and
    /// Andreas). Like [`Self::Meeus`] it is VSOP87-based, but sunrise and sunset are
    /// solved on the full topocentric zenith, so the observer's
    /// [`elevation`](crate::zmanim::types::location::Location::elevation) feeds a
    /// parallax term even when elevation adjustment is disabled. ΔT is applied when
    /// `apply_delta_t` is `true` (KosherJava's default).
    Spa {
        /// Whether to apply the ΔT (TT − UT) correction.
        apply_delta_t: bool,
    },
}

impl AstronomicalCalculator {
    /// The algorithm's descriptive name, matching KosherJava's `getCalculatorName`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Noaa => "US National Oceanic and Atmospheric Administration Algorithm",
            Self::Meeus { .. } => "Jean Meeus Higher-Accuracy (VSOP87) Algorithm",
            Self::Spa { .. } => "NREL Solar Position Algorithm",
        }
    }
}

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
    /// The solar position algorithm behind every astronomical zman. Default: [`AstronomicalCalculator::Noaa`].
    pub astronomical_calculator: AstronomicalCalculator,
}

impl Default for CalculatorConfig {
    fn default() -> Self {
        Self {
            candle_lighting_offset: SignedDuration::from_mins(18),
            use_astronomical_chatzos_for_other_zmanim: false,
            use_elevation: false,
            ateret_torah_sunset_offset: SignedDuration::from_mins(40),
            use_astronomical_chatzos: true,
            astronomical_calculator: AstronomicalCalculator::Noaa,
        }
    }
}

#[cfg(feature = "defmt")]
// TODO: Can derive when jiff/defmt integration is available upstream
impl defmt::Format for CalculatorConfig {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "CalculatorConfig {{ candle_lighting_offset: {}, use_astronomical_chatzos_for_other_zmanim: {}, ateret_torah_sunset_offset: {}, use_astronomical_chatzos: {}, use_elevation: {}, astronomical_calculator: {} }}",
            self.candle_lighting_offset.as_secs_f64(),
            self.use_astronomical_chatzos_for_other_zmanim,
            self.ateret_torah_sunset_offset.as_secs_f64(),
            self.use_astronomical_chatzos,
            self.use_elevation,
            self.astronomical_calculator
        )
    }
}
