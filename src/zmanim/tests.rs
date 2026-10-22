#![allow(clippy::unwrap_used, deprecated)]

use jiff::{SignedDuration as Duration, Timestamp, civil::Date, tz::TimeZone};
extern crate std;
use std::{string::String, string::ToString};

use super::formula::Formula;
use crate::Location;
use crate::zmanim::{AstronomicalCalculator, Config, Zman, Zmanim, ZmanimError};

/// Anything these tests calculate: a catalog `Zman` or a raw `Formula`.
trait Calc {
    fn calculate(&self, zmanim: &Zmanim) -> Result<Timestamp, ZmanimError>;
}

impl Calc for Formula {
    fn calculate(&self, zmanim: &Zmanim) -> Result<Timestamp, ZmanimError> {
        self.evaluate(zmanim)
    }
}

impl Calc for Zman {
    fn calculate(&self, zmanim: &Zmanim) -> Result<Timestamp, ZmanimError> {
        zmanim.get(*self)
    }
}

const LAKEWOOD_LAT: f64 = 40.0721087;
const LAKEWOOD_LON: f64 = -74.2400243;
const LAKEWOOD_ELEVATION_M: f64 = 15.0;
const MAX_TIME_DIFF_SECONDS: i64 = 30;
const MAX_SHAAH_ZMANIS_DIFF_MS: i64 = 1000;

fn lakewood_tz() -> TimeZone {
    TimeZone::get("America/New_York").unwrap()
}

fn lakewood_location(elevation_m: f64) -> Location {
    Location::new(LAKEWOOD_LAT, LAKEWOOD_LON)
        .and_then(|l| l.with_elevation(elevation_m))
        .unwrap()
}

fn lakewood_date() -> Date {
    Date::new(2017, 10, 17).unwrap()
}

fn new_calc(elevation_m: f64) -> Zmanim {
    Zmanim::with_config(
        lakewood_location(elevation_m),
        lakewood_date(),
        Some(lakewood_tz()),
        Config {
            candle_lighting_offset: Duration::from_mins(18),
            use_astronomical_chatzos_for_other_zmanim: false,
            use_elevation: true,
            ateret_torah_sunset_offset: Duration::from_mins(40),
            use_astronomical_chatzos: true,
            astronomical_calculator: AstronomicalCalculator::Noaa,
        },
    )
}

fn calc_for(lat: f64, lon: f64, elevation: f64, tz: TimeZone, date: Date) -> Zmanim {
    let location = Location::new(lat, lon)
        .and_then(|l| l.with_elevation(elevation))
        .unwrap();
    Zmanim::with_config(location, date, Some(tz), Default::default())
}

fn fmt_local(dt: Timestamp) -> String {
    dt.to_zoned(lakewood_tz()).strftime("%Y-%m-%dT%H:%M:%S%:z").to_string()
}

fn assert_zman_str(calc: &Zmanim, zman: &dyn Calc, expected: &str) {
    let dt = zman.calculate(calc).unwrap();
    assert_time_str(dt, expected, None);
}
fn assert_zman_str_with_max_time_diff(
    calc: &Zmanim,
    zman: &dyn Calc,
    expected: &str,
    max_time_diff_seconds: Option<i64>,
) {
    let dt = zman.calculate(calc).unwrap();
    assert_time_str(dt, expected, max_time_diff_seconds);
}

fn assert_time_str(dt: Timestamp, expected: &str, max_time_diff_seconds: Option<i64>) {
    let expected_dt: Timestamp = expected.parse().unwrap();
    let diff = dt.duration_since(expected_dt).as_secs().abs();
    assert!(
        diff <= max_time_diff_seconds.unwrap_or(MAX_TIME_DIFF_SECONDS),
        "time mismatch: expected {}, got {} (diff {}s)",
        expected,
        fmt_local(dt),
        diff
    );
}

fn assert_duration_ms_close(actual: Duration, expected_ms: i64) {
    let actual_ms = actual.as_millis();
    let diff = (actual_ms - i128::from(expected_ms)).abs();
    assert!(
        diff <= i128::from(MAX_SHAAH_ZMANIS_DIFF_MS),
        "duration mismatch: expected {}ms, got {}ms (diff {}ms)",
        expected_ms,
        actual_ms,
        diff
    );
}

