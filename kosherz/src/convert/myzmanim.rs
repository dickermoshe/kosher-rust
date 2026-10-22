use std::sync::Arc;

use jiff::Timestamp;
use kosher_rust::myzmanim::MyZman;

use super::{CallbackTimeZone, watching_callbacks};
use crate::api::{
    calendar::GregorianDate,
    times::{Location, TimeZoneOffset, ZmanimError, myzmanim::MyZmanTime},
};

pub(crate) type Inner = kosher_rust::myzmanim::MyZmanim<CallbackTimeZone>;

pub(crate) fn my_zmanim(
    location: &Location,
    date: &GregorianDate,
    candle_lighting_minutes: f64,
    time_zone: Option<Arc<dyn TimeZoneOffset>>,
) -> Result<Inner, ZmanimError> {
    let location = super::location(location, time_zone.is_some())?;
    let date = super::civil_date(date).ok_or(ZmanimError::InvalidGregorianDate)?;
    let candle_lighting_minutes = super::valid_minutes(candle_lighting_minutes)?;
    Ok(
        kosher_rust::myzmanim::MyZmanim::with_custom_timezone(location, date, super::time_zone(time_zone))
            .candle_lighting(candle_lighting_minutes),
    )
}

/// Runs `calculate`, reporting a failed time zone callback over its result.
pub(crate) fn checked(
    inner: &Inner,
    calculate: impl FnOnce(&Inner) -> Result<Timestamp, kosher_rust::zmanim::ZmanimError>,
) -> Result<i64, ZmanimError> {
    let (result, callback_failed) = watching_callbacks(|| calculate(inner));
    if callback_failed {
        return Err(ZmanimError::TimeZoneCallbackFailed);
    }
    Ok(result?.as_millisecond())
}

pub(crate) fn checked_duration(inner: &Inner) -> Result<i64, ZmanimError> {
    let (result, callback_failed) = watching_callbacks(|| inner.proportional_hour_gra());
    if callback_failed {
        return Err(ZmanimError::TimeZoneCallbackFailed);
    }
    // A twelfth of a day fits in i64 milliseconds many times over.
    Ok(i64::try_from(result?.as_millis()).unwrap_or(i64::MAX))
}

pub(crate) fn all(inner: &Inner) -> Vec<MyZmanTime> {
    MyZman::ALL
        .into_iter()
        .map(|zman| {
            let result = checked(inner, |inner| inner.get(zman));
            MyZmanTime {
                zman: zman.into(),
                time: result.ok(),
                failure: result.err().map(Into::into),
            }
        })
        .collect()
}

