use core::time::Duration;

use jiff::{Timestamp, civil};
use serde::{Deserialize, Serialize};
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use super::date::{self, GregorianDate};
use super::error::Error;
use super::js_date;
use super::zone::{self, JsZone};
use crate::Location;
use crate::myzmanim::{MyZman, MyZmanim};
use crate::zmanim::{AstronomicalCalculator, Category, Config, Zman, Zmanim, ZmanimError};

#[wasm_bindgen(typescript_custom_section)]
const ZMANIM_ERROR: &str = r#"
/**
 * Thrown when a time can't be calculated for a reason other than the day not having it. Its `name`
 * is `"ZmanimError"`.
 */
export interface ZmanimError extends Error {
  readonly name: "ZmanimError";
  readonly code: Exclude<Unavailable, "allDay" | "allNight" | "notApplicable">;
}
"#;

/// A place on earth.
#[derive(Clone, Copy, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct LocationOptions {
    /// Degrees north, from -90 to 90.
    latitude: f64,
    /// Degrees east, from -180 to 180. West is negative.
    longitude: f64,
    /// The observer's height above sea level, in meters. 0 when left out.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    elevation: Option<f64>,
    /// The height of the eastern horizon, over which the sun rises, in meters. 0 when left out.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    eastern_horizon: Option<f64>,
    /// The height of the western horizon, behind which the sun sets, in meters. 0 when left out.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    western_horizon: Option<f64>,
}

impl TryFrom<LocationOptions> for Location {
    type Error = Error;

    fn try_from(location: LocationOptions) -> Result<Self, Error> {
        Ok(Location::new(location.latitude, location.longitude)?
            .with_elevation(location.elevation.unwrap_or(0.0))?
            .with_horizons(
                location.eastern_horizon.unwrap_or(0.0),
                location.western_horizon.unwrap_or(0.0),
            )?)
    }
}

/// The solar position algorithm. All three agree to within seconds for ordinary sunrise and sunset.
#[derive(Clone, Copy, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum Calculator {
    Noaa,
    Meeus,
    Spa,
}

/// How [`Zmanim`] calculates. Every setting is optional.
#[derive(Clone, Copy, Default, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct ZmanimConfig {
    /// The solar algorithm. `"noaa"` by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    calculator: Option<Calculator>,
    /// Whether `"meeus"` and `"spa"` correct for ΔT, about a minute. `true` by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    apply_delta_t: Option<bool>,
    /// Minutes before sea-level sunset that candles are lit. 18 by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    candle_lighting_offset: Option<f64>,
    /// Whether elevation adjusts sunrise and sunset in the zmanim built on them. `false` by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    use_elevation: Option<bool>,
    /// Whether chatzos is the sun's transit, not the midpoint of sunrise and sunset. `true` by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    use_astronomical_chatzos: Option<bool>,
    /// Whether proportional zmanim divide the day at chatzos. `false` by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    use_astronomical_chatzos_for_other_zmanim: Option<bool>,
    /// Minutes after sunset of Tzais Ateret Torah. 40 by default.
    #[tsify(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ateret_torah_sunset_offset: Option<f64>,
}

impl TryFrom<ZmanimConfig> for Config {
    type Error = Error;

    fn try_from(options: ZmanimConfig) -> Result<Self, Error> {
        let defaults = Config::default();
        let apply_delta_t = options.apply_delta_t.unwrap_or(true);
        Ok(Config {
            candle_lighting_offset: match options.candle_lighting_offset {
                Some(minutes) => minutes_duration(minutes)?,
                None => defaults.candle_lighting_offset,
            },
            use_astronomical_chatzos_for_other_zmanim: options
                .use_astronomical_chatzos_for_other_zmanim
                .unwrap_or(defaults.use_astronomical_chatzos_for_other_zmanim),
            use_elevation: options.use_elevation.unwrap_or(defaults.use_elevation),
            ateret_torah_sunset_offset: match options.ateret_torah_sunset_offset {
                Some(minutes) => minutes_duration(minutes)?,
                None => defaults.ateret_torah_sunset_offset,
            },
            use_astronomical_chatzos: options
                .use_astronomical_chatzos
                .unwrap_or(defaults.use_astronomical_chatzos),
            astronomical_calculator: match options.calculator {
                Some(Calculator::Meeus) => AstronomicalCalculator::Meeus { apply_delta_t },
                Some(Calculator::Spa) => AstronomicalCalculator::Spa { apply_delta_t },
                Some(Calculator::Noaa) | None => AstronomicalCalculator::Noaa,
            },
        })
    }
}