fn shaah_zmanis_by_degrees_and_offset(calc: &Zmanim, degrees: f64, offset_minutes: i64) -> Duration {
    let (start, end) = if degrees.abs() > f64::EPSILON {
        (
            Formula::SunriseOffsetByDegrees(degrees).calculate(calc).unwrap(),
            Formula::SunsetOffsetByDegrees(degrees).calculate(calc).unwrap(),
        )
    } else {
        (
            Formula::ElevationAdjustedSunrise.calculate(calc).unwrap(),
            Formula::ElevationAdjustedSunset.calculate(calc).unwrap(),
        )
    };
    let start = start - Duration::from_mins(offset_minutes);
    let end = end + Duration::from_mins(offset_minutes);
    end.duration_since(start) / 12
}

#[test]
fn test_shaah_zmanis_from_zmanim() {
    let calc = new_calc(0.0);
    let alos = Zman::Alos16Point1Degrees.calculate(&calc).unwrap();
    let tzais = Zman::Tzais16Point1Degrees.calculate(&calc).unwrap();
    let shaah = tzais.duration_since(alos) / 12;
    assert!(shaah.as_secs() > 0);
}

#[test]
fn test_local_mean_time_invalid_hours() {
    let calc = new_calc(0.0);

    assert!(Formula::LocalMeanTime(-1.0).calculate(&calc).is_err());
}

