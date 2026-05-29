//! Java parity tests for Hebrew-date conversion and holiday calendar behavior.

mod assertions;
mod fixtures;
pub(crate) mod java_reference;
mod policy;
mod random;
mod rust_reference;
mod types;

use std::error::Error;

use rand::{SeedableRng, rngs::StdRng};

use crate::calendar::random::test_random_add_months_to_jewish_date;

use self::{
    assertions::{assert_date_results_match, assert_jewish_date_operations, assert_jewish_year_snapshot_matches},
    fixtures::{
        CALENDAR_EDGE_FIXTURE_DATES, GREGORIAN_REGRESSION_DATES, INVALID_JEWISH_REGRESSION_DATES,
        JEWISH_REGRESSION_DATES, JEWISH_YEAR_REGRESSION_YEARS,
    },
    java_reference::java_gregorian_date_to_jewish_date,
    random::{
        test_random_add_days_to_jewish_date, test_random_add_years_to_jewish_date,
        test_random_gregorian_to_jewish_date, test_random_invalid_jewish_date, test_random_jewish_calendar,
        test_random_jewish_date_snapshot, test_random_jewish_to_gregorian_date, test_random_jewish_year_snapshot,
        test_random_minus_days_to_jewish_date,
    },
    rust_reference::{rust_gregorian_date_to_jewish_date, rust_hebrew_date},
    types::DateTuple,
};

#[test]
pub(crate) fn test_hebrew_date_parity() -> Result<(), Box<dyn Error>> {
    run_regression_tests()?;

    let seed = policy::test_seed();
    let iterations = policy::test_iterations();
    eprintln!("ZMANIM_JAVA_PARITY_SEED={seed}");
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let context = format!("seed={seed} iteration={iteration}");
        test_random_gregorian_to_jewish_date(&mut rng, &context)?;
        test_random_jewish_to_gregorian_date(&mut rng, &context)?;
        test_random_invalid_jewish_date(&mut rng, &context)?;
        test_random_jewish_date_snapshot(&mut rng, &context)?;
        test_random_jewish_year_snapshot(&mut rng, &context)?;
        test_random_add_days_to_jewish_date(&mut rng, &context)?;
        test_random_minus_days_to_jewish_date(&mut rng, &context)?;
        test_random_add_months_to_jewish_date(&mut rng, &context)?;
        test_random_add_years_to_jewish_date(&mut rng, &context)?;
        test_random_jewish_calendar(&mut rng, &context)?;
    }

    Ok(())
}

fn run_regression_tests() -> Result<(), Box<dyn Error>> {
    for &date in GREGORIAN_REGRESSION_DATES {
        assert_date_results_match(
            date,
            "Jewish",
            java_gregorian_date_to_jewish_date(date)?,
            rust_gregorian_date_to_jewish_date(date),
            "regression",
        );
    }

    for &date in JEWISH_REGRESSION_DATES
        .iter()
        .chain(INVALID_JEWISH_REGRESSION_DATES.iter())
        .chain(CALENDAR_EDGE_FIXTURE_DATES.iter())
    {
        assert_jewish_date_operations(date, "regression")?;
    }

    for &year in JEWISH_YEAR_REGRESSION_YEARS {
        assert_jewish_year_snapshot_matches(year, "regression")?;
    }

    run_systematic_year_sweep()?;

    Ok(())
}

fn run_systematic_year_sweep() -> Result<(), Box<dyn Error>> {
    const REPRESENTATIVE_DATES: &[(u8, u8)] = &[(7, 1), (7, 10), (1, 15), (9, 25), (12, 14), (13, 14), (6, 29)];

    for year in 5600..=5900 {
        let context = format!("systematic year sweep year={year}");
        assert_jewish_year_snapshot_matches(year, &context)?;

        for &(month, day) in REPRESENTATIVE_DATES {
            let date = DateTuple::new(year, month, day);
            if rust_hebrew_date(date).is_none() {
                continue;
            }

            assert_jewish_date_operations(date, &context)?;
        }
    }

    Ok(())
}
