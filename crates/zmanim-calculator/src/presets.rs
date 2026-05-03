//! Predefined zmanim calculations built from reusable primitives.
//!
//! Prefer these presets for standard zmanim usage. Reach for `primitive_zman` only when
//! you need to compose a custom calculation that is not already provided here.

use crate::prelude::ZmanimCalculator;

use crate::types::error::ZmanimError;

#[cfg(test)]
use self::java::JavaCalc;
use crate::{calculator::ZmanLike, primitive_zman::ZmanPrimitive};
use jiff::{SignedDuration as Duration, Timestamp};

#[cfg(test)]
mod java {
    //! Java parity-test hooks for preset definitions.
    //!
    //! These callbacks intentionally return a raw Java `Instant` object instead of
    //! `Timestamp` so the test harness can share one conversion path for all
    //! Java calendar methods.

    use crate::java_bindings::com::kosherjava::zmanim::{
        AstronomicalCalendar, ComprehensiveZmanimCalendar, ZmanimCalendar,
    };
    use jni::objects::JObject;

    #[derive(Debug, Clone)]
    /// Type-safe Java method dispatch target for a preset.
    ///
    /// `ComprehensiveZmanimCalendar` extends `ZmanimCalendar`, which extends
    /// `AstronomicalCalendar`; keeping these variants separate lets each preset
    /// call the generated binding for the class that actually declares the method.
    pub(crate) enum JavaCalc {
        /// The Java method is declared directly on `ComprehensiveZmanimCalendar`.
        ComprehensiveCalendar(
            for<'env_local, 'calendar_local, 'borrow> fn(
                &mut jni::Env<'env_local>,
                calendar: &'borrow ComprehensiveZmanimCalendar<'calendar_local>,
            ) -> Result<
                JObject<'env_local>,
                jni::errors::Error,
            >,
        ),
        /// The Java method is declared on the `ZmanimCalendar` parent class.
        ZmanimCalendar(
            for<'env_local, 'calendar_local, 'borrow> fn(
                &mut jni::Env<'env_local>,
                calendar: &'borrow ZmanimCalendar<'calendar_local>,
            ) -> Result<
                JObject<'env_local>,
                jni::errors::Error,
            >,
        ),
        /// The Java method is declared on the `AstronomicalCalendar` base class.
        AstronomicalCalendar(
            for<'env_local, 'calendar_local, 'borrow> fn(
                &mut jni::Env<'env_local>,
                calendar: &'borrow AstronomicalCalendar<'calendar_local>,
            ) -> Result<
                JObject<'env_local>,
                jni::errors::Error,
            >,
        ),
    }

    impl JavaCalc {
        pub fn call<'env_local, 'calendar_local, 'borrow>(
            &self,
            env: &mut jni::Env<'env_local>,
            calendar: &'borrow ComprehensiveZmanimCalendar<'calendar_local>,
        ) -> Result<JObject<'env_local>, jni::errors::Error>
        where
            'calendar_local: 'borrow,
        {
            match self {
                JavaCalc::ComprehensiveCalendar(f) => f(env, calendar),
                JavaCalc::ZmanimCalendar(f) => {
                    let zmanim_calendar = env.as_cast::<ZmanimCalendar>(calendar)?;
                    f(env, &zmanim_calendar)
                }
                JavaCalc::AstronomicalCalendar(f) => {
                    let astronomical_calendar = env.as_cast::<AstronomicalCalendar>(calendar)?;
                    f(env, &astronomical_calendar)
                }
            }
        }
    }
}

/// A zman preset built from a low-level [`ZmanPrimitive`] definition.
///
/// Most users should consume these predefined presets directly rather than constructing
/// [`ZmanPrimitive`] values by hand.
#[derive(Debug, Clone)]
pub struct ZmanPreset<'a> {
    /// The underlying low-level computation definition for this preset.
    pub(crate) event: ZmanPrimitive<'a>,
    #[cfg(test)]
    /// The KosherJava-style preset name used by parity tests.
    pub name: &'a str,
    #[cfg(test)]
    pub(crate) calc: java::JavaCalc,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ZmanPreset<'_> {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "ZmanPreset {{ event: {} }}", self.event)
    }
}

impl<'a> ZmanLike for ZmanPreset<'a> {
    fn calculate(&self, calculator: &mut ZmanimCalculator) -> Result<Timestamp, ZmanimError> {
        self.event.calculate(calculator)
    }
}

macro_rules! count_presets {
    ($($preset:ident),* $(,)?) => {
        <[()]>::len(&[$(count_presets!(@unit $preset)),*])
    };
    (@unit $preset:ident) => {
        ()
    };
}

