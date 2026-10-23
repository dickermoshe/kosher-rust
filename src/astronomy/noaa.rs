//! The NOAA solar calculator, ported from KosherJava's `NOAACalculator`.
//!
//! Uses the low-accuracy Meeus formulas (three-term equation of center, truncated
//! equation of time) without a ΔT correction. This is KosherJava's default algorithm.

use core_maths::*;
use jiff::civil::Date;

use super::{
    SolarAlgorithm, SolarEvent, julian_centuries_from_julian_day, julian_day, sun_hour_angle, time_at_azimuth_iterative,
};
use crate::{Location, zmanim::ZmanimError};

/// The NOAA algorithm. Stateless.
pub(super) struct Noaa;

impl SolarAlgorithm for Noaa {
    fn rise_set_utc_minutes(
        &self,
        date: Date,
        location: &Location,
        adjusted_zenith: f64,
        event: SolarEvent,
    ) -> Result<f64, ZmanimError> {
        let latitude = location.latitude;
        let longitude = -location.longitude;
        let julian_day = julian_day(date);
        let noon_min = self.noon_midnight_utc_minutes(date, longitude, SolarEvent::Noon);
        let tnoon = julian_centuries_from_julian_day(julian_day + noon_min / 1440.0);
        let mut eq_time = equation_of_time(tnoon);
        let mut solar_declination = sun_declination(tnoon);
        let mut hour_angle = match sun_hour_angle(latitude, solar_declination, adjusted_zenith, event) {
            Ok(hour_angle) => hour_angle,
            Err(_) => {
                // Using the sun's declination at noon, the sun never reaches this zenith today. But the
                // declination changes during the day (by up to about 0.2° in 12 hours), so the sun may
                // still just reach it around midnight, when it is at its lowest. Before reporting that
                // there is no event, try again with the declination and equation of time at that solar
                // midnight: the one before sunrise, or the one after sunset.
                let midnight = noon_min + if event == SolarEvent::Sunrise { -720.0 } else { 720.0 };
                let tmidnight = julian_centuries_from_julian_day(julian_day + midnight / 1440.0);
                eq_time = equation_of_time(tmidnight);
                solar_declination = sun_declination(tmidnight);
                sun_hour_angle(latitude, solar_declination, adjusted_zenith, event)?
            }
        };
        let mut delta = longitude - hour_angle.to_degrees();
        let mut time_diff = 4.0 * delta;
        let mut time_utc = 720.0 + time_diff - eq_time;

        let newt = julian_centuries_from_julian_day(julian_day + time_utc / 1440.0);
        eq_time = equation_of_time(newt);
        solar_declination = sun_declination(newt);
        hour_angle = sun_hour_angle(latitude, solar_declination, adjusted_zenith, event)?;
        delta = longitude - hour_angle.to_degrees();
        time_diff = 4.0 * delta;
        time_utc = 720.0 + time_diff - eq_time;
        Ok(time_utc)
    }

    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        let julian_day = julian_day(date);
        let tnoon = julian_centuries_from_julian_day(julian_day + longitude_west / 360.0);
        let mut eq_time = equation_of_time(tnoon);
        let mut sol_noon_utc = longitude_west * 4.0 - eq_time;

        for _ in 0..2 {
            let newt = julian_centuries_from_julian_day(julian_day + sol_noon_utc / 1440.0);
            eq_time = equation_of_time(newt);
            sol_noon_utc = (if event == SolarEvent::Noon { 720.0 } else { 1440.0 }) + longitude_west * 4.0 - eq_time;
        }

        sol_noon_utc
    }

    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError> {
        time_at_azimuth_iterative(date, location, azimuth, 3, |julian_day| {
            let julian_centuries = julian_centuries_from_julian_day(julian_day);
            (sun_declination(julian_centuries), equation_of_time(julian_centuries))
        })
    }
}

fn sun_geometric_mean_longitude(julian_centuries: f64) -> f64 {
    let longitude = 280.46646 + julian_centuries * (36000.76983 + 0.0003032 * julian_centuries);
    (longitude % 360.0 + 360.0) % 360.0
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
    let sinm = CoreFloat::sin(mrad);
    let sin2m = CoreFloat::sin(mrad + mrad);
    let sin3m = CoreFloat::sin(mrad + mrad + mrad);
    sinm * (1.914602 - julian_centuries * (0.004817 + 0.000014 * julian_centuries))
        + sin2m * (0.019993 - 0.000101 * julian_centuries)
        + sin3m * 0.000289
}

fn sun_true_longitude(julian_centuries: f64) -> f64 {
    sun_geometric_mean_longitude(julian_centuries) + sun_equation_of_center(julian_centuries)
}

fn sun_apparent_longitude(julian_centuries: f64) -> f64 {
    let omega = 125.04 - 1934.136 * julian_centuries;
    sun_true_longitude(julian_centuries) - 0.00569 - 0.00478 * CoreFloat::sin(omega.to_radians())
}

fn mean_obliquity_of_ecliptic(julian_centuries: f64) -> f64 {
    let seconds = 21.448 - julian_centuries * (46.8150 + julian_centuries * (0.00059 - julian_centuries * 0.001813));
    23.0 + (26.0 + seconds / 60.0) / 60.0
}

fn obliquity_correction(julian_centuries: f64) -> f64 {
    let omega = 125.04 - 1934.136 * julian_centuries;
    mean_obliquity_of_ecliptic(julian_centuries) + 0.00256 * CoreFloat::cos(omega.to_radians())
}

fn sun_declination(julian_centuries: f64) -> f64 {
    let obliquity_correction = obliquity_correction(julian_centuries);
    let lambda = sun_apparent_longitude(julian_centuries);
    let sint = CoreFloat::sin(obliquity_correction.to_radians()) * CoreFloat::sin(lambda.to_radians());
    CoreFloat::asin(sint).to_degrees()
}

fn equation_of_time(julian_centuries: f64) -> f64 {
    let epsilon = obliquity_correction(julian_centuries);
    let geom_mean_long_sun = sun_geometric_mean_longitude(julian_centuries);
    let eccentricity_earth_orbit = earth_orbit_eccentricity(julian_centuries);
    let geom_mean_anomaly_sun = sun_geometric_mean_anomaly(julian_centuries);
    let mut y = CoreFloat::tan(epsilon.to_radians() / 2.0);
    y *= y;

    let sin2l0 = CoreFloat::sin(2.0 * geom_mean_long_sun.to_radians());
    let sinm = CoreFloat::sin(geom_mean_anomaly_sun.to_radians());
    let cos2l0 = CoreFloat::cos(2.0 * geom_mean_long_sun.to_radians());
    let sin4l0 = CoreFloat::sin(4.0 * geom_mean_long_sun.to_radians());
    let sin2m = CoreFloat::sin(2.0 * geom_mean_anomaly_sun.to_radians());
    let equation_of_time = y * sin2l0 - 2.0 * eccentricity_earth_orbit * sinm
        + 4.0 * eccentricity_earth_orbit * y * sinm * cos2l0
        - 0.5 * y * y * sin4l0
        - 1.25 * eccentricity_earth_orbit * eccentricity_earth_orbit * sin2m;
    equation_of_time.to_degrees() * 4.0
}
