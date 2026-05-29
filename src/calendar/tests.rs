use crate::calendar::prelude::*;

use super::*;
use icu_calendar::{Date, cal::Hebrew};
extern crate std;
use std::format;
use std::vec::Vec;

#[test]
fn test_hebrew_leap_year() {
    // Test leap years in 19-year cycle
    assert!(Hebrew::is_hebrew_leap_year(5784)); // Year 3 in cycle
    assert!(!Hebrew::is_hebrew_leap_year(5785));
    assert!(Hebrew::is_hebrew_leap_year(5787)); // Year 6 in cycle
    assert!(Hebrew::is_hebrew_leap_year(5790)); // Year 8 in cycle
}

#[test]
fn test_days_in_year() {
    let days = Hebrew::days_in_hebrew_year(5784).expect("valid year length");
    assert!((353..=385).contains(&days), "Year length out of range: {}", days);
}

#[test]
fn test_days_in_month() {
    // Regular month - always 30 days
    assert_eq!(Hebrew::days_in_hebrew_month(5784, NISAN), Some(30));

    // Regular month - always 29 days
    assert_eq!(Hebrew::days_in_hebrew_month(5784, IYYAR), Some(29));

    // Adar in non-leap year is 29 days
    assert_eq!(Hebrew::days_in_hebrew_month(5785, ADAR), Some(29));

    // Adar I in leap year is 30 days
    assert_eq!(Hebrew::days_in_hebrew_month(5784, ADARI), Some(30));

    // Adar I is not valid in a non-leap year
    assert_eq!(Hebrew::days_in_hebrew_month(5785, ADARI), None);
}

#[test]
fn test_hebrew_month_enum() {
    let month = TISHREI;
    assert_eq!(month.he(5784), Some("תשרי"));
}

#[test]
fn test_from_hebrew_date() {
    // Create Rosh Hashana 5784
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 1).unwrap();
    assert_eq!(date.day_of_month().0, 1);
    assert_eq!(date.input_month(), TISHREI);
    assert_eq!(date.year().extended_year(), 5784);
}

#[test]
fn test_rosh_hashana_holiday() {
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 1).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.contains(&&Holiday::RoshHashana));
    assert!(date.is_assur_bemelacha(false));
}

#[test]
fn holiday_methods_work_on_jiff_dates() {
    let date = jiff::civil::date(2025, 4, 13);

    assert!(date.holidays(false, false).any(|holiday| *holiday == Holiday::Pesach));
    assert!(date.is_assur_bemelacha(false));
}

#[test]
fn holiday_methods_work_on_icu_gregorian_dates() {
    let date = Date::try_new_gregorian(2025, 4, 13).unwrap();

    assert!(date.holidays(false, false).any(|holiday| *holiday == Holiday::Pesach));
    assert!(date.is_assur_bemelacha(false));
}

#[test]
fn jiff_and_icu_gregorian_match_hebrew_date_methods() {
    let hebrew = Date::try_new_hebrew_v2(5785, NISAN, 15).unwrap();
    let gregorian = Date::try_new_gregorian(2025, 4, 13).unwrap();
    let jiff = jiff::civil::date(2025, 4, 13);

    assert_eq!(gregorian.hebrew_date(), hebrew);
    assert_eq!(jiff.hebrew_date(), hebrew);
    assert_eq!(gregorian.gregorian_date(), hebrew.gregorian_date());
    assert_eq!(jiff.gregorian_date(), hebrew.gregorian_date());
    assert_eq!(gregorian.input_month(), hebrew.input_month());
    assert_eq!(jiff.input_month(), hebrew.input_month());
    assert_eq!(
        gregorian.holidays(false, false).copied().collect::<Vec<_>>(),
        hebrew.holidays(false, false).copied().collect::<Vec<_>>()
    );
    assert_eq!(
        jiff.holidays(false, false).copied().collect::<Vec<_>>(),
        hebrew.holidays(false, false).copied().collect::<Vec<_>>()
    );
    assert_eq!(gregorian.has_candle_lighting(false), hebrew.has_candle_lighting(false));
    assert_eq!(jiff.has_candle_lighting(false), hebrew.has_candle_lighting(false));
    assert_eq!(gregorian.is_aseres_yemei_teshuva(), hebrew.is_aseres_yemei_teshuva());
    assert_eq!(jiff.is_aseres_yemei_teshuva(), hebrew.is_aseres_yemei_teshuva());
    assert_eq!(gregorian.day_of_chanukah(), hebrew.day_of_chanukah());
    assert_eq!(jiff.day_of_chanukah(), hebrew.day_of_chanukah());
    assert_eq!(gregorian.day_of_the_omer(), hebrew.day_of_the_omer());
    assert_eq!(jiff.day_of_the_omer(), hebrew.day_of_the_omer());
}

