//! Behaviour of the public calendar API. Conformance against KosherJava over thousands of random
//! dates lives in interop-tests/java_interop_tests.

#![allow(clippy::unwrap_used, clippy::expect_used, missing_docs)]

use icu_calendar::Date;
use jiff::civil::{Weekday, date};
use jiff::tz::TimeZone;
use kosher_rust::calendar::{
    DateOutOfRange, HebrewDate, HebrewMonth, HebrewYear, Parsha, Region, SpecialDay, YearLength,
};

fn hebrew(year: i32, month: HebrewMonth, day: u8) -> HebrewDate {
    HebrewDate::new(year, month, day).unwrap()
}

fn special_days(day: HebrewDate, region: Region) -> Vec<SpecialDay> {
    day.special_days(region).collect()
}

#[test]
fn converts_from_jiff_and_icu_dates() {
    let from_jiff = HebrewDate::try_from(date(2024, 4, 23)).unwrap();
    let from_icu = HebrewDate::try_from(Date::try_new_gregorian(2024, 4, 23).unwrap()).unwrap();
    let from_iso = HebrewDate::try_from(Date::try_new_iso(2024, 4, 23).unwrap()).unwrap();

    assert_eq!(from_jiff, hebrew(5784, HebrewMonth::Nisan, 15));
    assert_eq!(from_icu, from_jiff);
    assert_eq!(from_iso, from_jiff);
    assert_eq!(from_jiff.to_gregorian(), Some(date(2024, 4, 23)));
}

#[test]
fn reads_year_month_day_and_weekday() {
    let day = hebrew(5784, HebrewMonth::Tishrei, 1);
    assert_eq!(day.year(), HebrewYear::new(5784).unwrap());
    assert_eq!(day.month(), HebrewMonth::Tishrei);
    assert_eq!(day.day(), 1);
    assert_eq!(day.day_of_year(), 1);
    assert_eq!(day.weekday(), Weekday::Saturday);
}

#[test]
fn displays_in_english_with_adar_ii_in_leap_years() {
    assert_eq!(hebrew(5784, HebrewMonth::Nisan, 15).to_string(), "15 Nisan 5784");
    assert_eq!(hebrew(5784, HebrewMonth::AdarI, 14).to_string(), "14 Adar I 5784");
    assert_eq!(hebrew(5784, HebrewMonth::Adar, 14).to_string(), "14 Adar II 5784");
    assert_eq!(hebrew(5785, HebrewMonth::Adar, 14).to_string(), "14 Adar 5785");
}

#[test]
fn rejects_days_that_do_not_exist() {
    assert!(HebrewDate::new(5785, HebrewMonth::AdarI, 14).is_none());
    assert!(HebrewDate::new(5784, HebrewMonth::Iyar, 30).is_none());
    assert!(HebrewDate::new(5784, HebrewMonth::Tishrei, 0).is_none());
}

#[test]
fn adds_days_across_months() {
    let day = hebrew(5784, HebrewMonth::Tishrei, 30);
    assert_eq!(day.add_days(1), Some(hebrew(5784, HebrewMonth::Cheshvan, 1)));
    assert_eq!(day.add_days(-29), Some(hebrew(5784, HebrewMonth::Tishrei, 1)));
}

