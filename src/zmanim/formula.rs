// The expression language every zman in the catalog is written in. A `Formula` is either an
// astronomical event (sunrise, sunset, the sun at some depression), an offset from another
// formula (fixed minutes or shaos zmaniyos), or a halachic time derived from the two ends of a
// day. `evaluate` computes it for one place and date.

use core_maths::CoreFloat;
use icu_calendar::{
    options::{DateAddOptions, Overflow},
    types::DateDuration,
};
use jiff::{SignedDuration as Duration, Timestamp, ToSpan, tz::Offset};

use crate::{
    Zone,
    calendar::{HebrewDate, Region, SpecialDay, molad::months_molad},
    zmanim::{Zmanim, ZmanimError, minus, offset, plus},
};

static CIVIL_ZENITH: f64 = 6.0;
static NAUTICAL_ZENITH: f64 = 12.0;
static ASTRONOMICAL_ZENITH: f64 = 18.0;

/// One node of a zman's formula. See the comment at the top of the file.
#[derive(Debug, Clone)]
pub(crate) enum Formula {
    /// Sunrise at the configured location/date.
    ElevationAdjustedSunrise,
    /// Sunrise at sea level (no elevation adjustment).
    SeaLevelSunrise,
    /// Sunrise using the configured elevation mode (sea-level or elevation-adjusted).
    ConfiguredSunrise,
    /// Sunset using the configured elevation mode (sea-level or elevation-adjusted).
    ConfiguredSunset,
    /// Solar transit (local apparent noon / astronomical chatzos).
    SolarTransit,
    /// Solar anti-transit (local apparent midnight / astronomical chatzos halayla).
    SolarMidnight,
    /// The midpoint between sunrise and sunset.
    ChatzosHayomAsHalfDay,
    /// Solar Transit or the midpoint between sunrise and sunset depending on the configuration.
    ChatzosHayom,
    /// The midpoint between sunset and the tomorrows sunrise.
    ChatzosHalaylaAsHalfDay,
    /// Solar Midnight or the midpoint between sunset and the tomorrows sunrise depending on the configuration.
    ChatzosHalayla,
    /// Sunset at the configured location/date.
    ElevationAdjustedSunset,
    /// Sunset at sea level (no elevation adjustment).
    SeaLevelSunset,
    /// Time before sunrise when the sun is `degrees` below the geometric horizon (no elevation adjustment).
    SunriseOffsetByDegrees(f64),
    /// Time after sunset when the sun is `degrees` below the geometric horizon (no elevation adjustment).
    SunsetOffsetByDegrees(f64),
    /// Local mean time at the given hour (0.0–24.0).
    LocalMeanTime(f64),
    /// Shabbos/Yom Tov candle lighting time based on configuration.
    CandleLighting,
    /// A fixed time offset from another [`Formula`].
    Offset(&'static Formula, Duration),
    /// An offset in "shaos zmaniyos" according to the GRA from another [`Formula`].
    ZmanisOffset(&'static Formula, f64),
    /// An offset expressed as a fraction of the half-day between two [`Formula`]s.
    HalfDayBasedOffset(&'static Formula, &'static Formula, f64),
    /// Sof zman shma derived from two bounding [`Formula`]s.
    Shema(&'static Formula, &'static Formula, bool),
    /// Mincha gedola derived from two bounding [`Formula`]s.
    MinchaGedola(&'static Formula, &'static Formula, bool),
    /// Samuch le-mincha ketana derived from two bounding [`Formula`]s.
    SamuchLeMinchaKetana(&'static Formula, &'static Formula, bool),
    /// Mincha ketana derived from two bounding [`Formula`]s.
    MinchaKetana(&'static Formula, &'static Formula, bool),
    /// Sof zman tefila derived from two bounding [`Formula`]s.
    Tefila(&'static Formula, &'static Formula, bool),
    /// Plag hamincha derived from two bounding [`Formula`]s.
    PlagHamincha(&'static Formula, &'static Formula, bool),
    /// Sof zman biur chametz derived from two bounding [`Formula`]s.
    SofZmanBiurChametz(&'static Formula, &'static Formula, bool),
    /// Sof zman achilas chametz derived from two bounding [`Formula`]s.
    SofZmanAchilasChametz(&'static Formula, &'static Formula, bool),
    /// Tzais according to the shita of Yeshivas Ateret Torah
    TzaisAteretTorah,
    /// The latest time of _Kiddush Levana_ calculated as 15 days after the molad.
    SofZmanKidushLevana15Days,
    /// The latest time of _Kiddush Levana_ according to the the opinion of the Maharil
    /// that it is calculated as halfway between molad and molad.
    SofZmanKidushLevanaBetweenMoldos,
    /// The earliest time of _Kiddush Levana_ according to Rabbeinu Yonahs opinion that it can be said 3 days after the molad.
    TchilasZmanKidushLevana3Days,
    /// The earliest time of _Kiddush Levana_ according to the opinions that it should
    /// not be said until 7 days after the molad.
    TchilasZmanKidushLevana7Days,
    /// Bain hashmashos (Rabbeinu Tam, 2-stars) is `sunset + (sunrise - alos19.8°) * 5/18`.
    BainHashmashosRt2Stars,
    /// Mincha gedola (Ahavat Shalom) is the later of `chatzos + 30m` and `chatzos + 1/2 shaah`.
    MinchaGedolaAhavatShalom,
    /// Mincha gedola GRA, but no earlier than 30 minutes after chatzos.
    MinchaGedolaGraGreaterThan30,
    /// Mincha ketana (Ahavat Shalom) is `2.5` shaos zmaniyos before tzais `3.8°` (day = alos16.1° → tzais3.8°).
    MinchaKetanaAhavatShalom,
    /// Plag hamincha (Ahavat Shalom) is `1.25` shaos zmaniyos before tzais `3.8°` (day = alos16.1° → tzais3.8°).
    PlagAhavatShalom,
    /// The beginning of civil twilight (dawn), when the sun is 6° below the geometric horizon (96° zenith).
    BeginCivilTwilight,
    /// The end of civil twilight, when the sun is 6° below the geometric horizon (96° zenith).
    EndCivilTwilight,
    /// The beginning of nautical twilight, when the sun is 12° below the geometric horizon (102° zenith).
    BeginNauticalTwilight,
    /// The end of nautical twilight, when the sun is 12° below the geometric horizon (102° zenith).
    EndNauticalTwilight,
    /// The beginning of astronomical twilight, when the sun is 18° below the geometric horizon (108° zenith).
    BeginAstronomicalTwilight,
    /// The end of astronomical twilight, when the sun is 18° below the geometric horizon (108° zenith).
    EndAstronomicalTwilight,
    /// Polar sunset (Ben Ish Chai) is due west azimuth on polar no-sunset days only.
    PolarSunsetBenIshChai,
    /// Polar sunrise (Ben Ish Chai) is due east azimuth on polar no-sunrise days only.
    PolarSunriseBenIshChai,
}

/// Whether `time` falls on `date` in `timezone`, from the local midnight starting `date` through
/// the local midnight ending it. This is the check KosherJava's `getMoladBasedTime` makes.
fn is_on_date(time: Timestamp, date: jiff::civil::Date, timezone: &dyn Zone) -> Result<bool, ZmanimError> {
    Ok(time >= timezone.start_of_day(date)? && time <= timezone.start_of_day(date.tomorrow()?)?)
}

/// The molad of `date`'s month.
fn molad_of(date: &icu_calendar::Date<icu_calendar::cal::Hebrew>) -> Result<Timestamp, ZmanimError> {
    months_molad(date).ok_or(ZmanimError::Internal)
}

impl Formula {
    pub(crate) fn evaluate<Z: Zone>(&self, calculator: &Zmanim<Z>) -> Result<Timestamp, ZmanimError> {
        match *self {
            Formula::ConfiguredSunrise => {
                if calculator.config.use_elevation {
                    Formula::ElevationAdjustedSunrise.evaluate(calculator)
                } else {
                    Formula::SeaLevelSunrise.evaluate(calculator)
                }
            }
            Formula::ConfiguredSunset => {
                if calculator.config.use_elevation {
                    Formula::ElevationAdjustedSunset.evaluate(calculator)
                } else {
                    Formula::SeaLevelSunset.evaluate(calculator)
                }
            }
            Formula::ElevationAdjustedSunrise => crate::astronomy::sunrise(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
                true,
            ),
            Formula::SeaLevelSunrise => crate::astronomy::sunrise(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
                false,
            ),
            Formula::SolarTransit => crate::astronomy::solar_noon(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
            ),
            Formula::SolarMidnight => crate::astronomy::solar_midnight(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
            ),
            Formula::ElevationAdjustedSunset => crate::astronomy::sunset(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
                true,
            ),
            Formula::SeaLevelSunset => crate::astronomy::sunset(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
                false,
            ),
            Formula::SunriseOffsetByDegrees(degrees) => crate::astronomy::sunrise_offset_by_degrees(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
                degrees,
            ),
            Formula::SunsetOffsetByDegrees(degrees) => crate::astronomy::sunset_offset_by_degrees(
                calculator.config.astronomical_calculator,
                calculator.date,
                &calculator.location,
                calculator.zone(),
                degrees,
            ),
            Formula::LocalMeanTime(hours) => {
                let date = calculator.date;
                let location = &calculator.location;
                if !(0.0..24.0).contains(&hours) {
                    return Err(ZmanimError::Internal);
                }

                let adjusted_date = crate::astronomy::adjusted_local_date(date, location, calculator.zone())?;
                let midnight = adjusted_date.at(0, 0, 0, 0);
                let lmt_nanos = CoreFloat::round(hours * 3600.0 * 1_000_000_000.0) as i64;
                let offset_nanos = CoreFloat::round(location.longitude * 240.0 * 1_000_000_000.0) as i64;
                let lmt_dt = midnight
                    .checked_add(Duration::from_nanos(lmt_nanos))
                    .and_then(|dt| dt.checked_sub(Duration::from_nanos(offset_nanos)))?;
                Ok(Offset::UTC.to_timestamp(lmt_dt)?)
            }
            Formula::CandleLighting => {
                // Sea-level sunset occurs earlier than elevation-adjusted sunset.
                // Since candle lighting times are used strictly *l’chumrah* (stringently),
                // we choose the earlier of the two values.
                //
                // This logic is intentionally limited to candle lighting. For other zmanim
                // (e.g., sunset itself), an earlier time is not universally considered
                // *l’chumrah*, so we do not apply this rule there.
                let sunset = Formula::SeaLevelSunset.evaluate(calculator)?;
                minus(sunset, offset(calculator.config.candle_lighting_offset)?)
            }
            Formula::Offset(event, duration) => {
                let event_time = event.evaluate(calculator)?;
                plus(event_time, duration)
            }
            Formula::ZmanisOffset(event, hours) => {
                let event_time = event.evaluate(calculator)?;
                let sunrise = Formula::ConfiguredSunrise.evaluate(calculator)?;
                let sunset = Formula::ConfiguredSunset.evaluate(calculator)?;
                let shaah_zmanis = sunset.duration_since(sunrise) / 12;
                let offset = shaah_zmanis.mul_f64(hours);

                plus(event_time, offset)
            }

            Formula::HalfDayBasedOffset(event1, event2, hours) => {
                let event1_time = event1.evaluate(calculator)?;
                let event2_time = event2.evaluate(calculator)?;
                let shaah_zmanis = event2_time.duration_since(event1_time) / 6;
                let offset = shaah_zmanis.mul_f64(hours);
                if hours >= 0.0 {
                    plus(event1_time, offset)
                } else {
                    plus(event2_time, offset)
                }
            }
            Formula::Shema(event1, event2, synchronous) => {
                let event1_time = event1.evaluate(calculator)?;
                let event2_time = event2.evaluate(calculator);
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = chatzos.duration_since(event1_time) / 6;
                    let offset = shaah_zmanis.mul_f64(3.0);
                    plus(event1_time, offset)
                } else {
                    let event2_time = event2_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(3.0);
                    plus(event1_time, offset)
                }
            }
            Formula::MinchaGedola(event1, event2, synchronous) => {
                let event1_time = event1.evaluate(calculator);
                let event2_time = event2.evaluate(calculator)?;
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = event2_time.duration_since(chatzos) / 6;
                    let offset = shaah_zmanis.mul_f64(0.5);
                    plus(chatzos, offset)
                } else {
                    let event1_time = event1_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(6.5);
                    plus(event1_time, offset)
                }
            }
            Formula::SamuchLeMinchaKetana(event1, event2, synchronous) => {
                let event1_time = event1.evaluate(calculator);
                let event2_time = event2.evaluate(calculator)?;
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = event2_time.duration_since(chatzos) / 6;
                    let offset = shaah_zmanis.mul_f64(3.0);
                    plus(chatzos, offset)
                } else {
                    let event1_time = event1_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(9.0);
                    plus(event1_time, offset)
                }
            }
            Formula::MinchaKetana(event1, event2, synchronous) => {
                let event1_time = event1.evaluate(calculator);
                let event2_time = event2.evaluate(calculator)?;
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = event2_time.duration_since(chatzos) / 6;
                    let offset = shaah_zmanis.mul_f64(3.5);
                    plus(chatzos, offset)
                } else {
                    let event1_time = event1_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(9.5);
                    plus(event1_time, offset)
                }
            }
            Formula::Tefila(event1, event2, synchronous) => {
                let event1_time = event1.evaluate(calculator)?;
                let event2_time = event2.evaluate(calculator);
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = chatzos.duration_since(event1_time) / 6;
                    let offset = shaah_zmanis.mul_f64(4.0);
                    plus(event1_time, offset)
                } else {
                    let event2_time = event2_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(4.0);
                    plus(event1_time, offset)
                }
            }
            Formula::PlagHamincha(event1, event2, synchronous) => {
                let event1_time = event1.evaluate(calculator);
                let event2_time = event2.evaluate(calculator)?;
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = event2_time.duration_since(chatzos) / 6;
                    let offset = shaah_zmanis.mul_f64(4.75);
                    plus(chatzos, offset)
                } else {
                    let event1_time = event1_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(10.75);
                    plus(event1_time, offset)
                }
            }
            Formula::SofZmanBiurChametz(event1, event2, synchronous) => {
                if !HebrewDate::try_from(calculator.date)?
                    .special_days(Region::Diaspora)
                    .any(|h| h == SpecialDay::ErevPesach)
                {
                    return Err(ZmanimError::NotApplicable);
                }
                let primitive = if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    Formula::HalfDayBasedOffset(event1, &Formula::ChatzosHayom, 5.0)
                } else {
                    let event1_time = event1.evaluate(calculator)?;
                    let event2_time = event2.evaluate(calculator)?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(5.0);
                    return plus(event1_time, offset);
                };
                primitive.evaluate(calculator)
            }
            Formula::SofZmanAchilasChametz(event1, event2, synchronous) => {
                if !HebrewDate::try_from(calculator.date)?
                    .special_days(Region::Diaspora)
                    .any(|h| h == SpecialDay::ErevPesach)
                {
                    return Err(ZmanimError::NotApplicable);
                }
                let event1_time = event1.evaluate(calculator)?;
                let event2_time = event2.evaluate(calculator);
                if calculator.config.use_astronomical_chatzos_for_other_zmanim && synchronous {
                    let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                    let shaah_zmanis = chatzos.duration_since(event1_time) / 6;
                    let offset = shaah_zmanis.mul_f64(4.0);
                    plus(event1_time, offset)
                } else {
                    let event2_time = event2_time?;
                    let shaah_zmanis = event2_time.duration_since(event1_time) / 12;
                    let offset = shaah_zmanis.mul_f64(4.0);
                    plus(event1_time, offset)
                }
            }
            Formula::TzaisAteretTorah => {
                let sunset = Formula::ConfiguredSunset.evaluate(calculator)?;
                plus(sunset, offset(calculator.config.ateret_torah_sunset_offset)?)
            }
            Formula::SofZmanKidushLevana15Days => {
                let tz = calculator.zone().ok_or(ZmanimError::TimeZoneRequired)?;
                let hebrew = HebrewDate::try_from(calculator.date)?.icu();

                if hebrew.day_of_month().0 < 11 || hebrew.day_of_month().0 > 17 {
                    return Err(ZmanimError::NotApplicable);
                }
                let molad = plus(molad_of(&hebrew)?, Duration::from_hours(24 * 15))?;
                if is_on_date(molad, calculator.date, tz)? {
                    return Ok(molad);
                }
                Err(ZmanimError::NotApplicable)
            }
            Formula::SofZmanKidushLevanaBetweenMoldos => {
                let tz = calculator.zone().ok_or(ZmanimError::TimeZoneRequired)?;
                let hebrew = HebrewDate::try_from(calculator.date)?.icu();

                if hebrew.day_of_month().0 < 11 || hebrew.day_of_month().0 > 16 {
                    return Err(ZmanimError::NotApplicable);
                }
                let molad = plus(
                    molad_of(&hebrew)?,
                    Duration::from_hours(24 * 14 + 18)
                        + Duration::from_mins(22)
                        + Duration::from_secs(1)
                        + Duration::from_millis(666),
                )?;
                if is_on_date(molad, calculator.date, tz)? {
                    return Ok(molad);
                }
                Err(ZmanimError::NotApplicable)
            }
            Formula::TchilasZmanKidushLevana3Days => {
                let tz = calculator.zone().ok_or(ZmanimError::TimeZoneRequired)?;
                let hebrew = HebrewDate::try_from(calculator.date)?.icu();

                if hebrew.day_of_month().0 > 5 && hebrew.day_of_month().0 < 30 {
                    return Err(ZmanimError::NotApplicable);
                }
                let molad = plus(molad_of(&hebrew)?, Duration::from_hours(72))?;

                if is_on_date(molad, calculator.date, tz)? {
                    return Ok(molad);
                }

                if hebrew.day_of_month().0 == 30 {
                    let mut add_option = DateAddOptions::default();
                    add_option.overflow = Some(Overflow::Constrain);

                    let new = hebrew.try_added_with_options(DateDuration::for_months(1), add_option)?;
                    let molad = plus(molad_of(&new)?, Duration::from_hours(72))?;
                    if is_on_date(molad, calculator.date, tz)? {
                        return Ok(molad);
                    }
                }
                Err(ZmanimError::NotApplicable)
            }
            Formula::TchilasZmanKidushLevana7Days => {
                let tz = calculator.zone().ok_or(ZmanimError::TimeZoneRequired)?;
                let hebrew = HebrewDate::try_from(calculator.date)?.icu();

                if hebrew.day_of_month().0 < 4 || hebrew.day_of_month().0 > 9 {
                    return Err(ZmanimError::NotApplicable);
                }
                let molad = plus(molad_of(&hebrew)?, Duration::from_hours(168))?;
                if is_on_date(molad, calculator.date, tz)? {
                    return Ok(molad);
                }
                Err(ZmanimError::NotApplicable)
            }
            Formula::BainHashmashosRt2Stars => {
                let alos19_point_8 = Formula::SunriseOffsetByDegrees(19.8).evaluate(calculator)?;
                let sunrise = Formula::ConfiguredSunrise.evaluate(calculator)?;
                let sunset = Formula::ConfiguredSunset.evaluate(calculator)?;
                let time_diff = sunrise.duration_since(alos19_point_8);
                let offset = time_diff.as_nanos() as f64 * (5.0 / 18.0);
                plus(sunset, Duration::from_nanos(offset as i64))
            }
            Formula::MinchaGedolaAhavatShalom => {
                let chatzos = Formula::ChatzosHayom.evaluate(calculator)?;
                let mincha_gedola_30 = plus(chatzos, Duration::from_mins(30))?;

                let alos = Formula::SunriseOffsetByDegrees(16.1).evaluate(calculator)?;
                let tzais = Formula::SunsetOffsetByDegrees(3.7).evaluate(calculator)?;
                let shaah_zmanis = tzais.duration_since(alos) / 12;
                let mincha_alternative = plus(chatzos, shaah_zmanis / 2)?;
                if mincha_gedola_30 > mincha_alternative {
                    Ok(mincha_gedola_30)
                } else {
                    Ok(mincha_alternative)
                }
            }
            Formula::MinchaGedolaGraGreaterThan30 => {
                let mincha_gedola_30 =
                    Formula::Offset(&Formula::ChatzosHayom, Duration::from_mins(30)).evaluate(calculator)?;
                let mincha_gedola_gra =
                    Formula::MinchaGedola(&Formula::ConfiguredSunrise, &Formula::ConfiguredSunset, true)
                        .evaluate(calculator)?;
                Ok(mincha_gedola_30.max(mincha_gedola_gra))
            }
            Formula::MinchaKetanaAhavatShalom => {
                let tzais = Formula::SunsetOffsetByDegrees(3.8).evaluate(calculator)?;
                let alos = Formula::SunriseOffsetByDegrees(16.1).evaluate(calculator)?;
                let shaah_zmanis = tzais.duration_since(alos) / 12;
                minus(tzais, shaah_zmanis * 5 / 2)
            }
            Formula::PlagAhavatShalom => {
                let tzais = Formula::SunsetOffsetByDegrees(3.8).evaluate(calculator)?;
                let alos = Formula::SunriseOffsetByDegrees(16.1).evaluate(calculator)?;
                let shaah_zmanis = tzais.duration_since(alos) / 12;
                minus(tzais, shaah_zmanis * 5 / 4)
            }
            Formula::BeginCivilTwilight => Formula::SunriseOffsetByDegrees(CIVIL_ZENITH).evaluate(calculator),
            Formula::EndCivilTwilight => Formula::SunsetOffsetByDegrees(CIVIL_ZENITH).evaluate(calculator),
            Formula::BeginNauticalTwilight => Formula::SunriseOffsetByDegrees(NAUTICAL_ZENITH).evaluate(calculator),
            Formula::EndNauticalTwilight => Formula::SunsetOffsetByDegrees(NAUTICAL_ZENITH).evaluate(calculator),
            Formula::BeginAstronomicalTwilight => {
                Formula::SunriseOffsetByDegrees(ASTRONOMICAL_ZENITH).evaluate(calculator)
            }
            Formula::EndAstronomicalTwilight => {
                Formula::SunsetOffsetByDegrees(ASTRONOMICAL_ZENITH).evaluate(calculator)
            }
            Formula::PolarSunsetBenIshChai => match Formula::ConfiguredSunset.evaluate(calculator) {
                Ok(_) => Err(ZmanimError::NotApplicable),
                Err(ZmanimError::AllDay | ZmanimError::AllNight) => crate::astronomy::time_at_azimuth(
                    calculator.config.astronomical_calculator,
                    calculator.date,
                    &calculator.location,
                    calculator.zone(),
                    270.0,
                ),
                Err(error) => Err(error),
            },
            Formula::PolarSunriseBenIshChai => match Formula::ConfiguredSunrise.evaluate(calculator) {
                Ok(_) => Err(ZmanimError::NotApplicable),
                Err(ZmanimError::AllDay | ZmanimError::AllNight) => crate::astronomy::time_at_azimuth(
                    calculator.config.astronomical_calculator,
                    calculator.date,
                    &calculator.location,
                    calculator.zone(),
                    90.0,
                ),
                Err(error) => Err(error),
            },
            Formula::ChatzosHayomAsHalfDay => {
                let sunrise = Formula::SeaLevelSunrise.evaluate(calculator)?;
                let sunset = Formula::SeaLevelSunset.evaluate(calculator)?;
                let diff = sunset.duration_since(sunrise) / 2;
                plus(sunrise, diff)
            }
            Formula::ChatzosHayom => {
                if calculator.config.use_astronomical_chatzos {
                    Formula::SolarTransit.evaluate(calculator)
                } else {
                    match Formula::ChatzosHayomAsHalfDay.evaluate(calculator) {
                        Ok(chatzos) => Ok(chatzos),
                        Err(_) => Formula::SolarTransit.evaluate(calculator),
                    }
                }
            }
            Formula::ChatzosHalaylaAsHalfDay => {
                let sunset = Formula::SeaLevelSunset.evaluate(calculator)?;
                let tomorrow = calculator.date.checked_add(1.day())?;
                let sunrise = crate::astronomy::sunrise(
                    calculator.config.astronomical_calculator,
                    tomorrow,
                    &calculator.location,
                    calculator.zone(),
                    false,
                )?;
                let diff = sunrise.duration_since(sunset) / 2;
                plus(sunset, diff)
            }
            Formula::ChatzosHalayla => {
                if calculator.config.use_astronomical_chatzos {
                    Formula::SolarMidnight.evaluate(calculator)
                } else {
                    match Formula::ChatzosHalaylaAsHalfDay.evaluate(calculator) {
                        Ok(chatzos) => Ok(chatzos),
                        Err(_) => Formula::SolarMidnight.evaluate(calculator),
                    }
                }
            }
        }
    }
}

