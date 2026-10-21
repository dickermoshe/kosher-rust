//! The NREL Solar Position Algorithm (SPA) calculator, ported from KosherJava's
//! `SPACalculator` (Reda and Andreas, NREL/TP-560-34302).
//!
//! Computes the sun's apparent geocentric right ascension and declination from the
//! abridged VSOP87 series with nutation and aberration, then performs the topocentric
//! reduction for observer parallax. Sunrise and sunset are solved by a secant iteration
//! on the topocentric *true* (un-refracted) zenith, so refraction and the solar radius
//! come only from the shared zenith adjustment in the parent module, exactly as for the
//! other calculators. Solar noon and midnight are solved as true meridian crossings.
//!
//! KosherJava's pressure and temperature settings only feed its instantaneous
//! elevation / azimuth queries, which this crate does not expose, so they are not ported.

#[allow(unused_imports)]
use core_maths::*;
use jiff::civil::Date;

use super::{
    JULIAN_DAY_JAN_1_2000, JULIAN_DAYS_PER_CENTURY, SolarAlgorithm, SolarEvent, hour_angle_from_cos, julian_day,
    time_at_azimuth_iterative,
    vsop87::{
        self, asin_degrees, cos_degrees, delta_t_seconds, nutation_arcsec, sin_degrees, sun_mean_longitude, tan_degrees,
    },
};
use crate::{Location, zmanim::ZmanimError};

/// The SPA algorithm.
pub(super) struct Spa {
    /// Whether to apply ΔT (TT − UT). KosherJava defaults this to `true`.
    pub(super) apply_delta_t: bool,
}

/// Geocentric apparent coordinates of the sun and the apparent sidereal time.
struct SolarCoords {
    /// Apparent right ascension in degrees (0–360).
    alpha: f64,
    /// Apparent declination in degrees.
    delta: f64,
    /// True obliquity of the ecliptic in degrees.
    epsilon: f64,
    /// Apparent Greenwich sidereal time in degrees (0–360).
    nu: f64,
    /// Earth radius vector in AU.
    radius: f64,
}

impl SolarAlgorithm for Spa {
    fn rise_set_utc_minutes(
        &self,
        date: Date,
        location: &Location,
        adjusted_zenith: f64,
        event: SolarEvent,
    ) -> Result<f64, ZmanimError> {
        let jd_day = julian_day(date);
        let lon_west = -location.longitude;
        let lat = location.latitude;
        // The observer's elevation feeds the parallax term regardless of the horizon dip.
        let elevation = location.elevation;

        // Closed-form first guess using the geocentric declination at local apparent noon.
        let noon_min = self.noon_midnight_utc_minutes(date, lon_west, SolarEvent::Noon);
        let noon_coords = self.solar_coords(jd_day + noon_min / 1440.0);
        let decl_noon = noon_coords.delta;
        let eot = self.equation_of_time(jd_day + noon_min / 1440.0);
        let cos_h0 = (cos_degrees(adjusted_zenith) - sin_degrees(lat) * sin_degrees(decl_noon))
            / (cos_degrees(lat) * cos_degrees(decl_noon));
        let h0 = hour_angle_from_cos(cos_h0, SolarEvent::Sunrise)?.to_degrees();
        let signed = if event == SolarEvent::Sunrise { h0 } else { -h0 };
        let guess = 720.0 + 4.0 * (lon_west - signed) - eot;

        // Secant refinement on f(t) = topocentricTrueZenith(t) - adjustedZenith.
        let mut t0 = guess;
        let mut t1 = guess + 0.5;
        let mut f0 =
            self.topocentric_true_zenith(jd_day + t0 / 1440.0, lat, location.longitude, elevation) - adjusted_zenith;
        let mut f1 =
            self.topocentric_true_zenith(jd_day + t1 / 1440.0, lat, location.longitude, elevation) - adjusted_zenith;
        for _ in 0..12 {
            // Mirrors Java's `Math.abs(f1) > 1e-9` loop guard, which also stops on NaN.
            if f1.is_nan() || f1.abs() <= 1e-9 {
                break;
            }
            let denom = f1 - f0;
            if denom == 0.0 {
                break;
            }
            let t2 = t1 - f1 * (t1 - t0) / denom;
            t0 = t1;
            f0 = f1;
            t1 = t2;
            f1 = self.topocentric_true_zenith(jd_day + t1 / 1440.0, lat, location.longitude, elevation)
                - adjusted_zenith;
        }
        if t1.is_nan() {
            return Err(ZmanimError::Internal);
        }
        Ok(t1)
    }

    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        // Rigorous meridian solve: find the UT at which the sun's apparent hour angle is 0
        // (transit / noon) or 180 (anti-transit / midnight).
        let julian_day = julian_day(date);
        let lon_east = -longitude_west;
        let target_hour_angle = if event == SolarEvent::Noon { 0.0 } else { 180.0 };
        let mut day_fraction = (if event == SolarEvent::Noon { 0.5 } else { 1.0 }) - lon_east / 360.0;
        for _ in 0..3 {
            let sc = self.solar_coords(julian_day + day_fraction);
            let hour_angle = ((sc.nu + lon_east - sc.alpha - target_hour_angle) % 360.0 + 540.0) % 360.0 - 180.0;
            day_fraction -= hour_angle / 360.0;
        }
        day_fraction * 1440.0
    }

    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError> {
        time_at_azimuth_iterative(date, location, azimuth, 4, |julian_day| {
            (self.solar_coords(julian_day).delta, self.equation_of_time(julian_day))
        })
    }
}

