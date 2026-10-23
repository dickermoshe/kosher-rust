//! The NREL Solar Position Algorithm (SPA) calculator, ported from Klaus Brunner's
//! solarpositioning.
//!
//! Computes the sun's geocentric position from the SPA series (Reda and Andreas,
//! NREL/TP-560-34302) and its unrefracted topocentric elevation, with parallax for the observer's
//! height. Sunrise, sunset and the depression-angle times are the first crossing of the target
//! elevation in the window that belongs to the date, and solar noon and midnight are the meridian
//! transits; both are found with a bracketing search that can not leave its window and reports no
//! event rather than a near miss. For a rise or set, a closed-form estimate refined by secant steps
//! is tried first and kept when it converges in the window on an ordinary crossing, which saves
//! most of the search's evaluations; grazing and polar days fall back to the search. Refraction, the solar radius and the horizon dip come only from
//! the zenith adjustment in the parent module, as for the other calculators. ΔT is estimated per
//! date unless [`Spa::apply_delta_t`] is `false`.
//!
//! solarpositioning's license:
//!
//! > Copyright (c) 2013, Klaus A. Brunner.
//! >
//! > Permission is hereby granted, free of charge, to any person obtaining a copy of this software
//! > and associated documentation files (the "Software"), to deal in the Software without
//! > restriction, including without limitation the rights to use, copy, modify, merge, publish,
//! > distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
//! > Software is furnished to do so, subject to the following conditions:
//! >
//! > The above copyright notice and this permission notice shall be included in all copies or
//! > substantial portions of the Software.
//! >
//! > THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
//! > BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
//! > NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//! > DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! > OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
mod coefficients;
mod delta_t;
mod search;

#[allow(unused_imports)]
use core_maths::*;
use jiff::civil::Date;

use self::coefficients::{NUTATION_COEFFS, OBLIQUITY_COEFFS, TERMS_B, TERMS_L, TERMS_PE, TERMS_R, TERMS_Y};
use super::{SolarAlgorithm, SolarEvent, julian_day, time_at_azimuth_iterative, vsop87::sun_mean_longitude};
use crate::{Location, zmanim::ZmanimError};

/// The SPA algorithm.
pub(super) struct Spa {
    /// Whether to apply ΔT (TT − UT). KosherJava defaults this to `true`.
    pub(super) apply_delta_t: bool,
}

/// A moment as a continuous UT Julian date, with the ΔT (TT − UT) in seconds that converts it to
/// Terrestrial Time.
#[derive(Clone, Copy)]
struct JulianDate {
    julian_date: f64,
    delta_t: f64,
}

impl JulianDate {
    fn julian_ephemeris_day(self) -> f64 {
        self.julian_date + self.delta_t / 86400.0
    }

    fn julian_century(self) -> f64 {
        (self.julian_date - 2451545.0) / 36525.0
    }

    fn julian_ephemeris_century(self) -> f64 {
        (self.julian_ephemeris_day() - 2451545.0) / 36525.0
    }

    fn julian_ephemeris_millennium(self) -> f64 {
        self.julian_ephemeris_century() / 10.0
    }
}

/// The parts of the sun's position that depend only on the time, not on the observer.
struct TimeDependent {
    /// Earth radius vector in AU.
    r: f64,
    /// Apparent sidereal time at Greenwich in degrees.
    nu_degrees: f64,
    /// Geocentric right ascension in degrees, 0 to 360.
    alpha_degrees: f64,
    /// Geocentric declination in degrees.
    delta_degrees: f64,
    /// Nutation in longitude in degrees.
    delta_psi: f64,
    /// True obliquity of the ecliptic in degrees.
    epsilon_degrees: f64,
}

/// The sun as seen by an observer, as needed by the event searches.
struct EventPosition {
    /// Unrefracted topocentric elevation of the solar centre in degrees.
    elevation: f64,
    /// Geocentric local hour angle in degrees, positive west, not normalised.
    hour_angle: f64,
}