#[test]
fn parsha_methods_work_on_non_hebrew_date_types() {
    let hebrew = Date::try_new_hebrew_v2(5784, SHEVAT, 10).unwrap();
    let gregorian = Date::try_new_gregorian(2024, 1, 20).unwrap();
    let iso = Date::try_new_iso(2024, 1, 20).unwrap();
    let jiff = jiff::civil::date(2024, 1, 20);

    assert_eq!(gregorian.todays_parsha(false), hebrew.todays_parsha(false));
    assert_eq!(iso.todays_parsha(false), hebrew.todays_parsha(false));
    assert_eq!(jiff.todays_parsha(false), hebrew.todays_parsha(false));
    assert_eq!(gregorian.special_parsha(false), hebrew.special_parsha(false));
    assert_eq!(iso.special_parsha(false), hebrew.special_parsha(false));
    assert_eq!(jiff.special_parsha(false), hebrew.special_parsha(false));
    assert_eq!(gregorian.upcoming_parsha(false), hebrew.upcoming_parsha(false));
    assert_eq!(iso.upcoming_parsha(false), hebrew.upcoming_parsha(false));
    assert_eq!(jiff.upcoming_parsha(false), hebrew.upcoming_parsha(false));
}

#[test]
fn test_yom_kippur() {
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 10).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.contains(&&Holiday::YomKippur));
    assert!(date.is_assur_bemelacha(false));
}

#[test]
fn test_chanukah() {
    // First day of Chanukah
    let date = Date::try_new_hebrew_v2(5784, KISLEV, 25).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.contains(&&Holiday::Chanukah));
    assert!(!date.is_assur_bemelacha(false)); // Work is permitted
}

#[test]
fn test_purim() {
    let date = Date::try_new_hebrew_v2(5784, ADAR, 14).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.contains(&&Holiday::Purim));
}

#[test]
fn test_pesach_israel_vs_diaspora() {
    // Second day of Pesach - Yom Tov in diaspora, Chol Hamoed in Israel
    let date = Date::try_new_hebrew_v2(5784, NISAN, 16).unwrap();

    // In diaspora
    let holidays_diaspora: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays_diaspora.contains(&&Holiday::Pesach));
    assert!(date.is_assur_bemelacha(false));

    // In Israel
    let holidays_israel: Vec<_> = date.holidays(true, false).collect();
    assert!(holidays_israel.contains(&&Holiday::CholHamoedPesach));
    assert!(!date.is_assur_bemelacha(true));
}

#[test]
fn test_candle_lighting() {
    // Erev Shabbat (Friday)
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 6).unwrap();
    if date.weekday() == Weekday::Friday {
        assert!(date.has_candle_lighting(false));
    }
}

#[test]
fn test_aseres_yemei_teshuva() {
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 5).unwrap();
    assert!(date.is_aseres_yemei_teshuva());

    let date = Date::try_new_hebrew_v2(5784, TISHREI, 11).unwrap();
    assert!(!date.is_aseres_yemei_teshuva());
}

#[test]
fn test_modern_holidays() {
    let date = Date::try_new_hebrew_v2(5784, IYYAR, 28).unwrap();

    // Without modern holidays
    let holidays_traditional: Vec<_> = date.holidays(true, false).collect();
    assert!(!holidays_traditional.iter().any(|h| h.is_modern_holiday()));

    // With modern holidays
    let holidays_modern: Vec<_> = date.holidays(true, true).collect();
    assert!(holidays_modern.contains(&&Holiday::YomYerushalayim));
}

#[test]
fn test_fast_days() {
    let date = Date::try_new_hebrew_v2(5784, TEVET, 10).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.iter().any(|h| h.is_fast_day()));
}

#[test]
fn test_rosh_chodesh() {
    // First day of month
    let date = Date::try_new_hebrew_v2(5784, ḤESHVAN, 1).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.contains(&&Holiday::RoshChodesh));

    // 30th of month (second day of Rosh Chodesh)
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 30).unwrap();
    let holidays: Vec<_> = date.holidays(false, false).collect();
    assert!(holidays.contains(&&Holiday::RoshChodesh));
}

#[test]
fn test_kviah_types() {
    let year = 5784;
    let kviah = Hebrew::cheshvan_kislev_kviah(year);

    // Should be one of the three types
    assert!(matches!(
        kviah,
        Some(YearLengthType::Chaserim | YearLengthType::Kesidran | YearLengthType::Shelaimim)
    ));

    // Test consistency
    let is_cheshvan_long = Hebrew::is_cheshvan_long(year).expect("valid cheshvan length");
    let is_kislev_short = Hebrew::is_kislev_short(year).expect("valid kislev length");

    match kviah {
        Some(YearLengthType::Shelaimim) => {
            assert!(is_cheshvan_long && !is_kislev_short);
        }
        Some(YearLengthType::Chaserim) => {
            assert!(!is_cheshvan_long && is_kislev_short);
        }
        Some(YearLengthType::Kesidran) => {
            assert!(!is_cheshvan_long && !is_kislev_short);
        }
        None => panic!("valid kviah"),
    }
}

