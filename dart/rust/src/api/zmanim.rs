//! The *zmanim* calculators Dart uses, with the time zone read from Dart, and the types around
//! them that kosher-rust's own can't cross as they are.

use core::time::Duration as StdDuration;

use extend::ext;
use flutter_rust_bridge::{frb, DartFnFuture};
use futures::executor::block_on;
use jiff::{civil::Date, tz::Offset, SignedDuration, Timestamp};
use kosher_rust::myzmanim::{self, MyZman};
use kosher_rust::zmanim::{self, AstronomicalCalculator, Zman};
use kosher_rust::{Location, LocationError, Zone};

#[ext]
pub impl Location {
    /// The same location with the observer `meters` above sea level.
    ///
    /// Elevation makes sunrise earlier and sunset later. Which *zmanim* use it depends on
    /// `ZmanimConfig.useElevation`.
    ///
    /// Throws `LocationError.invalidElevation` when `meters` is negative, NaN or infinite.
    fn frb_override_with_elevation(&self, meters: f64) -> Result<Location, LocationError> {
        self.with_elevation(meters)
    }

    /// The same location with the eastern horizon, over which the sun rises, and the western
    /// horizon, behind which it sets, at the given heights above sea level in meters.
    ///
    /// By default both are at sea level. In hilly places they can differ by hundreds of meters,
    /// and using the observer's elevation for both shifts sunrise or sunset by minutes. Sunrise
    /// is then adjusted for the observer's height above the eastern horizon, and sunset for the
    /// height above the western one.
    ///
    /// Throws `LocationError.invalidElevation` when either height is negative, NaN or infinite.
    fn frb_override_with_horizons(&self, east_meters: f64, west_meters: f64) -> Result<Location, LocationError> {
        self.with_horizons(east_meters, west_meters)
    }
}

/// How `ZmanimCalculator` calculates. It sets the solar algorithm, elevation, *chatzos*, and the
/// offsets some zmanim use. Start from [`ZmanimConfig::defaults`] and change what you need with
/// `copyWith`.
#[frb(dart_code = "
  /// This config with the given fields changed.
  ZmanimConfig copyWith({
    Duration? candleLightingOffset,
    bool? useAstronomicalChatzosForOtherZmanim,
    bool? useElevation,
    Duration? ateretTorahSunsetOffset,
    bool? useAstronomicalChatzos,
    AstronomicalCalculator? astronomicalCalculator,
  }) => ZmanimConfig(
    candleLightingOffset: candleLightingOffset ?? this.candleLightingOffset,
    useAstronomicalChatzosForOtherZmanim:
        useAstronomicalChatzosForOtherZmanim ?? this.useAstronomicalChatzosForOtherZmanim,
    useElevation: useElevation ?? this.useElevation,
    ateretTorahSunsetOffset: ateretTorahSunsetOffset ?? this.ateretTorahSunsetOffset,
    useAstronomicalChatzos: useAstronomicalChatzos ?? this.useAstronomicalChatzos,
    astronomicalCalculator: astronomicalCalculator ?? this.astronomicalCalculator,
  );
")]
pub struct ZmanimConfig {
    /// How long before sea-level sunset `Zman.candleLighting` is. The default is 18 minutes. An
    /// offset of a day or more gives `ZmanimException.invalidOffset`.
    pub candle_lighting_offset: StdDuration,
    /// Whether proportional zmanim divide the day at *chatzos*. When true, morning times are a
    /// fraction of the start of the day to *chatzos* and afternoon times a fraction of *chatzos* to
    /// the end of the day, instead of a fraction of the whole day. Which *chatzos* is set by
    /// `useAstronomicalChatzos`. It only affects zmanim whose day is symmetric around *chatzos*.
    /// The default is false.
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    /// Whether the observer's elevation adjusts sunrise and sunset in the zmanim built on them.
    /// When false, only `Zman.sunrise` and `Zman.sunset` use it. Either way `Zman.candleLighting`
    /// counts from sea level sunset. The default is false.
    pub use_elevation: bool,
    /// How long after sunset `Zman.tzaisAteretTorah` is, which also ends the day for the other
    /// Ateret Torah zmanim. The default is 40 minutes. An offset of a day or more gives
    /// `ZmanimException.invalidOffset`.
    pub ateret_torah_sunset_offset: StdDuration,
    /// Whether *chatzos* is the sun's transit (true) or the midpoint of sunrise and sunset
    /// (false). The default is true.
    pub use_astronomical_chatzos: bool,
    /// The solar position algorithm. The default is `AstronomicalCalculator.noaa()`.
    pub astronomical_calculator: AstronomicalCalculator,
}