#[test]
fn year_facts() {
    assert!(HebrewYear::new(5784).unwrap().is_leap());
    assert!(!HebrewYear::new(5785).unwrap().is_leap());
    assert!(HebrewYear::new(5787).unwrap().is_leap());
    assert!(HebrewYear::new(5790).unwrap().is_leap());

    let year = HebrewYear::new(5784).unwrap();
    assert_eq!(year.days(), 383);
    assert_eq!(year.length(), YearLength::Chaserim);
    assert!(!year.is_cheshvan_long() && year.is_kislev_short());
    assert_eq!(year.months().len(), 13);
    assert_eq!(HebrewYear::new(5785).unwrap().months().len(), 12);
    assert_eq!(year.rosh_hashana(), Some(hebrew(5784, HebrewMonth::Tishrei, 1)));

    for year in 5700..5900 {
        let year = HebrewYear::new(year).unwrap();
        let expected = match year.length() {
            YearLength::Chaserim => 353,
            YearLength::Kesidran => 354,
            YearLength::Shelaimim => 355,
        } + if year.is_leap() { 30 } else { 0 };
        assert_eq!(year.days(), expected, "{year}");
        let sum: u16 = year.months().iter().map(|&m| u16::from(year.days_in(m).unwrap())).sum();
        assert_eq!(sum, year.days(), "{year}");
    }
}

#[test]
fn month_lengths() {
    assert_eq!(HebrewYear::new(5784).unwrap().days_in(HebrewMonth::Nisan), Some(30));
    assert_eq!(HebrewYear::new(5784).unwrap().days_in(HebrewMonth::Iyar), Some(29));
    assert_eq!(HebrewYear::new(5785).unwrap().days_in(HebrewMonth::Adar), Some(29));
    assert_eq!(HebrewYear::new(5784).unwrap().days_in(HebrewMonth::AdarI), Some(30));
    assert_eq!(HebrewYear::new(5785).unwrap().days_in(HebrewMonth::AdarI), None);
}

#[test]
fn month_names() {
    assert_eq!(HebrewMonth::Tishrei.hebrew(), "תשרי");
    assert_eq!(HebrewMonth::Cheshvan.to_string(), "Cheshvan");
    assert_eq!(HebrewMonth::AdarI.to_string(), "Adar I");
}

#[test]
fn yamim_noraim() {
    let rosh_hashana = hebrew(5784, HebrewMonth::Tishrei, 1);
    assert!(special_days(rosh_hashana, Region::Diaspora).contains(&SpecialDay::RoshHashana));
    assert!(rosh_hashana.is_assur_bemelacha(Region::Diaspora));

    let yom_kippur = hebrew(5784, HebrewMonth::Tishrei, 10);
    assert!(special_days(yom_kippur, Region::Diaspora).contains(&SpecialDay::YomKippur));
    assert!(yom_kippur.is_assur_bemelacha(Region::Israel));

    assert!(hebrew(5784, HebrewMonth::Tishrei, 5).is_aseres_yemei_teshuva());
    assert!(!hebrew(5784, HebrewMonth::Tishrei, 11).is_aseres_yemei_teshuva());
}

#[test]
fn second_day_of_yom_tov_only_in_the_diaspora() {
    let day = hebrew(5784, HebrewMonth::Nisan, 16);

    assert!(special_days(day, Region::Diaspora).contains(&SpecialDay::Pesach));
    assert!(day.is_assur_bemelacha(Region::Diaspora));

    assert!(special_days(day, Region::Israel).contains(&SpecialDay::CholHamoedPesach));
    assert!(!day.is_assur_bemelacha(Region::Israel));
}

#[test]
fn chanukah_purim_and_fasts() {
    let chanukah = hebrew(5784, HebrewMonth::Kislev, 25);
    assert!(special_days(chanukah, Region::Diaspora).contains(&SpecialDay::Chanukah(1)));
    assert!(!chanukah.is_assur_bemelacha(Region::Diaspora));

    assert!(special_days(hebrew(5784, HebrewMonth::Adar, 14), Region::Diaspora).contains(&SpecialDay::Purim));
    assert!(special_days(hebrew(5784, HebrewMonth::AdarI, 14), Region::Diaspora).contains(&SpecialDay::PurimKatan));

    let teves = hebrew(5784, HebrewMonth::Teves, 10);
    assert!(special_days(teves, Region::Diaspora).iter().any(|h| h.is_fast_day()));
}