#[test]
fn test_parsha_on_shabbat() {
    // Create multiple dates and check parsha
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 21).unwrap();

    // Only returns parsha on Shabbat
    if date.weekday() == Weekday::Saturday {
        let parsha = date.todays_parsha(false);
        assert!(parsha.is_some() || date.holidays(false, false).count() > 0);
    } else {
        assert_eq!(date.todays_parsha(false), None);
    }
}

#[test]
fn valid_hebrew_years_have_parsha_list() {
    use super::parsha::get_parsha_list;

    for year in 5600..=5800 {
        let Ok(rh) = Date::try_new_hebrew_v2(year, TISHREI, 1) else {
            continue;
        };
        for in_israel in [false, true] {
            assert!(
                get_parsha_list(&rh, in_israel).is_some(),
                "year {year} should have a parsha list (israel={in_israel})"
            );
        }
    }
}

#[test]
fn test_parsha_golden_bo_5784() {
    // Shevat 10, 5784 is Shabbat; diaspora reads Bo (KosherJava / hebcal).
    let date = Date::try_new_hebrew_v2(5784, SHEVAT, 10).unwrap();
    assert_eq!(date.weekday(), Weekday::Saturday);
    assert_eq!(date.todays_parsha(false), Some(Parsha::Bo));
}

#[test]
fn test_special_parsha_shekalim_and_zachor() {
    // Non-leap year: Shekalim on Adar 1.
    let shekalim = Date::try_new_hebrew_v2(5785, ADAR, 1).unwrap();
    assert_eq!(shekalim.weekday(), Weekday::Saturday);
    assert_eq!(shekalim.special_parsha(false), Some(Parsha::Shekalim));

    // Leap year: Zachor on Adar II 13.
    let zachor = Date::try_new_hebrew_v2(5784, ADAR, 13).unwrap();
    assert_eq!(zachor.weekday(), Weekday::Saturday);
    assert_eq!(zachor.special_parsha(false), Some(Parsha::Zachor));
}

#[test]
fn test_upcoming_parsha() {
    let date = Date::try_new_hebrew_v2(5784, TISHREI, 15).unwrap();
    let upcoming = date.upcoming_parsha(false).expect("upcoming parsha");
    assert!(!upcoming.he().is_empty());
}

#[test]
fn test_holiday_hebrew_names() {
    assert_eq!(Holiday::YomKippurKatan.he(), "יום כיפור קטן");
    assert_eq!(Holiday::Behab.he(), "בה'ב");
    assert_eq!(Holiday::YomHaatzmaut.he(), "יום העצמאות");
}

#[test]
fn test_day_of_chanukah_and_omer() {
    let chanukah = Date::try_new_hebrew_v2(5784, KISLEV, 25).unwrap();
    assert_eq!(chanukah.day_of_chanukah(), Some(1));
    assert_eq!(chanukah.day_of_the_omer(), None);

    let omer = Date::try_new_hebrew_v2(5784, NISAN, 16).unwrap();
    assert_eq!(omer.day_of_the_omer(), Some(1));
}

#[test]
fn test_hebrew_month_for_adar() {
    let non_leap_adar = Date::try_new_hebrew_v2(5783, ADAR, 24).expect("valid non-leap Adar date");
    assert_eq!(non_leap_adar.input_month(), ADAR);

    let leap_adar_i = Date::try_new_hebrew_v2(5784, ADARI, 14).expect("valid leap-year Adar I date");
    assert_eq!(leap_adar_i.input_month(), ADARI);

    let leap_adar_ii = Date::try_new_hebrew_v2(5784, ADAR, 14).expect("valid leap-year Adar II date");
    assert_eq!(leap_adar_ii.input_month(), ADAR);

    assert!(Date::try_new_hebrew_v2(5783, ADARI, 14).is_err());
}

#[test]
fn test_holiday_display() {
    let holiday = Holiday::RoshHashana;
    assert_eq!(format!("{}", holiday), "ראש השנה");
    assert_eq!(holiday.he(), "ראש השנה");
}

#[test]
fn test_parsha_display() {
    let parsha = Parsha::Bereshis;
    assert_eq!(format!("{}", parsha), "בראשית");
    assert_eq!(parsha.he(), "בראשית");
}

#[test]
fn test_hebrew_elapsed_days() {
    // Basic sanity check - should be positive
    let elapsed = get_hebrew_elapsed_days(5784).expect("valid elapsed days");
    assert!(elapsed > 0);

    // Next year should have more elapsed days
    let next_elapsed = get_hebrew_elapsed_days(5785).expect("valid next elapsed days");
    assert!(next_elapsed > elapsed);

    // Difference should be the number of days in the year
    let year_length = Hebrew::days_in_hebrew_year(5784).expect("valid year length");
    assert_eq!(next_elapsed - elapsed, year_length);
}

