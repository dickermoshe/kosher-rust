//! The bracketing search behind the SPA events: the first time a function rises through zero in an
//! interval, to one millisecond.
//!
//! The interval is split until a piece can be shown to contain a rising crossing, using a bound on
//! the function's curvature, and that bracket is then narrowed with the ITP method. The search
//! never leaves the interval, and a function that only touches zero has no crossing.
//!
//! Times are in hours from the start of the interval.

#[allow(unused_imports)]
use core_maths::*;

/// The search resolution, one millisecond, in hours.
const TIME_TOLERANCE: f64 = 1.0 / 3600.0 / 1000.0;

/// Allowance for Julian-date quantisation and floating-point evaluation.
const ROUNDING_ERROR: f64 = 1e-8;

/// The first time in `(0, hours]` at which `function` rises through zero, in hours from the start
/// of the interval, or `None`.
///
/// `curvature` bounds `|f''|` in hours. `function_after_end` evaluates the function one millisecond
/// past the end: a zero exactly at the end counts only if the function goes on rising.
pub(super) fn search(
    hours: f64,
    curvature: f64,
    function: impl Fn(f64) -> f64,
    function_after_end: impl Fn() -> f64,
) -> Option<f64> {
    if hours == 0.0 {
        return None;
    }
    let search = CrossingSearch { function, curvature };
    let b = search.value(hours);
    let crossing = search.find(0.0, hours, search.value(0.0), b);
    if crossing.is_nan() && b == 0.0 {
        // Include a zero endpoint only when it crosses, rather than touches, the horizon.
        if search.value((hours - TIME_TOLERANCE).max(0.0)) < 0.0 && function_after_end() > 0.0 {
            return Some(hours);
        }
    }
    if crossing.is_nan() { None } else { Some(crossing) }
}

/// A function and the bound on its curvature.
struct CrossingSearch<F> {
    function: F,
    curvature: f64,
}

impl<F: Fn(f64) -> f64> CrossingSearch<F> {
    fn value(&self, time: f64) -> f64 {
        (self.function)(time)
    }

    fn find(&self, start: f64, end: f64, a: f64, b: f64) -> f64 {
        let width = end - start;
        let slope = (b - a) / width;
        let slope_error = self.curvature * width / 2.0 + 2.0 * ROUNDING_ERROR / width;
        // Standard interpolation bounds, using the conservative curvature estimate above:
        // slope error M*w/2, chord error M*w*w/8 (https://dlmf.nist.gov/3.3.E5).
        // Discard only intervals that cannot rise through zero; otherwise search left first.
        if slope + slope_error <= 0.0
            || ((a > 0.0) == (b > 0.0) && a.abs().min(b.abs()) > self.curvature * width * width / 8.0 + ROUNDING_ERROR)
        {
            return f64::NAN;
        }
        if a < 0.0 && b >= 0.0 && slope > slope_error {
            return self.refine(start, end, a, b);
        }
        if width <= TIME_TOLERANCE {
            return if a < 0.0 && b > 0.0 { end } else { f64::NAN };
        }
        let mut middle = start + width / 2.0;
        let mut m = self.value(middle);
        // Keep a shallow crossing inside a bracket instead of splitting at zero.
        // Stay strictly inside the interval so both recursive searches progress.
        if m == 0.0 && (a != 0.0 || b != 0.0) {
            middle += TIME_TOLERANCE.min(width / 4.0);
            m = self.value(middle);
        }
        let left = self.find(start, middle, a, m);
        if left.is_nan() {
            self.find(middle, end, m, b)
        } else {
            left
        }
    }

    fn refine(&self, mut start: f64, mut end: f64, mut a: f64, mut b: f64) -> f64 {
        // ITP (Oliveira & Takahashi): https://doi.org/10.1145/3423597.
        // Use k1 = 0.2 / initial width, k2 = 2, n0 = 1. The projection allows
        // at most one extra iteration over bisection while favouring interpolation.
        let scale = 0.2 / (end - start);
        // Maximum remaining width after a step; halve the allowance each time.
        let mut max_width = TIME_TOLERANCE;
        while max_width < end - start {
            max_width *= 2.0;
        }
        while end - start > TIME_TOLERANCE {
            let width = end - start;
            let middle = start + width / 2.0;
            // Equal values can occur on a rounded zero; use the midpoint then.
            let interpolated = if a == b { middle } else { start - a * width / (b - a) };
            let towards_middle = middle - interpolated;
            let truncated = interpolated
                + (scale * width * width)
                    .min(towards_middle.abs())
                    .copysign(towards_middle);
            let radius = (max_width - width / 2.0).max(0.0);
            let trial = middle + (-radius).max(radius.min(truncated - middle));
            let value = self.value(trial);
            if value > 0.0 {
                end = trial;
                b = value;
            } else {
                start = trial;
                a = value;
            }
            max_width /= 2.0;
        }
        // Keep the later endpoint, so the next search cannot rediscover this crossing.
        end
    }
}