/// The sun's geocentric position at a moment (SPA steps 1 to 11).
fn time_dependent(jd: JulianDate) -> TimeDependent {
    let jme = jd.julian_ephemeris_millennium();
    let jce = jd.julian_ephemeris_century();

    // Earth heliocentric longitude L, latitude B and radius vector R.
    let l_degrees = lbr_to_normalized_degrees(jme, TERMS_L);
    let b_degrees = lbr_to_normalized_degrees(jme, TERMS_B);
    let r = lbr_polynomial(jme, TERMS_R);

    // Geocentric longitude theta and latitude beta.
    let theta_degrees = limit_to(l_degrees + 180.0, 360.0);
    let beta_degrees = -b_degrees;
    let beta = beta_degrees.to_radians();

    // Nutation.
    let x_terms = nutation_terms(jce);
    let (delta_psi, delta_epsilon) = delta_psi_epsilon(jce, &x_terms);

    // True obliquity of the ecliptic.
    let epsilon_degrees = true_obliquity_of_ecliptic(jd, delta_epsilon);
    let epsilon = epsilon_degrees.to_radians();

    // Aberration correction.
    let delta_tau = -20.4898 / (3600.0 * r);

    // Apparent sun longitude.
    let lambda_degrees = theta_degrees + delta_psi + delta_tau;
    let lambda = lambda_degrees.to_radians();

    let nu_degrees = apparent_sidereal_time_at_greenwich(jd, delta_psi, epsilon_degrees);
    let (alpha_degrees, delta_degrees) = geocentric_sun_coordinates(beta, epsilon, lambda);

    TimeDependent {
        r,
        nu_degrees,
        alpha_degrees,
        delta_degrees,
        delta_psi,
        epsilon_degrees,
    }
}

/// The sun's unrefracted topocentric elevation for an observer at `latitude`, `longitude`
/// (positive east) and `height` metres, and its geocentric hour angle.
fn event_position_at_height(time: JulianDate, latitude: f64, longitude: f64, height: f64) -> EventPosition {
    let phi = latitude.to_radians();
    let u = (0.99664719 * phi.tan()).atan();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let x = u.cos() + (height * cos_phi) / 6378140.0;
    let z = 0.99664719 * u.sin() + (height * sin_phi) / 6378140.0;
    let parts = time_dependent(time);
    let delta = parts.delta_degrees.to_radians();
    let hour_angle = parts.nu_degrees + longitude - parts.alpha_degrees;
    let parallax = (8.794 / (3600.0 * parts.r)).to_radians().sin();
    // The parallax correction in vector form, which stays stable at the zenith.
    let vx = delta.cos() * hour_angle.to_radians().cos() - x * parallax;
    let vy = delta.cos() * hour_angle.to_radians().sin();
    let vz = delta.sin() - z * parallax;
    let projection = (cos_phi * vx + sin_phi * vz) / (vx * vx + vy * vy + vz * vz).sqrt();
    EventPosition {
        elevation: projection.clamp(-1.0, 1.0).asin().to_degrees(),
        hour_angle,
    }
}

/// The sun's geocentric right ascension (0 to 360) and declination, in degrees.
fn geocentric_sun_coordinates(beta: f64, epsilon: f64, lambda: f64) -> (f64, f64) {
    let sin_lambda = lambda.sin();
    let cos_lambda = lambda.cos();
    let sin_epsilon = epsilon.sin();
    let cos_epsilon = epsilon.cos();
    let sin_beta = beta.sin();
    let cos_beta = beta.cos();

    let alpha = (sin_lambda * cos_epsilon - (sin_beta / cos_beta) * sin_epsilon).atan2(cos_lambda);
    let delta = (sin_beta * cos_epsilon + cos_beta * sin_epsilon * sin_lambda).asin();

    (limit_to(alpha.to_degrees(), 360.0), delta.to_degrees())
}

/// The true obliquity of the ecliptic in degrees.
fn true_obliquity_of_ecliptic(jd: JulianDate, delta_epsilon: f64) -> f64 {
    let epsilon0 = polynomial(jd.julian_ephemeris_millennium() / 10.0, &OBLIQUITY_COEFFS);
    epsilon0 / 3600.0 + delta_epsilon
}

/// The apparent sidereal time at Greenwich in degrees.
fn apparent_sidereal_time_at_greenwich(jd: JulianDate, delta_psi: f64, epsilon_degrees: f64) -> f64 {
    let jc = jd.julian_century();
    let nu0_degrees = limit_to(
        280.46061837 + 360.98564736629 * (jd.julian_date - 2451545.0) + (jc * jc) * (0.000387933 - jc / 38710000.0),
        360.0,
    );
    nu0_degrees + delta_psi * epsilon_degrees.to_radians().cos()
}

