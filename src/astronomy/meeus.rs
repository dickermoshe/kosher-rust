//! The Jean Meeus higher-accuracy (VSOP87) solar calculator, ported from KosherJava's
//! `MeeusCalculator`.
//!
//! Computes the Earth's heliocentric position from the abridged VSOP87 series, derives
//! the sun's apparent geocentric longitude (FK5 frame correction, nutation in longitude,
//! aberration), apparent right ascension and declination, and the equation of time per
//! Meeus chapter 28. Positions are evaluated in Terrestrial Time, so an estimate of ΔT is
//! applied unless [`Meeus::apply_delta_t`] is `false`.
//!
//! For MyZmanim the same calculator can instead take the sun's longitude from the Astronomical
//! Almanac's low-precision formula, as [`Meeus::use_almanac_sun`] describes.

use core_maths::*;
use jiff::civil::Date;

use super::{
    JULIAN_DAY_JAN_1_2000, JULIAN_DAYS_PER_CENTURY, SolarAlgorithm, SolarEvent, julian_centuries_from_julian_day,
    julian_day, position_from_declination, sun_hour_angle, time_at_azimuth_iterative,
    vsop87::{
        self, asin_degrees, cos_degrees, delta_t_seconds, nutation_arcsec, sin_degrees, sun_mean_longitude, tan_degrees,
    },
};
use crate::{
    Location,
    zmanim::{SolarPosition, ZmanimError},
};

/// The Meeus algorithm.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Meeus {
    /// Whether to apply ΔT (TT − UT) when evaluating the sun's position. KosherJava
    /// defaults this to `true`; `false` reproduces NOAA-style UT = TT behaviour.
    pub(crate) apply_delta_t: bool,
    /// Whether to apply nutation (the 18.6-year wobble of the Earth's axis). With it off,
    /// positions are referred to the mean equinox and obliquity of date. Default `true`.
    pub(crate) apply_nutation: bool,
    /// Whether the sun's longitude comes from the Astronomical Almanac's low-precision formula
    /// instead of VSOP87. MyZmanim calculates the sun this way. The equation of time is then the
    /// Almanac's, and the declination comes from the Almanac's longitude and the obliquity of the
    /// ecliptic of date. The Almanac counts its days in Universal Time, so ΔT plays no part. From
    /// 1950 to 2050 its equation of time differs from the VSOP87 value by up to 2.8 s. Default
    /// `false`.
    pub(crate) use_almanac_sun: bool,
    /// Whether sunrise and sunset are refined until the time stops changing. Otherwise they get
    /// two refining passes, as in KosherJava. Two passes are almost always enough. On the nights
    /// the sun only just reaches the zenith, it moves so little that the time is still changing
    /// after them. Default `false`.
    pub(crate) iterate_to_convergence: bool,
}

impl Meeus {
    /// KosherJava's stock `MeeusCalculator`: VSOP87 with nutation, refined in two passes.
    pub(crate) const fn standard(apply_delta_t: bool) -> Self {
        Self {
            apply_delta_t,
            apply_nutation: true,
            use_almanac_sun: false,
            iterate_to_convergence: false,
        }
    }
}

/// The most refining passes [`Meeus::iterate_to_convergence`] makes. In a year of MyZmanim's times
/// at 746 places, no time needed more than 16.
const MAX_PASSES: usize = 50;