const INVALID_OFFSET: Error = Error::Range("an offset must be a finite number of minutes, 0 or more");

/// `minutes` as a duration of 0 or more.
fn minutes_duration(minutes: f64) -> Result<Duration, Error> {
    Duration::try_from_secs_f64(minutes * 60.0).ok().ok_or(INVALID_OFFSET)
}

/// What [`Zmanim`] calculates for.
#[derive(Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct ZmanimOptions {
    location: LocationOptions,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "DateInput")]
    date: JsValue,
    /// The location's IANA time zone, such as `"America/New_York"`. Every zman works with one.
    /// Without one, the four Kiddush Levana times fail, and so does every time more than 150° from
    /// Greenwich.
    #[tsify(optional)]
    #[serde(default)]
    time_zone: Option<String>,
    #[serde(flatten)]
    config: ZmanimConfig,
}

/// Why a zman has no time. `"allDay"` and `"allNight"` mean the sun stays above or below the angle
/// the time needs, as in the far north. `"notApplicable"` means the time only exists on some
/// dates. The others are errors, thrown by `get` as a `ZmanimError`.
#[derive(Clone, Copy, PartialEq, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub enum Unavailable {
    AllDay,
    AllNight,
    NotApplicable,
    DateOutOfRange,
    InvalidDegrees,
    TimeZoneRequired,
    InvalidOffset,
    TimeOutOfRange,
    Internal,
}

impl From<ZmanimError> for Unavailable {
    fn from(error: ZmanimError) -> Self {
        match error {
            ZmanimError::AllDay => Self::AllDay,
            ZmanimError::AllNight => Self::AllNight,
            ZmanimError::NotApplicable => Self::NotApplicable,
            ZmanimError::DateOutOfRange(_) => Self::DateOutOfRange,
            ZmanimError::InvalidDegrees => Self::InvalidDegrees,
            ZmanimError::TimeZoneRequired => Self::TimeZoneRequired,
            ZmanimError::InvalidOffset => Self::InvalidOffset,
            ZmanimError::TimeOutOfRange => Self::TimeOutOfRange,
            ZmanimError::Internal => Self::Internal,
        }
    }
}

/// `error` as the `ZmanimError` JavaScript receives.
fn zmanim_error(error: ZmanimError) -> Error {
    let message = match error {
        ZmanimError::DateOutOfRange(_) => {
            "this time needs the Hebrew date, and the date is outside the range a HebrewDate covers"
        }
        ZmanimError::InvalidDegrees => "the angle must be a number of degrees from -90 to 90",
        ZmanimError::TimeZoneRequired => "this time needs a time zone; pass timeZone",
        ZmanimError::InvalidOffset => "the offset must be shorter than a day",
        ZmanimError::TimeOutOfRange => "the time is outside the years -9999 to 9999",
        _ => "internal calculation error; please report the location and date",
    };
    let thrown = js_sys::Error::new(message);
    thrown.set_name("ZmanimError");
    let code = serde_wasm_bindgen::to_value(&Unavailable::from(error)).unwrap_or(JsValue::NULL);
    js_sys::Reflect::set(&thrown, &"code".into(), &code).ok();
    Error::Thrown(thrown.into())
}

/// `result` as a `Date`, `null` when the day has no such time, or a thrown `ZmanimError`. What the
/// time zone threw while `result` was calculated comes first.
fn time(result: Result<Timestamp, ZmanimError>, zone: Option<&JsZone>) -> Result<JsValue, Error> {
    zone.map_or(Ok(()), JsZone::check)?;
    match result {
        Ok(time) => Ok(js_date(time)),
        Err(ZmanimError::AllDay | ZmanimError::AllNight | ZmanimError::NotApplicable) => Ok(JsValue::NULL),
        Err(error) => Err(zmanim_error(error)),
    }
}

/// A halachic time this package can calculate, with its names and category.
#[derive(Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct ZmanInfo {
    id: Zman,
    /// The display name, such as "Sof Zman Shma (GR'A)".
    name: &'static str,
    /// The KosherJava method that calculates it, such as `"getSofZmanShmaGRA"`.
    java_name: &'static str,
    category: Category,
    /// Whether KosherJava deprecates it. It still calculates.
    deprecated: bool,
}

