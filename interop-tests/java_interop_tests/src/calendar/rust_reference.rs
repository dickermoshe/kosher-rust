//! Rust-side reference path for Hebrew-date Java parity tests.

use icu_calendar::{
    Date,
    cal::{Gregorian, Hebrew, Iso},
    options::{DateAddOptions, Overflow},
    types::{DateDuration, Month},
};
use jiff::civil::{Date as JiffDate, Weekday};
use jiff_icu::{ConvertFrom, ConvertTryInto};
use kosher_rust::calendar::{HebrewDate, HebrewYear, Region, SpecialDay};

// ICU's codes for the Hebrew months, which this reference path drives directly.
const TISHREI: Month = Month::new(1);
const KISLEV: Month = Month::new(3);
const TEVET: Month = Month::new(4);
const SHEVAT: Month = Month::new(5);
const ADARI: Month = Month::leap(5);
const ADAR: Month = Month::new(6);
const NISAN: Month = Month::new(7);
const IYYAR: Month = Month::new(8);
const SIVAN: Month = Month::new(9);
const TAMMUZ: Month = Month::new(10);
const AV: Month = Month::new(11);
const ELUL: Month = Month::new(12);

fn region(in_israel: bool) -> Region {
    if in_israel { Region::Israel } else { Region::Diaspora }
}

/// The holidays on `date`, without the modern Israeli ones unless `use_modern_holidays`, as
/// KosherJava's `setUseModernHolidays` does.
fn special_days(date: &Date<Hebrew>, in_israel: bool, use_modern_holidays: bool) -> Vec<SpecialDay> {
    HebrewDate::from(*date)
        .special_days(region(in_israel))
        .filter(|special_day| use_modern_holidays || !special_day.is_modern())
        .collect()
}

use super::{
    java_reference::java_holiday_index_from_rust,
    types::{DateTuple, JewishCalendarSnapshot, JewishDateSnapshot, JewishYearSnapshot},
};

pub(super) fn rust_gregorian_date_to_jewish_date(date: DateTuple) -> Option<DateTuple> {
    let gregorian = icu_gregorian_date(date)?;
    let hebrew = gregorian.to_calendar(Hebrew);
    Some(DateTuple::new(
        hebrew.year().extended_year(),
        java_month_from_hebrew(hebrew.month().to_input(), hebrew.year().extended_year())?,
        hebrew.day_of_month().0,
    ))
}

pub(super) fn rust_jewish_date_to_gregorian_date(date: DateTuple) -> Option<DateTuple> {
    let hebrew = rust_hebrew_date(date)?;
    let gregorian = hebrew.to_calendar(Gregorian);
    let jiff_date: JiffDate = gregorian.convert_try_into().ok()?;
    Some(DateTuple::new(
        i32::from(jiff_date.year()),
        jiff_date.month() as u8,
        jiff_date.day() as u8,
    ))
}

pub(super) fn rust_jewish_date_snapshot(date: DateTuple) -> Option<JewishDateSnapshot> {
    let hebrew = rust_hebrew_date(date)?;
    let gregorian = hebrew.to_calendar(Gregorian);
    let abs_date = gregorian.to_rata_die().to_i64_date() as i32;

    Some(JewishDateSnapshot {
        day_of_week: rust_java_day_of_week(&hebrew),
        abs_date,
        days_in_jewish_month: i32::from(hebrew.days_in_month()),
        days_in_jewish_year: i32::from(hebrew.days_in_year()),
        static_days_in_jewish_year: i32::from(hebrew.days_in_year()),
        days_since_start_of_jewish_year: i32::from(hebrew.day_of_year().0),
        jewish_calendar_elapsed_days: rust_jewish_calendar_elapsed_days(&hebrew),
        is_jewish_leap_year: hebrew.is_in_leap_year(),
        is_cheshvan_long: is_cheshvan_long(&hebrew),
        is_kislev_short: is_kislev_short(&hebrew),
        cheshvan_kislev_kviah: rust_cheshvan_kislev_kviah(&hebrew),
    })
}

pub(super) fn rust_jewish_year_snapshot(year: i32) -> Option<JewishYearSnapshot> {
    let rosh_hashana = hebrew_date(year, TISHREI, 1)?;
    Some(JewishYearSnapshot {
        days_in_jewish_year: i32::from(rosh_hashana.days_in_year()),
        jewish_calendar_elapsed_days: rust_jewish_calendar_elapsed_days(&rosh_hashana),
        is_jewish_leap_year: rosh_hashana.is_in_leap_year(),
        is_cheshvan_long: is_cheshvan_long(&rosh_hashana),
        is_kislev_short: is_kislev_short(&rosh_hashana),
    })
}