#[test]
fn test_half_day_based_zman_negative_hours() {
    let calc = new_calc(0.0);
    let sunrise = Formula::ElevationAdjustedSunrise.calculate(&calc).unwrap();
    let sunset = Formula::ElevationAdjustedSunset.calculate(&calc).unwrap();
    let shaah = sunset.duration_since(sunrise) / 6;
    let expected = sunset + shaah.mul_f64(-1.0);
    let actual = Formula::HalfDayBasedOffset(
        &Formula::ElevationAdjustedSunrise,
        &Formula::ElevationAdjustedSunset,
        -1.0,
    )
    .calculate(&calc)
    .unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_high_latitude_sunrise_sunset_ordering() {
    let date = Date::new(2017, 3, 21).unwrap();
    let calc = calc_for(
        64.1466,
        -21.9426,
        0.0,
        TimeZone::get("Atlantic/Reykjavik").unwrap(),
        date,
    );
    #[allow(clippy::expect_used)]
    let sunrise = Formula::ElevationAdjustedSunrise.calculate(&calc).unwrap();
    #[allow(clippy::expect_used)]
    let sunset = Formula::ElevationAdjustedSunset.calculate(&calc).unwrap();
    assert!(sunrise < sunset);

    let dawn = Formula::SunriseOffsetByDegrees(6.0).calculate(&calc).unwrap();
    let dusk = Formula::SunsetOffsetByDegrees(6.0).calculate(&calc).unwrap();
    assert!(dawn < sunrise);
    assert!(dusk > sunset);
}

#[test]
fn test_extreme_elevation_shifts_sunrise_sunset() {
    let date = Date::new(2017, 10, 17).unwrap();
    let high = calc_for(27.9881, 86.9250, 8848.0, TimeZone::get("Asia/Kathmandu").unwrap(), date);
    let sea = calc_for(27.9881, 86.9250, 0.0, TimeZone::get("Asia/Kathmandu").unwrap(), date);

    let sunrise_high = Formula::ElevationAdjustedSunrise.calculate(&high).unwrap();
    let sunrise_sea = Formula::ElevationAdjustedSunrise.calculate(&sea).unwrap();
    assert!(sunrise_high < sunrise_sea);

    let sunset_high = Formula::ElevationAdjustedSunset.calculate(&high).unwrap();
    let sunset_sea = Formula::ElevationAdjustedSunset.calculate(&sea).unwrap();
    assert!(sunset_high > sunset_sea);
}

#[test]
fn test_split_horizons_use_east_for_sunrise_and_west_for_sunset() {
    let date = Date::new(2017, 10, 17).unwrap();
    let tz = TimeZone::get("Asia/Kathmandu").unwrap();
    let split_location = Location::new(27.9881, 86.9250)
        .and_then(|l| l.with_elevation(8848.0))
        .and_then(|l| l.with_horizons(0.0, 8848.0))
        .unwrap();
    let split = Zmanim::with_config(
        split_location,
        date,
        Some(tz.clone()),
        Config {
            use_elevation: true,
            ..Default::default()
        },
    );
    let high = calc_for(27.9881, 86.9250, 8848.0, tz.clone(), date);
    let sea = calc_for(27.9881, 86.9250, 0.0, tz, date);

    assert_eq!(
        Formula::ElevationAdjustedSunrise.calculate(&split).unwrap(),
        Formula::ElevationAdjustedSunrise.calculate(&high).unwrap()
    );
    assert_eq!(
        Formula::ElevationAdjustedSunset.calculate(&split).unwrap(),
        Formula::ElevationAdjustedSunset.calculate(&sea).unwrap()
    );
}

#[test]
fn test_polar_ben_ish_chai_unavailable_on_normal_days() {
    let calc = new_calc(0.0);
    assert!(Formula::PolarSunriseBenIshChai.calculate(&calc).is_err());
    assert!(Formula::PolarSunsetBenIshChai.calculate(&calc).is_err());
    assert!(Zman::PolarSunriseBenIshChai.calculate(&calc).is_err());
    assert!(Zman::PolarSunsetBenIshChai.calculate(&calc).is_err());
}

#[test]
fn test_polar_ben_ish_chai_azimuth_fallback_on_polar_day() {
    let date = Date::new(2017, 6, 21).unwrap();
    let calc = calc_for(69.6492, 18.9553, 0.0, TimeZone::get("Europe/Oslo").unwrap(), date);

    assert!(Formula::ElevationAdjustedSunrise.calculate(&calc).is_err());
    assert!(Formula::PolarSunriseBenIshChai.calculate(&calc).is_ok());
    assert!(Formula::PolarSunsetBenIshChai.calculate(&calc).is_ok());
}

#[test]
fn test_polar_day_returns_none_for_sun_times() {
    let date = Date::new(2017, 6, 21).unwrap();
    let calc = calc_for(69.6492, 18.9553, 0.0, TimeZone::get("Europe/Oslo").unwrap(), date);

    assert!(Formula::ElevationAdjustedSunrise.calculate(&calc).is_err());
    assert!(Formula::ElevationAdjustedSunset.calculate(&calc).is_err());
    assert!(Formula::SeaLevelSunrise.calculate(&calc).is_err());
    assert!(Formula::SeaLevelSunset.calculate(&calc).is_err());
    assert!(Formula::SunriseOffsetByDegrees(6.0).calculate(&calc).is_err());
    assert!(Formula::SunsetOffsetByDegrees(6.0).calculate(&calc).is_err());
}

#[test]
fn test_reykjavik_equinox_java_expected_times() {
    let date = Date::new(2017, 3, 21).unwrap();
    let calc = calc_for(
        64.1466,
        -21.9426,
        0.0,
        TimeZone::get("Atlantic/Reykjavik").unwrap(),
        date,
    );

    assert_zman_str(&calc, &Zman::Sunrise, "2017-03-21T07:24:24Z");
    assert_zman_str(&calc, &Zman::Sunset, "2017-03-21T19:46:56Z");
    assert_zman_str(&calc, &Zman::SeaLevelSunrise, "2017-03-21T07:24:24Z");
    assert_zman_str(&calc, &Zman::SeaLevelSunset, "2017-03-21T19:46:56Z");
    assert_zman_str(&calc, &Zman::ChatzosHayom, "2017-03-21T13:34:59Z");
}

#[test]
fn test_everest_java_expected_times() {
    let date = Date::new(2017, 10, 17).unwrap();
    let calc = calc_for(27.9881, 86.9250, 8826.0, TimeZone::get("Asia/Kathmandu").unwrap(), date);
    assert_zman_str_with_max_time_diff(&calc, &Zman::Sunrise, "2017-10-17T05:44:51+05:45", Some(1));
    assert_zman_str_with_max_time_diff(&calc, &Zman::Sunset, "2017-10-17T17:40:04+05:45", Some(1));
    assert_zman_str_with_max_time_diff(&calc, &Zman::SeaLevelSunrise, "2017-10-17T05:58:42+05:45", Some(1));
    assert_zman_str_with_max_time_diff(&calc, &Zman::SeaLevelSunset, "2017-10-17T17:26:12+05:45", Some(1));
    assert_zman_str_with_max_time_diff(&calc, &Zman::ChatzosHayom, "2017-10-17T11:42:38+05:45", Some(1));
}

#[test]
fn test_lakewood_noaa_baseline_events() {
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Sunrise,
        "2017-10-17T07:09:11-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Sunset,
        "2017-10-17T18:14:38-04:00",
    );
    assert_zman_str(&new_calc(0.0), &Zman::ChatzosHayom, "2017-10-17T12:41:55-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Alos16Point1Degrees, "2017-10-17T05:49:30-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Tzais16Point1Degrees, "2017-10-17T19:34:14-04:00");
}

