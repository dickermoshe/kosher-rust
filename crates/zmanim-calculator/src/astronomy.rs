//! Internal KosherJava NOAA astronomy calculations.

use crate::types::{error::ZmanimError, location::Location};
#[allow(unused_imports)]
use core_maths::*;
use jiff::{civil::Date, tz::TimeZone, SignedDuration, Timestamp};

const GEOMETRIC_ZENITH: f64 = 90.0;
const REFRACTION: f64 = 34.0 / 60.0;
const SOLAR_RADIUS: f64 = 16.0 / 60.0;
const EARTH_RADIUS_KM: f64 = 6356.9;
const JULIAN_DAY_JAN_1_2000: f64 = 2_451_545.0;
const JULIAN_DAYS_PER_CENTURY: f64 = 36_525.0;
const HOUR_NANOS: f64 = 3_600_000.0 * 1_000_000.0;

#[derive(Clone, Copy)]
pub(crate) enum SolarEvent {
    Sunrise,
    Sunset,
    Noon,
    Midnight,
}

pub(crate) fn sunrise(
    date: Date,
    location: &Location,
    adjust_for_elevation: bool,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        date,
        location,
        GEOMETRIC_ZENITH,
        adjust_for_elevation,
        SolarEvent::Sunrise,
    )
}

pub(crate) fn sunset(
    date: Date,
    location: &Location,
    adjust_for_elevation: bool,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        date,
        location,
        GEOMETRIC_ZENITH,
        adjust_for_elevation,
        SolarEvent::Sunset,
    )
}

pub(crate) fn sunrise_offset_by_degrees(
    date: Date,
    location: &Location,
    degrees: f64,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        date,
        location,
        GEOMETRIC_ZENITH + degrees,
        false,
        SolarEvent::Sunrise,
    )
}

pub(crate) fn sunset_offset_by_degrees(
    date: Date,
    location: &Location,
    degrees: f64,
) -> Result<Timestamp, ZmanimError> {
    rise_set(
        date,
        location,
        GEOMETRIC_ZENITH + degrees,
        false,
        SolarEvent::Sunset,
    )
}

pub(crate) fn solar_noon(date: Date, location: &Location) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location)?;
    let noon = solar_noon_midnight_utc(
        julian_day(adjusted_date),
        -location.longitude,
        SolarEvent::Noon,
    );
    instant_from_utc_hours(
        normalize_utc_hours(noon / 60.0),
        adjusted_date,
        location,
        SolarEvent::Noon,
    )
}

pub(crate) fn solar_midnight(date: Date, location: &Location) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location)?;
    let midnight = solar_noon_midnight_utc(
        julian_day(adjusted_date),
        -location.longitude,
        SolarEvent::Midnight,
    );
    instant_from_utc_hours(
        normalize_utc_hours(midnight / 60.0),
        adjusted_date,
        location,
        SolarEvent::Midnight,
    )
}

fn rise_set(
    date: Date,
    location: &Location,
    zenith: f64,
    adjust_for_elevation: bool,
    event: SolarEvent,
) -> Result<Timestamp, ZmanimError> {
    let adjusted_date = adjusted_local_date(date, location)?;
    let elevation = if adjust_for_elevation {
        location.elevation
    } else {
        0.0
    };
    let adjusted_zenith = adjust_zenith(zenith, elevation);
    let utc_minutes = sun_rise_set_utc(
        adjusted_date,
        location.latitude,
        -location.longitude,
        adjusted_zenith,
        event,
    )?;
    instant_from_utc_hours(
        normalize_utc_hours(utc_minutes / 60.0),
        adjusted_date,
        location,
        event,
    )
}

fn adjusted_local_date(date: Date, location: &Location) -> Result<Date, ZmanimError> {
    let Some(timezone) = &location.timezone else {
        return Ok(date);
    };

    let midnight = timezone
        .to_ambiguous_timestamp(date.at(0, 0, 0, 0))
        .earlier()
        .map_err(|_| ZmanimError::TimeConversionError)?;
    let offset = midnight.to_zoned(timezone.clone()).offset().seconds();
    let local_hours_offset = (location.longitude * 240.0 - f64::from(offset)) / 3600.0;

    if local_hours_offset >= 20.0 {
        add_days(date, 1)
    } else if local_hours_offset <= -20.0 {
        add_days(date, -1)
    } else {
        Ok(date)
    }
}

