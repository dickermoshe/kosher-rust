//! Regression fixtures for Hebrew-date Java parity tests.

use super::types::DateTuple;

/// Fixed Gregorian inputs used by the Hebrew-date parity regression pass.
pub(super) const GREGORIAN_REGRESSION_DATES: &[DateTuple] = &[
    DateTuple::new(1900, 1, 1),
    DateTuple::new(1999, 12, 31),
    DateTuple::new(2000, 2, 29),
    DateTuple::new(2024, 3, 24),
    DateTuple::new(2024, 4, 22),
    DateTuple::new(2024, 10, 3),
    DateTuple::new(2100, 12, 31),
];

/// Fixed valid Jewish dates used by the Hebrew-date operation regression pass.
pub(super) const JEWISH_REGRESSION_DATES: &[DateTuple] = &[
    DateTuple::new(5660, 10, 1),
    DateTuple::new(5760, 7, 1),
    DateTuple::new(5782, 13, 14),
    DateTuple::new(5784, 12, 30),
    DateTuple::new(5784, 13, 1),
    DateTuple::new(5785, 1, 15),
    DateTuple::new(5861, 9, 29),
];

/// Fixed invalid Jewish date inputs that must fail the same way in Java and Rust.
pub(super) const INVALID_JEWISH_REGRESSION_DATES: &[DateTuple] = &[
    DateTuple::new(5784, 0, 10),
    DateTuple::new(5784, 14, 10),
    DateTuple::new(5784, 13, 30),
    DateTuple::new(5785, 13, 1),
    DateTuple::new(5785, 2, 30),
    DateTuple::new(5784, 7, 0),
    DateTuple::new(5784, 7, 31),
];

/// Fixed Jewish years used for year-level calendar metadata parity.
pub(super) const JEWISH_YEAR_REGRESSION_YEARS: &[i32] = &[5660, 5701, 5765, 5782, 5783, 5784, 5785, 5801, 5861];

/// Fixed Jewish dates chosen for holiday and calendar-behavior edge cases.
pub(super) const CALENDAR_EDGE_FIXTURE_DATES: &[DateTuple] = &[
    DateTuple::new(5784, 1, 14),
    DateTuple::new(5784, 1, 16),
    DateTuple::new(5784, 1, 22),
    DateTuple::new(5784, 3, 6),
    DateTuple::new(5784, 7, 1),
    DateTuple::new(5784, 7, 10),
    DateTuple::new(5784, 7, 15),
    DateTuple::new(5784, 7, 22),
    DateTuple::new(5784, 7, 23),
    DateTuple::new(5784, 9, 30),
    DateTuple::new(5784, 10, 1),
    DateTuple::new(5784, 12, 14),
    DateTuple::new(5784, 13, 14),
    DateTuple::new(5785, 2, 29),
    DateTuple::new(5785, 3, 30),
    DateTuple::new(5785, 8, 29),
    DateTuple::new(5785, 8, 30),
];