/// Defines the public preset statics and the [`ALL`] registry from one list.
macro_rules! define_presets {
    (
        $(
            $(#[$meta:meta])*
            $name:ident {
                event: $event:expr,
                name: $java_fn_name:expr,
                java: $java_calc:expr $(,)?
            }
        ),+ $(,)?
    ) => {
        $(
            $(#[$meta])*
            #[allow(deprecated)]
            pub static $name: ZmanPreset<'static> = ZmanPreset {
                event: $event,
                #[cfg(test)]
                name: $java_fn_name,
                #[cfg(test)]
                calc: $java_calc,
            };
        )+

        /// An array of all the presets.
        #[allow(deprecated)]
        pub static ALL: [&ZmanPreset<'static>; count_presets!($($name),+)] = [$(&$name),+];

        #[cfg(test)]
        #[allow(deprecated)]
        #[allow(non_snake_case)]
        mod java_parity_tests {
            $(
                #[allow(non_snake_case)]
                mod $name {
                    #[test]
                    fn standard() -> Result<(), Box<dyn std::error::Error>> {
                        crate::java_tests::zmanim::test_preset_in_jerusalem(&super::super::$name)
                    }

                    #[test]
                    fn regressions() {
                        crate::java_tests::zmanim::test_regressions(&super::super::$name);
                    }

                    #[test]
                    fn random() -> Result<(), Box<dyn std::error::Error>> {
                        crate::java_tests::zmanim::test_preset(&super::super::$name)
                    }
                }
            )+
        }
    };
}

define_presets! {
    /// Sunset (elevation-adjusted).

    ELEVATION_ADJUSTED_SUNRISE {
        event: ZmanPrimitive::ElevationAdjustedSunrise,
        name: "getSunriseWithElevation",
        java: JavaCalc::AstronomicalCalendar(|env, calendar| {
            calendar.get_sunrise(env)
        }),
    },
    /// Sunrise at sea level (elevation `0m`).

    SEA_LEVEL_SUNRISE {
        event: ZmanPrimitive::SeaLevelSunrise,
        name: "getSeaLevelSunrise",
        java: JavaCalc::AstronomicalCalendar(|env, calendar| {
            calendar.get_sea_level_sunrise(env)
        }),
    },
    /// Sunset (elevation-adjusted).

    ELEVATION_ADJUSTED_SUNSET {
        event: ZmanPrimitive::ElevationAdjustedSunset,
        name: "getSunsetWithElevation",
        java: JavaCalc::AstronomicalCalendar(|env, calendar| {
            calendar.get_sunset(env)
        }),
    },
    /// Sunset at sea level (elevation `0m`).

    SEA_LEVEL_SUNSET {
        event: ZmanPrimitive::SeaLevelSunset,
        name: "getSeaLevelSunset",
        java: JavaCalc::AstronomicalCalendar(|env, calendar| {
            calendar.get_sea_level_sunset(env)
        }),
    },
    /// *Alos* as a fixed `60` minutes before sunrise.

    ALOS_60_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-60)),
        name: "getAlos60Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos60minutes(env)
        }),
    },
    /// *Alos* as a fixed `72` minutes before sunrise.

    ALOS_72_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        name: "getAlos72Minutes",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_alos72minutes(env)
        }),
    },
    /// *Alos* as `72 zmaniyos` minutes before sunrise (1.2 *shaos zmaniyos*).

    ALOS_72_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        name: "getAlos72Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos72zmanis(env)
        }),
    },
    /// *Alos* as a fixed `90` minutes before sunrise.

    ALOS_90_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)),
        name: "getAlos90Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos90minutes(env)
        }),
    },
    /// *Alos* as `90 zmaniyos` minutes before sunrise (1.5 *shaos zmaniyos*).

    ALOS_90_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5),
        name: "getAlos90Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos90zmanis(env)
        }),
    },
    /// *Alos* as a fixed `96` minutes before sunrise.

    ALOS_96_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)),
        name: "getAlos96Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos96minutes(env)
        }),
    },
    /// *Alos* as `96 zmaniyos` minutes before sunrise (1.6 *shaos zmaniyos*).

    ALOS_96_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6),
        name: "getAlos96Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos96zmanis(env)
        }),
    },
    /// *Alos* as a fixed `120` minutes before sunrise.

    #[deprecated]
    ALOS_120_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)),
        name: "getAlos120Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos120minutes(env)
        }),
    },
    /// *Alos* as `120 zmaniyos` minutes before sunrise (2.0 *shaos zmaniyos*).

    #[deprecated]
    ALOS_120_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -2.0),
        name: "getAlos120Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos120zmanis(env)
        }),
    },
    /// *Alos* when the sun is `16.1°` below the geometric horizon (degrees-below-horizon dawn).

    ALOS_16_POINT_1_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        name: "getAlos16Point1Degrees",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_alos16point1degrees(env)
        }),
    },
    /// *Alos* when the sun is `18°` below the geometric horizon (degrees-below-horizon dawn).

    ALOS_18_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(18.0),
        name: "getAlos18Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos18degrees(env)
        }),
    },
    /// *Alos* when the sun is `19°` below the geometric horizon (degrees-below-horizon dawn).

    ALOS_19_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(19.0),
        name: "getAlos19Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos19degrees(env)
        }),
    },
    /// *Alos* when the sun is `19.8°` below the geometric horizon (degrees-below-horizon dawn).

    ALOS_19_POINT_8_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(19.8),
        name: "getAlos19Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos19point8degrees(env)
        }),
    },
    /// *Alos* when the sun is `26°` below the geometric horizon (degrees-below-horizon dawn).

    #[deprecated]
    ALOS_26_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(26.0),
        name: "getAlos26Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos26degrees(env)
        }),
    },
    /// *Alos* when the sun is `16.9°` below the geometric horizon (degrees-below-horizon dawn).

    ALOS_BAAL_HATANYA {
        event: ZmanPrimitive::SunriseOffsetByDegrees(16.9),
        name: "getAlosBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_alos_baal_hatanya(env)
        }),
    },
    /// Bain hashmashos (Rabbeinu Tam): when the sun is `13.24°` below the geometric horizon (after sunset).

    BAIN_HASHMASHOS_RT_13_POINT_24_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(13.24),
        name: "getBainHashmashosRT13Point24Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_rt13point24degrees(env)
        }),
    },
    /// Bain hashmashos (Rabbeinu Tam): `58.5` minutes after sunset.

    BAIN_HASHMASHOS_RT_58_POINT_5_MINUTES {
        event: ZmanPrimitive::Offset(
        &ZmanPrimitive::ConfiguredSunset,
        Duration::from_millis((58.5 * 60.0 * 1000.0) as i64),
    ),
        name: "getBainHashmashosRT58Point5Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_rt58point5minutes(env)
        }),
    },
    /// Bain hashmashos (Rabbeinu Tam): `13.5` minutes before when the sun will be `7.083°` below the geometric horizon.

    BAIN_HASHMASHOS_RT_13_POINT_5_MINUTES_BEFORE_7_POINT_083_DEGREES {
        event: ZmanPrimitive::Offset(
            &ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)),
            Duration::from_millis((-13.5 * 60.0 * 1000.0) as i64),
        ),
        name: "getBainHashmashosRT13Point5MinutesBefore7Point083Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_rt13point5minutes_before7point083degrees(env)
        }),
    },
    /// Bain hashmashos (Rabbeinu Tam, 2-stars): `sunset + (sunrise - alos19.8°) * 5/18`.

    BAIN_HASHMASHOS_RT_2_STARS {
        event: ZmanPrimitive::BainHashmashosRt2Stars,
        name: "getBainHashmashosRT2Stars",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_rt2stars(env)
        }),
    },
    /// Bain hashmashos (Yereim): `18` minutes before sunset.

    BAIN_HASHMASHOS_YEREIM_18_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(-18)),
        name: "getBainHashmashosYereim18Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_yereim18minutes(env)
        }),
    },
    /// Bain hashmashos (Yereim): `16.875` minutes before sunset.

    BAIN_HASHMASHOS_YEREIM_16_POINT_875_MINUTES {
        event: ZmanPrimitive::Offset(
        &ZmanPrimitive::ConfiguredSunset,
        Duration::from_millis((-16.875 * 60.0 * 1000.0) as i64),
    ),
        name: "getBainHashmashosYereim16Point875Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_yereim16point875minutes(env)
        }),
    },
    /// Bain hashmashos (Yereim): `13.5` minutes before sunset.

    BAIN_HASHMASHOS_YEREIM_13_POINT_5_MINUTES {
        event: ZmanPrimitive::Offset(
        &ZmanPrimitive::ConfiguredSunset,
        Duration::from_millis((-13.5 * 60.0 * 1000.0) as i64),
    ),
        name: "getBainHashmashosYereim13Point5Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_yereim13point5minutes(env)
        }),
    },
    /// Bain hashmashos (Yereim): when the sun is `2.1°` above the geometric horizon before sunset.

    BAIN_HASHMASHOS_YEREIM_2_POINT_1_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(-2.1),
        name: "getBainHashmashosYereim2Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_yereim2point1degrees(env)
        }),
    },
    /// Bain hashmashos (Yereim): when the sun is `2.8°` above the geometric horizon before sunset.

    BAIN_HASHMASHOS_YEREIM_2_POINT_8_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(-2.8),
        name: "getBainHashmashosYereim2Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_yereim2point8degrees(env)
        }),
    },
    /// Bain hashmashos (Yereim): when the sun is `3.05°` above the geometric horizon before sunset.

    BAIN_HASHMASHOS_YEREIM_3_POINT_05_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(-3.05),
        name: "getBainHashmashosYereim3Point05Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_bain_hashmashos_yereim3point05degrees(env)
        }),
    },
    /// Candle lighting: sea-level sunset minus [`crate::types::config::CalculatorConfig::candle_lighting_offset`].

    CANDLE_LIGHTING {
        event: ZmanPrimitive::CandleLighting,
        name: "getCandleLighting",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_candle_lighting(env)
        }),
    },
    /// Chatzos (astronomical noon): solar transit.

    CHATZOS_HAYOM_ASTRONOMICAL {
        event: ZmanPrimitive::SolarTransit,
        name: "getChatzosHayom",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_chatzos_hayom(env)
        }),
    },
    /// Chatzos halayla (astronomical midnight): solar anti-transit.

    CHATZOS_HALAYLA_ASTRONOMICAL {
        event: ZmanPrimitive::SolarMidnight,
        name: "getChatzosHalayla",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_chatzos_halayla(env)
        }),
    },
    /// Chatzos (half-day): midpoint between sea-level sunrise and sea-level sunset.

    CHATZOS_HAYOM_HALF_DAY {
        event: ZmanPrimitive::HalfDayBasedOffset(
        &ZmanPrimitive::SeaLevelSunrise,
        &ZmanPrimitive::SeaLevelSunset,
        3.0,
    ),
        name: "getChatzosHayomAsHalfDay",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_chatzos_hayom_as_half_day(env)
        }),
    },
    /// Chatzos (fixed local): 12:00 local mean time.

    CHATZOS_HAYOM_FIXED_LOCAL {
        event: ZmanPrimitive::LocalMeanTime(12.0),
        name: "getFixedLocalChatzosHayom",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_fixed_local_chatzos_hayom(env)
        }),
    },
    /// Mincha gedola: `6.5` shaos after sunrise (or `0.5` shaah after chatzos if configured).

    MINCHA_GEDOLA_SUNRISE_SUNSET {
        event: ZmanPrimitive::MinchaGedola(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getMinchaGedolaGRA",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_mincha_gedola_gra(env)
        }),
    },
    /// Mincha gedola: `6.5` shaos after alos `16.1°` (or `0.5` shaah after chatzos if configured).

    MINCHA_GEDOLA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::MinchaGedola(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getMinchaGedola16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola16point1degrees(env)
        }),
    },
    /// Mincha gedola: `30` minutes after solar transit.

    MINCHA_GEDOLA_MINUTES_30 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::SolarTransit, Duration::from_mins(30)),
        name: "getMinchaGedola30Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola30minutes(env)
        }),
    },
    /// Mincha gedola: `6.5` shaos after alos `72` minutes (or `0.5` shaah after chatzos if configured).

    MINCHA_GEDOLA_MINUTES_72 {
        event: ZmanPrimitive::MinchaGedola(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getMinchaGedola72Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola72minutes(env)
        }),
    },
    /// Mincha gedola (Ahavat Shalom): later of `chatzos + 30m` and `chatzos + 1/2 shaah`.

    MINCHA_GEDOLA_AHAVAT_SHALOM {
        event: ZmanPrimitive::MinchaGedolaAhavatShalom,
        name: "getMinchaGedolaAhavatShalom",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola_ahavat_shalom(env)
        }),
    },
    /// Mincha gedola: `6.5` shaos zmaniyos after alos `72 zmaniyos` (day end = Ateret Torah tzais).

    MINCHA_GEDOLA_ATERET_TORAH {
        event: ZmanPrimitive::MinchaGedola(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::TzaisAteretTorah,
        false,
    ),
        name: "getMinchaGedolaAteretTorah",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola_ateret_torah(env)
        }),
    },
    /// Mincha gedola: `6.5` shaos after Baal HaTanya day start (or `0.5` shaah after chatzos if configured).

    MINCHA_GEDOLA_BAAL_HATANYA {
        event: ZmanPrimitive::MinchaGedola(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getMinchaGedolaBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola_baal_hatanya(env)
        }),
    },
    /// Mincha gedola: `30` minutes after fixed local chatzos (12:00 local mean time).

    MINCHA_GEDOLA_GRA_FIXED_LOCAL_CHATZOS_30_MINUTES {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::LocalMeanTime(12.0), Duration::from_mins(30)),
        name: "getMinchaGedolaGRAFixedLocalChatzos30Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_gedola_grafixed_local_chatzos30minutes(env)
        }),
    },
    /// Mincha ketana: `9.5` shaos after sunrise (or `3.5` shaos after chatzos if configured).

    MINCHA_KETANA_SUNRISE_SUNSET {
        event: ZmanPrimitive::MinchaKetana(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getMinchaKetanaGRA",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_mincha_ketana_gra(env)
        }),
    },
    /// Mincha ketana: `9.5` shaos after alos `16.1°` (or `3.5` shaos after chatzos if configured).

    MINCHA_KETANA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::MinchaKetana(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getMinchaKetana16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_ketana16point1degrees(env)
        }),
    },
    /// Mincha ketana: `9.5` shaos after alos `72` minutes (or `3.5` shaos after chatzos if configured).

    MINCHA_KETANA_MINUTES_72 {
        event: ZmanPrimitive::MinchaKetana(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getMinchaKetana72Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_ketana72minutes(env)
        }),
    },
    /// Mincha ketana (Ahavat Shalom): `2.5` shaos zmaniyos before tzais `3.8°` (day = alos16.1° → tzais3.8°).

    MINCHA_KETANA_AHAVAT_SHALOM {
        event: ZmanPrimitive::MinchaKetanaAhavatShalom,
        name: "getMinchaKetanaAhavatShalom",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_ketana_ahavat_shalom(env)
        }),
    },
    /// Mincha ketana: `9.5` shaos zmaniyos after alos `72 zmaniyos` (day end = Ateret Torah tzais).

    MINCHA_KETANA_ATERET_TORAH {
        event: ZmanPrimitive::MinchaKetana(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::TzaisAteretTorah,
        false,
    ),
        name: "getMinchaKetanaAteretTorah",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_ketana_ateret_torah(env)
        }),
    },
    /// Mincha ketana: `9.5` shaos after Baal HaTanya day start (or `3.5` shaos after chatzos if configured).

    MINCHA_KETANA_BAAL_HATANYA {
        event: ZmanPrimitive::MinchaKetana(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getMinchaKetanaBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_ketana_baal_hatanya(env)
        }),
    },
    /// Mincha ketana: `3.5` shaos zmaniyos after fixed local chatzos, using fixed-local-chatzos → sunset half-day.

    MINCHA_KETANA_GRA_FIXED_LOCAL_CHATZOS_TO_SUNSET {
        event: ZmanPrimitive::HalfDayBasedOffset(
        &ZmanPrimitive::LocalMeanTime(12.0),
        &ZmanPrimitive::ConfiguredSunset,
        3.5,
    ),
        name: "getMinchaKetanaGRAFixedLocalChatzosToSunset",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_mincha_ketana_grafixed_local_chatzos_to_sunset(env)
        }),
    },
    /// Misheyakir when the sun is `10.2°` below the geometric horizon (degrees-below-horizon dawn).

    MISHEYAKIR_10_POINT_2_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(10.2),
        name: "getMisheyakir10Point2Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_misheyakir10point2degrees(env)
        }),
    },
    /// Misheyakir when the sun is `11°` below the geometric horizon (degrees-below-horizon dawn).

    MISHEYAKIR_11_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(11.0),
        name: "getMisheyakir11Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_misheyakir11degrees(env)
        }),
    },
    /// Misheyakir when the sun is `11.5°` below the geometric horizon (degrees-below-horizon dawn).

    MISHEYAKIR_11_POINT_5_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(11.5),
        name: "getMisheyakir11Point5Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_misheyakir11point5degrees(env)
        }),
    },
    /// Misheyakir when the sun is `12.85°` below the geometric horizon (degrees-below-horizon dawn).

    MISHEYAKIR_12_POINT_85_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(12.85),
        name: "getMisheyakir12Point85Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_misheyakir12point85degrees(env)
        }),
    },
    /// Misheyakir when the sun is `7.65°` below the geometric horizon (degrees-below-horizon dawn).

    MISHEYAKIR_7_POINT_65_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(7.65),
        name: "getMisheyakir7Point65Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_misheyakir7point65degrees(env)
        }),
    },
    /// Misheyakir when the sun is `9.5°` below the geometric horizon (degrees-below-horizon dawn).

    MISHEYAKIR_9_POINT_5_DEGREES {
        event: ZmanPrimitive::SunriseOffsetByDegrees(9.5),
        name: "getMisheyakir9Point5Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_misheyakir9point5degrees(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after sunrise (or `4.75` shaos after chatzos if configured).

    PLAG_HAMINCHA_SUNRISE_SUNSET {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getPlagHaminchaGRA",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_plag_hamincha_gra(env)
        }),
    },
    /// Plag hamincha (Ahavat Shalom): `1.25` shaos zmaniyos before tzais `3.8°` (day = alos16.1° → tzais3.8°).

    PLAG_HAMINCHA_AHAVAT_SHALOM {
        event: ZmanPrimitive::PlagAhavatShalom,
        name: "getPlagAhavatShalom",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_ahavat_shalom(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos zmaniyos after alos `16.1°` (day = alos16.1° → tzais7.083°).

    PLAG_HAMINCHA_16_POINT_1_TO_TZAIS_GEONIM_7_POINT_083 {
        event: ZmanPrimitive::PlagHamincha(
            &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
            &ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)),
            false,
        ),
        name: "getPlagAlos16Point1ToTzaisGeonim7Point083Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_alos16point1to_tzais_geonim7point083degrees(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos zmaniyos after alos `16.1°` (day end = sunset).

    #[deprecated]
    PLAG_HAMINCHA_ALOS_TO_SUNSET {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::ConfiguredSunset,
        false,
    ),
        name: "getPlagAlosToSunset",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_alos_to_sunset(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `60` minutes (or `4.75` shaos after chatzos if configured).

    PLAG_HAMINCHA_60_MINUTES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-60)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(60)),
        true,
    ),
        name: "getPlagHamincha60Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha60minutes(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `72` minutes (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_72_MINUTES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getPlagHamincha72Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha72minutes(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `72 zmaniyos` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_72_ZMANIS {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2),
        true,
    ),
        name: "getPlagHamincha72MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha72minutes_zmanis(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `90` minutes (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_90_MINUTES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)),
        true,
    ),
        name: "getPlagHamincha90Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha90minutes(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `90 zmaniyos` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_90_ZMANIS {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5),
        true,
    ),
        name: "getPlagHamincha90MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha90minutes_zmanis(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `96` minutes (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_96_MINUTES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)),
        true,
    ),
        name: "getPlagHamincha96Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha96minutes(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `96 zmaniyos` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_96_ZMANIS {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6),
        true,
    ),
        name: "getPlagHamincha96MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha96minutes_zmanis(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `120` minutes (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_120_MINUTES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)),
        true,
    ),
        name: "getPlagHamincha120Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha120minutes(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `120 zmaniyos` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_120_ZMANIS {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -2.0),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 2.0),
        true,
    ),
        name: "getPlagHamincha120MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha120minutes_zmanis(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `16.1°` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getPlagHamincha16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha16point1degrees(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `18°` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_18_DEGREES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::SunriseOffsetByDegrees(18.0),
        &ZmanPrimitive::SunsetOffsetByDegrees(18.0),
        true,
    ),
        name: "getPlagHamincha18Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha18degrees(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `19.8°` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_19_POINT_8_DEGREES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::SunriseOffsetByDegrees(19.8),
        &ZmanPrimitive::SunsetOffsetByDegrees(19.8),
        true,
    ),
        name: "getPlagHamincha19Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha19point8degrees(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after alos `26°` (or `4.75` shaos after chatzos if configured).

    #[deprecated]
    PLAG_HAMINCHA_26_DEGREES {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::SunriseOffsetByDegrees(26.0),
        &ZmanPrimitive::SunsetOffsetByDegrees(26.0),
        true,
    ),
        name: "getPlagHamincha26Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha26degrees(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos zmaniyos after alos `72 zmaniyos` (day end = Ateret Torah tzais).

    PLAG_HAMINCHA_ATERET_TORAH {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::TzaisAteretTorah,
        false,
    ),
        name: "getPlagHaminchaAteretTorah",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha_ateret_torah(env)
        }),
    },
    /// Plag hamincha: `10.75` shaos after Baal HaTanya day start (or `4.75` shaos after chatzos if configured).

    PLAG_HAMINCHA_BAAL_HATANYA {
        event: ZmanPrimitive::PlagHamincha(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getPlagHaminchaBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha_baal_hatanya(env)
        }),
    },
    /// Plag hamincha: `4.75` shaos zmaniyos after fixed local chatzos, using fixed-local-chatzos → sunset half-day.

    PLAG_HAMINCHA_GRA_FIXED_LOCAL_CHATZOS_TO_SUNSET {
        event: ZmanPrimitive::HalfDayBasedOffset(
        &ZmanPrimitive::LocalMeanTime(12.0),
        &ZmanPrimitive::ConfiguredSunset,
        4.75,
    ),
        name: "getPlagHaminchaGRAFixedLocalChatzosToSunset",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_plag_hamincha_grafixed_local_chatzos_to_sunset(env)
        }),
    },
    /// Samuch le-mincha ketana: `9` shaos after sunrise (or `3` shaos after chatzos if configured).

    SAMUCH_LE_MINCHA_KETANA_GRA {
        event: ZmanPrimitive::SamuchLeMinchaKetana(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getSamuchLeMinchaKetanaGRA",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_samuch_le_mincha_ketana_gra(env)
        }),
    },
    /// Samuch le-mincha ketana: `9` shaos after alos `16.1°` (or `3` shaos after chatzos if configured).

    SAMUCH_LE_MINCHA_KETANA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::SamuchLeMinchaKetana(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getSamuchLeMinchaKetana16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_samuch_le_mincha_ketana16point1degrees(env)
        }),
    },
    /// Samuch le-mincha ketana: `9` shaos after alos `72` minutes (or `3` shaos after chatzos if configured).

    SAMUCH_LE_MINCHA_KETANA_72_MINUTES {
        event: ZmanPrimitive::SamuchLeMinchaKetana(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getSamuchLeMinchaKetana72Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_samuch_le_mincha_ketana72minutes(env)
        }),
    },
    /// Sof zman achilas chametz: `4` shaos after sunrise (or half-day based if configured).

    SOF_ZMAN_ACHILAS_CHAMETZ_GRA {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getSofZmanAchilasChametzGRA",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_achilas_chametz_gra(env)
        }),
    },
    /// Sof zman achilas chametz: `4` shaos after alos `72` minutes (or half-day based if configured).

    SOF_ZMAN_ACHILAS_CHAMETZ_MGA_72_MINUTES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getSofZmanAchilasChametzMGA72Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_achilas_chametz_mga72minutes(env)
        }),
    },
    /// Sof zman achilas chametz: `4` shaos after alos `72 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_ACHILAS_CHAMETZ_MGA_72_ZMANIS {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2),
        true,
    ),
        name: "getSofZmanAchilasChametzMGA72MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_achilas_chametz_mga72minutes_zmanis(env)
        }),
    },
    /// Sof zman achilas chametz: `4` shaos after alos `16.1°` (or half-day based if configured).

    SOF_ZMAN_ACHILAS_CHAMETZ_MGA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getSofZmanAchilasChametzMGA16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_achilas_chametz_mga16point1degrees(env)
        }),
    },
    /// Sof zman achilas chametz: `4` shaos after Baal HaTanya day start (or half-day based if configured).

    SOF_ZMAN_ACHILAS_CHAMETZ_BAAL_HATANYA {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getSofZmanAchilasChametzBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_achilas_chametz_baal_hatanya(env)
        }),
    },
    /// Sof zman biur chametz: `5` shaos zmaniyos after sunrise (day = sunrise → sunset).

    SOF_ZMAN_BIUR_CHAMETZ_GRA {
        event: ZmanPrimitive::SofZmanBiurChametz(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getSofZmanBiurChametzGRA",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_biur_chametz_gra(env)
        }),
    },
    /// Sof zman biur chametz: `5` shaos zmaniyos after alos `72` minutes (day = alos72 → tzais72).

    SOF_ZMAN_BIUR_CHAMETZ_MGA_72_MINUTES {
        event: ZmanPrimitive::SofZmanBiurChametz(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getSofZmanBiurChametzMGA72Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_biur_chametz_mga72minutes(env)
        }),
    },
    /// Sof zman biur chametz: `5` shaos zmaniyos after alos `72 zmaniyos` (day = alos72 zmaniyos → tzais72 zmaniyos).

    SOF_ZMAN_BIUR_CHAMETZ_MGA_72_ZMANIS {
        event: ZmanPrimitive::SofZmanBiurChametz(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2),
        true,
    ),
        name: "getSofZmanBiurChametzMGA72MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_biur_chametz_mga72minutes_zmanis(env)
        }),
    },
    /// Sof zman biur chametz: `5` shaos zmaniyos after alos `16.1°` (day = alos16.1° → tzais16.1°).

    SOF_ZMAN_BIUR_CHAMETZ_MGA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::SofZmanBiurChametz(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getSofZmanBiurChametzMGA16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_biur_chametz_mga16point1degrees(env)
        }),
    },
    /// Sof zman biur chametz: `5` shaos zmaniyos after Baal HaTanya day start (day = Baal HaTanya sunrise → sunset).

    SOF_ZMAN_BIUR_CHAMETZ_BAAL_HATANYA {
        event: ZmanPrimitive::SofZmanBiurChametz(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getSofZmanBiurChametzBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_biur_chametz_baal_hatanya(env)
        }),
    },
    /// Sof zman shma: `3` shaos after sunrise (or half-day based if configured).

    SOF_ZMAN_SHMA_GRA {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getSofZmanShmaGRA",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_gra(env)
        }),
    },

    /// Sof zman shma: `3` shaos after alos `19.8°` (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_19_POINT_8_DEGREES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::SunriseOffsetByDegrees(19.8),
        &ZmanPrimitive::SunsetOffsetByDegrees(19.8),
        true,
    ),
        name: "getSofZmanShmaMGA19Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga19point8degrees(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `16.1°` (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getSofZmanShmaMGA16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga16point1degrees(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `18°` (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_18_DEGREES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::SunriseOffsetByDegrees(18.0),
        &ZmanPrimitive::SunsetOffsetByDegrees(18.0),
        true,
    ),
        name: "getSofZmanShmaMGA18Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga18degrees(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `72` minutes (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_72_MINUTES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getSofZmanShmaMGA72Minutes",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga72minutes(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `72 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_72_ZMANIS {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2),
        true,
    ),
        name: "getSofZmanShmaMGA72MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga72minutes_zmanis(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `90` minutes (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_90_MINUTES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)),
        true,
    ),
        name: "getSofZmanShmaMGA90Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga90minutes(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `90 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_90_ZMANIS {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5),
        true,
    ),
        name: "getSofZmanShmaMGA90MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga90minutes_zmanis(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `96` minutes (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_96_MINUTES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)),
        true,
    ),
        name: "getSofZmanShmaMGA96Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga96minutes(env)
        }),
    },
    /// Sof zman shma: `3` shaos after alos `96 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_SHMA_MGA_96_ZMANIS {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6),
        true,
    ),
        name: "getSofZmanShmaMGA96MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga96minutes_zmanis(env)
        }),
    },
    /// Sof zman shma: `3` hours before solar transit.

    SOF_ZMAN_SHMA_HOURS_3_BEFORE_CHATZOS {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::SolarTransit, Duration::from_mins(-180)),
        name: "getSofZmanShma3HoursBeforeChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma3hours_before_chatzos(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `120` minutes (day = alos120 → tzais120).

    SOF_ZMAN_SHMA_MGA_120_MINUTES {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)),
        true,
    ),
        name: "getSofZmanShmaMGA120Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga120minutes(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `16.1°` (day end = sunset).

    SOF_ZMAN_SHMA_ALOS_16_POINT_1_TO_SUNSET {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::ConfiguredSunset,
        false,
    ),
        name: "getSofZmanShmaAlos16Point1ToSunset",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_alos16point1to_sunset(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `16.1°` (day end = tzais7.083°).

    SOF_ZMAN_SHMA_ALOS_16_POINT_1_TO_TZAIS_GEONIM_7_POINT_083 {
        event: ZmanPrimitive::Shema(
            &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
            &ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)),
            false,
        ),
        name: "getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_alos16point1to_tzais_geonim7point083degrees(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `72 zmaniyos` (day end = Ateret Torah tzais).

    SOF_ZMAN_SHMA_ATERET_TORAH {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::TzaisAteretTorah,
        false,
    ),
        name: "getSofZmanShmaAteretTorah",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_ateret_torah(env)
        }),
    },
    /// Sof zman shma: `3` shaos after Baal HaTanya day start (or half-day based if configured).

    SOF_ZMAN_SHMA_BAAL_HATANYA {
        event: ZmanPrimitive::Shema(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getSofZmanShmaBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_baal_hatanya(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after sunrise (day end = fixed local chatzos).

    SOF_ZMAN_SHMA_GRA_SUNRISE_TO_FIXED_LOCAL_CHATZOS {
        event: ZmanPrimitive::HalfDayBasedOffset(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::LocalMeanTime(12.0),
        3.0,
    ),
        name: "getSofZmanShmaGRASunriseToFixedLocalChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_grasunrise_to_fixed_local_chatzos(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `18°` (day end = fixed local chatzos).

    SOF_ZMAN_SHMA_MGA_18_DEGREES_TO_FIXED_LOCAL_CHATZOS {
        event: ZmanPrimitive::HalfDayBasedOffset(
            &ZmanPrimitive::SunriseOffsetByDegrees(18.0),
            &ZmanPrimitive::LocalMeanTime(12.0),
            3.0,
        ),
        name: "getSofZmanShmaMGA18DegreesToFixedLocalChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga18degrees_to_fixed_local_chatzos(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `16.1°` (day end = fixed local chatzos).

    SOF_ZMAN_SHMA_MGA_16_POINT_1_DEGREES_TO_FIXED_LOCAL_CHATZOS {
        event: ZmanPrimitive::HalfDayBasedOffset(
            &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
            &ZmanPrimitive::LocalMeanTime(12.0),
            3.0,
        ),
        name: "getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga16point1degrees_to_fixed_local_chatzos(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `90` minutes (day end = fixed local chatzos).

    SOF_ZMAN_SHMA_MGA_90_MINUTES_TO_FIXED_LOCAL_CHATZOS {
        event: ZmanPrimitive::HalfDayBasedOffset(
            &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)),
            &ZmanPrimitive::LocalMeanTime(12.0),
            3.0,
        ),
        name: "getSofZmanShmaMGA90MinutesToFixedLocalChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga90minutes_to_fixed_local_chatzos(env)
        }),
    },
    /// Sof zman shma: `3` shaos zmaniyos after alos `72` minutes (day end = fixed local chatzos).

    SOF_ZMAN_SHMA_MGA_72_MINUTES_TO_FIXED_LOCAL_CHATZOS {
        event: ZmanPrimitive::HalfDayBasedOffset(
            &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
            &ZmanPrimitive::LocalMeanTime(12.0),
            3.0,
        ),
        name: "getSofZmanShmaMGA72MinutesToFixedLocalChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_shma_mga72minutes_to_fixed_local_chatzos(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after sunrise (or half-day based if configured).

    SOF_ZMAN_TFILA_GRA {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::ConfiguredSunset,
        true,
    ),
        name: "getSofZmanTfilaGRA",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_gra(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `72` minutes (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getSofZmanTfilaMGA72Minutes",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga72minutes(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `19.8°` (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_19_POINT_8_DEGREES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::SunriseOffsetByDegrees(19.8),
        &ZmanPrimitive::SunsetOffsetByDegrees(19.8),
        true,
    ),
        name: "getSofZmanTfilaMGA19Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga19point8degrees(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `16.1°` (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_16_POINT_1_DEGREES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::SunriseOffsetByDegrees(16.1),
        &ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        true,
    ),
        name: "getSofZmanTfilaMGA16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga16point1degrees(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `18°` (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_18_DEGREES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::SunriseOffsetByDegrees(18.0),
        &ZmanPrimitive::SunsetOffsetByDegrees(18.0),
        true,
    ),
        name: "getSofZmanTfilaMGA18Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga18degrees(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `72` minutes (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_72_MINUTES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-72)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        true,
    ),
        name: "getSofZmanTfilaMGA72Minutes",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga72minutes(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `72 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_72_ZMANIS {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2),
        true,
    ),
        name: "getSofZmanTfilaMGA72MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga72minutes_zmanis(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `90` minutes (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_90_MINUTES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-90)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)),
        true,
    ),
        name: "getSofZmanTfilaMGA90Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga90minutes(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `90 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_90_ZMANIS {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.5),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5),
        true,
    ),
        name: "getSofZmanTfilaMGA90MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga90minutes_zmanis(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `96` minutes (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_96_MINUTES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-96)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)),
        true,
    ),
        name: "getSofZmanTfilaMGA96Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga96minutes(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after alos `96 zmaniyos` (or half-day based if configured).

    SOF_ZMAN_TFILA_MGA_96_ZMANIS {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.6),
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6),
        true,
    ),
        name: "getSofZmanTfilaMGA96MinutesZmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga96minutes_zmanis(env)
        }),
    },
    /// Sof zman tfila: `2` hours before solar transit.

    SOF_ZMAN_TFILA_HOURS_2_BEFORE_CHATZOS {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::SolarTransit, Duration::from_mins(-120)),
        name: "getSofZmanTfila2HoursBeforeChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila2hours_before_chatzos(env)
        }),
    },
    /// Sof zman tfila: `4` shaos zmaniyos after alos `120` minutes (day = alos120 → tzais120).

    SOF_ZMAN_TFILA_MGA_120_MINUTES {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunrise, Duration::from_mins(-120)),
        &ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)),
        true,
    ),
        name: "getSofZmanTfilaMGA120Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_mga120minutes(env)
        }),
    },
    /// Sof zman tfila: `4` shaos zmaniyos after alos `72 zmaniyos` (day end = Ateret Torah tzais).

    SOF_ZMAN_TFILA_ATERET_TORAH {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunrise, -1.2),
        &ZmanPrimitive::TzaisAteretTorah,
        false,
    ),
        name: "getSofZmanTfilaAteretTorah",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_ateret_torah(env)
        }),
    },
    /// Sof zman tfila: `4` shaos after Baal HaTanya day start (or half-day based if configured).

    SOF_ZMAN_TFILA_BAAL_HATANYA {
        event: ZmanPrimitive::Tefila(
        &ZmanPrimitive::SunriseOffsetByDegrees(1.583),
        &ZmanPrimitive::SunsetOffsetByDegrees(1.583),
        true,
    ),
        name: "getSofZmanTfilaBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_baal_hatanya(env)
        }),
    },
    /// Sof zman tfila: `4` shaos zmaniyos after sunrise (day end = fixed local chatzos).

    SOF_ZMAN_TFILA_GRA_SUNRISE_TO_FIXED_LOCAL_CHATZOS {
        event: ZmanPrimitive::HalfDayBasedOffset(
        &ZmanPrimitive::ConfiguredSunrise,
        &ZmanPrimitive::LocalMeanTime(12.0),
        4.0,
    ),
        name: "getSofZmanTfilaGRASunriseToFixedLocalChatzos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_tfila_grasunrise_to_fixed_local_chatzos(env)
        }),
    },
    /// Tzais when the sun is `8.5°` below the geometric horizon (after sunset).

    TZAIS_DEGREES_8_POINT_5 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(8.5),
        name: "getTzaisGeonim8Point5Degrees",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_tzais_geonim8point5degrees(env)
        }),
    },
    /// Tzais: `50` minutes after sunset.

    TZAIS_MINUTES_50 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(50)),
        name: "getTzais50Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais50minutes(env)
        }),
    },
    /// Tzais: `60` minutes after sunset.

    TZAIS_MINUTES_60 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(60)),
        name: "getTzais60Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais60minutes(env)
        }),
    },
    /// Tzais: `72` minutes after sunset.

    TZAIS_MINUTES_72 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(72)),
        name: "getTzais72Minutes",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_tzais72minutes(env)
        }),
    },
    /// Tzais: `72 zmaniyos` minutes after sunset (1.2 *shaos zmaniyos*).

    TZAIS_72_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.2),
        name: "getTzais72Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais72zmanis(env)
        }),
    },
    /// Tzais: `90` minutes after sunset.

    TZAIS_MINUTES_90 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(90)),
        name: "getTzais90Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais90minutes(env)
        }),
    },
    /// Tzais: `90 zmaniyos` minutes after sunset (1.5 *shaos zmaniyos*).

    TZAIS_90_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.5),
        name: "getTzais90Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais90zmanis(env)
        }),
    },
    /// Tzais: `96` minutes after sunset.

    TZAIS_MINUTES_96 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(96)),
        name: "getTzais96Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais96minutes(env)
        }),
    },
    /// Tzais: `96 zmaniyos` minutes after sunset (1.6 *shaos zmaniyos*).

    TZAIS_96_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 1.6),
        name: "getTzais96Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais96zmanis(env)
        }),
    },
    /// Tzais: `120` minutes after sunset.

    #[deprecated]
    TZAIS_MINUTES_120 {
        event: ZmanPrimitive::Offset(&ZmanPrimitive::ConfiguredSunset, Duration::from_mins(120)),
        name: "getTzais120Minutes",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais120minutes(env)
        }),
    },
    /// Tzais: `120 zmaniyos` minutes after sunset (2.0 *shaos zmaniyos*).

    #[deprecated]
    TZAIS_120_ZMANIS {
        event: ZmanPrimitive::ZmanisOffset(&ZmanPrimitive::ConfiguredSunset, 2.0),
        name: "getTzais120Zmanis",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais120zmanis(env)
        }),
    },
    /// Tzais when the sun is `16.1°` below the geometric horizon (after sunset).

    TZAIS_16_POINT_1_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(16.1),
        name: "getTzais16Point1Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais16point1degrees(env)
        }),
    },
    /// Tzais when the sun is `18°` below the geometric horizon (after sunset).

    TZAIS_18_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(18.0),
        name: "getTzais18Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais18degrees(env)
        }),
    },
    /// Tzais when the sun is `19.8°` below the geometric horizon (after sunset).

    TZAIS_19_POINT_8_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(19.8),
        name: "getTzais19Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais19point8degrees(env)
        }),
    },
    /// Tzais when the sun is `26°` below the geometric horizon (after sunset).

    #[deprecated]
    TZAIS_26_DEGREES {
        event: ZmanPrimitive::SunsetOffsetByDegrees(26.0),
        name: "getTzais26Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais26degrees(env)
        }),
    },
    /// Tzais (Ateret Torah): (elevation-adjusted) sunset plus [`crate::types::config::CalculatorConfig::ateret_torah_sunset_offset`].

    TZAIS_ATERET_TORAH {
        event: ZmanPrimitive::TzaisAteretTorah,
        name: "getTzaisAteretTorah",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_ateret_torah(env)
        }),
    },
    /// Tzais (Baal HaTanya): when the sun is `6°` below the geometric horizon (after sunset).

    TZAIS_BAAL_HATANYA {
        event: ZmanPrimitive::SunsetOffsetByDegrees(6.0),
        name: "getTzaisBaalHatanya",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_baal_hatanya(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `3.7°` below the geometric horizon (after sunset).

    #[deprecated]
    TZAIS_GEONIM_DEGREES_3_POINT_7 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(3.7),
        name: "getTzaisGeonim3Point7Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim3point7degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `3.8°` below the geometric horizon (after sunset).

    #[deprecated]
    TZAIS_GEONIM_DEGREES_3_POINT_8 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(3.8),
        name: "getTzaisGeonim3Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim3point8degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `5.95°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_5_POINT_95 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(5.95),
        name: "getTzaisGeonim5Point95Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim5point95degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `4.66°` below the geometric horizon (after sunset).

    #[deprecated]
    TZAIS_GEONIM_DEGREES_4_POINT_66 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(4.66),
        name: "getTzaisGeonim4Point66Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim4point66degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `4.42°` below the geometric horizon (after sunset).

    #[deprecated]
    TZAIS_GEONIM_DEGREES_4_POINT_42 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(4.42),
        name: "getTzaisGeonim4Point42Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim4point42degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `4.8°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_4_POINT_8 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(4.8),
        name: "getTzaisGeonim4Point8Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim4point8degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `6.45°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_6_POINT_45 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(6.45),
        name: "getTzaisGeonim6Point45Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim6point45degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `7.083°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_7_POINT_083 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(7.0 + (5.0 / 60.0)),
        name: "getTzaisGeonim7Point083Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim7point083degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `7.67°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_7_POINT_67 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(7.67),
        name: "getTzaisGeonim7Point67Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim7point67degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `8.5°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_8_POINT_5 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(8.5),
        name: "getTzaisGeonim8Point5Degrees",
        java: JavaCalc::ZmanimCalendar(|env, calendar| {
            calendar.get_tzais_geonim8point5degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `9.3°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_9_POINT_3 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(9.3),
        name: "getTzaisGeonim9Point3Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim9point3degrees(env)
        }),
    },
    /// Tzais (Geonim): when the sun is `9.75°` below the geometric horizon (after sunset).

    TZAIS_GEONIM_DEGREES_9_POINT_75 {
        event: ZmanPrimitive::SunsetOffsetByDegrees(9.75),
        name: "getTzaisGeonim9Point75Degrees",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tzais_geonim9point75degrees(env)
        }),
    },
    /// Returns the latest time of _Kiddush Levana_ calculated as 15 days after the molad.
    ///
    /// Will return None if the zman will not occur on this day. If the location does not contain
    /// a timezone, this will always return None.

    SOF_ZMAN_KIDUSH_LEVANA_15_DAYS {
        event: ZmanPrimitive::SofZmanKidushLevana15Days,
        name: "getSofZmanKidushLevana15Days",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_kidush_levana15days(env)
        }),
    },
    /// The latest time of _Kiddush Levana_ according to the
    /// [Maharil](https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin)'s opinion that it
    /// is calculated as halfway between molad and molad.
    ///
    /// Will return None if the zman will not occur on this day. If the location does not contain
    /// a timezone, this will always return None.

    SOF_ZMAN_KIDUSH_LEVANA_BETWEEN_MOLDOS {
        event: ZmanPrimitive::SofZmanKidushLevanaBetweenMoldos,
        name: "getSofZmanKidushLevanaBetweenMoldos",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_sof_zman_kidush_levana_between_moldos(env)
        }),
    },
    /// The earliest time of _Kiddush Levana_ according to [Rabbeinu Yonah](https://en.wikipedia.org/wiki/Yonah_Gerondi)'s opinion that it can be said 3 days after the molad.
    ///
    /// Will return None if the zman will not occur on this day. If the location does not contain
    /// a timezone, this will always return None.

    TCHILAS_ZMAN_KIDUSH_LEVANA_3_DAYS {
        event: ZmanPrimitive::TchilasZmanKidushLevana3Days,
        name: "getTchilasZmanKidushLevana3Days",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tchilas_zman_kidush_levana3days(env)
        }),
    },
    /// The earliest time of _Kiddush Levana_ according to the opinions that it should
    /// not be said until 7 days after the molad.
    ///
    /// Will return None if the zman will not occur on this day. If the location does not contain
    /// a timezone, this will always return None.

    TCHILAS_ZMAN_KIDUSH_LEVANA_7_DAYS {
        event: ZmanPrimitive::TchilasZmanKidushLevana7Days,
        name: "getTchilasZmanKidushLevana7Days",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_tchilas_zman_kidush_levana7days(env)
        }),
    },
    /// The time of the molad (new moon) for the current date's Hebrew month.

    MOLAD {
        event: ZmanPrimitive::Molad,
        name: "getZmanMolad",
        java: JavaCalc::ComprehensiveCalendar(|env, calendar| {
            calendar.get_zman_molad(env)
        }),
    },

}