#[cfg(feature = "alloc")]
// What a formula reads from the config, so `Zman::description` can say how it was calculated.
impl Formula {
    /// Whether a proportional zman falls in the morning, so that with astronomical chatzos it divides
    /// the morning rather than the afternoon.
    pub(crate) fn is_in_the_morning(&self) -> bool {
        matches!(
            self,
            Formula::Shema(..)
                | Formula::Tefila(..)
                | Formula::SofZmanBiurChametz(..)
                | Formula::SofZmanAchilasChametz(..)
        )
    }

    /// Whether the chatzos the config chooses is chatzos halayla rather than chatzos hayom.
    pub(crate) fn uses_chatzos_halayla(&self) -> bool {
        match self {
            Formula::ChatzosHalayla => true,
            Formula::Offset(event, _) => event.uses_chatzos_halayla(),
            _ => false,
        }
    }

    pub(crate) fn uses_astronomical_chatzos_for_other_zmanim_from_config(&self) -> bool {
        match self {
            Formula::Shema(_, _, synchronous)
            | Formula::MinchaGedola(_, _, synchronous)
            | Formula::SamuchLeMinchaKetana(_, _, synchronous)
            | Formula::MinchaKetana(_, _, synchronous)
            | Formula::Tefila(_, _, synchronous)
            | Formula::PlagHamincha(_, _, synchronous)
            | Formula::SofZmanBiurChametz(_, _, synchronous)
            | Formula::SofZmanAchilasChametz(_, _, synchronous) => *synchronous,
            Formula::MinchaGedolaGraGreaterThan30 => true,
            Formula::ElevationAdjustedSunrise
            | Formula::SeaLevelSunrise
            | Formula::ConfiguredSunrise
            | Formula::ConfiguredSunset
            | Formula::SolarTransit
            | Formula::SolarMidnight
            | Formula::ChatzosHayomAsHalfDay
            | Formula::ChatzosHayom
            | Formula::ChatzosHalaylaAsHalfDay
            | Formula::ChatzosHalayla
            | Formula::ElevationAdjustedSunset
            | Formula::SeaLevelSunset
            | Formula::SunriseOffsetByDegrees(_)
            | Formula::SunsetOffsetByDegrees(_)
            | Formula::LocalMeanTime(_)
            | Formula::CandleLighting
            | Formula::Offset(_, _)
            | Formula::ZmanisOffset(_, _)
            | Formula::HalfDayBasedOffset(_, _, _)
            | Formula::TzaisAteretTorah
            | Formula::BainHashmashosRt2Stars
            | Formula::SofZmanKidushLevana15Days
            | Formula::SofZmanKidushLevanaBetweenMoldos
            | Formula::TchilasZmanKidushLevana3Days
            | Formula::TchilasZmanKidushLevana7Days
            | Formula::MinchaGedolaAhavatShalom
            | Formula::MinchaKetanaAhavatShalom
            | Formula::PlagAhavatShalom
            | Formula::BeginCivilTwilight
            | Formula::EndCivilTwilight
            | Formula::BeginNauticalTwilight
            | Formula::EndNauticalTwilight
            | Formula::BeginAstronomicalTwilight
            | Formula::EndAstronomicalTwilight
            | Formula::PolarSunsetBenIshChai
            | Formula::PolarSunriseBenIshChai => false,
        }
    }

