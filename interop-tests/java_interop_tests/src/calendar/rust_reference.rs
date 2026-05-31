//! Rust-side reference path for Hebrew-date Java parity tests.

use icu_calendar::{
    Date,
    cal::{Gregorian, Hebrew, Iso},
    options::{DateAddOptions, Overflow},
    types::{DateDuration, Month},
};
use jiff::civil::{Date as JiffDate, Weekday};
use jiff_icu::{ConvertFrom, ConvertTryInto};
use kosher_rust::calendar::{month::*, prelude::*};

use super::{
    java_reference::java_holiday_index_from_rust,
    types::{DateTuple, JewishCalendarSnapshot, JewishDateSnapshot, JewishYearSnapshot},
};

pub(super) fn rust_gregorian_date_to_jewish_date(date: DateTuple) -> Option<DateTuple> {
    let gregorian = icu_gregorian_date(date)?;
    let hebrew = gregorian.to_calendar(Hebrew);
    Some(DateTuple::new(
        hebrew.year().extended_year(),
        java_month_from_hebrew(hebrew.input_month(), hebrew.year().extended_year())?,
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
    let rust_holidays = hebrew
        .holidays(in_israel, use_modern_holidays)
        .collect::<Vec<Holiday>>();
    let day_of_chanukah = rust_holidays.iter().find_map(|holiday| match holiday {
        Holiday::Chanukah(day) => Some(*day),
        _ => None,
    });
    let day_of_omer = rust_holidays.iter().find_map(|holiday| match holiday {
        Holiday::CountOfTheOmer(day) => Some(*day),
        _ => None,
    });

    Some(JewishCalendarSnapshot {
        is_yom_tov: rust_is_yom_tov(&rust_holidays),
        is_yom_tov_assur_bemelacha: rust_is_yom_tov_assur_bemelacha(
            &rust_holidays,
            Weekday::convert_from(hebrew.weekday()),
        ),
        is_erev_yom_tov: rust_is_erev_yom_tov(&hebrew, in_israel),
        is_erev_yom_tov_sheni: rust_is_erev_yom_tov_sheni(&hebrew, in_israel),
        is_chol_hamoed: rust_holidays.contains(&Holiday::CholHamoedPesach)
            || rust_holidays.contains(&Holiday::CholHamoedSuccos)
            || rust_holidays.contains(&Holiday::HoshanaRabbah),
        is_chol_hamoed_pesach: rust_holidays.contains(&Holiday::CholHamoedPesach),
        is_chol_hamoed_succos: rust_holidays.contains(&Holiday::CholHamoedSuccos)
            || rust_holidays.contains(&Holiday::HoshanaRabbah),
        is_pesach: rust_holidays.contains(&Holiday::Pesach) || rust_holidays.contains(&Holiday::CholHamoedPesach),
        is_shavuos: rust_holidays.contains(&Holiday::Shavuos),
        is_succos: rust_holidays.contains(&Holiday::Succos)
            || rust_holidays.contains(&Holiday::CholHamoedSuccos)
            || rust_holidays.contains(&Holiday::HoshanaRabbah),
        is_shmini_atzeres: rust_holidays.contains(&Holiday::SheminiAtzeres),
        is_simchas_torah: rust_holidays.contains(&Holiday::SimchasTorah),
        is_rosh_hashana: rust_holidays.contains(&Holiday::RoshHashana),
        is_yom_kippur: rust_holidays.contains(&Holiday::YomKippur),
        is_taanis: rust_holidays.iter().any(|holiday| holiday.is_fast_day()),
        is_tisha_bav: rust_holidays.contains(&Holiday::TishahBav),
        is_chanukah: day_of_chanukah.is_some(),
        is_purim: rust_holidays.contains(&Holiday::Purim),
        is_isru_chag: rust_holidays.contains(&Holiday::IsruChag),
        is_erev_rosh_chodesh: rust_is_erev_rosh_chodesh(&hebrew),
        is_tomorrow_shabbos_or_yom_tov: rust_is_tomorrow_shabbos_or_yom_tov(&hebrew, in_israel),
        is_birkas_hachamah: rust_holidays.contains(&Holiday::BirchasHachamah),
        parshah: hebrew.todays_parsha(in_israel).map(|parsha| parsha as i32),
        upcoming_parshah: hebrew
            .upcoming_parsha(in_israel)
            .map(|parsha| i32::from(u8::from(parsha)))
            .expect("upcoming parsha"),
        special_shabbos: hebrew.special_parsha(in_israel).map(|parsha| parsha as i32),
        yom_tov_index: java_holiday_index_from_rust(&rust_holidays),
        is_assur_bemelacha: hebrew.is_assur_bemelacha(in_israel),
        has_candle_lighting: hebrew.has_candle_lighting(in_israel),
        is_aseres_yemei_teshuva: hebrew.is_aseres_yemei_teshuva(),
        is_yom_kippur_katan: rust_holidays.contains(&Holiday::YomKippurKatan),
        is_be_ha_b: rust_holidays.contains(&Holiday::Behab),
        is_taanis_bechoros: rust_holidays.contains(&Holiday::FastOfTheFirstborn),
        day_of_chanukah: day_of_chanukah,
        is_rosh_chodesh: rust_holidays.contains(&Holiday::RoshChodesh),
        is_machar_chodesh: rust_holidays.contains(&Holiday::MacharHachodesh),
        is_shabbos_mevorchim: rust_holidays.contains(&Holiday::ShabbosMevarchim),
        day_of_omer: day_of_omer,
    })
}

fn rust_date_tuple_from_hebrew(date: Date<Hebrew>) -> DateTuple {
    DateTuple::new(
        date.year().extended_year(),
        java_month_from_hebrew(date.input_month(), date.year().extended_year()).expect("valid Hebrew month"),
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
        12 if Hebrew::is_hebrew_leap_year(year) => Some(ADARI),
        12 => Some(ADAR),
        13 if Hebrew::is_hebrew_leap_year(year) => Some(ADAR),
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
        ADAR if Hebrew::is_hebrew_leap_year(year) => 13,
        ADAR => 12,
        _ => return None,
    })
}

fn constrained_date_add_options() -> DateAddOptions {
    let mut options = DateAddOptions::default();
    options.overflow = Some(Overflow::Constrain);
    options
}

fn rust_is_yom_tov(holidays: &[Holiday]) -> bool {
    holidays.iter().any(|holiday| {
        matches!(
            holiday,
            Holiday::Pesach
                | Holiday::CholHamoedPesach
                | Holiday::PesachSheni
                | Holiday::Shavuos
                | Holiday::TuBav
                | Holiday::RoshHashana
                | Holiday::YomKippur
                | Holiday::Succos
                | Holiday::CholHamoedSuccos
                | Holiday::HoshanaRabbah
                | Holiday::SheminiAtzeres
                | Holiday::SimchasTorah
                | Holiday::Chanukah(_)
                | Holiday::TuBshvat
                | Holiday::Purim
                | Holiday::ShushanPurim
                | Holiday::PurimKatan
                | Holiday::YomHaShoah
                | Holiday::YomHazikaron
                | Holiday::YomHaatzmaut
                | Holiday::YomYerushalayim
                | Holiday::LagBomer
                | Holiday::ShushanPurimKatan
        )
    })
}

fn rust_is_erev_yom_tov(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let today_holidays = date.holidays(in_israel, false).collect::<Vec<Holiday>>();
    if today_holidays.iter().any(|holiday| {
        matches!(
            holiday,
            Holiday::ErevPesach
                | Holiday::ErevShavuos
                | Holiday::ErevRoshHashana
                | Holiday::ErevYomKippur
                | Holiday::ErevSuccos
                | Holiday::HoshanaRabbah
        )
    }) {
        return true;
    }

    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let next_holidays = next_day.holidays(in_israel, false).collect::<Vec<Holiday>>();
    today_holidays.contains(&Holiday::CholHamoedPesach) && next_holidays.contains(&Holiday::Pesach)
}

fn rust_is_erev_yom_tov_sheni(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let today_holidays = date.holidays(in_israel, false).collect::<Vec<Holiday>>();
    let next_holidays = next_day.holidays(in_israel, false).collect::<Vec<Holiday>>();
    rust_is_yom_tov(&next_holidays)
        && rust_is_yom_tov_assur_bemelacha(&next_holidays, Weekday::convert_from(next_day.weekday()))
        && today_holidays.iter().any(|holiday| holiday.is_assur_bemelacha())
}

fn rust_is_yom_tov_assur_bemelacha(holidays: &[Holiday], _weekday: Weekday) -> bool {
    holidays.iter().any(|holiday| holiday.is_assur_bemelacha())
}

fn rust_is_erev_rosh_chodesh(date: &Date<Hebrew>) -> bool {
    if date
        .holidays(false, false)
        .any(|holiday| holiday == Holiday::RoshChodesh)
    {
        return false;
    }
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let mut next_holidays = next_day.holidays(false, false);
    let is_next_rosh_chodesh = next_holidays.any(|holiday| holiday == Holiday::RoshChodesh);
    is_next_rosh_chodesh && !(date.input_month() == ELUL && date.day_of_month().0 == 29)
}

fn rust_is_tomorrow_shabbos_or_yom_tov(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    next_day.is_assur_bemelacha(in_israel)
}