/// The nutation in longitude and in obliquity, in degrees.
fn delta_psi_epsilon(jce: f64, x: &[f64; 5]) -> (f64, f64) {
    let mut delta_psi = 0.0;
    let mut delta_epsilon = 0.0;

    for (terms_pe, terms_y) in TERMS_PE.iter().zip(TERMS_Y.iter()) {
        let xj_yterm_sum = xj_yterm_sum(terms_y, x).to_radians();
        delta_psi += (terms_pe[1] * jce + terms_pe[0]) * xj_yterm_sum.sin();
        delta_epsilon += (terms_pe[3] * jce + terms_pe[2]) * xj_yterm_sum.cos();
    }

    (delta_psi / 36_000_000.0, delta_epsilon / 36_000_000.0)
}

/// The argument of one nutation term: the sum of the five nutation arguments times its
/// multipliers, in degrees.
fn xj_yterm_sum(terms_y: &[f64; 5], x: &[f64; 5]) -> f64 {
    let mut sum = 0.0;
    for (x_j, y_j) in x.iter().zip(terms_y.iter()) {
        sum += x_j * *y_j;
    }
    sum
}

/// The five nutation arguments X0 to X4 (mean elongation of the moon, mean anomalies of the sun
/// and moon, the moon's argument of latitude and the longitude of its ascending node), in degrees.
fn nutation_terms(jce: f64) -> [f64; 5] {
    let mut x = [0.0; 5];
    for (x_i, coeffs) in x.iter_mut().zip(NUTATION_COEFFS.iter()) {
        *x_i = polynomial(jce, coeffs);
    }
    x
}

/// Evaluates one of the Earth series: the sum of each power's terms `A · cos(B + C · jme)`, as a
/// polynomial in `jme`, in radians or AU. Every series has at most six powers.
fn lbr_polynomial(jme: f64, term_coeffs: &[&[[f64; 3]]]) -> f64 {
    let mut per_power_sums = [0.0; 6];

    for (sum_i, terms) in per_power_sums.iter_mut().zip(term_coeffs.iter()) {
        let mut sum = 0.0;
        for coeffs in terms.iter() {
            sum += coeffs[0] * (coeffs[1] + coeffs[2] * jme).cos();
        }
        *sum_i = sum;
    }

    polynomial(jme, &per_power_sums[..term_coeffs.len()]) / 1e8
}

/// Evaluates one of the Earth series in degrees, reduced to 0 to 360.
fn lbr_to_normalized_degrees(jme: f64, term_coeffs: &[&[[f64; 3]]]) -> f64 {
    limit_to(lbr_polynomial(jme, term_coeffs).to_degrees(), 360.0)
}

/// Evaluates a polynomial by Horner's rule, lowest coefficient first.
fn polynomial(x: f64, coeffs: &[f64]) -> f64 {
    let n = coeffs.len() - 1;
    let mut sum = coeffs[n];
    for coeff in coeffs[..n].iter().rev() {
        sum = x * sum + *coeff;
    }
    sum
}

/// Reduces `degrees` to `[0, max)`.
fn limit_to(degrees: f64, max: f64) -> f64 {
    let remainder = degrees % max;
    let limited = if remainder < 0.0 { remainder + max } else { remainder };
    // Adding max to a tiny negative remainder can round up to the excluded boundary.
    if limited >= max { 0.0 } else { limited }
}

/// The bound on the curvature of `sin(hour angle)` used by the transit search, in hours.
const TRANSIT_CURVATURE: f64 = 0.1;

/// How far, in minutes, on each side of its approximate time a transit is searched for.
const TRANSIT_WINDOW_MINUTES: f64 = 360.0;

/// The window, in UTC minutes after 0:00 of the date, in which a sunrise or sunset belongs to the
/// date: `(18:00, 18:00]` local mean time from the evening before for a sunrise, and `[06:00,
/// 06:00)` local mean time into the next morning for a sunset. These are the inverse of
/// [`super::instant_from_utc_hours`], which assigns a sunrise after 18:00 to the previous date and
/// a sunset before 06:00 to the next, so a result found here always maps back to this date.
fn rise_set_window_minutes(longitude: f64, event: SolarEvent) -> (f64, f64) {
    let start_hours = if event == SolarEvent::Sunrise { -6.0 } else { 6.0 } - longitude / 15.0;
    (start_hours * 60.0, (start_hours + 24.0) * 60.0)
}