impl ZmanimConfig {
    /// The default config. Candles are lit 18 minutes before sunset, the Ateret Torah day ends
    /// 40 minutes after sunset, *chatzos* is the sun's transit, and the calculator is NOAA.
    pub fn defaults() -> Self {
        zmanim::Config::default().into()
    }
}

impl From<zmanim::Config> for ZmanimConfig {
    fn from(config: zmanim::Config) -> Self {
        Self {
            candle_lighting_offset: config.candle_lighting_offset,
            use_astronomical_chatzos_for_other_zmanim: config.use_astronomical_chatzos_for_other_zmanim,
            use_elevation: config.use_elevation,
            ateret_torah_sunset_offset: config.ateret_torah_sunset_offset,
            use_astronomical_chatzos: config.use_astronomical_chatzos,
            astronomical_calculator: config.astronomical_calculator,
        }
    }
}

impl From<ZmanimConfig> for zmanim::Config {
    fn from(config: ZmanimConfig) -> Self {
        Self {
            candle_lighting_offset: config.candle_lighting_offset,
            use_astronomical_chatzos_for_other_zmanim: config.use_astronomical_chatzos_for_other_zmanim,
            use_elevation: config.use_elevation,
            ateret_torah_sunset_offset: config.ateret_torah_sunset_offset,
            use_astronomical_chatzos: config.use_astronomical_chatzos,
            astronomical_calculator: config.astronomical_calculator,
        }
    }
}

/// Why a time could not be calculated for this place and date.
pub enum ZmanimException {
    /// The sun stays above the angle the time needs all day, as in the far north in summer.
    AllDay,
    /// The sun stays below the angle the time needs all day, as in the far north in winter.
    AllNight,
    /// The time only exists on certain dates, and this is not one. Examples are the latest time
    /// to eat chametz outside Erev Pesach, Kiddush Levana outside its window, and the polar times
    /// on a day with a normal sunrise. On a very short day near the poles, a time that would fall
    /// outside the day also gets this, such as candle lighting before plag hamincha or mincha
    /// gedola after sunset.
    NotApplicable,
    /// The time depends on the Hebrew date, and the date is outside the range a `HebrewDate`
    /// covers, 1 January of year 1 through 29 Elul 9999. The chametz and Kiddush Levana times
    /// depend on the Hebrew date.
    DateOutOfRange,
    /// An angle passed to `sunriseOffsetByDegrees` or `sunsetOffsetByDegrees` is not a number
    /// from -90 to 90.
    InvalidDegrees,
    /// The time needs a time zone and none was given.
    ///
    /// The four Kiddush Levana times always need a time zone. At a location more than 150° from
    /// Greenwich every time needs one.
    TimeZoneRequired,
    /// An offset is a day or more. The offsets are the two in `ZmanimConfig` and the candle
    /// lighting offset of `MyZmanimCalculator`.
    InvalidOffset,
    /// The time is outside the years -9999 and 9999. Only dates within a day or two of either end
    /// get this.
    TimeOutOfRange,
    /// A calculation failed in a way that should not happen. Please report it with the location
    /// and date.
    Internal,
}

impl From<zmanim::ZmanimError> for ZmanimException {
    fn from(error: zmanim::ZmanimError) -> Self {
        match error {
            zmanim::ZmanimError::AllDay => Self::AllDay,
            zmanim::ZmanimError::AllNight => Self::AllNight,
            zmanim::ZmanimError::NotApplicable => Self::NotApplicable,
            zmanim::ZmanimError::DateOutOfRange(_) => Self::DateOutOfRange,
            zmanim::ZmanimError::InvalidDegrees => Self::InvalidDegrees,
            zmanim::ZmanimError::TimeZoneRequired => Self::TimeZoneRequired,
            zmanim::ZmanimError::InvalidOffset => Self::InvalidOffset,
            zmanim::ZmanimError::TimeOutOfRange => Self::TimeOutOfRange,
            zmanim::ZmanimError::Internal => Self::Internal,
        }
    }
}

