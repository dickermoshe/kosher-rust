//! Assertions for Hebrew-date Java parity tests.

use std::error::Error;

use super::{
    java_reference::{
        java_add_days_to_jewish_date, java_add_months_to_jewish_date, java_add_years_to_jewish_date,
        java_add_years_to_jewish_date_with_adar, java_jewish_calendar_snapshot, java_jewish_date_snapshot,
        java_jewish_date_to_gregorian_date, java_jewish_year_snapshot, java_minus_days_to_jewish_date,
    },
    policy,
    rust_reference::{
        rust_add_days_to_jewish_date, rust_add_months_to_jewish_date, rust_add_years_to_jewish_date_with_adar,
        rust_jewish_calendar_snapshot, rust_jewish_date_snapshot, rust_jewish_date_to_gregorian_date,
        rust_jewish_year_snapshot,
    },
    types::DateTuple,
};

pub(super) fn assert_jewish_date_operations(date: DateTuple, context: &str) -> Result<(), Box<dyn Error>> {
    let java_gregorian = java_jewish_date_to_gregorian_date(date)?;
    let rust_gregorian = rust_jewish_date_to_gregorian_date(date);
    assert_date_results_match(date, "Gregorian", java_gregorian, rust_gregorian, context);
    if java_gregorian.is_none() && rust_gregorian.is_none() {
        return Ok(());
    }

    assert_jewish_date_snapshots_match(date, context)?;

    for &day_offset in policy::DAY_OFFSETS {
        let java = if day_offset >= 0 {
            java_add_days_to_jewish_date(date, day_offset)?
        } else {
            java_minus_days_to_jewish_date(date, -day_offset)?
        };
        assert_date_results_match(
            date,
            &format!("Jewish after adding {day_offset} days"),
            java,
            rust_add_days_to_jewish_date(date, day_offset),
            context,
        );
    }

    for &month_offset in policy::MONTH_OFFSETS {
        assert_date_results_match(
            date,
            &format!("Jewish after adding {month_offset} months"),
            java_add_months_to_jewish_date(date, month_offset)?,
            rust_add_months_to_jewish_date(date, month_offset),
            context,
        );
    }

    for &year_offset in policy::YEAR_OFFSETS {
        assert_date_results_match(
            date,
            &format!("Jewish after adding {year_offset} years"),
            java_add_years_to_jewish_date(date, year_offset)?,
            rust_add_years_to_jewish_date_with_adar(date, year_offset, false),
            context,
        );
        assert_date_results_match(
            date,
            &format!("Jewish after adding {year_offset} years (adar aleph)"),
            java_add_years_to_jewish_date_with_adar(date, year_offset, true)?,
            rust_add_years_to_jewish_date_with_adar(date, year_offset, true),
            context,
        );
    }

    for in_israel in [false, true] {
        for use_modern_holidays in [false, true] {
            assert_calendar_snapshots_match(date, in_israel, use_modern_holidays, context)?;
        }
    }

    Ok(())
}

pub(super) fn assert_calendar_snapshots_match(
    date: DateTuple,
    in_israel: bool,
    use_modern_holidays: bool,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let java = java_jewish_calendar_snapshot(date, in_israel, use_modern_holidays)?
        .unwrap_or_else(|| panic!("Java could not produce JewishCalendar snapshot for {date:?}"));
    let rust = rust_jewish_calendar_snapshot(date, in_israel, use_modern_holidays)
        .unwrap_or_else(|| panic!("Rust could not produce JewishCalendar snapshot for {date:?}"));

    assert_eq!(
        rust, java,
        "JewishCalendar snapshot mismatch for {date:?} in_israel={in_israel} use_modern_holidays={use_modern_holidays}\n{context}"
    );
    Ok(())
}

pub(super) fn assert_jewish_date_snapshots_match(date: DateTuple, context: &str) -> Result<(), Box<dyn Error>> {
    let java = java_jewish_date_snapshot(date)?
        .unwrap_or_else(|| panic!("Java could not produce JewishDate snapshot for {date:?}"));
    let rust = rust_jewish_date_snapshot(date)
        .unwrap_or_else(|| panic!("Rust could not produce JewishDate snapshot for {date:?}"));
    assert_eq!(rust, java, "JewishDate snapshot mismatch for {date:?}\n{context}");
    Ok(())
}

pub(super) fn assert_jewish_year_snapshot_matches(year: i32, context: &str) -> Result<(), Box<dyn Error>> {
    let java = java_jewish_year_snapshot(year)?
        .unwrap_or_else(|| panic!("Java could not produce Jewish year snapshot for {year}"));
    let rust = rust_jewish_year_snapshot(year)
        .unwrap_or_else(|| panic!("Rust could not produce Jewish year snapshot for {year}"));
    assert_eq!(rust, java, "Jewish year snapshot mismatch for year={year}\n{context}");
    Ok(())
}

pub(super) fn assert_date_results_match(
    input_date: DateTuple,
    target_date_type: &str,
    java_date: Option<DateTuple>,
    rust_date: Option<DateTuple>,
    context: &str,
) {
    assert_eq!(
        rust_date, java_date,
        "{target_date_type} date mismatch for {input_date:?}: java={java_date:?} rust={rust_date:?}\n{context}"
    );
}