/// The change in a sunrise or sunset, in minutes, below which [`Meeus::iterate_to_convergence`]
/// stops refining it. It is 6 microseconds.
const CONVERGED_MINUTES: f64 = 1e-7;

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
        let noon = julian_day + noon_min / 1440.0;
        let mut equation_of_time = self.equation_of_time(noon);
        let mut solar_declination = self.declination(noon);
        let mut hour_angle = match sun_hour_angle(latitude, solar_declination, adjusted_zenith, event) {
            Ok(hour_angle) => hour_angle,
            Err(_) => {
                // Using the sun's declination at noon, the sun never reaches this zenith today. But the
                // declination changes during the day (by up to about 0.2° in 12 hours), so the sun may
                // still just reach it around midnight, when it is at its lowest. Before reporting that
                // there is no event, try again with the declination and equation of time at that solar
                // midnight: the one before sunrise, or the one after sunset.
                let midnight = noon_min + if event == SolarEvent::Sunrise { -720.0 } else { 720.0 };
                let midnight_day = julian_day + midnight / 1440.0;
                equation_of_time = self.equation_of_time(midnight_day);
                solar_declination = self.declination(midnight_day);
                sun_hour_angle(latitude, solar_declination, adjusted_zenith, event)?
            }
        };
        let mut delta = longitude - hour_angle.to_degrees();
        let mut time_utc = 720.0 + (4.0 * delta) - equation_of_time;

        // Each pass takes the sun's position at the time the previous pass found. KosherJava makes
        // two passes, one more than NOAA. When iterating to convergence, the passes stop once the
        // time stops changing.
        let passes = if self.iterate_to_convergence { MAX_PASSES } else { 2 };
        for _ in 0..passes {
            let event_time = julian_day + time_utc / 1440.0;
            equation_of_time = self.equation_of_time(event_time);
            solar_declination = self.declination(event_time);
            hour_angle = sun_hour_angle(latitude, solar_declination, adjusted_zenith, event)?;
            delta = longitude - hour_angle.to_degrees();
            let previous = time_utc;
            time_utc = 720.0 + (4.0 * delta) - equation_of_time;
            if self.iterate_to_convergence && (time_utc - previous).abs() < CONVERGED_MINUTES {
                break;
            }
        }
        Ok(time_utc)
    }

    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        let julian_day = julian_day(date);
        // The seed includes the 720 / 1440 minute transit base so the equation of time is
        // sampled near the true transit (NOAA omits the base from its seed).
        let base = if event == SolarEvent::Noon { 720.0 } else { 1440.0 };
        let mut equation_of_time = self.equation_of_time(julian_day + longitude_west / 360.0);
        let mut sol_noon_utc = base + (longitude_west * 4.0) - equation_of_time;

        for _ in 0..3 {
            equation_of_time = self.equation_of_time(julian_day + sol_noon_utc / 1440.0);
            sol_noon_utc = base + (longitude_west * 4.0) - equation_of_time;
        }
        sol_noon_utc
    }

    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError> {
        time_at_azimuth_iterative(date, location, azimuth, 3, |julian_day| {
            (self.declination(julian_day), self.equation_of_time(julian_day))
        })
    }

    fn solar_position(&self, date: Date, fraction_of_day: f64, location: &Location) -> SolarPosition {
        let julian_day = julian_day(date) + fraction_of_day;
        position_from_declination(
            fraction_of_day,
            location,
            self.declination(julian_day),
            self.equation_of_time(julian_day),
        )
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

impl Meeus {
    /// The sun's declination in degrees at the UT Julian day `julian_day_ut`. It comes from the
    /// Almanac's formula when [`use_almanac_sun`](Self::use_almanac_sun) is set, and otherwise from
    /// VSOP87.
    fn declination(&self, julian_day_ut: f64) -> f64 {
        if self.use_almanac_sun {
            let julian_centuries = julian_centuries_from_julian_day(julian_day_ut);
            return almanac_declination(julian_centuries, self.true_obliquity(julian_centuries));
        }
        self.sun_declination(self.julian_centuries(julian_day_ut))
    }

    /// Apparent declination of the sun in degrees.
    fn sun_declination(&self, julian_centuries: f64) -> f64 {
        let (lambda, beta) = self.sun_apparent_ecliptic_coordinates(julian_centuries);
        let epsilon = self.true_obliquity(julian_centuries);
        let sin_dec =
            sin_degrees(beta) * cos_degrees(epsilon) + cos_degrees(beta) * sin_degrees(epsilon) * sin_degrees(lambda);
        asin_degrees(sin_dec)
    }

    /// Apparent geocentric right ascension of the sun in degrees (0–360).
    fn sun_right_ascension(&self, julian_centuries: f64) -> f64 {
        let (lambda, beta) = self.sun_apparent_ecliptic_coordinates(julian_centuries);
        let epsilon = self.true_obliquity(julian_centuries);
        let y = sin_degrees(lambda) * cos_degrees(epsilon) - tan_degrees(beta) * sin_degrees(epsilon);
        let x = cos_degrees(lambda);
        let ra = CoreFloat::atan2(y, x).to_degrees();
        (ra % 360.0 + 360.0) % 360.0
    }

    /// Apparent geocentric ecliptic `(longitude λ, latitude β)` of the sun in degrees.
    fn sun_apparent_ecliptic_coordinates(&self, julian_centuries: f64) -> (f64, f64) {
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
        let (delta_psi, _) = self.nutation_arcsec(julian_centuries);
        let aberration = -20.4898 / earth_r;
        let lambda = theta + delta_psi / 3600.0 + aberration / 3600.0;

        ((lambda % 360.0 + 360.0) % 360.0, beta)
    }

    /// True obliquity of the ecliptic ε = ε₀ + Δε in degrees.
    fn true_obliquity(&self, julian_centuries: f64) -> f64 {
        let seconds =
            21.448 - julian_centuries * (46.8150 + julian_centuries * (0.00059 - julian_centuries * 0.001813));
        let epsilon0 = 23.0 + (26.0 + (seconds / 60.0)) / 60.0;
        let delta_epsilon = self.nutation_arcsec(julian_centuries).1 / 3600.0;
        epsilon0 + delta_epsilon
    }

    /// Nutation `(Δψ, Δε)` in arc-seconds, or zero when nutation is switched off.
    fn nutation_arcsec(&self, julian_centuries: f64) -> (f64, f64) {
        if self.apply_nutation {
            nutation_arcsec(julian_centuries)
        } else {
            (0.0, 0.0)
        }
    }

    /// Equation of time in minutes (Meeus chapter 28), with the NOAA sign convention, at the UT
    /// Julian day `julian_day_ut`.
    ///
    /// The Almanac's low-precision formula counts its days in Universal Time, as published, so it
    /// is given the day without ΔT. The VSOP87 equation of time adds ΔT when
    /// [`apply_delta_t`](Self::apply_delta_t) is set.
    fn equation_of_time(&self, julian_day_ut: f64) -> f64 {
        if self.use_almanac_sun {
            return almanac_equation_of_time(julian_centuries_from_julian_day(julian_day_ut));
        }
        let julian_centuries = self.julian_centuries(julian_day_ut);
        let tau = julian_centuries / 10.0;
        let l0 = sun_mean_longitude(tau);
        let alpha = self.sun_right_ascension(julian_centuries);
        let (delta_psi, _) = self.nutation_arcsec(julian_centuries);
        let epsilon = self.true_obliquity(julian_centuries);
        let mut e = l0 - 0.0057183 - alpha + (delta_psi / 3600.0) * cos_degrees(epsilon);
        e = ((e + 180.0) % 360.0 + 360.0) % 360.0 - 180.0;
        e * 4.0
    }
}

/// The sun's ecliptic longitude in degrees from the Astronomical Almanac's "low precision formulas
/// for the Sun", with the equation of center that it adds to the mean longitude. The Almanac
/// rates these formulas at 0.01° for the sun's coordinates from 1950 to 2050.
/// `julian_centuries` counts from J2000.0 in Universal Time.
///
/// The longitude λ is the mean longitude L plus two terms of the equation of center, with no
/// planets, Moon or nutation. The equation of center is 1.915° sin g + 0.020° sin 2g, where g is
/// the mean anomaly.
fn almanac_longitude(julian_centuries: f64) -> (f64, f64) {
    let days = julian_centuries * JULIAN_DAYS_PER_CENTURY;
    let mean_longitude = 280.460 + 0.9856474 * days;
    let mean_anomaly = 357.528 + 0.9856003 * days;
    let center = 1.915 * sin_degrees(mean_anomaly) + 0.020 * sin_degrees(2.0 * mean_anomaly);
    (mean_longitude + center, center)
}

/// Equation of time in minutes from the Astronomical Almanac's "low precision formulas for the
/// Sun", rated by the Almanac at 0.1 minute from 1950 to 2050. `julian_centuries` counts from
/// J2000.0 in Universal Time.
///
/// The equation of time L − α, in degrees, is
/// −1.915° sin g − 0.020° sin 2g + 2.466° sin 2λ − 0.053° sin 4λ, with L, g and λ as in
/// [`almanac_longitude`]. The first two terms are L − λ. The last two turn λ into the right
/// ascension α. Their coefficients are the first two terms of the series for α worked out once for
/// an obliquity of 23.44°, so they do not change with the date.
///
/// This form is quoted from the U.S. Naval Observatory, and attributed to the Explanatory Supplement
/// to the Astronomical Almanac. Of the published formulas tested, it matches MyZmanim's noon best.
/// The 2002 Almanac's own series (p. C24) works the two coefficients out from the obliquity of each
/// date, and puts MyZmanim's noon up to 0.3 s off. The U.S. Naval Observatory's "Approximate Solar
/// Coordinates", which converts to right ascension exactly, is further off still.
pub(super) fn almanac_equation_of_time(julian_centuries: f64) -> f64 {
    let (longitude, center) = almanac_longitude(julian_centuries);
    let reduction = 2.466 * sin_degrees(2.0 * longitude) - 0.053 * sin_degrees(4.0 * longitude);
    (reduction - center) * 4.0
}

/// The sun's declination in degrees from the Almanac's longitude λ and the given `obliquity` ε of
/// the ecliptic, in degrees. The sun is taken to lie on the ecliptic, so sin δ = sin ε sin λ.
/// `julian_centuries` counts from J2000.0 in Universal Time.
///
/// The Almanac gives its own obliquity, 23.439° − 0.0000004° per day from J2000.0. Its constant is
/// rounded to three decimals, and in 2027 it is 2.5″ below Meeus's mean obliquity (equation 22.2,
/// the IAU 1980 value). MyZmanim's times match Meeus's mean obliquity without nutation.
pub(super) fn almanac_declination(julian_centuries: f64, obliquity: f64) -> f64 {
    let (longitude, _) = almanac_longitude(julian_centuries);
    asin_degrees(sin_degrees(obliquity) * sin_degrees(longitude))
}