impl From<Zman> for ZmanInfo {
    fn from(zman: Zman) -> Self {
        Self {
            id: zman,
            name: zman.name(),
            java_name: zman.java_name(),
            category: zman.category(),
            deprecated: zman.is_deprecated(),
        }
    }
}

/// A zman and its time on the day, or why it has none.
#[derive(Serialize, Tsify)]
pub struct ZmanTime {
    zman: ZmanInfo,
    /// The time, or `null` when there is none.
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "Date | null")]
    time: JsValue,
    /// Why there is no time. `null` when there is one.
    unavailable: Option<Unavailable>,
}

/// `times`, or what the time zone threw while they were calculated.
fn times<T, E>(zone: Option<&JsZone>, times: Result<Vec<T>, E>) -> Result<Vec<T>, Error>
where
    Error: From<E>,
{
    zone.map_or(Ok(()), JsZone::check)?;
    Ok(times?)
}

/// `result` as a time, or `null` with the reason.
fn time_or_reason(result: Result<Timestamp, ZmanimError>) -> (JsValue, Option<Unavailable>) {
    match result {
        Ok(time) => (js_date(time), None),
        Err(error) => (JsValue::NULL, Some(error.into())),
    }
}

/// The *zmanim* for one place on one day.
#[wasm_bindgen(js_name = Zmanim)]
pub struct JsZmanim {
    zmanim: Zmanim<JsZone>,
    location: LocationOptions,
    config: ZmanimConfig,
}

#[wasm_bindgen(js_class = Zmanim)]
impl JsZmanim {
    /// The *zmanim* at `options.location` on `options.date`.
    #[wasm_bindgen(constructor)]
    pub fn new(options: Ts<ZmanimOptions>) -> Result<JsZmanim, Error> {
        let options = options.to_rust()?;
        let zone = JsZone::from_name(options.time_zone)?;
        Self::build(options.location, &options.date, zone, options.config)
    }

    fn build(
        location: LocationOptions,
        date: &JsValue,
        zone: Option<JsZone>,
        config: ZmanimConfig,
    ) -> Result<JsZmanim, Error> {
        let date: civil::Date = date::gregorian(date)?;
        let zone = zone::checked(zone, date)?;
        let zmanim = Zmanim::with_zone(location.try_into()?, date, zone, config.try_into()?);
        Ok(Self {
            zmanim,
            location,
            config,
        })
    }

    /// The same place, time zone and settings on another day.
    pub fn on(&self, #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue) -> Result<JsZmanim, Error> {
        Self::build(self.location, &date, self.zmanim.timezone().cloned(), self.config)
    }

    /// The place.
    #[wasm_bindgen(getter)]
    pub fn location(&self) -> Result<Ts<LocationOptions>, Error> {
        Ok(Ts::from_rust(&self.location)?)
    }

    /// The day.
    #[wasm_bindgen(getter)]
    pub fn date(&self) -> Result<Ts<GregorianDate>, Error> {
        Ok(Ts::from_rust(&self.zmanim.date().into())?)
    }

    /// The time zone, when one was given.
    #[wasm_bindgen(getter, js_name = timeZone)]
    pub fn time_zone(&self) -> Option<String> {
        self.zmanim.timezone().map(|zone| zone.name().to_owned())
    }

    /// The settings it was built with.
    #[wasm_bindgen(getter)]
    pub fn config(&self) -> Result<Ts<ZmanimConfig>, Error> {
        Ok(Ts::from_rust(&self.config)?)
    }

    /// The time of `zman`, or `null` when the day has none. That happens where the sun never
    /// reaches the angle the time needs, and on a day the time doesn't apply to. Throws a
    /// `ZmanimError` when the time needs a time zone and none was given, or the Hebrew date it
    /// needs is out of range.
    #[wasm_bindgen(unchecked_return_type = "Date | null")]
    pub fn get(&self, zman: Ts<Zman>) -> Result<JsValue, Error> {
        time(self.zmanim.get(zman.to_rust()?), self.zmanim.timezone())
    }