fn instant_from_utc_hours(
    time: f64,
    mut date: Date,
    location: &Location,
    event: SolarEvent,
) -> Result<Timestamp, ZmanimError> {
    if time.is_nan() {
        return Err(ZmanimError::CalculationError);
    }

    let local_time_hours = location.longitude / 15.0 + time;
    match event {
        SolarEvent::Sunrise if local_time_hours > 18.0 => date = add_days(date, -1)?,
        SolarEvent::Sunset if local_time_hours < 6.0 => date = add_days(date, 1)?,
        SolarEvent::Midnight if local_time_hours < 12.0 => date = add_days(date, 1)?,
        SolarEvent::Noon if local_time_hours < 0.0 => date = add_days(date, 1)?,
        SolarEvent::Noon if local_time_hours > 24.0 => date = add_days(date, -1)?,
        _ => {}
    }

    let nanos = (time * HOUR_NANOS).round() as i64;
    let date_time = date
        .at(0, 0, 0, 0)
        .checked_add(SignedDuration::from_nanos(nanos))
        .map_err(|_| ZmanimError::TimeConversionError)?;
    date_time
        .to_zoned(TimeZone::UTC)
        .map_err(|_| ZmanimError::TimeConversionError)
        .map(|zdt| zdt.timestamp())
}

fn add_days(date: Date, days: i64) -> Result<Date, ZmanimError> {
    date.checked_add(SignedDuration::from_hours(24 * days))
        .map_err(|_| ZmanimError::TimeConversionError)
}

fn normalize_utc_hours(time: f64) -> f64 {
    if time > 0.0 {
        time % 24.0
    } else {
        time % 24.0 + 24.0
    }
}

fn adjust_zenith(zenith: f64, elevation: f64) -> f64 {
    if zenith == GEOMETRIC_ZENITH {
        zenith + SOLAR_RADIUS + REFRACTION + elevation_adjustment(elevation)
    } else {
        zenith
    }
}

fn elevation_adjustment(elevation_m: f64) -> f64 {
    (EARTH_RADIUS_KM / (EARTH_RADIUS_KM + elevation_m / 1000.0))
        .acos()
        .to_degrees()
}

fn julian_day(date: Date) -> f64 {
    let mut year = i32::from(date.year());
    let mut month = i32::from(date.month());
    let day = i32::from(date.day());

    if month <= 2 {
        year -= 1;
        month += 12;
    }

    let a = year / 100;
    let b = 2 - a + a / 4;

    (365.25 * f64::from(year + 4716)).floor()
        + (30.6001 * f64::from(month + 1)).floor()
        + f64::from(day)
        + f64::from(b)
        - 1524.5
}

