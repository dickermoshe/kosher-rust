use std::sync::Arc;

use jiff::{SignedDuration, Timestamp};
use kosher_rust::zmanim::{self as k, AstronomicalCalculator};

use super::{CallbackTimeZone, calendar::mirror, watching_callbacks};
use crate::api::{
    calendar::GregorianDate,
    times::{Calculator, Category, Config, Location, TimeZoneOffset, ZmanTime, ZmanimError},
};

pub(crate) type Inner = kosher_rust::Zmanim<CallbackTimeZone>;

pub(crate) fn zmanim(
    location: &Location,
    date: &GregorianDate,
    config: &Config,
    time_zone: Option<Arc<dyn TimeZoneOffset>>,
) -> Result<Inner, ZmanimError> {
    let location = super::location(location, time_zone.is_some())?;
    let date = super::civil_date(date).ok_or(ZmanimError::InvalidGregorianDate)?;
    super::valid_minutes(f64::from(config.candle_lighting_minutes))?;
    super::valid_minutes(f64::from(config.ateret_torah_sunset_minutes))?;
    Ok(kosher_rust::Zmanim::with_custom_timezone(
        location,
        date,
        super::time_zone(time_zone),
        (*config).into(),
    ))
}

/// A twelfth of the time from `start` to `end`, in milliseconds. The difference is taken in
/// `i128`; a twelfth of it always fits in an `i64`.
pub(crate) fn shaah_zmanis(start: i64, end: i64) -> i64 {
    ((i128::from(end) - i128::from(start)) / 12) as i64
}

/// Runs `calculate`, reporting a failed time zone callback over its result.
pub(crate) fn checked(
    inner: &Inner,
    calculate: impl FnOnce(&Inner) -> Result<Timestamp, k::ZmanimError>,
) -> Result<i64, ZmanimError> {
    let (result, callback_failed) = watching_callbacks(|| calculate(inner));
    if callback_failed {
        return Err(ZmanimError::TimeZoneCallbackFailed);
    }
    Ok(result?.as_millisecond())
}

/// `offset_by_degrees` (kosher-rust's sunrise or sunset version) for `degrees`, if they are
/// finite.
pub(crate) fn offset_by_degrees(
    inner: &Inner,
    degrees: f64,
    offset_by_degrees: impl FnOnce(&Inner, f64) -> Result<Timestamp, k::ZmanimError>,
) -> Result<i64, ZmanimError> {
    let degrees = super::valid_degrees(degrees)?;
    checked(inner, |inner| offset_by_degrees(inner, degrees))
}

pub(crate) fn all(inner: &Inner) -> Vec<ZmanTime> {
    k::Zman::ALL
        .into_iter()
        .map(|zman| {
            let result = checked(inner, |inner| inner.get(zman));
            ZmanTime {
                zman: zman.into(),
                time: result.ok(),
                failure: result.err().map(Into::into),
            }
        })
        .collect()
}

impl From<Config> for k::Config {
    fn from(config: Config) -> Self {
        let apply_delta_t = config.apply_delta_t;
        Self {
            candle_lighting_offset: SignedDuration::from_mins(i64::from(config.candle_lighting_minutes)),
            use_astronomical_chatzos_for_other_zmanim: config.use_astronomical_chatzos_for_other_zmanim,
            use_elevation: config.use_elevation,
            ateret_torah_sunset_offset: SignedDuration::from_mins(i64::from(config.ateret_torah_sunset_minutes)),
            use_astronomical_chatzos: config.use_astronomical_chatzos,
            astronomical_calculator: match config.calculator {
                Calculator::Noaa => AstronomicalCalculator::Noaa,
                Calculator::Meeus => AstronomicalCalculator::Meeus { apply_delta_t },
                Calculator::Spa => AstronomicalCalculator::Spa { apply_delta_t },
            },
        }
    }
}

impl From<k::Config> for Config {
    fn from(config: k::Config) -> Self {
        let (calculator, apply_delta_t) = match config.astronomical_calculator {
            // KosherJava applies delta-T by default where it applies at all.
            AstronomicalCalculator::Noaa => (Calculator::Noaa, true),
            AstronomicalCalculator::Meeus { apply_delta_t } => (Calculator::Meeus, apply_delta_t),
            AstronomicalCalculator::Spa { apply_delta_t } => (Calculator::Spa, apply_delta_t),
        };
        Self {
            candle_lighting_minutes: minutes(config.candle_lighting_offset),
            use_astronomical_chatzos_for_other_zmanim: config.use_astronomical_chatzos_for_other_zmanim,
            use_elevation: config.use_elevation,
            ateret_torah_sunset_minutes: minutes(config.ateret_torah_sunset_offset),
            use_astronomical_chatzos: config.use_astronomical_chatzos,
            calculator,
            apply_delta_t,
        }
    }
}

