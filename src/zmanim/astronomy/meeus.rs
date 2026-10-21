//! The Jean Meeus higher-accuracy (VSOP87) solar calculator, ported from KosherJava's
//! `MeeusCalculator`.
//!
//! Computes the Earth's heliocentric position from the abridged VSOP87 series, derives
//! the sun's apparent geocentric longitude (FK5 frame correction, nutation in longitude,
//! aberration), apparent right ascension and declination, and the equation of time per
//! Meeus chapter 28. Positions are evaluated in Terrestrial Time, so an estimate of ΔT is
//! applied unless [`Meeus::apply_delta_t`] is `false`.

#[allow(unused_imports)]
use core_maths::*;
use jiff::civil::Date;

use super::{
    JULIAN_DAY_JAN_1_2000, JULIAN_DAYS_PER_CENTURY, SolarAlgorithm, SolarEvent, julian_day, sun_hour_angle,
    time_at_azimuth_iterative,
    vsop87::{
        self, asin_degrees, cos_degrees, delta_t_seconds, nutation_arcsec, sin_degrees, sun_mean_longitude, tan_degrees,
    },
};
use crate::zmanim::types::{error::ZmanimError, location::Location};

/// The Meeus algorithm.
pub(super) struct Meeus {
    /// Whether to apply ΔT (TT − UT) when evaluating the sun's position. KosherJava
    /// defaults this to `true`; `false` reproduces NOAA-style UT = TT behaviour.
    pub(super) apply_delta_t: bool,
}

impl SolarAlgorithm for Meeus {
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

        // Use the declination at local solar noon as the starting point, then iterate.
        let noon_min = self.noon_midnight_utc_minutes(date, longitude, SolarEvent::Noon);
        let tnoon = self.julian_centuries(julian_day + noon_min / 1440.0);
        let mut equation_of_time = equation_of_time(tnoon);
        let mut solar_declination = sun_declination(tnoon);
        let mut hour_angle = sun_hour_angle(latitude, solar_declination, adjusted_zenith, event)?;
        let mut delta = longitude - hour_angle.to_degrees();
        let mut time_utc = 720.0 + (4.0 * delta) - equation_of_time;

        // Two further passes including the fractional Julian day (one more than NOAA).
        for _ in 0..2 {
            let newt = self.julian_centuries(julian_day + time_utc / 1440.0);
            equation_of_time = self::equation_of_time(newt);
            solar_declination = sun_declination(newt);
            hour_angle = sun_hour_angle(latitude, solar_declination, adjusted_zenith, event)?;
            delta = longitude - hour_angle.to_degrees();
            time_utc = 720.0 + (4.0 * delta) - equation_of_time;
        }
        Ok(time_utc)
    }

    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        let julian_day = julian_day(date);
        // The seed includes the 720 / 1440 minute transit base so the equation of time is
        // sampled near the true transit (NOAA omits the base from its seed).
        let base = if event == SolarEvent::Noon { 720.0 } else { 1440.0 };
        let tnoon = self.julian_centuries(julian_day + longitude_west / 360.0);
        let mut equation_of_time = equation_of_time(tnoon);
        let mut sol_noon_utc = base + (longitude_west * 4.0) - equation_of_time;

        for _ in 0..3 {
            let newt = self.julian_centuries(julian_day + sol_noon_utc / 1440.0);
            equation_of_time = self::equation_of_time(newt);
            sol_noon_utc = base + (longitude_west * 4.0) - equation_of_time;
        }
        sol_noon_utc
    }

    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError> {
        time_at_azimuth_iterative(date, location, azimuth, 3, |julian_day| {
            let julian_centuries = self.julian_centuries(julian_day);
            (sun_declination(julian_centuries), equation_of_time(julian_centuries))
        })
    }
}