#[test]
fn test_polar_night_returns_none_for_sun_times() {
    let date = Date::new(2017, 12, 21).unwrap();
    let calc = calc_for(69.6492, 18.9553, 0.0, TimeZone::get("Europe/Oslo").unwrap(), date);

    assert!(matches!(
        Formula::ElevationAdjustedSunrise.calculate(&calc),
        Err(ZmanimError::AllNight)
    ));
    assert!(matches!(
        Formula::ElevationAdjustedSunset.calculate(&calc),
        Err(ZmanimError::AllNight)
    ));
}

#[test]
fn test_anti_meridian_timezone_date_adjustment() {
    let date = Date::new(2018, 2, 3).unwrap();
    let timezone = TimeZone::get("Pacific/Apia").unwrap();
    let calc = calc_for(-13.8599, -171.7513, 0.0, timezone.clone(), date);

    let sunrise = Zman::Sunrise.calculate(&calc).unwrap();
    let sunset = Zman::Sunset.calculate(&calc).unwrap();
    let sunrise_local = sunrise.to_zoned(timezone.clone());
    let sunset_local = sunset.to_zoned(timezone);

    assert_eq!(sunrise_local.date(), date);
    assert_eq!(sunset_local.date(), date);
    assert!(sunrise_local.hour() < 12);
    assert!(sunset_local.hour() >= 12);
}

#[test]
fn test_solar_midnight_rolls_to_next_local_date() {
    let date = lakewood_date();
    let calc = new_calc(0.0);
    let midnight = Zman::ChatzosHalayla.calculate(&calc).unwrap();
    let local = midnight.to_zoned(lakewood_tz());

    assert_eq!(local.date(), Date::new(2017, 10, 18).unwrap());
    assert!(local.hour() < 2);
    assert!(local.date() > date);
}

