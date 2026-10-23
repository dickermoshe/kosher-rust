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
    assert_eq!(from_jiff.to_gregorian(), date(2024, 4, 23));
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
fn candle_lighting_on_the_last_day_of_the_range() {
    // 29 Elul 9999 is Erev Rosh Hashana, though Rosh Hashana is past the range.
    let last_day = hebrew(9999, HebrewMonth::Elul, 29);
    assert_eq!(last_day.add_days(1), None);
    assert!(last_day.has_candle_lighting(Region::Israel));
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
    assert_eq!(SpecialDay::YomHaAtzmaut.hebrew(), "יום העצמאות");

    assert_eq!(Parsha::Bereshis.to_string(), "Bereshis");
    assert_eq!(Parsha::Bereshis.hebrew(), "בראשית");
    assert_eq!(Parsha::VayakhelPekudei.to_string(), "Vayakhel-Pekudei");
}

#[test]
fn molad_of_tishrei_5784_was_friday_morning() {
    // Announced as Friday 5:49 and 0 chalakim, Jerusalem mean time (about 6:28 Israel daylight time).
    let molad = hebrew(5784, HebrewMonth::Tishrei, 1).molad();
    let jerusalem = molad.timestamp().to_zoned(TimeZone::get("Asia/Jerusalem").unwrap());
    assert_eq!(jerusalem.date(), date(2023, 9, 15));
    assert_eq!((jerusalem.hour(), jerusalem.minute()), (6, 28));
    assert_eq!(molad.weekday(), Weekday::Friday);
    assert_eq!((molad.hours(), molad.minutes(), molad.chalakim()), (5, 49, 0));
}

#[test]
fn molad_of_teves_3761_is_in_gregorian_year_0() {
    // Tuesday 20:07 and 2 chalakim, Jerusalem mean time, on 12 December of year 0, a leap year.
    let molad = hebrew(3761, HebrewMonth::Teves, 20).molad();
    assert_eq!(molad.weekday(), Weekday::Tuesday);
    assert_eq!((molad.hours(), molad.minutes(), molad.chalakim()), (20, 7, 2));
    assert_eq!(molad.timestamp().to_zoned(TimeZone::UTC).date(), date(0, 12, 12));
}

#[test]
fn every_month_has_its_own_molad() {
    // The molad falls within three days of the first of its month.
    let check = |day: HebrewDate| {
        let first = day.to_gregorian() - jiff::Span::new().days(i64::from(day.day()) - 1);
        let molad = day.molad().timestamp().to_zoned(TimeZone::UTC).date();
        let days = first.until(molad).unwrap().get_days().abs();
        assert!(days <= 3, "{day}: molad on {molad}");
    };
    let mut day = HebrewDate::try_from(date(1, 1, 1)).unwrap();
    while let Some(next) = day.add_days(29) {
        check(day);
        day = next;
    }
    check(hebrew(9999, HebrewMonth::Elul, 29));
}