#[test]
fn omer_count() {
    let first = special_days(hebrew(5784, HebrewMonth::Nisan, 16), Region::Diaspora);
    assert!(first.contains(&SpecialDay::CountOfTheOmer(1)));

    let chanukah = special_days(hebrew(5784, HebrewMonth::Kislev, 25), Region::Diaspora);
    assert!(!chanukah.iter().any(|h| matches!(h, SpecialDay::CountOfTheOmer(_))));
}

#[test]
fn rosh_chodesh() {
    assert!(special_days(hebrew(5784, HebrewMonth::Cheshvan, 1), Region::Diaspora).contains(&SpecialDay::RoshChodesh));
    assert!(special_days(hebrew(5784, HebrewMonth::Tishrei, 30), Region::Diaspora).contains(&SpecialDay::RoshChodesh));
}

#[test]
fn modern_holidays_are_yielded_and_flagged() {
    let yom_yerushalayim = special_days(hebrew(5784, HebrewMonth::Iyar, 28), Region::Israel);
    assert!(yom_yerushalayim.contains(&SpecialDay::YomYerushalayim));
    assert!(SpecialDay::YomYerushalayim.is_modern());
    assert!(!SpecialDay::Pesach.is_modern());
}

#[test]
fn candle_lighting_on_erev_shabbos_and_yom_tov() {
    let friday = HebrewDate::try_from(date(2024, 4, 26)).unwrap();
    assert_eq!(friday.weekday(), Weekday::Friday);
    assert!(friday.has_candle_lighting(Region::Diaspora));

    let erev_pesach = hebrew(5784, HebrewMonth::Nisan, 14);
    assert!(erev_pesach.has_candle_lighting(Region::Israel));

    let wednesday = HebrewDate::try_from(date(2024, 5, 1)).unwrap();
    assert!(!wednesday.has_candle_lighting(Region::Diaspora));
}

#[test]
fn weekly_parsha() {
    let bo = hebrew(5784, HebrewMonth::Shevat, 10);
    assert_eq!(bo.weekday(), Weekday::Saturday);
    assert_eq!(bo.parsha(Region::Diaspora), Some(Parsha::Bo));

    assert_eq!(
        HebrewDate::try_from(date(2024, 5, 4)).unwrap().parsha(Region::Israel),
        Some(Parsha::AchreiMos)
    );

    // No parsha on a weekday.
    assert_eq!(
        HebrewDate::try_from(date(2024, 5, 1)).unwrap().parsha(Region::Diaspora),
        None
    );

    // Shabbos Chol Hamoed Succos 5784 has the Yom Tov reading.
    let chol_hamoed = HebrewDate::try_from(date(2023, 10, 7)).unwrap();
    assert_eq!(chol_hamoed.weekday(), Weekday::Saturday);
    assert_eq!(chol_hamoed.parsha(Region::Diaspora), None);
}

#[test]
fn next_parsha_skips_yom_tov() {
    // Tuesday 15 Nisan 5784: the next Shabbos is Chol Hamoed, so the next reading is Achrei Mos.
    let pesach = HebrewDate::try_from(date(2024, 4, 23)).unwrap();
    assert_eq!(pesach.next_parsha(Region::Diaspora), Some(Parsha::AchreiMos));
}

#[test]
fn special_shabbosos() {
    let shekalim = hebrew(5785, HebrewMonth::Adar, 1);
    assert_eq!(shekalim.weekday(), Weekday::Saturday);
    assert_eq!(shekalim.special_parsha(Region::Diaspora), Some(Parsha::Shekalim));

    let zachor = hebrew(5784, HebrewMonth::Adar, 13);
    assert_eq!(zachor.weekday(), Weekday::Saturday);
    assert_eq!(zachor.special_parsha(Region::Diaspora), Some(Parsha::Zachor));
}

