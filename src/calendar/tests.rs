// Tests of private calendar arithmetic. Public behaviour is tested in tests/calendar.rs.

use icu_calendar::Date;

use super::molad::{CHALAKIM_PER_MONTH, chalakim_since_molad_tohu};
use super::month::*;
use super::parsha_tables::get_parsha_list;
use super::year::{HebrewYear, elapsed_days};

#[test]
fn elapsed_days_differ_by_the_year_length() {
    let this_year = elapsed_days(5784);
    let next_year = elapsed_days(5785);
    assert!(this_year > 0);
    assert_eq!(next_year - this_year, i64::from(HebrewYear::new(5784).unwrap().days()));
}

#[test]
fn chalakim_advance_one_lunar_month_per_month() {
    let tishrei = chalakim_since_molad_tohu(5784, TISHREI).unwrap();
    let cheshvan = chalakim_since_molad_tohu(5784, CHESHVAN).unwrap();
    assert_eq!(cheshvan - tishrei, CHALAKIM_PER_MONTH);
    assert_eq!(chalakim_since_molad_tohu(5785, ADARI), None);
}

#[test]
fn every_year_has_a_parsha_table() {
    for year in 5600..=5800 {
        let Ok(rosh_hashana) = Date::try_new_hebrew_v2(year, TISHREI, 1) else {
            continue;
        };
        for in_israel in [false, true] {
            assert!(
                get_parsha_list(&rosh_hashana, in_israel).is_some(),
                "year {year} should have a parsha table (israel={in_israel})"
            );
        }
    }
}

#[test]
fn icu_month_round_trip() {
    for month in HebrewMonth::LEAP_YEAR {
        assert_eq!(HebrewMonth::from_icu(month.to_icu()), Some(month));
    }
    assert_eq!(month_of_year(ADAR, 5784), Some(7));
    assert_eq!(month_of_year(ADAR, 5785), Some(6));
    assert_eq!(month_of_year(ADARI, 5785), None);
}

#[test]
fn extreme_years_do_not_overflow() {
    for year in [-20_000, 20_000, i32::MIN, i32::MAX] {
        let _ = elapsed_days(i64::from(year));
        let _ = HebrewYear::unchecked(year).is_leap();
        assert_eq!(HebrewYear::new(year), None);
    }
}