fn julian_centuries_from_julian_day(julian_day: f64) -> f64 {
    (julian_day - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY
}

fn sun_geometric_mean_longitude(julian_centuries: f64) -> f64 {
    let longitude = 280.46646 + julian_centuries * (36000.76983 + 0.0003032 * julian_centuries);
    if longitude > 0.0 {
        longitude % 360.0
    } else {
        longitude % 360.0 + 360.0
    }
}

fn sun_geometric_mean_anomaly(julian_centuries: f64) -> f64 {
    357.52911 + julian_centuries * (35999.05029 - 0.0001537 * julian_centuries)
}

fn earth_orbit_eccentricity(julian_centuries: f64) -> f64 {
    0.016708634 - julian_centuries * (0.000042037 + 0.0000001267 * julian_centuries)
}

fn sun_equation_of_center(julian_centuries: f64) -> f64 {
    let m = sun_geometric_mean_anomaly(julian_centuries);
    let mrad = m.to_radians();
    let sinm = mrad.sin();
    let sin2m = (mrad + mrad).sin();
    let sin3m = (mrad + mrad + mrad).sin();
    sinm * (1.914602 - julian_centuries * (0.004817 + 0.000014 * julian_centuries))
        + sin2m * (0.019993 - 0.000101 * julian_centuries)
        + sin3m * 0.000289
}

fn sun_true_longitude(julian_centuries: f64) -> f64 {
    sun_geometric_mean_longitude(julian_centuries) + sun_equation_of_center(julian_centuries)
}

fn sun_apparent_longitude(julian_centuries: f64) -> f64 {
    let omega = 125.04 - 1934.136 * julian_centuries;
    sun_true_longitude(julian_centuries) - 0.00569 - 0.00478 * omega.to_radians().sin()
}

fn mean_obliquity_of_ecliptic(julian_centuries: f64) -> f64 {
    let seconds = 21.448
        - julian_centuries * (46.8150 + julian_centuries * (0.00059 - julian_centuries * 0.001813));
    23.0 + (26.0 + seconds / 60.0) / 60.0
}

fn obliquity_correction(julian_centuries: f64) -> f64 {
    let omega = 125.04 - 1934.136 * julian_centuries;
    mean_obliquity_of_ecliptic(julian_centuries) + 0.00256 * omega.to_radians().cos()
}

fn sun_declination(julian_centuries: f64) -> f64 {
    let obliquity_correction = obliquity_correction(julian_centuries);
    let lambda = sun_apparent_longitude(julian_centuries);
    let sint = obliquity_correction.to_radians().sin() * lambda.to_radians().sin();
    sint.asin().to_degrees()
}

fn equation_of_time(julian_centuries: f64) -> f64 {
    let epsilon = obliquity_correction(julian_centuries);
    let geom_mean_long_sun = sun_geometric_mean_longitude(julian_centuries);
    let eccentricity_earth_orbit = earth_orbit_eccentricity(julian_centuries);
    let geom_mean_anomaly_sun = sun_geometric_mean_anomaly(julian_centuries);
    let mut y = (epsilon.to_radians() / 2.0).tan();
    y *= y;

    let sin2l0 = (2.0 * geom_mean_long_sun.to_radians()).sin();
    let sinm = geom_mean_anomaly_sun.to_radians().sin();
    let cos2l0 = (2.0 * geom_mean_long_sun.to_radians()).cos();
    let sin4l0 = (4.0 * geom_mean_long_sun.to_radians()).sin();
    let sin2m = (2.0 * geom_mean_anomaly_sun.to_radians()).sin();
    let equation_of_time = y * sin2l0 - 2.0 * eccentricity_earth_orbit * sinm
        + 4.0 * eccentricity_earth_orbit * y * sinm * cos2l0
        - 0.5 * y * y * sin4l0
        - 1.25 * eccentricity_earth_orbit * eccentricity_earth_orbit * sin2m;
    equation_of_time.to_degrees() * 4.0
}

fn sun_hour_angle(
    latitude: f64,
    solar_declination: f64,
    zenith: f64,
    event: SolarEvent,
) -> Result<f64, ZmanimError> {
    let lat_rad = latitude.to_radians();
    let sd_rad = solar_declination.to_radians();
    let cos_hour_angle =
        zenith.to_radians().cos() / (lat_rad.cos() * sd_rad.cos()) - lat_rad.tan() * sd_rad.tan();

    if cos_hour_angle > 1.0 {
        return Err(ZmanimError::AllNight);
    }
    if cos_hour_angle < -1.0 {
        return Err(ZmanimError::AllDay);
    }

    let mut hour_angle = cos_hour_angle.acos();
    if matches!(event, SolarEvent::Sunset) {
        hour_angle = -hour_angle;
    }
    Ok(hour_angle)
}

fn solar_noon_midnight_utc(mut julian_day: f64, longitude: f64, event: SolarEvent) -> f64 {
    julian_day = if matches!(event, SolarEvent::Noon) {
        julian_day
    } else {
        julian_day + 0.5
    };
    let tnoon = julian_centuries_from_julian_day(julian_day + longitude / 360.0);
    let mut eq_time = equation_of_time(tnoon);
    let mut sol_noon_utc = longitude * 4.0 - eq_time;

    for _ in 0..2 {
        let newt = julian_centuries_from_julian_day(julian_day + sol_noon_utc / 1440.0);
        eq_time = equation_of_time(newt);
        sol_noon_utc = if matches!(event, SolarEvent::Noon) {
            720.0
        } else {
            1440.0
        } + longitude * 4.0
            - eq_time;
    }

    (if matches!(event, SolarEvent::Noon) {
        720.0
    } else {
        1440.0
    }) + longitude * 4.0
        - eq_time
}

fn sun_rise_set_utc(
    local_date: Date,
    latitude: f64,
    longitude: f64,
    zenith: f64,
    event: SolarEvent,
) -> Result<f64, ZmanimError> {
    let julian_day = julian_day(local_date);
    let noon_min = solar_noon_midnight_utc(julian_day, longitude, SolarEvent::Noon);
    let tnoon = julian_centuries_from_julian_day(julian_day + noon_min / 1440.0);
    let mut eq_time = equation_of_time(tnoon);
    let mut solar_declination = sun_declination(tnoon);
    let mut hour_angle = sun_hour_angle(latitude, solar_declination, zenith, event)?;
    let mut delta = longitude - hour_angle.to_degrees();
    let mut time_diff = 4.0 * delta;
    let mut time_utc = 720.0 + time_diff - eq_time;

    let newt = julian_centuries_from_julian_day(julian_day + time_utc / 1440.0);
    eq_time = equation_of_time(newt);
    solar_declination = sun_declination(newt);
    hour_angle = sun_hour_angle(latitude, solar_declination, zenith, event)?;
    delta = longitude - hour_angle.to_degrees();
    time_diff = 4.0 * delta;
    time_utc = 720.0 + time_diff - eq_time;
    Ok(time_utc)
}