pub(super) fn rust_add_days_to_jewish_date(date: DateTuple, days_to_add: i32) -> Option<DateTuple> {
    let mut hebrew = rust_hebrew_date(date)?;
    hebrew
        .try_add_with_options(DateDuration::for_days(days_to_add), constrained_date_add_options())
        .ok()?;
    Some(rust_date_tuple_from_hebrew(hebrew))
}

pub(super) fn rust_add_months_to_jewish_date(date: DateTuple, months_to_add: i32) -> Option<DateTuple> {
    let mut hebrew = rust_hebrew_date(date)?;
    hebrew
        .try_add_with_options(DateDuration::for_months(months_to_add), constrained_date_add_options())
        .ok()?;
    Some(rust_date_tuple_from_hebrew(hebrew))
}

pub(super) fn rust_add_years_to_jewish_date_with_adar(
    date: DateTuple,
    years_to_add: i32,
    use_adar_aleph_for_leap_year: bool,
) -> Option<DateTuple> {
    let mut hebrew = rust_hebrew_date(date)?;

    let target_year = date.year + years_to_add;
    if use_adar_aleph_for_leap_year
        && date.month == 12
        && !hebrew.is_in_leap_year()
        && hebrew_date(target_year, TISHREI, 1)?.is_in_leap_year()
    {
        return clamped_hebrew_date(target_year, ADARI, date.day);
    }

    hebrew
        .try_add_with_options(DateDuration::for_years(years_to_add), constrained_date_add_options())
        .ok()?;
    Some(rust_date_tuple_from_hebrew(hebrew))
}

pub(super) fn rust_hebrew_date(date: DateTuple) -> Option<Date<Hebrew>> {
    hebrew_date(date.year, hebrew_month_from_java(date.year, date.month)?, date.day)
}

pub(super) fn rust_jewish_calendar_snapshot(
    date: DateTuple,
    in_israel: bool,
    use_modern_holidays: bool,
) -> Option<JewishCalendarSnapshot> {
    let hebrew = rust_hebrew_date(date)?;
    let rust_special_days = special_days(&hebrew, in_israel, use_modern_holidays);
    let day = HebrewDate::from(hebrew);
    let region = region(in_israel);
    let day_of_chanukah = rust_special_days.iter().find_map(|special_day| match special_day {
        SpecialDay::Chanukah(day) => Some(*day),
        _ => None,
    });
    let day_of_omer = rust_special_days.iter().find_map(|special_day| match special_day {
        SpecialDay::CountOfTheOmer(day) => Some(*day),
        _ => None,
    });

    Some(JewishCalendarSnapshot {
        is_yom_tov: rust_is_yom_tov(&rust_special_days),
        is_yom_tov_assur_bemelacha: rust_is_yom_tov_assur_bemelacha(
            &rust_special_days,
            Weekday::convert_from(hebrew.weekday()),
        ),
        is_erev_yom_tov: rust_is_erev_yom_tov(&hebrew, in_israel),
        is_erev_yom_tov_sheni: rust_is_erev_yom_tov_sheni(&hebrew, in_israel),
        is_chol_hamoed: rust_special_days.contains(&SpecialDay::CholHamoedPesach)
            || rust_special_days.contains(&SpecialDay::CholHamoedSuccos)
            || rust_special_days.contains(&SpecialDay::HoshanaRabbah),
        is_chol_hamoed_pesach: rust_special_days.contains(&SpecialDay::CholHamoedPesach),
        is_chol_hamoed_succos: rust_special_days.contains(&SpecialDay::CholHamoedSuccos)
            || rust_special_days.contains(&SpecialDay::HoshanaRabbah),
        is_pesach: rust_special_days.contains(&SpecialDay::Pesach)
            || rust_special_days.contains(&SpecialDay::CholHamoedPesach),
        is_shavuos: rust_special_days.contains(&SpecialDay::Shavuos),
        is_succos: rust_special_days.contains(&SpecialDay::Succos)
            || rust_special_days.contains(&SpecialDay::CholHamoedSuccos)
            || rust_special_days.contains(&SpecialDay::HoshanaRabbah),
        is_shmini_atzeres: rust_special_days.contains(&SpecialDay::SheminiAtzeres),
        is_simchas_torah: rust_special_days.contains(&SpecialDay::SimchasTorah),
        is_rosh_hashana: rust_special_days.contains(&SpecialDay::RoshHashana),
        is_yom_kippur: rust_special_days.contains(&SpecialDay::YomKippur),
        is_taanis: rust_special_days.iter().any(|special_day| special_day.is_fast_day()),
        is_tisha_bav: rust_special_days.contains(&SpecialDay::TishahBav),
        is_chanukah: day_of_chanukah.is_some(),
        is_purim: rust_special_days.contains(&SpecialDay::Purim),
        is_isru_chag: rust_special_days.contains(&SpecialDay::IsruChag),
        is_erev_rosh_chodesh: rust_is_erev_rosh_chodesh(&hebrew),
        is_tomorrow_shabbos_or_yom_tov: rust_is_tomorrow_shabbos_or_yom_tov(&hebrew, in_israel),
        is_birkas_hachamah: rust_special_days.contains(&SpecialDay::BirchasHachamah),
        parshah: day.parsha(region).map(|parsha| parsha as i32),
        upcoming_parshah: day
            .next_parsha(region)
            .map(|parsha| i32::from(u8::from(parsha)))
            .expect("upcoming parsha"),
        special_shabbos: day.special_parsha(region).map(|parsha| parsha as i32),
        yom_tov_index: java_holiday_index_from_rust(&rust_special_days),
        is_assur_bemelacha: day.is_assur_bemelacha(region),
        has_candle_lighting: day.has_candle_lighting(region),
        is_aseres_yemei_teshuva: day.is_aseres_yemei_teshuva(),
        is_yom_kippur_katan: rust_special_days.contains(&SpecialDay::YomKippurKatan),
        is_be_ha_b: rust_special_days.contains(&SpecialDay::Behab),
        is_taanis_bechoros: rust_special_days.contains(&SpecialDay::FastOfTheFirstborn),
        day_of_chanukah: day_of_chanukah,
        is_rosh_chodesh: rust_special_days.contains(&SpecialDay::RoshChodesh),
        is_machar_chodesh: rust_special_days.contains(&SpecialDay::MacharHachodesh),
        is_shabbos_mevorchim: rust_special_days.contains(&SpecialDay::ShabbosMevarchim),
        day_of_omer: day_of_omer,
    })
}