// BEGIN GENERATED: MyZman
#[allow(deprecated)]
#[rustfmt::skip]
impl From<crate::api::times::myzmanim::MyZman> for kosher_rust::myzmanim::MyZman {
    fn from(value: crate::api::times::myzmanim::MyZman) -> Self {
        use crate::api::times::myzmanim::MyZman as F;
        match value {
            F::Dawn90Degrees => Self::Dawn90Degrees,
            F::Dawn72Fixed => Self::Dawn72Fixed,
            F::Dawn72Degrees => Self::Dawn72Degrees,
            F::Talis11Point5Degrees => Self::Talis11Point5Degrees,
            F::Talis11Degrees => Self::Talis11Degrees,
            F::Talis10Point2Degrees => Self::Talis10Point2Degrees,
            F::SunriseLevel => Self::SunriseLevel,
            F::SunriseElevated => Self::SunriseElevated,
            F::ShemaMA90Degrees => Self::ShemaMA90Degrees,
            F::ShemaMA72Fixed => Self::ShemaMA72Fixed,
            F::ShemaMA72Degrees => Self::ShemaMA72Degrees,
            F::ShemaGra => Self::ShemaGra,
            F::ShacharisMA90Degrees => Self::ShacharisMA90Degrees,
            F::ShacharisMA72Fixed => Self::ShacharisMA72Fixed,
            F::ShacharisMA72Degrees => Self::ShacharisMA72Degrees,
            F::ShacharisGra => Self::ShacharisGra,
            F::Midday => Self::Midday,
            F::Mincha30Fixed => Self::Mincha30Fixed,
            F::MinchaGra => Self::MinchaGra,
            F::MinchaLechumra => Self::MinchaLechumra,
            F::MinchaMA72Fixed => Self::MinchaMA72Fixed,
            F::KetanaGra => Self::KetanaGra,
            F::KetanaMA72Fixed => Self::KetanaMA72Fixed,
            F::PlagGra => Self::PlagGra,
            F::PlagMA72Fixed => Self::PlagMA72Fixed,
            F::CandleLighting => Self::CandleLighting,
            F::SunsetLevel => Self::SunsetLevel,
            F::SunsetElevated => Self::SunsetElevated,
            F::NightGra18Minutes => Self::NightGra18Minutes,
            F::NightGra22Point5Minutes => Self::NightGra22Point5Minutes,
            F::NightGra24Minutes => Self::NightGra24Minutes,
            F::NightZalman => Self::NightZalman,
            F::NightFastTukaccinsky => Self::NightFastTukaccinsky,
            F::NightFastFeinstein => Self::NightFastFeinstein,
            F::NightMoed => Self::NightMoed,
            F::NightShabbos => Self::NightShabbos,
            F::NightChazonIsh => Self::NightChazonIsh,
            F::NightRTam72Fixed => Self::NightRTam72Fixed,
            F::NightRTam72Degrees => Self::NightRTam72Degrees,
            F::Midnight => Self::Midnight,
        }
    }
}
#[allow(deprecated)]
#[rustfmt::skip]
impl From<kosher_rust::myzmanim::MyZman> for crate::api::times::myzmanim::MyZman {
    fn from(value: kosher_rust::myzmanim::MyZman) -> Self {
        use kosher_rust::myzmanim::MyZman as F;
        match value {
            F::Dawn90Degrees => Self::Dawn90Degrees,
            F::Dawn72Fixed => Self::Dawn72Fixed,
            F::Dawn72Degrees => Self::Dawn72Degrees,
            F::Talis11Point5Degrees => Self::Talis11Point5Degrees,
            F::Talis11Degrees => Self::Talis11Degrees,
            F::Talis10Point2Degrees => Self::Talis10Point2Degrees,
            F::SunriseLevel => Self::SunriseLevel,
            F::SunriseElevated => Self::SunriseElevated,
            F::ShemaMA90Degrees => Self::ShemaMA90Degrees,
            F::ShemaMA72Fixed => Self::ShemaMA72Fixed,
            F::ShemaMA72Degrees => Self::ShemaMA72Degrees,
            F::ShemaGra => Self::ShemaGra,
            F::ShacharisMA90Degrees => Self::ShacharisMA90Degrees,
            F::ShacharisMA72Fixed => Self::ShacharisMA72Fixed,
            F::ShacharisMA72Degrees => Self::ShacharisMA72Degrees,
            F::ShacharisGra => Self::ShacharisGra,
            F::Midday => Self::Midday,
            F::Mincha30Fixed => Self::Mincha30Fixed,
            F::MinchaGra => Self::MinchaGra,
            F::MinchaLechumra => Self::MinchaLechumra,
            F::MinchaMA72Fixed => Self::MinchaMA72Fixed,
            F::KetanaGra => Self::KetanaGra,
            F::KetanaMA72Fixed => Self::KetanaMA72Fixed,
            F::PlagGra => Self::PlagGra,
            F::PlagMA72Fixed => Self::PlagMA72Fixed,
            F::CandleLighting => Self::CandleLighting,
            F::SunsetLevel => Self::SunsetLevel,
            F::SunsetElevated => Self::SunsetElevated,
            F::NightGra18Minutes => Self::NightGra18Minutes,
            F::NightGra22Point5Minutes => Self::NightGra22Point5Minutes,
            F::NightGra24Minutes => Self::NightGra24Minutes,
            F::NightZalman => Self::NightZalman,
            F::NightFastTukaccinsky => Self::NightFastTukaccinsky,
            F::NightFastFeinstein => Self::NightFastFeinstein,
            F::NightMoed => Self::NightMoed,
            F::NightShabbos => Self::NightShabbos,
            F::NightChazonIsh => Self::NightChazonIsh,
            F::NightRTam72Fixed => Self::NightRTam72Fixed,
            F::NightRTam72Degrees => Self::NightRTam72Degrees,
            F::Midnight => Self::Midnight,
        }
    }
}
// END GENERATED: MyZman
