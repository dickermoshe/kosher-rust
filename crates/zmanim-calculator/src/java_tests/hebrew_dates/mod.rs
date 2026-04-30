//! Java parity tests for Hebrew-date conversion and holiday calendar behavior.

mod assertions;
mod fixtures;
mod java_reference;
mod policy;
mod random;
mod rust_reference;
mod types;

use std::error::Error;

use rand::{rngs::StdRng, SeedableRng};

use self::{
    assertions::{
        assert_date_results_match, assert_jewish_date_operations,
        assert_jewish_year_snapshot_matches,
    },
    fixtures::{
        CALENDAR_EDGE_FIXTURE_DATES, GREGORIAN_REGRESSION_DATES, INVALID_JEWISH_REGRESSION_DATES,
        JEWISH_REGRESSION_DATES, JEWISH_YEAR_REGRESSION_YEARS,
    },
    java_reference::java_gregorian_date_to_jewish_date,
    random::{
        test_random_add_days_to_jewish_date, test_random_add_months_to_jewish_date,
        test_random_add_years_to_jewish_date, test_random_gregorian_to_jewish_date,
        test_random_jewish_calendar, test_random_jewish_date_snapshot,
        test_random_jewish_to_gregorian_date, test_random_jewish_year_snapshot,
        test_random_minus_days_to_jewish_date,
    },
    rust_reference::rust_gregorian_date_to_jewish_date,
};

#[test]
pub(crate) fn test_hebrew_date_parity() -> Result<(), Box<dyn Error>> {
    run_regression_tests()?;

    let seed = policy::test_seed();
    let iterations = policy::test_iterations();
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let context = format!("seed={seed} iteration={iteration}");
        test_random_gregorian_to_jewish_date(&mut rng, &context)?;
        test_random_jewish_to_gregorian_date(&mut rng, &context)?;
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

    Ok(())
}