fn rust_date_tuple_from_hebrew(date: Date<Hebrew>) -> DateTuple {
    DateTuple::new(
        date.year().extended_year(),
        java_month_from_hebrew(date.month().to_input(), date.year().extended_year()).expect("valid Hebrew month"),
        date.day_of_month().0,
    )
}

fn icu_gregorian_date(date: DateTuple) -> Option<Date<Gregorian>> {
    let jiff_date = JiffDate::new(i16::try_from(date.year).ok()?, date.month as i8, date.day as i8).ok()?;
    let iso_date: Date<Iso> = Date::convert_from(jiff_date);
    Some(iso_date.to_calendar(Gregorian))
}

fn rust_java_day_of_week(date: &Date<Hebrew>) -> i32 {
    match date.weekday() {
        icu_calendar::types::Weekday::Sunday => 1,
        icu_calendar::types::Weekday::Monday => 2,
        icu_calendar::types::Weekday::Tuesday => 3,
        icu_calendar::types::Weekday::Wednesday => 4,
        icu_calendar::types::Weekday::Thursday => 5,
        icu_calendar::types::Weekday::Friday => 6,
        icu_calendar::types::Weekday::Saturday => 7,
    }
}

fn rust_cheshvan_kislev_kviah(date: &Date<Hebrew>) -> i32 {
    if is_cheshvan_long(date) && !is_kislev_short(date) {
        2
    } else if !is_cheshvan_long(date) && is_kislev_short(date) {
        0
    } else {
        1
    }
}

fn rust_jewish_calendar_elapsed_days(date: &Date<Hebrew>) -> i32 {
    let start_of_year =
        hebrew_date(date.year().extended_year(), TISHREI, 1).expect("valid Hebrew date has a valid start of year");
    let calendar_epoch = hebrew_date(1, TISHREI, 1).expect("Hebrew calendar epoch should be representable");
    (start_of_year.to_rata_die() - calendar_epoch.to_rata_die()) as i32 + 1
}

fn clamped_hebrew_date(year: i32, month: Month, day: u8) -> Option<DateTuple> {
    let first_of_month = hebrew_date(year, month, 1)?;
    let day = day.min(first_of_month.days_in_month());
    let date = hebrew_date(year, month, day)?;
    Some(rust_date_tuple_from_hebrew(date))
}

fn is_cheshvan_long(date: &Date<Hebrew>) -> bool {
    hebrew_date(date.year().extended_year(), Month::new(2), 1)
        .expect("valid Hebrew date has Cheshvan")
        .days_in_month()
        == 30
}

fn is_kislev_short(date: &Date<Hebrew>) -> bool {
    hebrew_date(date.year().extended_year(), KISLEV, 1)
        .expect("valid Hebrew date has Kislev")
        .days_in_month()
        == 29
}