#[test]
fn test_default_zmanim_times() {
    let calc = new_calc(0.0);

    assert_eq!(
        Zman::SeaLevelSunrise.calculate(&calc),
        Formula::SeaLevelSunrise.calculate(&calc)
    );
    assert_eq!(
        Zman::SeaLevelSunset.calculate(&calc),
        Formula::SeaLevelSunset.calculate(&calc)
    );

    assert_zman_str(
        &new_calc(0.0),
        &Zman::TzaisGeonim8Point5Degrees,
        "2017-10-17T18:54:29-04:00",
    );
    assert_zman_str(&new_calc(0.0), &Zman::Tzais19Point8Degrees, "2017-10-17T19:53:34-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Tzais60Minutes, "2017-10-17T19:13:58-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Tzais90Zmanis, "2017-10-17T19:36:59-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Tzais72Minutes, "2017-10-17T19:25:58-04:00");

    assert_zman_str(&new_calc(0.0), &Zman::Alos16Point1Degrees, "2017-10-17T05:49:30-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Alos19Point8Degrees, "2017-10-17T05:30:07-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Alos60Minutes, "2017-10-17T06:09:51-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Alos90Zmanis, "2017-10-17T05:46:50-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Alos72Minutes, "2017-10-17T05:57:51-04:00");

    assert_zman_str(&new_calc(0.0), &Zman::ChatzosHayom, "2017-10-17T12:41:55-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::MinchaGedolaGra, "2017-10-17T13:09:35-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::MinchaKetanaGra, "2017-10-17T15:55:37-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::PlagHaminchaGra, "2017-10-17T17:04:48-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::CandleLighting, "2017-10-17T17:55:58-04:00");
}

#[test]
fn test_default_zmanim_calculations() {
    let calc = new_calc(0.0);

    let sof_zman_shma = Formula::Shema(
        &Formula::SunriseOffsetByDegrees(6.0),
        &Formula::SunsetOffsetByDegrees(6.0),
        true,
    )
    .calculate(&calc)
    .unwrap();
    assert_time_str(sof_zman_shma, "2017-10-17T09:42:10-04:00", None);

    assert_zman_str(&new_calc(0.0), &Zman::SofZmanShmaGra, "2017-10-17T09:55:53-04:00");
    assert_zman_str(
        &new_calc(0.0),
        &Zman::SofZmanShmaMga72Minutes,
        "2017-10-17T09:19:53-04:00",
    );

    let calc = new_calc(0.0);

    let sof_zman_tfila = Formula::Tefila(
        &Formula::SunriseOffsetByDegrees(6.0),
        &Formula::SunsetOffsetByDegrees(6.0),
        true,
    )
    .calculate(&calc)
    .unwrap();
    assert_time_str(sof_zman_tfila, "2017-10-17T10:42:05-04:00", None);

    assert_zman_str(&new_calc(0.0), &Zman::SofZmanTfilaGra, "2017-10-17T10:51:14-04:00");
    assert_zman_str(
        &new_calc(0.0),
        &Zman::SofZmanTfilaMga72Minutes,
        "2017-10-17T10:27:14-04:00",
    );
}

#[test]
fn test_default_shaah_zmanis() {
    let calc = new_calc(0.0);
    let day_start = Formula::SunriseOffsetByDegrees(6.0).calculate(&calc).unwrap();
    let day_end = Formula::SunsetOffsetByDegrees(6.0).calculate(&calc).unwrap();
    let shaah = day_end.duration_since(day_start) / 12;
    assert_duration_ms_close(shaah, 3_594_499);

    let calc = new_calc(0.0);
    let shaah_degrees = shaah_zmanis_by_degrees_and_offset(&calc, 6.0, 0);
    assert_duration_ms_close(shaah_degrees, 3_594_499);

    let calc = new_calc(0.0);
    let shaah_offset = shaah_zmanis_by_degrees_and_offset(&calc, 0.0, 72);
    assert_duration_ms_close(shaah_offset, 4_040_608);

    let calc = new_calc(0.0);
    let shaah_both = shaah_zmanis_by_degrees_and_offset(&calc, 6.0, 72);
    assert_duration_ms_close(shaah_both, 4_314_499);
}

#[test]
fn test_use_elevation_zmanim_times() {
    assert_zman_str(
        &new_calc(0.0),
        &Zman::TzaisGeonim8Point5Degrees,
        "2017-10-17T18:54:29-04:00",
    );
    assert_zman_str(&new_calc(0.0), &Zman::Tzais19Point8Degrees, "2017-10-17T19:53:34-04:00");
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Tzais60Minutes,
        "2017-10-17T19:14:38-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Tzais90Zmanis,
        "2017-10-17T19:37:49-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Tzais72Minutes,
        "2017-10-17T19:26:38-04:00",
    );

    assert_zman_str(&new_calc(0.0), &Zman::Alos16Point1Degrees, "2017-10-17T05:49:30-04:00");
    assert_zman_str(&new_calc(0.0), &Zman::Alos19Point8Degrees, "2017-10-17T05:30:07-04:00");
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Alos60Minutes,
        "2017-10-17T06:09:11-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Alos90Zmanis,
        "2017-10-17T05:46:00-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::Alos72Minutes,
        "2017-10-17T05:57:11-04:00",
    );

    assert_zman_str(&new_calc(0.0), &Zman::ChatzosHayom, "2017-10-17T12:41:55-04:00");
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::MinchaGedolaGra,
        "2017-10-17T13:09:38-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::MinchaKetanaGra,
        "2017-10-17T15:56:00-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::PlagHaminchaGra,
        "2017-10-17T17:05:19-04:00",
    );
    assert_zman_str(&new_calc(0.0), &Zman::CandleLighting, "2017-10-17T17:55:58-04:00");
}

#[test]
fn test_use_elevation_zmanim_calculations() {
    let calc = new_calc(0.0);

    let sof_zman_shma = Formula::Shema(
        &Formula::SunriseOffsetByDegrees(6.0),
        &Formula::SunsetOffsetByDegrees(6.0),
        true,
    )
    .calculate(&calc)
    .unwrap();
    assert_time_str(sof_zman_shma, "2017-10-17T09:42:10-04:00", None);

    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::SofZmanShmaGra,
        "2017-10-17T09:55:33-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::SofZmanShmaMga72Minutes,
        "2017-10-17T09:19:33-04:00",
    );

    let calc = new_calc(0.0);

    let sof_zman_tfila = Formula::Tefila(
        &Formula::SunriseOffsetByDegrees(6.0),
        &Formula::SunsetOffsetByDegrees(6.0),
        true,
    )
    .calculate(&calc)
    .unwrap();
    assert_time_str(sof_zman_tfila, "2017-10-17T10:42:05-04:00", None);

    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::SofZmanTfilaGra,
        "2017-10-17T10:51:00-04:00",
    );
    assert_zman_str(
        &new_calc(LAKEWOOD_ELEVATION_M),
        &Zman::SofZmanTfilaMga72Minutes,
        "2017-10-17T10:27:00-04:00",
    );
}