    pub(crate) fn uses_astronomical_chatzos_from_config(&self) -> bool {
        match self {
            Formula::ChatzosHayom
            | Formula::ChatzosHalayla
            | Formula::MinchaGedolaAhavatShalom
            | Formula::MinchaGedolaGraGreaterThan30 => true,
            Formula::Offset(event, _) => event.uses_astronomical_chatzos_from_config(),
            Formula::ElevationAdjustedSunrise
            | Formula::SeaLevelSunrise
            | Formula::ConfiguredSunrise
            | Formula::ConfiguredSunset
            | Formula::SolarTransit
            | Formula::SolarMidnight
            | Formula::ChatzosHayomAsHalfDay
            | Formula::ChatzosHalaylaAsHalfDay
            | Formula::ElevationAdjustedSunset
            | Formula::SeaLevelSunset
            | Formula::SunriseOffsetByDegrees(_)
            | Formula::SunsetOffsetByDegrees(_)
            | Formula::LocalMeanTime(_)
            | Formula::CandleLighting
            | Formula::ZmanisOffset(_, _)
            | Formula::HalfDayBasedOffset(_, _, _)
            | Formula::Shema(_, _, _)
            | Formula::MinchaGedola(_, _, _)
            | Formula::SamuchLeMinchaKetana(_, _, _)
            | Formula::MinchaKetana(_, _, _)
            | Formula::Tefila(_, _, _)
            | Formula::PlagHamincha(_, _, _)
            | Formula::SofZmanBiurChametz(_, _, _)
            | Formula::SofZmanAchilasChametz(_, _, _)
            | Formula::TzaisAteretTorah
            | Formula::BainHashmashosRt2Stars
            | Formula::SofZmanKidushLevana15Days
            | Formula::SofZmanKidushLevanaBetweenMoldos
            | Formula::TchilasZmanKidushLevana3Days
            | Formula::TchilasZmanKidushLevana7Days
            | Formula::MinchaKetanaAhavatShalom
            | Formula::PlagAhavatShalom
            | Formula::BeginCivilTwilight
            | Formula::EndCivilTwilight
            | Formula::BeginNauticalTwilight
            | Formula::EndNauticalTwilight
            | Formula::BeginAstronomicalTwilight
            | Formula::EndAstronomicalTwilight
            | Formula::PolarSunsetBenIshChai
            | Formula::PolarSunriseBenIshChai => false,
        }
    }

