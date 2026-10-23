use std::time::Duration;

use jiff::Timestamp;
use kosher_rust::myzmanim::MyZman;

use super::DayTimeZone;
use crate::api::{
    calendar::GregorianDate,
    times::{LocalDay, Location, ZmanimError, myzmanim::MyZmanTime},
};

pub(crate) type Inner = kosher_rust::myzmanim::MyZmanim<DayTimeZone>;

pub(crate) fn my_zmanim(
    location: &Location,
    date: &GregorianDate,
    candle_lighting_minutes: f64,
    local_day: Option<&LocalDay>,
) -> Result<Inner, ZmanimError> {
    let location = super::location(location, local_day.is_some())?;
    let date = super::civil_date(date).ok_or(ZmanimError::InvalidGregorianDate)?;
    let time_zone = super::time_zone(date, local_day)?;
    let candle_lighting = Duration::try_from_secs_f64(super::valid_minutes(candle_lighting_minutes)? * 60.0)?;
    Ok(
        kosher_rust::myzmanim::MyZmanim::with_custom_timezone(location, date, time_zone)
            .candle_lighting(candle_lighting),
    )
}

/// A twelfth of the day from sea-level sunrise to sea-level sunset, in milliseconds.
pub(crate) fn proportional_hour(inner: &Inner) -> Result<i64, ZmanimError> {
    // A twelfth of a day fits in i64 milliseconds many times over.
    Ok(i64::try_from(inner.proportional_hour_gra()?.as_millis()).unwrap_or(i64::MAX))
}

pub(crate) fn all(inner: &Inner) -> Vec<MyZmanTime> {
    MyZman::ALL
        .into_iter()
        .filter_map(|zman| {
            let ours = zman.try_into().ok()?;
            let result = inner.get(zman).map(Timestamp::as_millisecond);
            Some(MyZmanTime {
                zman: ours,
                time: result.ok(),
                failure: result.err().map(|error| ZmanimError::from(error).into()),
            })
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
impl TryFrom<kosher_rust::myzmanim::MyZman> for crate::api::times::myzmanim::MyZman {
    type Error = ();

    fn try_from(value: kosher_rust::myzmanim::MyZman) -> Result<Self, ()> {
        use kosher_rust::myzmanim::MyZman as F;
        Ok(match value {
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
            _ => return Err(()),
        })
    }
}
// END GENERATED: MyZman