#[test]
fn test_chalakim_calculations() {
    // Test that chalakim are calculated consistently
    let chalakim1 = chalakim_since_molad_tohu(5784, TISHREI).expect("valid tishrei chalakim");
    let chalakim2 = chalakim_since_molad_tohu(5784, ḤESHVAN).expect("valid cheshvan chalakim");

    // Next month should have more chalakim
    assert!(chalakim2 > chalakim1);

    // Difference should be approximately one month
    let diff = chalakim2 - chalakim1;
    assert!((diff - CHALAKIM_PER_MONTH).abs() < 100);
}

const EXTREME_YEAR_MIN: i32 = -20_000;
const EXTREME_YEAR_MAX: i32 = 20_000;

/// Exercises every [`HebrewHolidayCalendar`] method; must not panic for any input date.
fn exercise_hebrew_holiday_calendar<D: HebrewHolidayCalendar>(date: &D) {
    let _ = date.hebrew_date();
    let _ = date.gregorian_date();
    let _ = date.input_month();
    let _ = date.is_assur_bemelacha(false);
    let _ = date.is_assur_bemelacha(true);
    let _ = date.has_candle_lighting(false);
    let _ = date.has_candle_lighting(true);
    let _ = date.is_aseres_yemei_teshuva();
    let _ = date.todays_parsha(false);
    let _ = date.todays_parsha(true);
    let _ = date.special_parsha(false);
    let _ = date.special_parsha(true);
    let _ = date.upcoming_parsha(false);
    let _ = date.upcoming_parsha(true);
    let _ = date.day_of_chanukah();
    let _ = date.day_of_the_omer();
    let _ = date.holidays(false, false).count();
    let _ = date.holidays(true, false).count();
    let _ = date.holidays(false, true).count();
    let _ = date.holidays(true, true).count();
}

fn exercise_hebrew_year_metadata(year: i32) {
    let _ = Hebrew::days_in_hebrew_year(year);
    let _ = Hebrew::is_hebrew_leap_year(year);
    let _ = Hebrew::is_cheshvan_long(year);
    let _ = Hebrew::is_kislev_short(year);
    let _ = Hebrew::cheshvan_kislev_kviah(year);
    let _ = get_hebrew_elapsed_days(year);
    let _ = get_hebrew_elapsed_days(year + 1);
    for month in Month::hebrew_months_in_year(year) {
        let _ = Hebrew::days_in_hebrew_month(year, *month);
        let _ = month.he(year);
        let _ = month.hebrew_month_of_year(year);
        let _ = chalakim_since_molad_tohu(year, *month);
    }
    let _ = TISHREI.he(year);
    let _ = ADARI.he(year);
}

#[test]
fn extreme_hebrew_years_do_not_panic() {
    exercise_hebrew_year_metadata(EXTREME_YEAR_MIN);
    exercise_hebrew_year_metadata(EXTREME_YEAR_MAX);
}

#[test]
fn extreme_gregorian_dates_do_not_panic() {
    const GREGORIAN_SAMPLES: &[(u8, u8)] = &[(1, 1), (2, 29), (6, 15), (12, 31), (12, 28), (7, 0)];

    for year in [EXTREME_YEAR_MIN, EXTREME_YEAR_MAX] {
        for &(month, day) in GREGORIAN_SAMPLES {
            if day == 0 {
                continue;
            }
            let Ok(gregorian) = Date::<Gregorian>::try_new_gregorian(year, month, day) else {
                continue;
            };
            exercise_hebrew_holiday_calendar(&gregorian);

            if year >= i16::MIN as i32 && year <= i16::MAX as i32 {
                let jiff = jiff::civil::date(year as i16, month as i8, day as i8);
                exercise_hebrew_holiday_calendar(&jiff);
            }
        }
    }
}

#[test]
fn extreme_hebrew_dates_do_not_panic() {
    const HEBREW_SAMPLES: &[(Month, u8)] = &[
        (TISHREI, 1),
        (TISHREI, 30),
        (NISAN, 15),
        (ADAR, 14),
        (ADARI, 1),
        (KISLEV, 25),
        (TISHREI, 0),
        (TISHREI, 32),
    ];

    for year in [EXTREME_YEAR_MIN, EXTREME_YEAR_MAX] {
        for &(month, day) in HEBREW_SAMPLES {
            let Ok(hebrew) = Date::<Hebrew>::try_new_hebrew_v2(year, month, day) else {
                continue;
            };
            exercise_hebrew_holiday_calendar(&hebrew);
        }
    }
}