/// A Dart `TimeZone`, which Rust reads by calling back into Dart. It blocks the Rust thread until
/// Dart answers, so only calls that run on a Rust thread, marked `dart_async`, may read it.
#[frb(ignore)]
pub struct DartZone(Box<dyn Fn(Timestamp) -> DartFnFuture<Offset> + Send + Sync>);

impl DartZone {
    fn new(offset: impl Fn(Timestamp) -> DartFnFuture<Offset> + Send + Sync + 'static) -> Self {
        Self(Box::new(offset))
    }
}

impl Zone for DartZone {
    #[frb(ignore)]
    fn offset(&self, instant: Timestamp) -> Offset {
        block_on((self.0)(instant))
    }
}

/// The *zmanim* for one place and date.
///
/// Get a time with [`ZmanimCalculator::time`]. It is asynchronous, because it reads the time zone
/// by calling back into Dart.
#[frb(opaque)]
pub struct ZmanimCalculator(zmanim::Zmanim<DartZone>);

impl ZmanimCalculator {
    /// The *zmanim* at `location` on `date` in the time zone whose UTC offset at each instant
    /// `offset` gives, calculated with `config`, or the default config when it is null.
    ///
    /// Pass the `offset` method of a `TimeZone`, such as `TimeZone.local.offset`.
    pub fn new(
        location: &Location,
        date: Date,
        offset: impl Fn(Timestamp) -> DartFnFuture<Offset> + Send + Sync + 'static,
        config: Option<ZmanimConfig>,
    ) -> Self {
        Self(zmanim::Zmanim::with_zone(
            *location,
            date,
            Some(DartZone::new(offset)),
            config.unwrap_or_else(ZmanimConfig::defaults).into(),
        ))
    }

    /// The *zmanim* at `location` on `date` with no time zone, calculated with `config`, or the
    /// default config when it is null.
    ///
    /// Without a time zone the four Kiddush Levana times do not work, and at a location more than
    /// 150° from Greenwich no time does.
    pub fn without_zone(location: &Location, date: Date, config: Option<ZmanimConfig>) -> Self {
        Self(zmanim::Zmanim::with_zone(
            *location,
            date,
            None,
            config.unwrap_or_else(ZmanimConfig::defaults).into(),
        ))
    }

    /// The time of `zman` on this day, in seconds since the Unix epoch.
    ///
    /// Throws a `ZmanimException` when there is no such time. That is `allDay` or `allNight` where
    /// the sun never reaches the angle the time needs, as in the far north or south. It is
    /// `notApplicable` for a time that only exists on some days, such as the latest time to eat
    /// chametz outside Erev Pesach. It is `timeZoneRequired` when there is no time zone and the
    /// time needs one, and `invalidOffset` for candle lighting and the Ateret Torah times when
    /// their offset in the `ZmanimConfig` is a day or more.
    #[frb(dart_async, name = "time")]
    pub fn get(&self, zman: Zman) -> Result<Timestamp, ZmanimException> {
        Ok(self.0.get(zman)?)
    }

    /// When the sun is `degrees` below the eastern horizon before sunrise, in seconds since the
    /// Unix epoch, for an opinion the catalog doesn't have. The angle is measured from the
    /// geometric horizon, without elevation. A negative angle is above the horizon. At exactly 0
    /// the result is sea-level sunrise, which also allows for refraction and the size of the sun.
    ///
    /// Throws `ZmanimException.invalidDegrees` when `degrees` is not a number from -90 to 90, and
    /// `allDay` or `allNight` when the sun doesn't reach that angle on this day.
    #[frb(dart_async)]
    pub fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimException> {
        Ok(self.0.sunrise_offset_by_degrees(degrees)?)
    }

    /// When the sun is `degrees` below the western horizon after sunset, in seconds since the Unix
    /// epoch, for example 8.5 for a custom *tzais*. The angle is measured from the geometric
    /// horizon, without elevation. A negative angle is above the horizon. At exactly 0 the result
    /// is sea-level sunset, which also allows for refraction and the size of the sun.
    ///
    /// Throws `ZmanimException.invalidDegrees` when `degrees` is not a number from -90 to 90, and
    /// `allDay` or `allNight` when the sun doesn't reach that angle on this day.
    #[frb(dart_async)]
    pub fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<Timestamp, ZmanimException> {
        Ok(self.0.sunset_offset_by_degrees(degrees)?)
    }