#[test]
fn molad_parts_agree_with_its_timestamp() {
    // Jerusalem mean time is 2 hours, 20 minutes and 56.496 seconds ahead of UTC.
    let mean_time = jiff::tz::TimeZone::fixed(jiff::tz::offset(2));
    let ahead = jiff::SignedDuration::from_millis(20 * 60_000 + 56_496);
    for year in [3762, 5000, 5784, 9999] {
        let months = if HebrewYear::new(year).unwrap().is_leap() {
            HebrewMonth::LEAP_YEAR.as_slice()
        } else {
            HebrewMonth::COMMON_YEAR.as_slice()
        };
        for &month in months {
            let molad = hebrew(year, month, 1).molad();
            let local = (molad.timestamp() + ahead).to_zoned(mean_time.clone());
            let chalakim = (local.second() as f64 + local.subsec_nanosecond() as f64 / 1e9) * 0.3;
            assert_eq!(molad.weekday(), local.weekday(), "{year} {month}");
            assert_eq!(i8::try_from(molad.hours()).unwrap(), local.hour(), "{year} {month}");
            assert_eq!(i8::try_from(molad.minutes()).unwrap(), local.minute(), "{year} {month}");
            assert_eq!(f64::from(molad.chalakim()), chalakim.round(), "{year} {month}");
        }
    }
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
    let after = last.to_gregorian().tomorrow().unwrap();
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

#[test]
fn names_parse_back() {
    for parsha in Parsha::ALL {
        assert_eq!(Parsha::from_name(parsha.name()), Some(parsha));
    }
    for month in HebrewMonth::ALL {
        assert_eq!(HebrewMonth::from_name(month.name()), Some(month));
    }
    assert_eq!(Parsha::from_name("bereshis"), None);
    assert_eq!(HebrewMonth::from_name("Adar II"), None);
}

// The dates below are checked against hebcal.

fn civil(year: i16, month: i8, day: i8) -> HebrewDate {
    HebrewDate::try_from(date(year, month, day)).unwrap()
}

/// `special` is on `day` in both regions.
fn assert_on(day: HebrewDate, special: SpecialDay) {
    for region in [Region::Israel, Region::Diaspora] {
        assert!(
            special_days(day, region).contains(&special),
            "{special:?} on {day} ({region:?})"
        );
    }
}

/// `special` is not on `day` in either region.
fn assert_not_on(day: HebrewDate, special: SpecialDay) {
    for region in [Region::Israel, Region::Diaspora] {
        assert!(
            !special_days(day, region).contains(&special),
            "{special:?} on {day} ({region:?})"
        );
    }
}

#[test]
fn birchas_hachamah_every_28_years() {
    assert_on(civil(2009, 4, 8), SpecialDay::BirchasHachamah);
    assert_on(civil(2037, 4, 8), SpecialDay::BirchasHachamah);
    assert_not_on(civil(2009, 4, 9), SpecialDay::BirchasHachamah);
    assert_not_on(civil(2023, 4, 8), SpecialDay::BirchasHachamah);
}

#[test]
fn eighth_day_of_chanukah_after_a_short_kislev() {
    let day = civil(2023, 12, 15);
    assert!(day.year().is_kislev_short());
    assert_eq!((day.day(), day.month()), (3, HebrewMonth::Teves));
    assert_on(day, SpecialDay::Chanukah(8));
    assert_not_on(civil(2023, 12, 16), SpecialDay::Chanukah(9));
}

#[test]
fn yom_hazikaron_and_yom_haatzmaut_move_off_shabbos_friday_and_monday() {
    // 5 Iyar on Shabbos, Friday, Wednesday and Monday.
    for (zikaron, atzmaut) in [
        ((2021, 4, 14), (2021, 4, 15)),
        ((2022, 5, 4), (2022, 5, 5)),
        ((2023, 4, 25), (2023, 4, 26)),
        ((2024, 5, 13), (2024, 5, 14)),
    ] {
        let zikaron = civil(zikaron.0, zikaron.1, zikaron.2);
        let atzmaut = civil(atzmaut.0, atzmaut.1, atzmaut.2);
        assert_on(zikaron, SpecialDay::YomHaZikaron);
        assert_on(atzmaut, SpecialDay::YomHaAtzmaut);
        assert_not_on(zikaron, SpecialDay::YomHaAtzmaut);
        assert_not_on(atzmaut, SpecialDay::YomHaZikaron);
    }
    assert_not_on(civil(2021, 4, 17), SpecialDay::YomHaAtzmaut);
}

#[test]
fn yom_hashoah_moves_off_friday_and_sunday() {
    assert_on(civil(2021, 4, 8), SpecialDay::YomHaShoah);
    assert_not_on(civil(2021, 4, 9), SpecialDay::YomHaShoah);
    assert_on(civil(2024, 5, 6), SpecialDay::YomHaShoah);
    assert_not_on(civil(2024, 5, 5), SpecialDay::YomHaShoah);
}

#[test]
fn fasts_on_shabbos_are_postponed_or_moved_earlier() {
    // Tishah B'Av, 17 Tammuz and Tzom Gedalyah move to Sunday.
    assert_not_on(civil(2015, 7, 25), SpecialDay::TishahBav);
    assert_on(civil(2015, 7, 26), SpecialDay::TishahBav);
    assert_not_on(civil(2015, 7, 4), SpecialDay::SeventeenthOfTammuz);
    assert_on(civil(2015, 7, 5), SpecialDay::SeventeenthOfTammuz);
    assert_not_on(civil(2014, 9, 27), SpecialDay::FastOfGedalyah);
    assert_on(civil(2014, 9, 28), SpecialDay::FastOfGedalyah);
    // The Fast of the Firstborn and Taanis Esther move back to Thursday.
    assert_not_on(civil(2021, 3, 27), SpecialDay::FastOfTheFirstborn);
    assert_on(civil(2021, 3, 25), SpecialDay::FastOfTheFirstborn);
    assert_not_on(civil(2024, 3, 23), SpecialDay::FastOfEsther);
    assert_on(civil(2024, 3, 21), SpecialDay::FastOfEsther);
}

#[test]
fn purim_katan_and_shushan_purim_katan_in_adar_i() {
    assert_on(civil(2024, 2, 23), SpecialDay::PurimKatan);
    assert_on(civil(2024, 2, 24), SpecialDay::ShushanPurimKatan);
    assert_not_on(civil(2024, 3, 24), SpecialDay::PurimKatan);
    assert_not_on(civil(2024, 3, 25), SpecialDay::ShushanPurimKatan);
}

#[test]
fn special_shabbosos_of_the_year() {
    for ((year, month, day), parsha) in [
        ((2024, 3, 9), Parsha::Shekalim),
        ((2024, 3, 23), Parsha::Zachor),
        ((2024, 3, 30), Parsha::Parah),
        ((2023, 3, 11), Parsha::Parah),
        ((2024, 4, 6), Parsha::Hachodesh),
        ((2018, 3, 17), Parsha::Hachodesh),
        ((2024, 4, 20), Parsha::Hagadol),
        ((2021, 3, 27), Parsha::Hagadol),
        ((2024, 8, 10), Parsha::Chazon),
        ((2015, 7, 25), Parsha::Chazon),
        ((2024, 8, 17), Parsha::Nachamu),
        ((2024, 10, 5), Parsha::Shuva),
        ((2014, 9, 27), Parsha::Shuva),
        ((2024, 1, 27), Parsha::Shira),
    ] {
        for region in [Region::Israel, Region::Diaspora] {
            assert_eq!(
                civil(year, month, day).special_parsha(region),
                Some(parsha),
                "{year}-{month}-{day} ({region:?})"
            );
        }
    }
    assert_eq!(civil(2024, 6, 8).special_parsha(Region::Diaspora), None);
}

#[test]
fn machar_chodesh_and_mevarchim() {
    // 30 Sivan 5784 is a Shabbos and Rosh Chodesh, and Sunday is Rosh Chodesh too.
    let shabbos = civil(2024, 7, 6);
    assert_on(shabbos, SpecialDay::RoshChodesh);
    assert_on(shabbos, SpecialDay::MacharChodesh);
    // 29 Adar I and 27 Adar II 5784 bless the coming month.
    assert_on(civil(2024, 3, 9), SpecialDay::ShabbosMevarchim);
    assert_on(civil(2024, 4, 6), SpecialDay::ShabbosMevarchim);
    assert_not_on(civil(2024, 3, 30), SpecialDay::ShabbosMevarchim);
}

/// Each of the 14 kinds of year (leap or not, the length of Cheshvan and Kislev, and the weekday
/// of Rosh Hashana) reads every week's parsha from its own table, so every Shabbos of a year has
/// a parsha, a special reading or a Yom Tov reading.
#[test]
fn every_kind_of_year_has_a_parsha_table() {
    let mut kinds = Vec::new();
    for year in 5700..=5800 {
        let year = HebrewYear::new(year).unwrap();
        let rosh_hashana = year.rosh_hashana().unwrap();
        let kind = (year.is_leap(), year.length(), rosh_hashana.weekday());
        if kinds.contains(&kind) {
            continue;
        }
        kinds.push(kind);
        for region in [Region::Israel, Region::Diaspora] {
            let mut day = rosh_hashana;
            let mut readings = 0;
            while day.year() == year {
                if day.weekday() == Weekday::Saturday {
                    if day.parsha(region).is_some() {
                        readings += 1;
                    } else {
                        assert!(day.is_assur_bemelacha(region), "no reading on {day} ({region:?})");
                    }
                }
                day = day.add_days(1).unwrap();
            }
            assert!(readings >= 46, "{readings} parshiyos in {year} ({region:?})");
        }
    }
    assert_eq!(kinds.len(), 14);
}