#[test]
fn names_in_english_and_hebrew() {
    assert_eq!(SpecialDay::RoshHashana.to_string(), "Rosh Hashana");
    assert_eq!(SpecialDay::RoshHashana.hebrew(), "ראש השנה");
    assert_eq!(SpecialDay::YomKippurKatan.hebrew(), "יום כיפור קטן");
    assert_eq!(SpecialDay::Behab.hebrew(), "בה'ב");
    assert_eq!(SpecialDay::YomHaatzmaut.hebrew(), "יום העצמאות");

    assert_eq!(Parsha::Bereshis.to_string(), "Bereshis");
    assert_eq!(Parsha::Bereshis.hebrew(), "בראשית");
    assert_eq!(Parsha::VayakhelPekudei.to_string(), "Vayakhel-Pekudei");
}

#[test]
fn molad_of_tishrei_5784_was_friday_morning() {
    // Published as Friday 5:49 and 3 chalakim, Jerusalem mean time (about 6:28 Israel daylight time).
    let molad = hebrew(5784, HebrewMonth::Tishrei, 1).molad().unwrap();
    let jerusalem = molad.timestamp().to_zoned(TimeZone::get("Asia/Jerusalem").unwrap());
    assert_eq!(jerusalem.date(), date(2023, 9, 15));
    assert_eq!((jerusalem.hour(), jerusalem.minute()), (6, 28));
}

#[test]
fn extreme_dates_do_not_panic() {
    let samples = [
        (HebrewMonth::Tishrei, 1),
        (HebrewMonth::Tishrei, 30),
        (HebrewMonth::Teves, 18),
        (HebrewMonth::Nisan, 15),
        (HebrewMonth::Adar, 14),
        (HebrewMonth::AdarI, 1),
        (HebrewMonth::Kislev, 25),
        (HebrewMonth::Elul, 29),
    ];
    for year in [3761, 3762, 9998, 9999] {
        let y = HebrewYear::new(year).unwrap();
        let _ = (y.days(), y.length(), y.is_leap(), y.rosh_hashana());
        for &month in y.months() {
            let _ = y.days_in(month);
        }
        for (month, day) in samples {
            if let Some(day) = HebrewDate::new(year, month, day) {
                exercise(day);
            }
        }
    }
}

#[test]
fn covers_1_january_of_year_1_through_the_end_of_9999() {
    let first = HebrewDate::try_from(date(1, 1, 1)).unwrap();
    assert_eq!(first, hebrew(3761, HebrewMonth::Teves, 18));
    assert_eq!(first.add_days(-1), None);
    assert_eq!(HebrewDate::try_from(date(0, 12, 31)), Err(DateOutOfRange));
    assert_eq!(HebrewDate::try_from(jiff::civil::Date::MIN), Err(DateOutOfRange));
    assert_eq!(HebrewDate::new(3761, HebrewMonth::Teves, 17), None);

    let last = hebrew(9999, HebrewMonth::Elul, 29);
    assert_eq!(last.add_days(1), None);
    let after = last.to_gregorian().unwrap().tomorrow().unwrap();
    assert_eq!(HebrewDate::try_from(after), Err(DateOutOfRange));
    assert_eq!(HebrewDate::try_from(jiff::civil::Date::MAX), Err(DateOutOfRange));

    assert_eq!(HebrewYear::new(3760), None);
    assert_eq!(HebrewYear::new(10000), None);
    assert_eq!(HebrewYear::new(3761).unwrap().rosh_hashana(), None);
}

fn exercise(day: HebrewDate) {
    for region in [Region::Israel, Region::Diaspora] {
        let _ = day.special_days(region).count();
        let _ = day.is_assur_bemelacha(region);
        let _ = day.has_candle_lighting(region);
        let _ = day.parsha(region);
        let _ = day.special_parsha(region);
        let _ = day.next_parsha(region);
    }
    let _ = (
        day.is_aseres_yemei_teshuva(),
        day.molad(),
        day.to_gregorian(),
        day.to_string(),
    );
}
