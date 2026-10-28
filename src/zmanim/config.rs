use core::time::Duration;

/// The solar position algorithm used for sunrise, sunset, solar noon / midnight and every
/// degree-based zman.
///
/// Mirrors the `AstronomicalCalculator` implementations in KosherJava. All three agree to
/// within a few seconds for ordinary sunrise and sunset, and the differences grow at high
/// latitudes and for solar noon. Set it through
/// [`Config::astronomical_calculator`].
///
/// ```
/// use kosher_rust::zmanim::{AstronomicalCalculator, Config};
///
/// let config = Config {
///     astronomical_calculator: AstronomicalCalculator::Meeus { apply_delta_t: true },
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "__dart_enabled"), non_exhaustive)]
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
    /// [`elevation`](crate::Location::with_elevation) feeds a
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

/// How [`Zmanim`](super::Zmanim) calculates. It sets the solar algorithm, elevation, *chatzos*,
/// and the offsets some zmanim use. [`Config::default`] matches KosherJava's defaults.
///
/// ```
/// use core::time::Duration;
/// use kosher_rust::zmanim::Config;
///
/// let config = Config {
///     candle_lighting_offset: Duration::from_mins(40), // Jerusalem custom
///     ..Config::default()
/// };
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// How long before sea-level sunset [`Zman::CandleLighting`](super::Zman::CandleLighting)
    /// is. The default is 18 minutes. An offset of a day or more gives
    /// [`ZmanimError::InvalidOffset`](super::ZmanimError::InvalidOffset).
    pub candle_lighting_offset: Duration,
    /// Whether proportional zmanim divide the day at *chatzos*. When `true`, morning times are a
    /// fraction of the start of the day to *chatzos* and afternoon times a fraction of *chatzos* to
    /// the end of the day, instead of a fraction of the whole day. Which *chatzos* is set by
    /// [`use_astronomical_chatzos`](Self::use_astronomical_chatzos). It only affects zmanim whose
    /// day is symmetric around *chatzos*. The default is `false`.
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    /// Whether the observer's elevation adjusts sunrise and sunset in the zmanim built on them.
    /// When `false`, only [`Zman::Sunrise`](super::Zman::Sunrise) and
    /// [`Zman::Sunset`](super::Zman::Sunset) use it. Either way
    /// [`Zman::CandleLighting`](super::Zman::CandleLighting) counts from sea level sunset. The
    /// default is `false`.
    pub use_elevation: bool,
    /// How long after sunset [`Zman::TzaisAteretTorah`](super::Zman::TzaisAteretTorah) is,
    /// which also ends the day for the other Ateret Torah zmanim. The default is 40 minutes. An
    /// offset of a day or more gives [`ZmanimError::InvalidOffset`](super::ZmanimError::InvalidOffset).
    pub ateret_torah_sunset_offset: Duration,
    /// Whether *chatzos* is the sun's transit (`true`) or the midpoint of sunrise and sunset
    /// (`false`). The default is `true`.
    pub use_astronomical_chatzos: bool,
    /// The solar position algorithm. The default is [`AstronomicalCalculator::Noaa`].
    pub astronomical_calculator: AstronomicalCalculator,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            candle_lighting_offset: Duration::from_mins(18),
            use_astronomical_chatzos_for_other_zmanim: false,
            use_elevation: false,
            ateret_torah_sunset_offset: Duration::from_mins(40),
            use_astronomical_chatzos: true,
            astronomical_calculator: AstronomicalCalculator::Noaa,
        }
    }
}