fn hebrew_date(year: i32, month: Month, day: u8) -> Option<Date<Hebrew>> {
    Date::try_new_hebrew_v2(year, month, day).ok()
}

fn hebrew_month_from_java(year: i32, java_month: u8) -> Option<Month> {
    match java_month {
        1 => Some(NISAN),
        2 => Some(IYYAR),
        3 => Some(SIVAN),
        4 => Some(TAMMUZ),
        5 => Some(AV),
        6 => Some(ELUL),
        7 => Some(TISHREI),
        8 => Some(Month::new(2)),
        9 => Some(KISLEV),
        10 => Some(TEVET),
        11 => Some(SHEVAT),
        12 if HebrewYear::new(year).is_leap() => Some(ADARI),
        12 => Some(ADAR),
        13 if HebrewYear::new(year).is_leap() => Some(ADAR),
        _ => None,
    }
}

fn java_month_from_hebrew(month: Month, year: i32) -> Option<u8> {
    Some(match month {
        NISAN => 1,
        IYYAR => 2,
        SIVAN => 3,
        TAMMUZ => 4,
        AV => 5,
        ELUL => 6,
        TISHREI => 7,
        month if month == Month::new(2) => 8,
        KISLEV => 9,
        TEVET => 10,
        SHEVAT => 11,
        ADARI => 12,
        ADAR if HebrewYear::new(year).is_leap() => 13,
        ADAR => 12,
        _ => return None,
    })
}

fn constrained_date_add_options() -> DateAddOptions {
    let mut options = DateAddOptions::default();
    options.overflow = Some(Overflow::Constrain);
    options
}

fn rust_is_yom_tov(special_days: &[SpecialDay]) -> bool {
    special_days.iter().any(|special_day| {
        matches!(
            special_day,
            SpecialDay::Pesach
                | SpecialDay::CholHamoedPesach
                | SpecialDay::PesachSheni
                | SpecialDay::Shavuos
                | SpecialDay::TuBav
                | SpecialDay::RoshHashana
                | SpecialDay::YomKippur
                | SpecialDay::Succos
                | SpecialDay::CholHamoedSuccos
                | SpecialDay::HoshanaRabbah
                | SpecialDay::SheminiAtzeres
                | SpecialDay::SimchasTorah
                | SpecialDay::Chanukah(_)
                | SpecialDay::TuBshvat
                | SpecialDay::Purim
                | SpecialDay::ShushanPurim
                | SpecialDay::PurimKatan
                | SpecialDay::YomHaShoah
                | SpecialDay::YomHazikaron
                | SpecialDay::YomHaatzmaut
                | SpecialDay::YomYerushalayim
                | SpecialDay::LagBomer
                | SpecialDay::ShushanPurimKatan
        )
    })
}

fn rust_is_erev_yom_tov(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let today_special_days = special_days(date, in_israel, false);
    if today_special_days.iter().any(|special_day| {
        matches!(
            special_day,
            SpecialDay::ErevPesach
                | SpecialDay::ErevShavuos
                | SpecialDay::ErevRoshHashana
                | SpecialDay::ErevYomKippur
                | SpecialDay::ErevSuccos
                | SpecialDay::HoshanaRabbah
        )
    }) {
        return true;
    }

    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let next_special_days = special_days(&next_day, in_israel, false);
    today_special_days.contains(&SpecialDay::CholHamoedPesach) && next_special_days.contains(&SpecialDay::Pesach)
}

fn rust_is_erev_yom_tov_sheni(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let today_special_days = special_days(date, in_israel, false);
    let next_special_days = special_days(&next_day, in_israel, false);
    rust_is_yom_tov(&next_special_days)
        && rust_is_yom_tov_assur_bemelacha(&next_special_days, Weekday::convert_from(next_day.weekday()))
        && today_special_days
            .iter()
            .any(|special_day| special_day.is_assur_bemelacha())
}

fn rust_is_yom_tov_assur_bemelacha(special_days: &[SpecialDay], _weekday: Weekday) -> bool {
    special_days.iter().any(|special_day| special_day.is_assur_bemelacha())
}

fn rust_is_erev_rosh_chodesh(date: &Date<Hebrew>) -> bool {
    if special_days(date, false, false).contains(&SpecialDay::RoshChodesh) {
        return false;
    }
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let is_next_rosh_chodesh = special_days(&next_day, false, false).contains(&SpecialDay::RoshChodesh);
    is_next_rosh_chodesh && !(date.month().to_input() == ELUL && date.day_of_month().0 == 29)
}

fn rust_is_tomorrow_shabbos_or_yom_tov(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    HebrewDate::from(next_day).is_assur_bemelacha(region(in_israel))
}