#[test]
fn test_use_elevation_shaah_zmanis() {
    let calc = new_calc(0.0);
    let day_start = Formula::SunriseOffsetByDegrees(6.0).calculate(&calc).unwrap();
    let day_end = Formula::SunsetOffsetByDegrees(6.0).calculate(&calc).unwrap();
    let shaah = day_end.duration_since(day_start) / 12;
    assert_duration_ms_close(shaah, 3_594_499);

    let calc = new_calc(0.0);
    let shaah_degrees = shaah_zmanis_by_degrees_and_offset(&calc, 6.0, 0);
    assert_duration_ms_close(shaah_degrees, 3_594_499);

    let calc = new_calc(LAKEWOOD_ELEVATION_M);
    let shaah_offset = shaah_zmanis_by_degrees_and_offset(&calc, 0.0, 72);
    assert_duration_ms_close(shaah_offset, 4_047_251);

    let calc = new_calc(0.0);
    let shaah_both = shaah_zmanis_by_degrees_and_offset(&calc, 6.0, 72);
    assert_duration_ms_close(shaah_both, 4_314_499);
}

fn polar_day_calc() -> Zmanim {
    let date = Date::new(2017, 6, 21).unwrap();
    let location = Location::new(69.6492, 18.9553).unwrap();
    Zmanim::with_config(
        location,
        date,
        Some(TimeZone::get("Europe/Oslo").unwrap()),
        Default::default(),
    )
}

#[test]
fn test_polar_day_zmanim_return_none() {
    let alos_variants: [&dyn Calc; 9] = [
        &Zman::Alos60Minutes,
        &Zman::Alos72Minutes,
        &Zman::Alos72Zmanis,
        &Zman::Alos90Minutes,
        &Zman::Alos90Zmanis,
        &Zman::Alos96Minutes,
        &Zman::Alos96Zmanis,
        #[allow(deprecated)]
        &Zman::Alos120Minutes,
        #[allow(deprecated)]
        &Zman::Alos120Zmanis,
    ];
    for zman in alos_variants {
        let calc = polar_day_calc();
        assert!(zman.calculate(&calc).is_err());
    }

    let bain_variants: [&dyn Calc; 6] = [
        &Zman::BainHashmashosRt58Point5Minutes,
        &Zman::BainHashmashosRt13Point5MinutesBefore7Point083Degrees,
        &Zman::BainHashmashosRt2Stars,
        &Zman::BainHashmashosYereim18Minutes,
        &Zman::BainHashmashosYereim16Point875Minutes,
        &Zman::BainHashmashosYereim13Point5Minutes,
    ];
    for zman in bain_variants {
        let calc = polar_day_calc();
        assert!(zman.calculate(&calc).is_err());
    }

    let calc = polar_day_calc();
    assert!(Zman::CandleLighting.calculate(&calc).is_err());

    let calc = polar_day_calc();
    assert!(Zman::ChatzosHayomAsHalfDay.calculate(&calc).is_err());

    let mincha_variants: [&dyn Calc; 5] = [
        &Zman::MinchaGedola16Point1Degrees,
        &Zman::MinchaGedola72Minutes,
        &Zman::MinchaGedolaAhavatShalom,
        &Zman::MinchaGedolaAteretTorah,
        &Zman::MinchaGedolaBaalHatanya,
    ];
    for zman in mincha_variants {
        let calc = polar_day_calc();
        assert!(zman.calculate(&calc).is_err());
    }
}

#[test]
fn test_mcmurdo_antimeridian_alos_matches_java() {
    let tz = TimeZone::get("Antarctica/McMurdo").unwrap();
    let date = Date::new(1901, 4, 18).unwrap();
    let location = Location::new(-77.10300593682803, -166.39303270398793)
        .and_then(|l| l.with_elevation(61.48216934280892))
        .unwrap();
    let config = Config {
        candle_lighting_offset: Duration::from_mins(12),
        use_astronomical_chatzos_for_other_zmanim: true,
        use_elevation: false,
        ateret_torah_sunset_offset: Duration::from_mins(4),
        use_astronomical_chatzos: false,
        astronomical_calculator: AstronomicalCalculator::Noaa,
    };
    let calc = Zmanim::with_config(location, date, Some(tz.clone()), config);

    let alos = Zman::Alos120Minutes.calculate(&calc).unwrap();
    let local = alos.to_zoned(tz);

    assert_eq!(local.date(), date);
    assert!(local.hour() < 12, "alos should be in the morning, got {local}");
    assert_time_str(alos, "1901-04-18T05:45:48.837784872+11:30", Some(1));
}