    /// Every zman with its time, or why it has none. Throws only what the time zone throws.
    pub fn all(&self) -> Result<Vec<Ts<ZmanTime>>, Error> {
        let all = Zman::ALL.into_iter().map(|zman| {
            let (time, unavailable) = time_or_reason(self.zmanim.get(zman));
            Ts::from_rust(&ZmanTime {
                zman: zman.into(),
                time,
                unavailable,
            })
        });
        times(self.zmanim.timezone(), all.collect())
    }

    /// When the sun is `degrees` below the eastern horizon before sunrise, for an opinion the
    /// catalog doesn't have. `null` when the sun doesn't get that low.
    #[wasm_bindgen(js_name = sunriseOffsetByDegrees, unchecked_return_type = "Date | null")]
    pub fn sunrise_offset_by_degrees(&self, degrees: f64) -> Result<JsValue, Error> {
        time(self.zmanim.sunrise_offset_by_degrees(degrees), self.zmanim.timezone())
    }

    /// When the sun is `degrees` below the western horizon after sunset, for example 8.5 for a
    /// custom *tzais*.
    #[wasm_bindgen(js_name = sunsetOffsetByDegrees, unchecked_return_type = "Date | null")]
    pub fn sunset_offset_by_degrees(&self, degrees: f64) -> Result<JsValue, Error> {
        time(self.zmanim.sunset_offset_by_degrees(degrees), self.zmanim.timezone())
    }
}

/// Every zman, grouped by category.
#[wasm_bindgen(js_name = allZmanim)]
pub fn all_zmanim() -> Result<Vec<Ts<ZmanInfo>>, Error> {
    Ok(Zman::ALL
        .into_iter()
        .map(|zman| Ts::from_rust(&zman.into()))
        .collect::<Result<_, _>>()?)
}

/// The zman `id`, with its names and category.
#[wasm_bindgen(js_name = zmanInfo)]
pub fn zman_info(id: Ts<Zman>) -> Result<Ts<ZmanInfo>, Error> {
    Ok(Ts::from_rust(&id.to_rust()?.into())?)
}

/// The zmanim in `category`.
#[wasm_bindgen(js_name = zmanimIn)]
pub fn zmanim_in(category: Ts<Category>) -> Result<Vec<Ts<ZmanInfo>>, Error> {
    let category = category.to_rust()?;
    let zmanim = Zman::ALL.into_iter().filter(|zman| zman.category() == category);
    Ok(zmanim
        .map(|zman| Ts::from_rust(&zman.into()))
        .collect::<Result<_, _>>()?)
}

/// A *shaah zmanis*, a twelfth of the day from `start` to `end`, in milliseconds.
#[wasm_bindgen(js_name = shaahZmanis)]
pub fn shaah_zmanis(start: &js_sys::Date, end: &js_sys::Date) -> f64 {
    (end.get_time() - start.get_time()) / 12.0
}

/// What [`MyZmanim`] calculates for.
#[derive(Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct MyZmanimOptions {
    /// MyZmanim's own coordinates, elevation and horizons for the area, to match the site.
    location: LocationOptions,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "DateInput")]
    date: JsValue,
    /// The location's IANA time zone, such as `"Asia/Jerusalem"`. Needed more than 150° from
    /// Greenwich.
    #[tsify(optional)]
    #[serde(default)]
    time_zone: Option<String>,
    /// Minutes before sunset that candles are lit, such as 40 in Jerusalem.
    #[tsify(optional)]
    #[serde(default)]
    candle_lighting_offset: Option<f64>,
}

/// A row of MyZmanim's table.
#[derive(Serialize, Tsify)]
pub struct MyZmanInfo {
    id: MyZman,
    /// The row's label on the website.
    name: &'static str,
}

impl From<MyZman> for MyZmanInfo {
    fn from(zman: MyZman) -> Self {
        Self {
            id: zman,
            name: zman.name(),
        }
    }
}

/// A row of MyZmanim's table and its time on the day, or why it has none.
#[derive(Serialize, Tsify)]
pub struct MyZmanTime {
    zman: MyZmanInfo,
    /// The time, or `null` when there is none.
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "Date | null")]
    time: JsValue,
    /// Why there is no time. `null` when there is one.
    unavailable: Option<Unavailable>,
}

/// The *zmanim* the way MyZmanim calculates them. Use it when your users compare your times with
/// that site.
#[wasm_bindgen(js_name = MyZmanim)]
pub struct JsMyZmanim {
    zmanim: MyZmanim<JsZone>,
    location: LocationOptions,
    candle_lighting_offset: Option<f64>,
}

