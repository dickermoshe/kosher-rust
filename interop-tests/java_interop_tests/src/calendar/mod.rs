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
    assertions::{
        assert_calendar_snapshots_match, assert_date_results_match, assert_jewish_date_operations,
        assert_jewish_year_snapshot_matches,
    },
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

/// The molad of Teves 3761 is on 12 December of Gregorian year 0, which is a leap year. It is the
/// only molad in range before 1 January of year 1.
#[test]
fn test_molad_in_gregorian_year_0() -> Result<(), Box<dyn Error>> {
    assert_calendar_snapshots_match(DateTuple::new(3761, 10, 20), false, false, "molad in Gregorian year 0")
}

/// Every day of one year of each of the 14 kinds of year, in both regions. Together they reach
/// every parsha table and every rule that moves a holiday or fast.
#[test]
fn test_every_day_of_each_kind_of_year() -> Result<(), Box<dyn Error>> {
    let mut kinds = Vec::new();
    for year in 5700..=5800 {
        let hebrew_year = kosher_rust::calendar::HebrewYear::new(year).expect("in range");
        let rosh_hashana = hebrew_year.rosh_hashana().expect("in range");
        let kind = (hebrew_year.is_leap(), hebrew_year.length(), rosh_hashana.weekday());
        if kinds.contains(&kind) {
            continue;
        }
        kinds.push(kind);
        for month in 1..=13 {
            for day in 1..=30 {
                let date = DateTuple::new(year, month, day);
                if rust_hebrew_date(date).is_none() {
                    continue;
                }
                for in_israel in [false, true] {
                    assert_calendar_snapshots_match(date, in_israel, true, &format!("year kind {kind:?}"))?;
                }
            }
        }
    }
    assert_eq!(kinds.len(), 14);
    Ok(())
}

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