#[test]
fn test_astronomical_calculator_config_flows_through_presets() {
    let noaa = new_calc(LAKEWOOD_ELEVATION_M);
    let meeus = Zmanim::with_config(
        lakewood_location(LAKEWOOD_ELEVATION_M),
        lakewood_date(),
        Some(lakewood_tz()),
        Config {
            astronomical_calculator: AstronomicalCalculator::Meeus { apply_delta_t: true },
            ..noaa.config
        },
    );
    let spa = Zmanim::with_config(
        lakewood_location(LAKEWOOD_ELEVATION_M),
        lakewood_date(),
        Some(lakewood_tz()),
        Config {
            astronomical_calculator: AstronomicalCalculator::Spa { apply_delta_t: true },
            ..noaa.config
        },
    );

    for preset in [
        &Zman::Sunrise,
        &Zman::SeaLevelSunset,
        &Zman::SunTransit,
        &Zman::SolarMidnight,
        &Zman::Alos16Point1Degrees,
        &Zman::TzaisGeonim8Point5Degrees,
        &Zman::SofZmanShmaGra,
        &Zman::ChatzosHalayla,
    ] {
        let base = preset.calculate(&noaa).unwrap();
        for other in [&meeus, &spa] {
            let value = preset.calculate(other).unwrap();
            let diff_s = value.duration_since(base).as_secs().abs();
            assert_ne!(value, base, "{} should differ from NOAA", preset.name());
            assert!(diff_s < 60, "{} differs from NOAA by {diff_s}s", preset.name());
        }
    }
}

#[test]
fn test_polar_ben_ish_chai_available_for_every_calculator() {
    for calculator in [
        AstronomicalCalculator::Noaa,
        AstronomicalCalculator::Meeus { apply_delta_t: true },
        AstronomicalCalculator::Spa { apply_delta_t: false },
    ] {
        let mut calc = polar_day_calc();
        calc.config.astronomical_calculator = calculator;
        assert!(Zman::PolarSunriseBenIshChai.calculate(&calc).is_ok(), "{calculator:?}");
        assert!(Zman::PolarSunsetBenIshChai.calculate(&calc).is_ok(), "{calculator:?}");
        assert!(Zman::Sunrise.calculate(&calc).is_err(), "{calculator:?}");
    }
}

#[cfg(feature = "alloc")]
#[test]
#[allow(clippy::field_reassign_with_default)]
fn test_preset_description_uses_chatzos_hayom_config_flag() {
    let calc = new_calc(0.0);
    let desc = Zman::ChatzosHayom.description(calc.config());
    assert!(desc.contains("astronomical chatzos (solar transit)"));

    let mut config = Config::default();
    config.use_astronomical_chatzos = false;
    let calc = Zmanim::with_config(lakewood_location(0.0), lakewood_date(), Some(lakewood_tz()), config);
    let desc = Zman::ChatzosHayom.description(calc.config());
    assert!(desc.contains("midpoint between sunrise and sunset"));
}

#[cfg(feature = "alloc")]
#[test]
#[allow(clippy::field_reassign_with_default)]
fn test_preset_description_uses_chatzos_for_other_zmanim_config_flag() {
    let mut config = Config::default();
    config.use_astronomical_chatzos_for_other_zmanim = true;
    let calc = Zmanim::with_config(lakewood_location(0.0), lakewood_date(), Some(lakewood_tz()), config);
    let desc = Zman::SofZmanShmaGra.description(calc.config());
    assert!(desc.contains("divides the afternoon from astronomical chatzos"));
    assert!(!desc.contains("midpoint between sunrise and sunset"));

    config.use_astronomical_chatzos_for_other_zmanim = false;
    let calc = Zmanim::with_config(lakewood_location(0.0), lakewood_date(), Some(lakewood_tz()), config);
    let desc = Zman::SofZmanShmaGra.description(calc.config());
    assert!(desc.contains("fixed fraction of the sunrise-to-sunset interval"));
    assert!(!desc.contains("divides the afternoon from astronomical chatzos"));
}

