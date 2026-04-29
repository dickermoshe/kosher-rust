//! Predefined zmanim calculations built from reusable primitives.
//!
//! Prefer these presets for standard zmanim usage. Reach for `primitive_zman` only when
//! you need to compose a custom calculation that is not already provided here.

use chrono::TimeZone;

use crate::prelude::ZmanimCalculator;

use crate::types::error::ZmanimError;

#[cfg(feature = "_java_testing")]
use crate::presets::java::JavaCalc;
use crate::{calculator::ZmanLike, primitive_zman::ZmanPrimitive};
use chrono::{DateTime, Utc};

#[cfg(feature = "_java_testing")]
mod java {
    //! Java parity-test hooks for preset definitions.
    //!
    //! These callbacks intentionally return a raw Java `Instant` object instead of
    //! `DateTime<Utc>` so the test harness can share one conversion path for all
    //! Java calendar methods.

    use crate::java_bindings::com::kosherjava::zmanim::{
        AstronomicalCalendar, ComprehensiveZmanimCalendar, ZmanimCalendar,
    };
    use jni::objects::JObject;

    type JavaPresetWithComprehensiveCalendarFn =
        for<'env_local, 'calendar_local, 'borrow> fn(
            &mut jni::Env<'env_local>,
            calendar: &'borrow ComprehensiveZmanimCalendar<'calendar_local>,
        ) -> Result<
            JObject<'env_local>,
            jni::errors::Error,
        >;

    type JavaPresetWithZmanimCalendarFn = for<'env_local, 'calendar_local, 'borrow> fn(
        &mut jni::Env<'env_local>,
        calendar: &'borrow ZmanimCalendar<'calendar_local>,
    )
        -> Result<
        JObject<'env_local>,
        jni::errors::Error,
    >;
    /// Calls an `AstronomicalCalendar` grandparent-class method for any JNI local-frame lifetime.
    type JavaPresetWithAstronomicalCalendarFn =
        for<'env_local, 'calendar_local, 'borrow> fn(
            &mut jni::Env<'env_local>,
            calendar: &'borrow AstronomicalCalendar<'calendar_local>,
        ) -> Result<
            JObject<'env_local>,
            jni::errors::Error,
        >;

    #[derive(Debug, Clone)]
    /// Type-safe Java method dispatch target for a preset.
    ///
    /// `ComprehensiveZmanimCalendar` extends `ZmanimCalendar`, which extends
    /// `AstronomicalCalendar`; keeping these variants separate lets each preset
    /// call the generated binding for the class that actually declares the method.
    pub(super) enum JavaCalc {
        /// The Java method is declared directly on `ComprehensiveZmanimCalendar`.
        ComprehensiveCalendar(JavaPresetWithComprehensiveCalendarFn),
        /// The Java method is declared on the `ZmanimCalendar` parent class.
        ZmanimCalendar(JavaPresetWithZmanimCalendarFn),
        /// The Java method is declared on the `AstronomicalCalendar` base class.
        AstronomicalCalendar(JavaPresetWithAstronomicalCalendarFn),
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ZmanPreset<'a> {
    /// The underlying low-level computation definition for this preset.
    pub(crate) event: ZmanPrimitive<'a>,
    #[cfg(feature = "_java_testing")]
    calc: java::JavaCalc,
    #[cfg(feature = "_java_testing")]
    name: &'a str,
}

impl<'a, Tz: TimeZone> ZmanLike<Tz> for ZmanPreset<'a> {
    fn calculate(
        &self,
        calculator: &mut ZmanimCalculator<Tz>,
    ) -> Result<DateTime<Utc>, ZmanimError> {
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
///
/// Add new presets here instead of manually updating [`ALL`]. The `java`
/// expression is only compiled for `_java_testing`, but every entry should still
/// name the matching KosherJava method so parity tests can call the typed binding.
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
            pub static $name: ZmanPreset<'static> = ZmanPreset {
                event: $event,
                #[cfg(feature = "_java_testing")]
                name: $java_fn_name,
                #[cfg(feature = "_java_testing")]
                calc: $java_calc,
            };
        )+

        /// An array of all the presets.
        pub static ALL: [&ZmanPreset<'static>; count_presets!($($name),+)] = [$(&$name),+];
    };
}

define_presets! {
    /// Sunrise (elevation-adjusted).
    ELEVATION_ADJUSTED_SUNRISE {
        event: ZmanPrimitive::ElevationAdjustedSunrise,
        name: "getSunriseWithElevation",
        java: JavaCalc::AstronomicalCalendar(|env, calendar| {
            calendar.get_sunrise_with_elevation(env)
        }),
    },
}