impl Spa {
    /// ΔT in seconds for a UT Julian day, or zero when disabled.
    fn delta_t(&self, julian_day_ut: f64) -> f64 {
        if self.apply_delta_t {
            delta_t_seconds(julian_day_ut)
        } else {
            0.0
        }
    }

    /// Geocentric apparent coordinates of the sun and apparent sidereal time for a UT Julian day.
    fn solar_coords(&self, julian_day_ut: f64) -> SolarCoords {
        let delta_t_seconds = self.delta_t(julian_day_ut);
        let jde = julian_day_ut + delta_t_seconds / 86400.0;
        let jce = (jde - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY;
        let jc = (julian_day_ut - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY;
        let jme = jce / 10.0;

        let earth_l = vsop87::earth_longitude(jme, false).to_degrees();
        let earth_b = vsop87::earth_latitude(jme, false).to_degrees();
        let radius = vsop87::earth_radius(jme, false);

        let theta = (earth_l + 180.0) % 360.0;
        let beta = -earth_b;

        let (delta_psi, delta_epsilon) = nutation_degrees(jce);
        let epsilon = mean_obliquity(jme) + delta_epsilon;
        let aberration = -20.4898 / (3600.0 * radius);
        let lambda = theta + delta_psi + aberration;

        // Apparent sidereal time at Greenwich.
        let mut nu0 = 280.46061837 + 360.98564736629 * (julian_day_ut - JULIAN_DAY_JAN_1_2000) + 0.000387933 * jc * jc
            - jc * jc * jc / 38710000.0;
        nu0 = (nu0 % 360.0 + 360.0) % 360.0;
        let nu = nu0 + delta_psi * cos_degrees(epsilon);

        let mut alpha = (sin_degrees(lambda) * cos_degrees(epsilon) - tan_degrees(beta) * sin_degrees(epsilon))
            .atan2(cos_degrees(lambda))
            .to_degrees();
        alpha = (alpha % 360.0 + 360.0) % 360.0;
        let delta = asin_degrees(
            sin_degrees(beta) * cos_degrees(epsilon) + cos_degrees(beta) * sin_degrees(epsilon) * sin_degrees(lambda),
        );

        SolarCoords {
            alpha,
            delta,
            epsilon,
            nu,
            radius,
        }
    }

    /// Topocentric true (un-refracted) elevation of the sun's center in degrees, after the
    /// SPA parallax reduction for the observer's latitude, longitude (positive east) and
    /// elevation in meters.
    fn topocentric_true_elevation(
        &self,
        julian_day_ut: f64,
        latitude: f64,
        longitude: f64,
        elevation_meters: f64,
    ) -> f64 {
        let sc = self.solar_coords(julian_day_ut);
        let (alpha, delta, nu, radius) = (sc.alpha, sc.delta, sc.nu, sc.radius);

        let mut h = (nu + longitude - alpha) % 360.0;
        h = (h + 360.0) % 360.0;

        // Equatorial horizontal parallax of the sun.
        let xi = 8.794 / (3600.0 * radius);
        let u = (0.99664719 * tan_degrees(latitude)).atan();
        let x = u.cos() + (elevation_meters / 6378140.0) * cos_degrees(latitude);
        let y = 0.99664719 * u.sin() + (elevation_meters / 6378140.0) * sin_degrees(latitude);

        let delta_alpha = (-x * sin_degrees(xi) * sin_degrees(h))
            .atan2(cos_degrees(delta) - x * sin_degrees(xi) * cos_degrees(h))
            .to_degrees();
        let delta_prime = ((sin_degrees(delta) - y * sin_degrees(xi)) * cos_degrees(delta_alpha))
            .atan2(cos_degrees(delta) - x * sin_degrees(xi) * cos_degrees(h))
            .to_degrees();
        let h_prime = h - delta_alpha;

        asin_degrees(
            sin_degrees(latitude) * sin_degrees(delta_prime)
                + cos_degrees(latitude) * cos_degrees(delta_prime) * cos_degrees(h_prime),
        )
    }

    /// Topocentric true zenith of the sun's center in degrees.
    fn topocentric_true_zenith(&self, julian_day_ut: f64, latitude: f64, longitude: f64, elevation_meters: f64) -> f64 {
        90.0 - self.topocentric_true_elevation(julian_day_ut, latitude, longitude, elevation_meters)
    }

    /// Equation of time in minutes (Meeus chapter 28) from the geocentric apparent right
    /// ascension, with the NOAA sign convention.
    fn equation_of_time(&self, julian_day_ut: f64) -> f64 {
        let sc = self.solar_coords(julian_day_ut);
        let (alpha, epsilon) = (sc.alpha, sc.epsilon);
        let delta_t_seconds = self.delta_t(julian_day_ut);
        let jme =
            ((julian_day_ut + delta_t_seconds / 86400.0) - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY / 10.0;
        let l0 = sun_mean_longitude(jme);
        let delta_psi = nutation_degrees(
            ((julian_day_ut + delta_t_seconds / 86400.0) - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY,
        )
        .0;
        let mut e = l0 - 0.0057183 - alpha + delta_psi * cos_degrees(epsilon);
        e = ((e + 180.0) % 360.0 + 360.0) % 360.0 - 180.0;
        e * 4.0
    }
}

/// Mean obliquity of the ecliptic in degrees (SPA polynomial in U = JME / 10).
fn mean_obliquity(jme: f64) -> f64 {
    let u = jme / 10.0;
    let seconds = 84381.448 - 4680.93 * u - 1.55 * u * u + 1999.25 * u.powf(3.0)
        - 51.38 * u.powf(4.0)
        - 249.67 * u.powf(5.0)
        - 39.05 * u.powf(6.0)
        + 7.12 * u.powf(7.0)
        + 27.87 * u.powf(8.0)
        + 5.79 * u.powf(9.0)
        + 2.45 * u.powf(10.0);
    seconds / 3600.0
}

/// Principal nutation terms `(Δψ, Δε)` in degrees.
fn nutation_degrees(jce: f64) -> (f64, f64) {
    let (delta_psi_arcsec, delta_eps_arcsec) = nutation_arcsec(jce);
    (delta_psi_arcsec / 3600.0, delta_eps_arcsec / 3600.0)
}
