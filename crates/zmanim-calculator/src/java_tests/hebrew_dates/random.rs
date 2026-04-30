//! Randomized cases for Hebrew-date Java parity tests.

use std::error::Error;

use chrono::{Datelike, Months, NaiveDate};
use rand::{rngs::StdRng, RngExt};

use super::{
    assertions::{
        assert_calendar_snapshots_match, assert_date_results_match,
        assert_jewish_date_snapshots_match, assert_jewish_year_snapshot_matches,
    },
    java_reference::{
        java_add_days_to_jewish_date, java_add_months_to_jewish_date,
        java_add_years_to_jewish_date, java_add_years_to_jewish_date_with_adar,
        java_gregorian_date_to_jewish_date, java_jewish_date_to_gregorian_date,
        java_minus_days_to_jewish_date,
    },
    policy,
    rust_reference::{
        rust_add_days_to_jewish_date, rust_add_months_to_jewish_date,
        rust_add_years_to_jewish_date_with_adar, rust_gregorian_date_to_jewish_date,
        rust_jewish_date_to_gregorian_date,
    },
    types::DateTuple,
};

pub(super) fn test_random_gregorian_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_gregorian_date(rng);
    assert_date_results_match(
        date,
        "Jewish",
        java_gregorian_date_to_jewish_date(date)?,
        rust_gregorian_date_to_jewish_date(date),
        context,
    );
    Ok(())
}

pub(super) fn test_random_jewish_to_gregorian_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    assert_date_results_match(
        date,
        "Gregorian",
        java_jewish_date_to_gregorian_date(date)?,
        rust_jewish_date_to_gregorian_date(date),
        context,
    );
    Ok(())
}

pub(super) fn test_random_jewish_date_snapshot(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    if rust_jewish_date_to_gregorian_date(date).is_some() {
        assert_jewish_date_snapshots_match(date, context)?;
    }
    Ok(())
}

pub(super) fn test_random_jewish_year_snapshot(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let (min_jewish_year, max_jewish_year) = policy::jewish_year_range();
    let year = rng.random_range(min_jewish_year..=max_jewish_year);
    assert_jewish_year_snapshot_matches(year, context)
}

pub(super) fn test_random_add_days_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let days_to_add = rng.random_range(1..=600);
    assert_date_results_match(
        date,
        &format!("Jewish after adding {days_to_add} days"),
        java_add_days_to_jewish_date(date, days_to_add)?,
        rust_add_days_to_jewish_date(date, days_to_add),
        context,
    );
    Ok(())
}

pub(super) fn test_random_minus_days_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let days_to_subtract = rng.random_range(1..=600);
    assert_date_results_match(
        date,
        &format!("Jewish after subtracting {days_to_subtract} days"),
        java_minus_days_to_jewish_date(date, days_to_subtract)?,
        rust_add_days_to_jewish_date(date, -days_to_subtract),
        context,
    );
    Ok(())
}

pub(super) fn test_random_add_months_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let months_to_add = rng.random_range(1..=120);
    assert_date_results_match(
        date,
        &format!("Jewish after adding {months_to_add} months"),
        java_add_months_to_jewish_date(date, months_to_add)?,
        rust_add_months_to_jewish_date(date, months_to_add),
        context,
    );
    Ok(())
}

pub(super) fn test_random_add_years_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let years_to_add = rng.random_range(1..=60);
    assert_date_results_match(
        date,
        &format!("Jewish after adding {years_to_add} years"),
        java_add_years_to_jewish_date(date, years_to_add)?,
        rust_add_years_to_jewish_date_with_adar(date, years_to_add, false),
        context,
    );
    assert_date_results_match(
        date,
        &format!("Jewish after adding {years_to_add} years (adar aleph)"),
        java_add_years_to_jewish_date_with_adar(date, years_to_add, true)?,
        rust_add_years_to_jewish_date_with_adar(date, years_to_add, true),
        context,
    );
    Ok(())
}

pub(super) fn test_random_jewish_calendar(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let java_gregorian = java_jewish_date_to_gregorian_date(date)?;
    let rust_gregorian = rust_jewish_date_to_gregorian_date(date);
    assert_date_results_match(date, "Gregorian", java_gregorian, rust_gregorian, context);
    if java_gregorian.is_none() && rust_gregorian.is_none() {
        return Ok(());
    }

    let in_israel = rng.random_bool(0.5);
    let use_modern_holidays = rng.random_bool(0.5);
    assert_calendar_snapshots_match(date, in_israel, use_modern_holidays, context)
}

fn random_gregorian_date(rng: &mut StdRng) -> DateTuple {
    let (min_gregorian_year, max_gregorian_year) =
        (policy::MIN_GREGORIAN_YEAR, policy::MAX_GREGORIAN_YEAR);
    let year = rng.random_range(min_gregorian_year..=max_gregorian_year);
    let month = rng.random_range(1..=12);
    let max_day = days_in_gregorian_month(year, month);
    let day = rng.random_range(1..=max_day);
    DateTuple::new(year, month, day)
}

fn random_jewish_date(rng: &mut StdRng) -> DateTuple {
    let (min_jewish_year, max_jewish_year) = policy::jewish_year_range();
    DateTuple::new(
        rng.random_range(min_jewish_year..=max_jewish_year),
        rng.random_range(1..=13),
        rng.random_range(1..=30),
    )
}

fn days_in_gregorian_month(year: i32, month: u8) -> u8 {
    let first_of_month = NaiveDate::from_ymd_opt(year, u32::from(month), 1)
        .expect("generated month is always valid");
    first_of_month
        .checked_add_months(Months::new(1))
        .and_then(|date| date.pred_opt())
        .expect("generated month has a previous day")
        .day() as u8
}
