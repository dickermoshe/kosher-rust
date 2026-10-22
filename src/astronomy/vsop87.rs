//! Shared high-accuracy solar-position building blocks used by the Meeus and SPA calculators.
//!
//! Contains the abridged VSOP87 Earth series (Meeus, *Astronomical Algorithms* 2nd ed.,
//! Appendix III), the principal nutation terms (Meeus chapter 22, abridged) and the
//! Espenak–Meeus ΔT polynomials. Mirrors the private tables in KosherJava's
//! `MeeusCalculator` and `SPACalculator`, which are identical.

// The published VSOP87 phase coefficients happen to round to π; they are table data, not π.
#![allow(clippy::approx_constant)]

#[allow(unused_imports)]
use core_maths::*;

use super::JULIAN_DAY_JAN_1_2000;

/// One VSOP87 term `{A, B, C}` contributing `A · cos(B + C · τ)`.
type Term = [f64; 3];

/// The synodic month's frequency in the VSOP87 series, in radians per Julian millennium:
/// the terms at this frequency are the Earth's own motion around the Earth–Moon barycenter.
const SYNODIC_MONTH_FREQUENCY: f64 = 77713.77;

/// Evaluates a VSOP87 series `Σₙ (Σᵢ Aᵢ · cos(Bᵢ + Cᵢ · τ)) · τⁿ` with coefficients stored in
/// 1e-8 of the native unit (radians for longitude / latitude, AU for the radius vector).
///
/// With `barycenter` set, the terms at the synodic month's frequency are left out. That removes
/// most of the Earth's monthly motion around the Earth–Moon barycenter. A smaller Moon-driven
/// latitude term, at frequency 84334.662, stays in.
pub(super) fn sum_series(series: &[&[Term]], tau: f64, barycenter: bool) -> f64 {
    let mut total = 0.0;
    let mut tau_pow = 1.0;
    for terms in series {
        let mut sum = 0.0;
        for term in *terms {
            if barycenter && (term[2] - SYNODIC_MONTH_FREQUENCY).abs() <= 0.1 {
                continue;
            }
            sum += term[0] * (term[1] + term[2] * tau).cos();
        }
        total += sum * tau_pow;
        tau_pow *= tau;
    }
    total / 1.0e8
}

/// Earth heliocentric longitude in radians for `tau` Julian millennia from J2000.0.
pub(super) fn earth_longitude(tau: f64, barycenter: bool) -> f64 {
    sum_series(EARTH_L, tau, barycenter)
}

/// Earth heliocentric latitude in radians for `tau` Julian millennia from J2000.0.
pub(super) fn earth_latitude(tau: f64, barycenter: bool) -> f64 {
    sum_series(EARTH_B, tau, barycenter)
}

/// Earth radius vector in AU for `tau` Julian millennia from J2000.0.
pub(super) fn earth_radius(tau: f64, barycenter: bool) -> f64 {
    sum_series(EARTH_R, tau, barycenter)
}

/// Principal nutation terms in longitude (Δψ) and obliquity (Δε), in arc-seconds, for
/// `t` Julian centuries of ephemeris time since J2000.0.
pub(super) fn nutation_arcsec(t: f64) -> (f64, f64) {
    let omega = 125.04452 - 1934.136261 * t + 0.0020708 * t * t + t * t * t / 450000.0;
    let l_sun = 280.4665 + 36000.7698 * t;
    let l_moon = 218.3165 + 481267.8813 * t;
    let delta_psi = -17.20 * sin_degrees(omega) - 1.32 * sin_degrees(2.0 * l_sun) - 0.23 * sin_degrees(2.0 * l_moon)
        + 0.21 * sin_degrees(2.0 * omega);
    let delta_epsilon = 9.20 * cos_degrees(omega) + 0.57 * cos_degrees(2.0 * l_sun) + 0.10 * cos_degrees(2.0 * l_moon)
        - 0.09 * cos_degrees(2.0 * omega);
    (delta_psi, delta_epsilon)
}