    /// The date.
    #[frb(getter)]
    pub fn date(&self) -> Date {
        self.0.date()
    }

    /// The config.
    #[frb(getter)]
    pub fn config(&self) -> ZmanimConfig {
        (*self.0.config()).into()
    }
}

/// The times shown on the MyZmanim website for one place and date. Almost all of them are within
/// one second of the site.
///
/// Get a time with [`MyZmanimCalculator::time`]. It is asynchronous, because it reads the time
/// zone by calling back into Dart.
#[frb(opaque)]
pub struct MyZmanimCalculator(myzmanim::MyZmanim<DartZone>);

impl MyZmanimCalculator {
    /// The *zmanim* at `location` on `date` in the time zone whose UTC offset at each instant
    /// `offset` gives. Candles are lit `candleLighting` before sunset, or 18 minutes when it is
    /// null.
    ///
    /// Pass the `offset` method of a `TimeZone`, such as `TimeZone.local.offset`.
    pub fn new(
        location: &Location,
        date: Date,
        offset: impl Fn(Timestamp) -> DartFnFuture<Offset> + Send + Sync + 'static,
        candle_lighting: Option<StdDuration>,
    ) -> Self {
        Self(
            myzmanim::MyZmanim::with_zone(*location, date, Some(DartZone::new(offset)))
                .candle_lighting(candle_lighting.unwrap_or(myzmanim::DEFAULT_CANDLE_LIGHTING)),
        )
    }

    /// The *zmanim* at `location` on `date` with no time zone. Candles are lit `candleLighting`
    /// before sunset, or 18 minutes when it is null.
    ///
    /// Without a time zone no time works at a location more than 150° from Greenwich.
    pub fn without_zone(location: &Location, date: Date, candle_lighting: Option<StdDuration>) -> Self {
        Self(
            myzmanim::MyZmanim::with_zone(*location, date, None)
                .candle_lighting(candle_lighting.unwrap_or(myzmanim::DEFAULT_CANDLE_LIGHTING)),
        )
    }

    /// The time of `zman` on this day, in seconds since the Unix epoch.
    ///
    /// Throws `ZmanimException.allDay` or `allNight` where the website shows no time, because the sun
    /// never reaches the angle this time, or one it is built from, needs. Throws
    /// `timeZoneRequired` when there is no time zone and the location is more than 150° from
    /// Greenwich, and `invalidOffset` for candle lighting when its offset is a day or more.
    #[frb(dart_async, name = "time")]
    pub fn get(&self, zman: MyZman) -> Result<Timestamp, ZmanimException> {
        Ok(self.0.get(zman)?)
    }

    /// A twelfth of the day from level sunrise to level sunset, the proportional hour of the Gra
    /// and the Baal HaTanya.
    ///
    /// It is not rounded. The proportional times, such as `MyZman.shemaGra`, round sunrise and
    /// sunset to the nearest second first, so adding these hours to sunrise can differ from them
    /// by up to a second.
    ///
    /// Throws `ZmanimException.allDay` or `allNight` when the sun doesn't rise or set.
    #[frb(dart_async)]
    pub fn proportional_hour_gra(&self) -> Result<SignedDuration, ZmanimException> {
        Ok(self.0.proportional_hour_gra()?)
    }

    /// The date.
    #[frb(getter)]
    pub fn date(&self) -> Date {
        self.0.date()
    }

    /// How long before sunset candles are lit.
    #[frb(getter)]
    pub fn candle_lighting_offset(&self) -> StdDuration {
        self.0.candle_lighting_offset()
    }
}

#[ext]
pub impl Zman {
    /// The display name, for example "Sof Zman Shma (GR'A)".
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }

    /// The KosherJava method that calculates this time, for example "getSofZmanShmaGRA".
    #[frb(getter)]
    fn frb_override_java_name(self) -> String {
        self.java_name().into()
    }

    /// A paragraph or two explaining this time to your users. It says what the time is, its
    /// source, and where it may be unavailable, with the offsets and options from `config` filled
    /// in.
    fn describe(self, config: ZmanimConfig) -> String {
        self.description(&config.into())
    }
}

#[ext]
pub impl MyZman {
    /// The row's label on the website.
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }
}

#[ext]
pub impl AstronomicalCalculator {
    /// The algorithm's descriptive name.
    #[frb(getter)]
    fn display_name(self) -> String {
        self.name().into()
    }
}