    pub(crate) fn uses_elevation_from_config(&self) -> bool {
        match self {
            Formula::ConfiguredSunrise | Formula::ConfiguredSunset => true,
            Formula::Offset(event, _) => event.uses_elevation_from_config(),
            Formula::ZmanisOffset(_, _) => true,
            Formula::HalfDayBasedOffset(start, end, _)
            | Formula::Shema(start, end, _)
            | Formula::MinchaGedola(start, end, _)
            | Formula::SamuchLeMinchaKetana(start, end, _)
            | Formula::MinchaKetana(start, end, _)
            | Formula::Tefila(start, end, _)
            | Formula::PlagHamincha(start, end, _)
            | Formula::SofZmanBiurChametz(start, end, _)
            | Formula::SofZmanAchilasChametz(start, end, _) => {
                start.uses_elevation_from_config() || end.uses_elevation_from_config()
            }
            Formula::TzaisAteretTorah
            | Formula::BainHashmashosRt2Stars
            | Formula::MinchaGedolaGraGreaterThan30
            | Formula::PolarSunsetBenIshChai
            | Formula::PolarSunriseBenIshChai => true,
            Formula::ElevationAdjustedSunrise
            | Formula::SeaLevelSunrise
            | Formula::SolarTransit
            | Formula::SolarMidnight
            | Formula::ChatzosHayomAsHalfDay
            | Formula::ChatzosHayom
            | Formula::ChatzosHalaylaAsHalfDay
            | Formula::ChatzosHalayla
            | Formula::ElevationAdjustedSunset
            | Formula::SeaLevelSunset
            | Formula::SunriseOffsetByDegrees(_)
            | Formula::SunsetOffsetByDegrees(_)
            | Formula::LocalMeanTime(_)
            | Formula::CandleLighting
            | Formula::SofZmanKidushLevana15Days
            | Formula::SofZmanKidushLevanaBetweenMoldos
            | Formula::TchilasZmanKidushLevana3Days
            | Formula::TchilasZmanKidushLevana7Days
            | Formula::MinchaGedolaAhavatShalom
            | Formula::MinchaKetanaAhavatShalom
            | Formula::PlagAhavatShalom
            | Formula::BeginCivilTwilight
            | Formula::EndCivilTwilight
            | Formula::BeginNauticalTwilight
            | Formula::EndNauticalTwilight
            | Formula::BeginAstronomicalTwilight
            | Formula::EndAstronomicalTwilight => false,
        }
    }
}