/// On the first and last days a `Timestamp` holds, with any offsets, every zman is a time or an
/// error.
#[test]
fn test_range_edges_report_errors_not_panics() {
    let dates = [
        Date::new(-9999, 1, 1).unwrap(),
        Date::new(-9999, 1, 2).unwrap(),
        Date::new(9999, 12, 29).unwrap(),
        Date::new(9999, 12, 30).unwrap(),
        Date::new(9999, 12, 31).unwrap(),
    ];
    let configs = [
        Config::default(),
        Config {
            candle_lighting_offset: Duration::MIN,
            ateret_torah_sunset_offset: Duration::MAX,
            ..Config::default()
        },
        Config {
            candle_lighting_offset: Duration::MAX,
            ateret_torah_sunset_offset: Duration::MIN,
            ..Config::default()
        },
    ];
    for date in dates {
        for longitude in [-179.0, -120.0, -55.0, 0.0, 55.0, 120.0, 179.0] {
            let zone = TimeZone::fixed(jiff::tz::Offset::from_hours((longitude / 15.0) as i8).unwrap());
            for timezone in [None, Some(zone)] {
                let Ok(location) = Location::new(0.0, longitude) else {
                    continue;
                };
                for config in configs {
                    let zmanim = Zmanim::with_config(location.clone(), date, timezone.clone(), config);
                    for zman in Zman::ALL {
                        let _ = zmanim.get(zman);
                    }
                }
            }
        }
    }
}

/// Without a time zone, a place more than 150° from Greenwich has no day to calculate for; up to
/// 150° the date is used as given.
#[test]
fn test_time_zone_required_only_near_the_date_line() {
    let date = Date::new(2024, 4, 23).unwrap();
    let far = Zmanim::new(Location::new(0.0, 150.1).unwrap(), date, None);
    assert_eq!(far.get(Zman::Sunrise), Err(ZmanimError::TimeZoneRequired));
    let near = Zmanim::new(Location::new(0.0, 150.0).unwrap(), date, None);
    assert!(near.get(Zman::Sunrise).is_ok());
    let zoned = Zmanim::new(Location::new(0.0, 150.1).unwrap(), date, Some(TimeZone::UTC));
    assert!(zoned.get(Zman::Sunrise).is_ok());
}

/// `get_zoned` is `get` in the *zmanim*'s time zone, and needs one.
#[test]
fn test_get_zoned() {
    let location = Location::new(LAKEWOOD_LAT, LAKEWOOD_LON).unwrap();
    let zoned = Zmanim::new(location.clone(), lakewood_date(), Some(lakewood_tz()));
    let time = zoned.get_zoned(Zman::Sunrise).unwrap();
    assert_eq!(time.timestamp(), zoned.get(Zman::Sunrise).unwrap());
    assert_eq!(time.time_zone(), &lakewood_tz());
    let plain = Zmanim::new(location, lakewood_date(), None);
    assert_eq!(plain.get_zoned(Zman::Sunrise), Err(ZmanimError::TimeZoneRequired));
}

/// Without a time zone, only the four Kiddush Levana times need one; more than 150° from
/// Greenwich, no time can be calculated.
#[test]
fn test_what_needs_a_time_zone() {
    let kiddush_levana = [
        Zman::TchilasZmanKidushLevana3Days,
        Zman::TchilasZmanKidushLevana7Days,
        Zman::SofZmanKidushLevanaBetweenMoldos,
        Zman::SofZmanKidushLevana15Days,
    ];
    let mut date = Date::new(2024, 1, 1).unwrap();
    while date.year() == 2024 {
        let plain = Zmanim::new(Location::new(LAKEWOOD_LAT, LAKEWOOD_LON).unwrap(), date, None);
        for zman in Zman::ALL {
            let needs_one = plain.get(zman) == Err(ZmanimError::TimeZoneRequired);
            assert_eq!(needs_one, kiddush_levana.contains(&zman), "{date} {zman:?}");
        }
        let far = Zmanim::new(Location::new(0.0, 150.1).unwrap(), date, None);
        for zman in Zman::ALL {
            assert!(
                matches!(
                    far.get(zman),
                    Err(ZmanimError::TimeZoneRequired | ZmanimError::NotApplicable)
                ),
                "{date} {zman:?}"
            );
        }
        date = date.tomorrow().unwrap();
    }
}