impl Spa {
    /// ΔT in seconds for the date, or zero when disabled. It is held constant during a search.
    fn delta_t(&self, date: Date) -> f64 {
        if self.apply_delta_t {
            delta_t::estimate(date)
        } else {
            0.0
        }
    }

    /// The upper (`Noon`) or lower (`Midnight`) meridian transit in UTC minutes after 0:00 of the
    /// date, searched for within [`TRANSIT_WINDOW_MINUTES`] of its approximate time. The lower
    /// transit is the one after the date's noon. NaN if the search finds none.
    fn transit_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        let jd_day = julian_day(date);
        let delta_t = self.delta_t(date);
        let longitude = -longitude_west;
        let lower = event == SolarEvent::Midnight;
        let approximate = 720.0 + 4.0 * longitude_west + if lower { 720.0 } else { 0.0 };
        let start = approximate - TRANSIT_WINDOW_MINUTES;
        let hours = 2.0 * TRANSIT_WINDOW_MINUTES / 60.0;
        // The hour angle rises through 0 at the upper transit and through 180 at the lower one;
        // the search looks for sin(hour angle) rising through zero, so negate it for the lower.
        let sign = if lower { -1.0 } else { 1.0 };
        let value = |minutes: f64| {
            let time = JulianDate {
                julian_date: jd_day + minutes / 1440.0,
                delta_t,
            };
            sign * event_position_at_height(time, 0.0, longitude, 0.0)
                .hour_angle
                .to_radians()
                .sin()
        };
        let found = search::search(
            hours,
            TRANSIT_CURVATURE,
            |t| value(start + t * 60.0),
            || value(start + hours * 60.0 + 1.0 / 60000.0),
        );
        // The window spans about 180° of hour angle around local mean noon or midnight, so it holds
        // exactly one transit, and sin(hour angle) bends less than TRANSIT_CURVATURE allows. A
        // miss is a bug; NaN makes the caller report it as an internal error.
        debug_assert!(
            found.is_some(),
            "no transit within {TRANSIT_WINDOW_MINUTES} minutes of {approximate}"
        );
        found.map_or(f64::NAN, |t| start + t * 60.0)
    }
}

/// Equation of time in minutes (Meeus chapter 28) from the SPA right ascension at `time`, with the
/// NOAA sign convention.
fn equation_of_time(time: JulianDate, parts: &TimeDependent) -> f64 {
    let l0 = sun_mean_longitude(time.julian_ephemeris_millennium());
    let e = l0 - 0.0057183 - parts.alpha_degrees + parts.delta_psi * parts.epsilon_degrees.to_radians().cos();
    (((e + 180.0) % 360.0 + 360.0) % 360.0 - 180.0) * 4.0
}

/// Largest difference, in degrees, between the requested zenith and the sun's zenith at a secant
/// result for it to be kept. Converged results end within about 1e-7° to 8e-7° (the time
/// resolution of a Julian date held in an `f64`); failed ones end far away.
const MAX_ZENITH_RESIDUAL: f64 = 1e-4;

/// Slowest rate, in degrees per minute, at which the sun may cross the zenith anywhere along the
/// secant search for its result to be kept. Ordinary crossings run at about 0.1° to 0.25° a
/// minute; a slower one means a near-grazing day, where the secant can settle on a near miss or on
/// another crossing.
const GRAZING_MIN_SLOPE: f64 = 0.02;