#[wasm_bindgen(js_class = MyZmanim)]
impl JsMyZmanim {
    /// MyZmanim's times at `options.location` on `options.date`.
    #[wasm_bindgen(constructor)]
    pub fn new(options: Ts<MyZmanimOptions>) -> Result<JsMyZmanim, Error> {
        let options = options.to_rust()?;
        let zone = JsZone::from_name(options.time_zone)?;
        Self::build(options.location, &options.date, zone, options.candle_lighting_offset)
    }

    fn build(
        location: LocationOptions,
        date: &JsValue,
        zone: Option<JsZone>,
        candle_lighting_offset: Option<f64>,
    ) -> Result<JsMyZmanim, Error> {
        let date = date::gregorian(date)?;
        let zone = zone::checked(zone, date)?;
        let mut zmanim = MyZmanim::with_zone(location.try_into()?, date, zone);
        if let Some(minutes) = candle_lighting_offset {
            zmanim = zmanim.candle_lighting(minutes_duration(minutes)?);
        }
        Ok(Self {
            zmanim,
            location,
            candle_lighting_offset,
        })
    }

    /// The same place, time zone and candle lighting on another day.
    pub fn on(&self, #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue) -> Result<JsMyZmanim, Error> {
        Self::build(
            self.location,
            &date,
            self.zmanim.timezone().cloned(),
            self.candle_lighting_offset,
        )
    }

    /// The place.
    #[wasm_bindgen(getter)]
    pub fn location(&self) -> Result<Ts<LocationOptions>, Error> {
        Ok(Ts::from_rust(&self.location)?)
    }

    /// The day.
    #[wasm_bindgen(getter)]
    pub fn date(&self) -> Result<Ts<GregorianDate>, Error> {
        Ok(Ts::from_rust(&self.zmanim.date().into())?)
    }

    /// The time zone, when one was given.
    #[wasm_bindgen(getter, js_name = timeZone)]
    pub fn time_zone(&self) -> Option<String> {
        self.zmanim.timezone().map(|zone| zone.name().to_owned())
    }

    /// Minutes before sunset that candles are lit, when it was given.
    #[wasm_bindgen(getter, js_name = candleLightingOffset)]
    pub fn candle_lighting_offset(&self) -> Option<f64> {
        self.candle_lighting_offset
    }

    /// The time of a row of the table, or `null` where the site shows none.
    #[wasm_bindgen(unchecked_return_type = "Date | null")]
    pub fn get(&self, zman: Ts<MyZman>) -> Result<JsValue, Error> {
        time(self.zmanim.get(zman.to_rust()?), self.zmanim.timezone())
    }

    /// Every row of the table, in its order. Throws only what the time zone throws.
    pub fn all(&self) -> Result<Vec<Ts<MyZmanTime>>, Error> {
        let all = MyZman::ALL.into_iter().map(|zman| {
            let (time, unavailable) = time_or_reason(self.zmanim.get(zman));
            Ts::from_rust(&MyZmanTime {
                zman: zman.into(),
                time,
                unavailable,
            })
        });
        times(self.zmanim.timezone(), all.collect())
    }

    /// The Gra's proportional hour in milliseconds, or `undefined` when the sun doesn't rise or set.
    #[wasm_bindgen(js_name = proportionalHourGra)]
    pub fn proportional_hour_gra(&self) -> Result<Option<f64>, Error> {
        let hour = self.zmanim.proportional_hour_gra();
        self.zmanim.timezone().map_or(Ok(()), JsZone::check)?;
        match hour {
            Ok(hour) => Ok(Some(hour.as_millis_f64())),
            Err(ZmanimError::AllDay | ZmanimError::AllNight | ZmanimError::NotApplicable) => Ok(None),
            Err(error) => Err(zmanim_error(error)),
        }
    }
}

/// Every row of MyZmanim's table, in its order.
#[wasm_bindgen(js_name = allMyZmanim)]
pub fn all_my_zmanim() -> Result<Vec<Ts<MyZmanInfo>>, Error> {
    Ok(MyZman::ALL
        .into_iter()
        .map(|zman| Ts::from_rust(&zman.into()))
        .collect::<Result<_, _>>()?)
}
