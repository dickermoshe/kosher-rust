//! ΔT (TT − UT1) estimates in seconds for the SPA calculator.
//!
//! Based on Espenak and Meeus, "Five Millennium Canon of Solar Eclipses: -1999 to +3000"
//! (NASA/TP-2006-214141), with Espenak's 2014 update. From 2015 the polynomials are Klaus
//! Brunner's 2026 adaptation (<https://klaus.brunners.name/posts/delta-t-polynomials/>): a quartic
//! fitted to IERS observations through mid-2026, followed by a quadratic fitted to a median of
//! simulations by Agnew (2026). Future values are (very) uncertain extrapolations, particularly
//! beyond 2100, and the last branch is extrapolated past the year 3000.

use jiff::civil::Date;

use super::polynomial;

/// ΔT in seconds for the midpoint of the date's calendar month.
pub(super) fn estimate(date: Date) -> f64 {
    estimate_for_year(decimal_year(date))
}

/// The date's year plus the fraction of the year at the middle of its month.
fn decimal_year(date: Date) -> f64 {
    f64::from(date.year()) + (f64::from(date.month()) - 0.5) / 12.0
}

/// ΔT in seconds for a decimal year.
fn estimate_for_year(year: f64) -> f64 {
    if year < -500.0 {
        let u = (year - 1820.0) / 100.0;
        polynomial(u, &[-20.0, 0.0, 32.0])
    } else if year < 500.0 {
        let u = year / 100.0;
        polynomial(
            u,
            &[
                10583.6,
                -1014.41,
                33.78311,
                -5.952053,
                -0.1798452,
                0.022174192,
                0.0090316521,
            ],
        )
    } else if year < 1600.0 {
        let u = (year - 1000.0) / 100.0;
        polynomial(
            u,
            &[
                1574.2,
                -556.01,
                71.23472,
                0.319781,
                -0.8503463,
                -0.005050998,
                0.0083572073,
            ],
        )
    } else if year < 1700.0 {
        let t = year - 1600.0;
        polynomial(t, &[120.0, -0.9808, -0.01532, 1.0 / 7129.0])
    } else if year < 1800.0 {
        let t = year - 1700.0;
        polynomial(t, &[8.83, 0.1603, -0.0059285, 0.00013336, -1.0 / 1174000.0])
    } else if year < 1860.0 {
        let t = year - 1800.0;
        polynomial(
            t,
            &[
                13.72,
                -0.332447,
                0.0068612,
                0.0041116,
                -0.00037436,
                0.0000121272,
                -0.0000001699,
                0.000000000875,
            ],
        )
    } else if year < 1900.0 {
        let t = year - 1860.0;
        polynomial(t, &[7.62, 0.5737, -0.251754, 0.01680668, -0.0004473624, 1.0 / 233174.0])
    } else if year < 1920.0 {
        let t = year - 1900.0;
        polynomial(t, &[-2.79, 1.494119, -0.0598939, 0.0061966, -0.000197])
    } else if year < 1941.0 {
        let t = year - 1920.0;
        polynomial(t, &[21.20, 0.84493, -0.076100, 0.0020936])
    } else if year < 1961.0 {
        let t = year - 1950.0;
        polynomial(t, &[29.07, 0.407, -1.0 / 233.0, 1.0 / 2547.0])
    } else if year < 1986.0 {
        let t = year - 1975.0;
        polynomial(t, &[45.45, 1.067, -1.0 / 260.0, -1.0 / 718.0])
    } else if year < 2005.0 {
        let t = year - 2000.0;
        polynomial(t, &[63.86, 0.3345, -0.060374, 0.0017275, 0.000651814, 0.00002373599])
    } else if year < 2015.0 {
        let t = year - 2005.0;
        polynomial(t, &[64.69, 0.2930])
    } else if year < 2026.5 {
        let t = year - 2015.0;
        // Retain full precision; the branches join in value and slope at 2015 and 2026.5.
        polynomial(
            t,
            &[
                67.62,
                0.2930,
                0.08753166427153103,
                -0.020049795884351372,
                0.0009758496126416308,
            ],
        )
    } else {
        // Also used past the year 3000, where it is an extrapolation.
        let t = year - 2026.5;
        polynomial(t, &[69.14, 0.28805287963416737, 0.0057380400318460655])
    }
}