/// Mean longitude of the sun in degrees, reduced to `[0, 360)` (Meeus 28.1), for `tau`
/// Julian millennia of ephemeris time since J2000.0.
pub(super) fn sun_mean_longitude(tau: f64) -> f64 {
    let l0 = 280.4664567
        + tau * (360007.6982779 + tau * (0.03032028 + tau * (1.0 / 49931.0 - tau * (1.0 / 15300.0 + tau / 2000000.0))));
    (l0 % 360.0 + 360.0) % 360.0
}

/// Estimates ΔT = TT − UT in seconds for a UT Julian day using the Espenak and Meeus
/// polynomial expressions.
pub(super) fn delta_t_seconds(julian_day: f64) -> f64 {
    let year = 2000.0 + (julian_day - JULIAN_DAY_JAN_1_2000) / 365.25;
    if (2005.0..2050.0).contains(&year) {
        let t = year - 2000.0;
        62.92 + 0.32217 * t + 0.005589 * t * t
    } else if (1986.0..2005.0).contains(&year) {
        let t = year - 2000.0;
        63.86 + 0.3345 * t - 0.060374 * t * t
            + 0.0017275 * t * t * t
            + 0.000651814 * t.powf(4.0)
            + 0.00002373599 * t.powf(5.0)
    } else if (1961.0..1986.0).contains(&year) {
        let t = year - 1975.0;
        45.45 + 1.067 * t - t * t / 260.0 - t * t * t / 718.0
    } else if (2050.0..2150.0).contains(&year) {
        -20.0 + 32.0 * ((year - 1820.0) / 100.0).powf(2.0) - 0.5628 * (2150.0 - year)
    } else {
        let u = (year - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    }
}

/// Sine of an angle given in degrees.
pub(super) fn sin_degrees(angle: f64) -> f64 {
    angle.to_radians().sin()
}

/// Cosine of an angle given in degrees.
pub(super) fn cos_degrees(angle: f64) -> f64 {
    angle.to_radians().cos()
}

/// Tangent of an angle given in degrees.
pub(super) fn tan_degrees(angle: f64) -> f64 {
    angle.to_radians().tan()
}

/// Arc sine in degrees.
pub(super) fn asin_degrees(value: f64) -> f64 {
    value.asin().to_degrees()
}

/// Earth heliocentric longitude series L0..L5 (abridged VSOP87, Meeus Appendix III).
const EARTH_L: &[&[Term]] = &[
    &[
        // L0
        [175347046.0, 0.0, 0.0],
        [3341656.0, 4.6692568, 6283.07585],
        [34894.0, 4.6261, 12566.1517],
        [3497.0, 2.7441, 5753.3849],
        [3418.0, 2.8289, 3.5231],
        [3136.0, 3.6277, 77713.7715],
        [2676.0, 4.4181, 7860.4194],
        [2343.0, 6.1352, 3930.2097],
        [1324.0, 0.7425, 11506.7698],
        [1273.0, 2.0371, 529.6910],
        [1199.0, 1.1096, 1577.3435],
        [990.0, 5.233, 5884.927],
        [902.0, 2.045, 26.298],
        [857.0, 3.508, 398.149],
        [780.0, 1.179, 5223.694],
        [753.0, 2.533, 5507.553],
        [505.0, 4.583, 18849.228],
        [492.0, 4.205, 775.523],
        [357.0, 2.920, 0.067],
        [317.0, 5.849, 11790.629],
        [284.0, 1.899, 796.298],
        [271.0, 0.315, 10977.079],
        [243.0, 0.345, 5486.778],
        [206.0, 4.806, 2544.314],
        [205.0, 1.869, 5573.143],
        [202.0, 2.458, 6069.777],
        [156.0, 0.833, 213.299],
        [132.0, 3.411, 2942.463],
        [126.0, 1.083, 20.775],
        [115.0, 0.645, 0.980],
        [103.0, 0.636, 4694.003],
        [102.0, 0.976, 15720.839],
        [102.0, 4.267, 7.114],
        [99.0, 6.21, 2146.17],
        [98.0, 0.68, 155.42],
        [86.0, 5.98, 161000.69],
        [85.0, 1.30, 6275.96],
        [85.0, 3.67, 71430.70],
        [80.0, 1.81, 17260.15],
        [79.0, 3.04, 12036.46],
        [75.0, 1.76, 5088.63],
        [74.0, 3.50, 3154.69],
        [74.0, 4.68, 801.82],
        [70.0, 0.83, 9437.76],
        [62.0, 3.98, 8827.39],
        [61.0, 1.82, 7084.90],
        [57.0, 2.78, 6286.60],
        [56.0, 4.39, 14143.50],
        [56.0, 3.47, 6279.55],
        [52.0, 0.19, 12139.55],
        [52.0, 1.33, 1748.02],
        [51.0, 0.28, 5856.48],
        [49.0, 0.49, 1194.45],
        [41.0, 5.37, 8429.24],
        [41.0, 2.40, 19651.05],
        [39.0, 6.17, 10447.39],
        [37.0, 6.04, 10213.29],
        [37.0, 2.57, 1059.38],
        [36.0, 1.71, 2352.87],
        [36.0, 1.78, 6812.77],
        [33.0, 0.59, 17789.85],
        [30.0, 0.44, 83996.85],
        [30.0, 2.74, 1349.87],
        [25.0, 3.16, 4690.48],
    ],
    &[
        // L1
        [628331966747.0, 0.0, 0.0],
        [206059.0, 2.678235, 6283.07585],
        [4303.0, 2.6351, 12566.1517],
        [425.0, 1.590, 3.523],
        [119.0, 5.796, 26.298],
        [109.0, 2.966, 1577.344],
        [93.0, 2.59, 18849.23],
        [72.0, 1.14, 529.69],
        [68.0, 1.87, 398.15],
        [67.0, 4.41, 5507.55],
        [59.0, 2.89, 5223.69],
        [56.0, 2.17, 155.42],
        [45.0, 0.40, 796.30],
        [36.0, 0.47, 775.52],
        [29.0, 2.65, 7.11],
        [21.0, 5.34, 0.98],
        [19.0, 1.85, 5486.78],
        [19.0, 4.97, 213.30],
        [17.0, 2.99, 6275.96],
        [16.0, 0.03, 2544.31],
        [16.0, 1.43, 2146.17],
        [15.0, 1.21, 10977.08],
        [12.0, 2.83, 1748.02],
        [12.0, 3.26, 5088.63],
        [12.0, 5.27, 1194.45],
        [12.0, 2.08, 4694.00],
        [11.0, 0.77, 553.57],
        [10.0, 1.30, 6286.60],
        [10.0, 4.24, 1349.87],
        [9.0, 2.70, 242.73],
        [9.0, 5.64, 951.72],
        [8.0, 5.30, 2352.87],
        [6.0, 2.65, 9437.76],
        [6.0, 4.67, 4690.48],
    ],
    &[
        // L2
        [52919.0, 0.0, 0.0],
        [8720.0, 1.0721, 6283.0758],
        [309.0, 0.867, 12566.152],
        [27.0, 0.05, 3.52],
        [16.0, 5.19, 26.30],
        [16.0, 3.68, 155.42],
        [10.0, 0.76, 18849.23],
        [9.0, 2.06, 77713.77],
        [7.0, 0.83, 775.52],
        [5.0, 4.66, 1577.34],
        [4.0, 1.03, 7.11],
        [4.0, 3.44, 5573.14],
        [3.0, 5.14, 796.30],
        [3.0, 6.05, 5507.55],
        [3.0, 1.19, 242.73],
        [3.0, 6.12, 529.69],
        [3.0, 0.31, 398.15],
        [3.0, 2.28, 553.57],
        [2.0, 4.38, 5223.69],
        [2.0, 3.75, 0.98],
    ],
    &[
        // L3
        [289.0, 5.844, 6283.076],
        [35.0, 0.0, 0.0],
        [17.0, 5.49, 12566.15],
        [3.0, 5.20, 155.42],
        [1.0, 4.72, 3.52],
        [1.0, 5.30, 18849.23],
        [1.0, 5.97, 242.73],
    ],
    &[
        // L4
        [114.0, 3.142, 0.0],
        [8.0, 4.13, 6283.08],
        [1.0, 3.84, 12566.15],
    ],
    &[
        // L5
        [1.0, 3.14, 0.0],
    ],
];

/// Earth heliocentric latitude series B0..B1.
const EARTH_B: &[&[Term]] = &[
    &[
        // B0
        [280.0, 3.199, 84334.662],
        [102.0, 5.422, 5507.553],
        [80.0, 3.88, 5223.69],
        [44.0, 3.70, 2352.87],
        [32.0, 4.00, 1577.34],
    ],
    &[
        // B1
        [9.0, 3.90, 5507.55],
        [6.0, 1.73, 5223.69],
    ],
];

/// Earth radius-vector series R0..R4 (AU).
const EARTH_R: &[&[Term]] = &[
    &[
        // R0
        [100013989.0, 0.0, 0.0],
        [1670700.0, 3.0984635, 6283.07585],
        [13956.0, 3.05525, 12566.1517],
        [3084.0, 5.1985, 77713.7715],
        [1628.0, 1.1739, 5753.3849],
        [1576.0, 2.8469, 7860.4194],
        [925.0, 5.453, 11506.770],
        [542.0, 4.564, 3930.210],
        [472.0, 3.661, 5884.927],
        [346.0, 0.964, 5507.553],
        [329.0, 5.900, 5223.694],
        [307.0, 0.299, 5573.143],
        [243.0, 4.273, 11790.629],
        [212.0, 5.847, 1577.344],
        [186.0, 5.022, 10977.079],
        [175.0, 3.012, 18849.228],
        [110.0, 5.055, 5486.778],
        [98.0, 0.89, 6069.78],
        [86.0, 5.69, 15720.84],
        [86.0, 1.27, 161000.69],
        [65.0, 0.27, 17260.15],
        [63.0, 0.92, 529.69],
        [57.0, 2.01, 83996.85],
        [56.0, 5.24, 71430.70],
        [49.0, 3.25, 2544.31],
        [47.0, 2.58, 775.52],
        [45.0, 5.54, 9437.76],
        [43.0, 6.01, 6275.96],
        [39.0, 5.36, 4694.00],
        [38.0, 2.39, 8827.39],
        [37.0, 0.83, 19651.05],
        [37.0, 4.90, 12139.55],
        [36.0, 1.67, 12036.46],
        [35.0, 1.84, 2942.46],
        [33.0, 0.24, 7084.90],
        [32.0, 0.18, 5088.63],
        [32.0, 1.78, 398.15],
        [28.0, 1.21, 6286.60],
        [28.0, 1.90, 6279.55],
        [26.0, 4.59, 10447.39],
    ],
    &[
        // R1
        [103019.0, 1.10749, 6283.07585],
        [1721.0, 1.0644, 12566.1517],
        [702.0, 3.142, 0.0],
        [32.0, 1.02, 18849.23],
        [31.0, 2.84, 5507.55],
        [25.0, 1.32, 5223.69],
        [18.0, 1.42, 1577.34],
        [10.0, 5.91, 10977.08],
        [9.0, 1.42, 6275.96],
        [9.0, 0.27, 5486.78],
    ],
    &[
        // R2
        [4359.0, 5.7846, 6283.0758],
        [124.0, 5.579, 12566.152],
        [12.0, 3.14, 0.0],
        [9.0, 3.63, 77713.77],
        [6.0, 1.87, 5573.14],
        [3.0, 5.47, 18849.23],
    ],
    &[
        // R3
        [145.0, 4.273, 6283.076],
        [7.0, 3.92, 12566.15],
    ],
    &[
        // R4
        [4.0, 2.56, 6283.08],
    ],
];