/// Whole minutes, saturating: configured offsets are tens of minutes.
fn minutes(duration: SignedDuration) -> i32 {
    i32::try_from(duration.as_mins()).unwrap_or(if duration.is_negative() { i32::MIN } else { i32::MAX })
}

mirror!(
    Category,
    k::Category,
    [
        Twilight,
        Alos,
        Misheyakir,
        Netz,
        SofZmanShma,
        SofZmanTefila,
        SofZmanAchilasChametz,
        SofZmanBiurChametz,
        ChatzosHayom,
        MinchaGedola,
        PlagHamincha,
        SamuchLeMinchaKetana,
        MinchaKetana,
        BeinHashmashos,
        CandleLighting,
        Shkiya,
        Tzais,
        KidushLevana,
        ChatzosHalayla,
    ]
);

// BEGIN GENERATED: Zman
#[allow(deprecated)]
#[rustfmt::skip]
impl From<crate::api::times::Zman> for kosher_rust::Zman {
    fn from(value: crate::api::times::Zman) -> Self {
        use crate::api::times::Zman as F;
        match value {
            F::BeginCivilTwilight => Self::BeginCivilTwilight,
            F::EndCivilTwilight => Self::EndCivilTwilight,
            F::BeginNauticalTwilight => Self::BeginNauticalTwilight,
            F::EndNauticalTwilight => Self::EndNauticalTwilight,
            F::BeginAstronomicalTwilight => Self::BeginAstronomicalTwilight,
            F::EndAstronomicalTwilight => Self::EndAstronomicalTwilight,
            F::Alos60Minutes => Self::Alos60Minutes,
            F::Alos72Minutes => Self::Alos72Minutes,
            F::Alos72Zmanis => Self::Alos72Zmanis,
            F::Alos90Minutes => Self::Alos90Minutes,
            F::Alos90Zmanis => Self::Alos90Zmanis,
            F::Alos96Minutes => Self::Alos96Minutes,
            F::Alos96Zmanis => Self::Alos96Zmanis,
            F::Alos120Minutes => Self::Alos120Minutes,
            F::Alos120Zmanis => Self::Alos120Zmanis,
            F::Alos16Point1Degrees => Self::Alos16Point1Degrees,
            F::Alos18Degrees => Self::Alos18Degrees,
            F::Alos19Degrees => Self::Alos19Degrees,
            F::Alos19Point8Degrees => Self::Alos19Point8Degrees,
            F::Alos26Degrees => Self::Alos26Degrees,
            F::AlosBaalHatanya => Self::AlosBaalHatanya,
            F::Misheyakir10Point2Degrees => Self::Misheyakir10Point2Degrees,
            F::Misheyakir11Degrees => Self::Misheyakir11Degrees,
            F::Misheyakir11Point5Degrees => Self::Misheyakir11Point5Degrees,
            F::Misheyakir12Point85Degrees => Self::Misheyakir12Point85Degrees,
            F::Misheyakir7Point65Degrees => Self::Misheyakir7Point65Degrees,
            F::Misheyakir9Point5Degrees => Self::Misheyakir9Point5Degrees,
            F::PolarSunriseBenIshChai => Self::PolarSunriseBenIshChai,
            F::Sunrise => Self::Sunrise,
            F::SeaLevelSunrise => Self::SeaLevelSunrise,
            F::SofZmanShmaGra => Self::SofZmanShmaGra,
            F::SofZmanShmaMga19Point8Degrees => Self::SofZmanShmaMga19Point8Degrees,
            F::SofZmanShmaMga16Point1Degrees => Self::SofZmanShmaMga16Point1Degrees,
            F::SofZmanShmaMga18Degrees => Self::SofZmanShmaMga18Degrees,
            F::SofZmanShmaMga72Minutes => Self::SofZmanShmaMga72Minutes,
            F::SofZmanShmaMga72MinutesZmanis => Self::SofZmanShmaMga72MinutesZmanis,
            F::SofZmanShmaMga90Minutes => Self::SofZmanShmaMga90Minutes,
            F::SofZmanShmaMga90MinutesZmanis => Self::SofZmanShmaMga90MinutesZmanis,
            F::SofZmanShmaMga96Minutes => Self::SofZmanShmaMga96Minutes,
            F::SofZmanShmaMga96MinutesZmanis => Self::SofZmanShmaMga96MinutesZmanis,
            F::SofZmanShma3HoursBeforeChatzos => Self::SofZmanShma3HoursBeforeChatzos,
            F::SofZmanShmaMga120Minutes => Self::SofZmanShmaMga120Minutes,
            F::SofZmanShmaAlos16Point1ToSunset => Self::SofZmanShmaAlos16Point1ToSunset,
            F::SofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees => Self::SofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees,
            F::SofZmanShmaAteretTorah => Self::SofZmanShmaAteretTorah,
            F::SofZmanShmaBaalHatanya => Self::SofZmanShmaBaalHatanya,
            F::SofZmanShmaGraSunriseToFixedLocalChatzos => Self::SofZmanShmaGraSunriseToFixedLocalChatzos,
            F::SofZmanShmaMga18DegreesToFixedLocalChatzos => Self::SofZmanShmaMga18DegreesToFixedLocalChatzos,
            F::SofZmanShmaMga16Point1DegreesToFixedLocalChatzos => Self::SofZmanShmaMga16Point1DegreesToFixedLocalChatzos,
            F::SofZmanShmaMga90MinutesToFixedLocalChatzos => Self::SofZmanShmaMga90MinutesToFixedLocalChatzos,
            F::SofZmanShmaMga72MinutesToFixedLocalChatzos => Self::SofZmanShmaMga72MinutesToFixedLocalChatzos,
            F::SofZmanTfilaGra => Self::SofZmanTfilaGra,
            F::SofZmanTfilaMga72Minutes => Self::SofZmanTfilaMga72Minutes,
            F::SofZmanTfilaMga19Point8Degrees => Self::SofZmanTfilaMga19Point8Degrees,
            F::SofZmanTfilaMga16Point1Degrees => Self::SofZmanTfilaMga16Point1Degrees,
            F::SofZmanTfilaMga18Degrees => Self::SofZmanTfilaMga18Degrees,
            F::SofZmanTfilaMga72MinutesZmanis => Self::SofZmanTfilaMga72MinutesZmanis,
            F::SofZmanTfilaMga90Minutes => Self::SofZmanTfilaMga90Minutes,
            F::SofZmanTfilaMga90MinutesZmanis => Self::SofZmanTfilaMga90MinutesZmanis,
            F::SofZmanTfilaMga96Minutes => Self::SofZmanTfilaMga96Minutes,
            F::SofZmanTfilaMga96MinutesZmanis => Self::SofZmanTfilaMga96MinutesZmanis,
            F::SofZmanTfila2HoursBeforeChatzos => Self::SofZmanTfila2HoursBeforeChatzos,
            F::SofZmanTfilaMga120Minutes => Self::SofZmanTfilaMga120Minutes,
            F::SofZmanTfilaAteretTorah => Self::SofZmanTfilaAteretTorah,
            F::SofZmanTfilaBaalHatanya => Self::SofZmanTfilaBaalHatanya,
            F::SofZmanTfilaGraSunriseToFixedLocalChatzos => Self::SofZmanTfilaGraSunriseToFixedLocalChatzos,
            F::SofZmanAchilasChametzGra => Self::SofZmanAchilasChametzGra,
            F::SofZmanAchilasChametzMga72Minutes => Self::SofZmanAchilasChametzMga72Minutes,
            F::SofZmanAchilasChametzMga72MinutesZmanis => Self::SofZmanAchilasChametzMga72MinutesZmanis,
            F::SofZmanAchilasChametzMga16Point1Degrees => Self::SofZmanAchilasChametzMga16Point1Degrees,
            F::SofZmanAchilasChametzBaalHatanya => Self::SofZmanAchilasChametzBaalHatanya,
            F::SofZmanBiurChametzGra => Self::SofZmanBiurChametzGra,
            F::SofZmanBiurChametzMga72Minutes => Self::SofZmanBiurChametzMga72Minutes,
            F::SofZmanBiurChametzMga72MinutesZmanis => Self::SofZmanBiurChametzMga72MinutesZmanis,
            F::SofZmanBiurChametzMga16Point1Degrees => Self::SofZmanBiurChametzMga16Point1Degrees,
            F::SofZmanBiurChametzBaalHatanya => Self::SofZmanBiurChametzBaalHatanya,
            F::SunTransit => Self::SunTransit,
            F::ChatzosHayom => Self::ChatzosHayom,
            F::ChatzosHayomAsHalfDay => Self::ChatzosHayomAsHalfDay,
            F::FixedLocalChatzosHayom => Self::FixedLocalChatzosHayom,
            F::MinchaGedolaGra => Self::MinchaGedolaGra,
            F::MinchaGedola16Point1Degrees => Self::MinchaGedola16Point1Degrees,
            F::MinchaGedola30Minutes => Self::MinchaGedola30Minutes,
            F::MinchaGedola72Minutes => Self::MinchaGedola72Minutes,
            F::MinchaGedolaAhavatShalom => Self::MinchaGedolaAhavatShalom,
            F::MinchaGedolaGraGreaterThan30 => Self::MinchaGedolaGraGreaterThan30,
            F::MinchaGedolaAteretTorah => Self::MinchaGedolaAteretTorah,
            F::MinchaGedolaBaalHatanya => Self::MinchaGedolaBaalHatanya,
            F::MinchaGedolaGraFixedLocalChatzos30Minutes => Self::MinchaGedolaGraFixedLocalChatzos30Minutes,
            F::PlagHaminchaGra => Self::PlagHaminchaGra,
            F::PlagAhavatShalom => Self::PlagAhavatShalom,
            F::PlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees => Self::PlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees,
            F::PlagAlosToSunset => Self::PlagAlosToSunset,
            F::PlagHamincha60Minutes => Self::PlagHamincha60Minutes,
            F::PlagHamincha72Minutes => Self::PlagHamincha72Minutes,
            F::PlagHamincha72MinutesZmanis => Self::PlagHamincha72MinutesZmanis,
            F::PlagHamincha90Minutes => Self::PlagHamincha90Minutes,
            F::PlagHamincha90MinutesZmanis => Self::PlagHamincha90MinutesZmanis,
            F::PlagHamincha96Minutes => Self::PlagHamincha96Minutes,
            F::PlagHamincha96MinutesZmanis => Self::PlagHamincha96MinutesZmanis,
            F::PlagHamincha120Minutes => Self::PlagHamincha120Minutes,
            F::PlagHamincha120MinutesZmanis => Self::PlagHamincha120MinutesZmanis,
            F::PlagHamincha16Point1Degrees => Self::PlagHamincha16Point1Degrees,
            F::PlagHamincha18Degrees => Self::PlagHamincha18Degrees,
            F::PlagHamincha19Point8Degrees => Self::PlagHamincha19Point8Degrees,
            F::PlagHamincha26Degrees => Self::PlagHamincha26Degrees,
            F::PlagHaminchaAteretTorah => Self::PlagHaminchaAteretTorah,
            F::PlagHaminchaBaalHatanya => Self::PlagHaminchaBaalHatanya,
            F::PlagHaminchaGraFixedLocalChatzosToSunset => Self::PlagHaminchaGraFixedLocalChatzosToSunset,
            F::SamuchLeMinchaKetanaGra => Self::SamuchLeMinchaKetanaGra,
            F::SamuchLeMinchaKetana16Point1Degrees => Self::SamuchLeMinchaKetana16Point1Degrees,
            F::SamuchLeMinchaKetana72Minutes => Self::SamuchLeMinchaKetana72Minutes,
            F::MinchaKetanaGra => Self::MinchaKetanaGra,
            F::MinchaKetana16Point1Degrees => Self::MinchaKetana16Point1Degrees,
            F::MinchaKetana72Minutes => Self::MinchaKetana72Minutes,
            F::MinchaKetanaAhavatShalom => Self::MinchaKetanaAhavatShalom,
            F::MinchaKetanaAteretTorah => Self::MinchaKetanaAteretTorah,
            F::MinchaKetanaBaalHatanya => Self::MinchaKetanaBaalHatanya,
            F::MinchaKetanaGraFixedLocalChatzosToSunset => Self::MinchaKetanaGraFixedLocalChatzosToSunset,
            F::BainHashmashosRt13Point24Degrees => Self::BainHashmashosRt13Point24Degrees,
            F::BainHashmashosRt58Point5Minutes => Self::BainHashmashosRt58Point5Minutes,
            F::BainHashmashosRt13Point5MinutesBefore7Point083Degrees => Self::BainHashmashosRt13Point5MinutesBefore7Point083Degrees,
            F::BainHashmashosRt2Stars => Self::BainHashmashosRt2Stars,
            F::BainHashmashosYereim18Minutes => Self::BainHashmashosYereim18Minutes,
            F::BainHashmashosYereim16Point875Minutes => Self::BainHashmashosYereim16Point875Minutes,
            F::BainHashmashosYereim13Point5Minutes => Self::BainHashmashosYereim13Point5Minutes,
            F::BainHashmashosYereim2Point1Degrees => Self::BainHashmashosYereim2Point1Degrees,
            F::BainHashmashosYereim2Point8Degrees => Self::BainHashmashosYereim2Point8Degrees,
            F::BainHashmashosYereim3Point05Degrees => Self::BainHashmashosYereim3Point05Degrees,
            F::CandleLighting => Self::CandleLighting,
            F::PolarSunsetBenIshChai => Self::PolarSunsetBenIshChai,
            F::Sunset => Self::Sunset,
            F::SeaLevelSunset => Self::SeaLevelSunset,
            F::TzaisGeonim8Point5Degrees => Self::TzaisGeonim8Point5Degrees,
            F::Tzais50Minutes => Self::Tzais50Minutes,
            F::Tzais60Minutes => Self::Tzais60Minutes,
            F::Tzais72Minutes => Self::Tzais72Minutes,
            F::Tzais72Zmanis => Self::Tzais72Zmanis,
            F::Tzais90Minutes => Self::Tzais90Minutes,
            F::Tzais90Zmanis => Self::Tzais90Zmanis,
            F::Tzais96Minutes => Self::Tzais96Minutes,
            F::Tzais96Zmanis => Self::Tzais96Zmanis,
            F::Tzais120Minutes => Self::Tzais120Minutes,
            F::Tzais120Zmanis => Self::Tzais120Zmanis,
            F::Tzais16Point1Degrees => Self::Tzais16Point1Degrees,
            F::Tzais18Degrees => Self::Tzais18Degrees,
            F::Tzais19Point8Degrees => Self::Tzais19Point8Degrees,
            F::Tzais26Degrees => Self::Tzais26Degrees,
            F::TzaisAteretTorah => Self::TzaisAteretTorah,
            F::TzaisBaalHatanya => Self::TzaisBaalHatanya,
            F::TzaisGeonim3Point7Degrees => Self::TzaisGeonim3Point7Degrees,
            F::TzaisGeonim3Point8Degrees => Self::TzaisGeonim3Point8Degrees,
            F::TzaisGeonim5Point95Degrees => Self::TzaisGeonim5Point95Degrees,
            F::TzaisGeonim4Point66Degrees => Self::TzaisGeonim4Point66Degrees,
            F::TzaisGeonim4Point42Degrees => Self::TzaisGeonim4Point42Degrees,
            F::TzaisGeonim4Point8Degrees => Self::TzaisGeonim4Point8Degrees,
            F::TzaisGeonim6Point45Degrees => Self::TzaisGeonim6Point45Degrees,
            F::TzaisGeonim7Point083Degrees => Self::TzaisGeonim7Point083Degrees,
            F::TzaisGeonim7Point67Degrees => Self::TzaisGeonim7Point67Degrees,
            F::TzaisGeonim9Point3Degrees => Self::TzaisGeonim9Point3Degrees,
            F::TzaisGeonim9Point75Degrees => Self::TzaisGeonim9Point75Degrees,
            F::SofZmanKidushLevana15Days => Self::SofZmanKidushLevana15Days,
            F::SofZmanKidushLevanaBetweenMoldos => Self::SofZmanKidushLevanaBetweenMoldos,
            F::TchilasZmanKidushLevana3Days => Self::TchilasZmanKidushLevana3Days,
            F::TchilasZmanKidushLevana7Days => Self::TchilasZmanKidushLevana7Days,
            F::SolarMidnight => Self::SolarMidnight,
            F::ChatzosHalayla => Self::ChatzosHalayla,
        }
    }
}
#[allow(deprecated)]
#[rustfmt::skip]
impl From<kosher_rust::Zman> for crate::api::times::Zman {
    fn from(value: kosher_rust::Zman) -> Self {
        use kosher_rust::Zman as F;
        match value {
            F::BeginCivilTwilight => Self::BeginCivilTwilight,
            F::EndCivilTwilight => Self::EndCivilTwilight,
            F::BeginNauticalTwilight => Self::BeginNauticalTwilight,
            F::EndNauticalTwilight => Self::EndNauticalTwilight,
            F::BeginAstronomicalTwilight => Self::BeginAstronomicalTwilight,
            F::EndAstronomicalTwilight => Self::EndAstronomicalTwilight,
            F::Alos60Minutes => Self::Alos60Minutes,
            F::Alos72Minutes => Self::Alos72Minutes,
            F::Alos72Zmanis => Self::Alos72Zmanis,
            F::Alos90Minutes => Self::Alos90Minutes,
            F::Alos90Zmanis => Self::Alos90Zmanis,
            F::Alos96Minutes => Self::Alos96Minutes,
            F::Alos96Zmanis => Self::Alos96Zmanis,
            F::Alos120Minutes => Self::Alos120Minutes,
            F::Alos120Zmanis => Self::Alos120Zmanis,
            F::Alos16Point1Degrees => Self::Alos16Point1Degrees,
            F::Alos18Degrees => Self::Alos18Degrees,
            F::Alos19Degrees => Self::Alos19Degrees,
            F::Alos19Point8Degrees => Self::Alos19Point8Degrees,
            F::Alos26Degrees => Self::Alos26Degrees,
            F::AlosBaalHatanya => Self::AlosBaalHatanya,
            F::Misheyakir10Point2Degrees => Self::Misheyakir10Point2Degrees,
            F::Misheyakir11Degrees => Self::Misheyakir11Degrees,
            F::Misheyakir11Point5Degrees => Self::Misheyakir11Point5Degrees,
            F::Misheyakir12Point85Degrees => Self::Misheyakir12Point85Degrees,
            F::Misheyakir7Point65Degrees => Self::Misheyakir7Point65Degrees,
            F::Misheyakir9Point5Degrees => Self::Misheyakir9Point5Degrees,
            F::PolarSunriseBenIshChai => Self::PolarSunriseBenIshChai,
            F::Sunrise => Self::Sunrise,
            F::SeaLevelSunrise => Self::SeaLevelSunrise,
            F::SofZmanShmaGra => Self::SofZmanShmaGra,
            F::SofZmanShmaMga19Point8Degrees => Self::SofZmanShmaMga19Point8Degrees,
            F::SofZmanShmaMga16Point1Degrees => Self::SofZmanShmaMga16Point1Degrees,
            F::SofZmanShmaMga18Degrees => Self::SofZmanShmaMga18Degrees,
            F::SofZmanShmaMga72Minutes => Self::SofZmanShmaMga72Minutes,
            F::SofZmanShmaMga72MinutesZmanis => Self::SofZmanShmaMga72MinutesZmanis,
            F::SofZmanShmaMga90Minutes => Self::SofZmanShmaMga90Minutes,
            F::SofZmanShmaMga90MinutesZmanis => Self::SofZmanShmaMga90MinutesZmanis,
            F::SofZmanShmaMga96Minutes => Self::SofZmanShmaMga96Minutes,
            F::SofZmanShmaMga96MinutesZmanis => Self::SofZmanShmaMga96MinutesZmanis,
            F::SofZmanShma3HoursBeforeChatzos => Self::SofZmanShma3HoursBeforeChatzos,
            F::SofZmanShmaMga120Minutes => Self::SofZmanShmaMga120Minutes,
            F::SofZmanShmaAlos16Point1ToSunset => Self::SofZmanShmaAlos16Point1ToSunset,
            F::SofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees => Self::SofZmanShmaAlos16Point1DegreesToTzaisGeonim7Point083Degrees,
            F::SofZmanShmaAteretTorah => Self::SofZmanShmaAteretTorah,
            F::SofZmanShmaBaalHatanya => Self::SofZmanShmaBaalHatanya,
            F::SofZmanShmaGraSunriseToFixedLocalChatzos => Self::SofZmanShmaGraSunriseToFixedLocalChatzos,
            F::SofZmanShmaMga18DegreesToFixedLocalChatzos => Self::SofZmanShmaMga18DegreesToFixedLocalChatzos,
            F::SofZmanShmaMga16Point1DegreesToFixedLocalChatzos => Self::SofZmanShmaMga16Point1DegreesToFixedLocalChatzos,
            F::SofZmanShmaMga90MinutesToFixedLocalChatzos => Self::SofZmanShmaMga90MinutesToFixedLocalChatzos,
            F::SofZmanShmaMga72MinutesToFixedLocalChatzos => Self::SofZmanShmaMga72MinutesToFixedLocalChatzos,
            F::SofZmanTfilaGra => Self::SofZmanTfilaGra,
            F::SofZmanTfilaMga72Minutes => Self::SofZmanTfilaMga72Minutes,
            F::SofZmanTfilaMga19Point8Degrees => Self::SofZmanTfilaMga19Point8Degrees,
            F::SofZmanTfilaMga16Point1Degrees => Self::SofZmanTfilaMga16Point1Degrees,
            F::SofZmanTfilaMga18Degrees => Self::SofZmanTfilaMga18Degrees,
            F::SofZmanTfilaMga72MinutesZmanis => Self::SofZmanTfilaMga72MinutesZmanis,
            F::SofZmanTfilaMga90Minutes => Self::SofZmanTfilaMga90Minutes,
            F::SofZmanTfilaMga90MinutesZmanis => Self::SofZmanTfilaMga90MinutesZmanis,
            F::SofZmanTfilaMga96Minutes => Self::SofZmanTfilaMga96Minutes,
            F::SofZmanTfilaMga96MinutesZmanis => Self::SofZmanTfilaMga96MinutesZmanis,
            F::SofZmanTfila2HoursBeforeChatzos => Self::SofZmanTfila2HoursBeforeChatzos,
            F::SofZmanTfilaMga120Minutes => Self::SofZmanTfilaMga120Minutes,
            F::SofZmanTfilaAteretTorah => Self::SofZmanTfilaAteretTorah,
            F::SofZmanTfilaBaalHatanya => Self::SofZmanTfilaBaalHatanya,
            F::SofZmanTfilaGraSunriseToFixedLocalChatzos => Self::SofZmanTfilaGraSunriseToFixedLocalChatzos,
            F::SofZmanAchilasChametzGra => Self::SofZmanAchilasChametzGra,
            F::SofZmanAchilasChametzMga72Minutes => Self::SofZmanAchilasChametzMga72Minutes,
            F::SofZmanAchilasChametzMga72MinutesZmanis => Self::SofZmanAchilasChametzMga72MinutesZmanis,
            F::SofZmanAchilasChametzMga16Point1Degrees => Self::SofZmanAchilasChametzMga16Point1Degrees,
            F::SofZmanAchilasChametzBaalHatanya => Self::SofZmanAchilasChametzBaalHatanya,
            F::SofZmanBiurChametzGra => Self::SofZmanBiurChametzGra,
            F::SofZmanBiurChametzMga72Minutes => Self::SofZmanBiurChametzMga72Minutes,
            F::SofZmanBiurChametzMga72MinutesZmanis => Self::SofZmanBiurChametzMga72MinutesZmanis,
            F::SofZmanBiurChametzMga16Point1Degrees => Self::SofZmanBiurChametzMga16Point1Degrees,
            F::SofZmanBiurChametzBaalHatanya => Self::SofZmanBiurChametzBaalHatanya,
            F::SunTransit => Self::SunTransit,
            F::ChatzosHayom => Self::ChatzosHayom,
            F::ChatzosHayomAsHalfDay => Self::ChatzosHayomAsHalfDay,
            F::FixedLocalChatzosHayom => Self::FixedLocalChatzosHayom,
            F::MinchaGedolaGra => Self::MinchaGedolaGra,
            F::MinchaGedola16Point1Degrees => Self::MinchaGedola16Point1Degrees,
            F::MinchaGedola30Minutes => Self::MinchaGedola30Minutes,
            F::MinchaGedola72Minutes => Self::MinchaGedola72Minutes,
            F::MinchaGedolaAhavatShalom => Self::MinchaGedolaAhavatShalom,
            F::MinchaGedolaGraGreaterThan30 => Self::MinchaGedolaGraGreaterThan30,
            F::MinchaGedolaAteretTorah => Self::MinchaGedolaAteretTorah,
            F::MinchaGedolaBaalHatanya => Self::MinchaGedolaBaalHatanya,
            F::MinchaGedolaGraFixedLocalChatzos30Minutes => Self::MinchaGedolaGraFixedLocalChatzos30Minutes,
            F::PlagHaminchaGra => Self::PlagHaminchaGra,
            F::PlagAhavatShalom => Self::PlagAhavatShalom,
            F::PlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees => Self::PlagAlos16Point1DegreesToTzaisGeonim7Point083Degrees,
            F::PlagAlosToSunset => Self::PlagAlosToSunset,
            F::PlagHamincha60Minutes => Self::PlagHamincha60Minutes,
            F::PlagHamincha72Minutes => Self::PlagHamincha72Minutes,
            F::PlagHamincha72MinutesZmanis => Self::PlagHamincha72MinutesZmanis,
            F::PlagHamincha90Minutes => Self::PlagHamincha90Minutes,
            F::PlagHamincha90MinutesZmanis => Self::PlagHamincha90MinutesZmanis,
            F::PlagHamincha96Minutes => Self::PlagHamincha96Minutes,
            F::PlagHamincha96MinutesZmanis => Self::PlagHamincha96MinutesZmanis,
            F::PlagHamincha120Minutes => Self::PlagHamincha120Minutes,
            F::PlagHamincha120MinutesZmanis => Self::PlagHamincha120MinutesZmanis,
            F::PlagHamincha16Point1Degrees => Self::PlagHamincha16Point1Degrees,
            F::PlagHamincha18Degrees => Self::PlagHamincha18Degrees,
            F::PlagHamincha19Point8Degrees => Self::PlagHamincha19Point8Degrees,
            F::PlagHamincha26Degrees => Self::PlagHamincha26Degrees,
            F::PlagHaminchaAteretTorah => Self::PlagHaminchaAteretTorah,
            F::PlagHaminchaBaalHatanya => Self::PlagHaminchaBaalHatanya,
            F::PlagHaminchaGraFixedLocalChatzosToSunset => Self::PlagHaminchaGraFixedLocalChatzosToSunset,
            F::SamuchLeMinchaKetanaGra => Self::SamuchLeMinchaKetanaGra,
            F::SamuchLeMinchaKetana16Point1Degrees => Self::SamuchLeMinchaKetana16Point1Degrees,
            F::SamuchLeMinchaKetana72Minutes => Self::SamuchLeMinchaKetana72Minutes,
            F::MinchaKetanaGra => Self::MinchaKetanaGra,
            F::MinchaKetana16Point1Degrees => Self::MinchaKetana16Point1Degrees,
            F::MinchaKetana72Minutes => Self::MinchaKetana72Minutes,
            F::MinchaKetanaAhavatShalom => Self::MinchaKetanaAhavatShalom,
            F::MinchaKetanaAteretTorah => Self::MinchaKetanaAteretTorah,
            F::MinchaKetanaBaalHatanya => Self::MinchaKetanaBaalHatanya,
            F::MinchaKetanaGraFixedLocalChatzosToSunset => Self::MinchaKetanaGraFixedLocalChatzosToSunset,
            F::BainHashmashosRt13Point24Degrees => Self::BainHashmashosRt13Point24Degrees,
            F::BainHashmashosRt58Point5Minutes => Self::BainHashmashosRt58Point5Minutes,
            F::BainHashmashosRt13Point5MinutesBefore7Point083Degrees => Self::BainHashmashosRt13Point5MinutesBefore7Point083Degrees,
            F::BainHashmashosRt2Stars => Self::BainHashmashosRt2Stars,
            F::BainHashmashosYereim18Minutes => Self::BainHashmashosYereim18Minutes,
            F::BainHashmashosYereim16Point875Minutes => Self::BainHashmashosYereim16Point875Minutes,
            F::BainHashmashosYereim13Point5Minutes => Self::BainHashmashosYereim13Point5Minutes,
            F::BainHashmashosYereim2Point1Degrees => Self::BainHashmashosYereim2Point1Degrees,
            F::BainHashmashosYereim2Point8Degrees => Self::BainHashmashosYereim2Point8Degrees,
            F::BainHashmashosYereim3Point05Degrees => Self::BainHashmashosYereim3Point05Degrees,
            F::CandleLighting => Self::CandleLighting,
            F::PolarSunsetBenIshChai => Self::PolarSunsetBenIshChai,
            F::Sunset => Self::Sunset,
            F::SeaLevelSunset => Self::SeaLevelSunset,
            F::TzaisGeonim8Point5Degrees => Self::TzaisGeonim8Point5Degrees,
            F::Tzais50Minutes => Self::Tzais50Minutes,
            F::Tzais60Minutes => Self::Tzais60Minutes,
            F::Tzais72Minutes => Self::Tzais72Minutes,
            F::Tzais72Zmanis => Self::Tzais72Zmanis,
            F::Tzais90Minutes => Self::Tzais90Minutes,
            F::Tzais90Zmanis => Self::Tzais90Zmanis,
            F::Tzais96Minutes => Self::Tzais96Minutes,
            F::Tzais96Zmanis => Self::Tzais96Zmanis,
            F::Tzais120Minutes => Self::Tzais120Minutes,
            F::Tzais120Zmanis => Self::Tzais120Zmanis,
            F::Tzais16Point1Degrees => Self::Tzais16Point1Degrees,
            F::Tzais18Degrees => Self::Tzais18Degrees,
            F::Tzais19Point8Degrees => Self::Tzais19Point8Degrees,
            F::Tzais26Degrees => Self::Tzais26Degrees,
            F::TzaisAteretTorah => Self::TzaisAteretTorah,
            F::TzaisBaalHatanya => Self::TzaisBaalHatanya,
            F::TzaisGeonim3Point7Degrees => Self::TzaisGeonim3Point7Degrees,
            F::TzaisGeonim3Point8Degrees => Self::TzaisGeonim3Point8Degrees,
            F::TzaisGeonim5Point95Degrees => Self::TzaisGeonim5Point95Degrees,
            F::TzaisGeonim4Point66Degrees => Self::TzaisGeonim4Point66Degrees,
            F::TzaisGeonim4Point42Degrees => Self::TzaisGeonim4Point42Degrees,
            F::TzaisGeonim4Point8Degrees => Self::TzaisGeonim4Point8Degrees,
            F::TzaisGeonim6Point45Degrees => Self::TzaisGeonim6Point45Degrees,
            F::TzaisGeonim7Point083Degrees => Self::TzaisGeonim7Point083Degrees,
            F::TzaisGeonim7Point67Degrees => Self::TzaisGeonim7Point67Degrees,
            F::TzaisGeonim9Point3Degrees => Self::TzaisGeonim9Point3Degrees,
            F::TzaisGeonim9Point75Degrees => Self::TzaisGeonim9Point75Degrees,
            F::SofZmanKidushLevana15Days => Self::SofZmanKidushLevana15Days,
            F::SofZmanKidushLevanaBetweenMoldos => Self::SofZmanKidushLevanaBetweenMoldos,
            F::TchilasZmanKidushLevana3Days => Self::TchilasZmanKidushLevana3Days,
            F::TchilasZmanKidushLevana7Days => Self::TchilasZmanKidushLevana7Days,
            F::SolarMidnight => Self::SolarMidnight,
            F::ChatzosHalayla => Self::ChatzosHalayla,
        }
    }
}
// END GENERATED: Zman