/// The fast path for a sunrise or sunset, in UTC minutes after 0:00 of the date: the closed-form
/// hour angle at the declination of local mean noon, refined by secant steps on the topocentric
/// zenith.
///
/// Returns `None` when the estimate puts the zenith out of reach, or when the result can not be
/// trusted: it did not converge onto the zenith, lies outside `window`, or the sun crossed the
/// zenith slowly somewhere along the search. The bracketing search decides those cases.
fn secant_rise_set(
    jd_day: f64,
    delta_t: f64,
    location: &Location,
    adjusted_zenith: f64,
    event: SolarEvent,
    window: (f64, f64),
) -> Option<f64> {
    let (latitude, longitude, height) = (location.latitude, location.longitude, location.elevation);
    let time_at = |minutes: f64| JulianDate {
        julian_date: jd_day + minutes / 1440.0,
        delta_t,
    };
    let zenith_difference = |minutes: f64| {
        90.0 - event_position_at_height(time_at(minutes), latitude, longitude, height).elevation - adjusted_zenith
    };

    let noon_time = time_at(720.0 - 4.0 * longitude);
    let noon = time_dependent(noon_time);
    let declination = noon.delta_degrees.to_radians();
    let phi = latitude.to_radians();
    let cos_hour_angle =
        (adjusted_zenith.to_radians().cos() - phi.sin() * declination.sin()) / (phi.cos() * declination.cos());
    if !(-1.0..=1.0).contains(&cos_hour_angle) {
        return None;
    }
    let hour_angle = cos_hour_angle.acos().to_degrees();
    let signed = if event == SolarEvent::Sunrise {
        hour_angle
    } else {
        -hour_angle
    };
    let guess = 720.0 - 4.0 * (longitude + signed) - equation_of_time(noon_time, &noon);

    let mut t0 = guess;
    let mut t1 = guess + 0.5;
    let mut f0 = zenith_difference(t0);
    let mut f1 = zenith_difference(t1);
    let mut min_slope = ((f1 - f0) / (t1 - t0)).abs();
    for _ in 0..12 {
        if f1.abs() <= 1e-9 {
            break;
        }
        let denom = f1 - f0;
        if denom == 0.0 {
            break;
        }
        min_slope = min_slope.min((denom / (t1 - t0)).abs());
        let t2 = t1 - f1 * (t1 - t0) / denom;
        t0 = t1;
        f0 = f1;
        t1 = t2;
        f1 = zenith_difference(t1);
    }

    let in_window = if event == SolarEvent::Sunrise {
        t1 > window.0 && t1 <= window.1
    } else {
        t1 >= window.0 && t1 < window.1
    };
    (in_window && f1.abs() <= MAX_ZENITH_RESIDUAL && min_slope >= GRAZING_MIN_SLOPE).then_some(t1)
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
        let delta_t = self.delta_t(date);
        let elevation_angle = 90.0 - adjusted_zenith;
        if !(-90.0..=90.0).contains(&elevation_angle) {
            return Err(ZmanimError::Internal);
        }
        let (latitude, longitude, height) = (location.latitude, location.longitude, location.elevation);

        let (start, end) = rise_set_window_minutes(longitude, event);
        if let Some(minutes) = secant_rise_set(jd_day, delta_t, location, adjusted_zenith, event, (start, end)) {
            return Ok(minutes);
        }
        let hours = (end - start) / 60.0;

        // sin(elevation) − sin(target), which rises through zero at sunrise; negated for sunset.
        let horizon = elevation_angle.to_radians().sin();
        let direction = if event == SolarEvent::Sunrise { 1.0 } else { -1.0 };
        // Engineering estimate for |f''| in hours: daily rotation contributes about
        // (2*pi/24)^2 * cos(latitude) = 0.069 * cos(latitude). Round up to 0.1 and
        // retain a floor for slower solar motion and parallax, including at the poles.
        let curvature = 0.1 * latitude.to_radians().cos().abs() + 0.0001;
        let value = |minutes: f64| {
            let time = JulianDate {
                julian_date: jd_day + minutes / 1440.0,
                delta_t,
            };
            direction
                * (event_position_at_height(time, latitude, longitude, height)
                    .elevation
                    .to_radians()
                    .sin()
                    - horizon)
        };

        match search::search(
            hours,
            curvature,
            |t| value(start + t * 60.0),
            || value(start + hours * 60.0 + 1.0 / 60000.0),
        ) {
            Some(t) => Ok(start + t * 60.0),
            // No crossing in the window: report which side of the elevation the sun is on at local
            // mean noon, the middle of the day.
            None if direction * value((12.0 - longitude / 15.0) * 60.0) > 0.0 => Err(ZmanimError::AllDay),
            None => Err(ZmanimError::AllNight),
        }
    }

    fn noon_midnight_utc_minutes(&self, date: Date, longitude_west: f64, event: SolarEvent) -> f64 {
        self.transit_minutes(date, longitude_west, event)
    }

    fn time_at_azimuth_utc_hours(&self, date: Date, location: &Location, azimuth: f64) -> Result<f64, ZmanimError> {
        let delta_t = self.delta_t(date);
        time_at_azimuth_iterative(date, location, azimuth, 4, |julian_day| {
            let time = JulianDate {
                julian_date: julian_day,
                delta_t,
            };
            let parts = time_dependent(time);
            (parts.delta_degrees, equation_of_time(time, &parts))
        })
    }
}

#[cfg(test)]
mod tests;