impl Meeus {
    /// Converts a UT Julian day to Julian centuries since J2000.0 in Terrestrial Time.
    fn julian_centuries(&self, julian_day_ut: f64) -> f64 {
        let mut jde = julian_day_ut;
        if self.apply_delta_t {
            jde += delta_t_seconds(julian_day_ut) / 86400.0;
        }
        (jde - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY
    }
}

/// Apparent declination of the sun in degrees.
fn sun_declination(julian_centuries: f64) -> f64 {
    let (lambda, beta) = sun_apparent_ecliptic_coordinates(julian_centuries);
    let epsilon = true_obliquity(julian_centuries);
    let sin_dec =
        sin_degrees(beta) * cos_degrees(epsilon) + cos_degrees(beta) * sin_degrees(epsilon) * sin_degrees(lambda);
    asin_degrees(sin_dec)
}

/// Apparent geocentric right ascension of the sun in degrees (0–360).
fn sun_right_ascension(julian_centuries: f64) -> f64 {
    let (lambda, beta) = sun_apparent_ecliptic_coordinates(julian_centuries);
    let epsilon = true_obliquity(julian_centuries);
    let y = sin_degrees(lambda) * cos_degrees(epsilon) - tan_degrees(beta) * sin_degrees(epsilon);
    let x = cos_degrees(lambda);
    let ra = y.atan2(x).to_degrees();
    (ra % 360.0 + 360.0) % 360.0
}

/// Apparent geocentric ecliptic `(longitude λ, latitude β)` of the sun in degrees.
fn sun_apparent_ecliptic_coordinates(julian_centuries: f64) -> (f64, f64) {
    let tau = julian_centuries / 10.0;
    let earth_l = vsop87::earth_longitude(tau).to_degrees();
    let earth_b = vsop87::earth_latitude(tau).to_degrees();
    let earth_r = vsop87::earth_radius(tau);

    // Geocentric position of the sun is diametrically opposite the heliocentric Earth.
    let mut theta = earth_l + 180.0;
    let mut beta = -earth_b;

    // FK5 frame correction (Meeus 25.9).
    let lambda_prime = theta - 1.397 * julian_centuries - 0.00031 * julian_centuries * julian_centuries;
    let delta_long_fk5 =
        -0.09033 + 0.03916 * (cos_degrees(lambda_prime) + sin_degrees(lambda_prime)) * tan_degrees(beta);
    let delta_lat_fk5 = 0.03916 * (cos_degrees(lambda_prime) - sin_degrees(lambda_prime));
    theta += delta_long_fk5 / 3600.0;
    beta += delta_lat_fk5 / 3600.0;

    // Nutation in longitude and aberration of light.
    let (delta_psi, _) = nutation_arcsec(julian_centuries);
    let aberration = -20.4898 / earth_r;
    let lambda = theta + delta_psi / 3600.0 + aberration / 3600.0;

    ((lambda % 360.0 + 360.0) % 360.0, beta)
}

/// True obliquity of the ecliptic ε = ε₀ + Δε in degrees.
fn true_obliquity(julian_centuries: f64) -> f64 {
    let seconds = 21.448 - julian_centuries * (46.8150 + julian_centuries * (0.00059 - julian_centuries * 0.001813));
    let epsilon0 = 23.0 + (26.0 + (seconds / 60.0)) / 60.0;
    let delta_epsilon = nutation_arcsec(julian_centuries).1 / 3600.0;
    epsilon0 + delta_epsilon
}

/// Equation of time in minutes (Meeus chapter 28), with the NOAA sign convention.
fn equation_of_time(julian_centuries: f64) -> f64 {
    let tau = julian_centuries / 10.0;
    let l0 = sun_mean_longitude(tau);
    let alpha = sun_right_ascension(julian_centuries);
    let (delta_psi, _) = nutation_arcsec(julian_centuries);
    let epsilon = true_obliquity(julian_centuries);
    let mut e = l0 - 0.0057183 - alpha + (delta_psi / 3600.0) * cos_degrees(epsilon);
    e = ((e + 180.0) % 360.0 + 360.0) % 360.0 - 180.0;
    e * 4.0
}
