//! Rust-side reference path for Hebrew-date Java parity tests.

use chrono::{Datelike, NaiveDate};
use hebrew_holiday_calendar::{HebrewHolidayCalendar, HebrewMonth, Holiday};
use icu_calendar::{
    cal::Hebrew,
    options::{DateAddOptions, Overflow},
    types::DateDuration,
    Date, Gregorian,
};

use super::{
    java_reference::java_holiday_index_from_rust,
    types::{DateTuple, JewishCalendarSnapshot, JewishDateSnapshot, JewishYearSnapshot},
};

pub(super) fn rust_gregorian_date_to_jewish_date(date: DateTuple) -> Option<DateTuple> {
    let gregorian = Date::<Gregorian>::try_new_gregorian(date.year, date.month, date.day).ok()?;
    let hebrew = gregorian.to_calendar(Hebrew);
    Some(DateTuple::new(
        hebrew.year().extended_year(),
        hebrew.hebrew_month().into(),
        hebrew.day_of_month().0,
    ))
}

pub(super) fn rust_jewish_date_to_gregorian_date(date: DateTuple) -> Option<DateTuple> {
    let hebrew = rust_hebrew_date(date)?;
    let gregorian = hebrew.to_calendar(Gregorian);
    Some(DateTuple::new(
        gregorian.year().extended_year(),
        gregorian.month().number(),
        gregorian.day_of_month().0,
    ))
}

pub(super) fn rust_jewish_date_snapshot(date: DateTuple) -> Option<JewishDateSnapshot> {
    let hebrew = rust_hebrew_date(date)?;
    let gregorian = hebrew.to_calendar(Gregorian);
    let gregorian_date = NaiveDate::from_ymd_opt(
        gregorian.year().extended_year(),
        u32::from(gregorian.month().number()),
        u32::from(gregorian.day_of_month().0),
    )?;
    let abs_date = gregorian_date.num_days_from_ce();
    let year = hebrew.year().extended_year();

    Some(JewishDateSnapshot {
        day_of_week: rust_java_day_of_week(&hebrew),
        abs_date,
        days_in_jewish_month: i32::from(Date::<Hebrew>::days_in_hebrew_month(
            year,
            hebrew.hebrew_month(),
        )),
        days_in_jewish_year: Date::<Hebrew>::days_in_hebrew_year(year),
        static_days_in_jewish_year: Date::<Hebrew>::days_in_hebrew_year(year),
        days_since_start_of_jewish_year: i32::from(hebrew.day_of_year().0),
        jewish_calendar_elapsed_days: rust_jewish_calendar_elapsed_days(year),
        is_jewish_leap_year: is_hebrew_leap_year(year),
        is_cheshvan_long: Date::<Hebrew>::is_cheshvan_long(year),
        is_kislev_short: Date::<Hebrew>::is_kislev_short(year),
        cheshvan_kislev_kviah: rust_cheshvan_kislev_kviah(year),
    })
}

pub(super) fn rust_jewish_year_snapshot(year: i32) -> Option<JewishYearSnapshot> {
    Date::<Hebrew>::from_hebrew_date(year, HebrewMonth::Tishrei, 1)?;
    Some(JewishYearSnapshot {
        days_in_jewish_year: Date::<Hebrew>::days_in_hebrew_year(year),
        jewish_calendar_elapsed_days: rust_jewish_calendar_elapsed_days(year),
        is_jewish_leap_year: is_hebrew_leap_year(year),
        is_cheshvan_long: Date::<Hebrew>::is_cheshvan_long(year),
        is_kislev_short: Date::<Hebrew>::is_kislev_short(year),
    })
}

pub(super) fn rust_add_days_to_jewish_date(date: DateTuple, days_to_add: i32) -> Option<DateTuple> {
    let mut hebrew = rust_hebrew_date(date)?;
    hebrew
        .try_add_with_options(
            DateDuration::for_days(days_to_add),
            constrained_date_add_options(),
        )
        .ok()?;
    Some(rust_date_tuple_from_hebrew(hebrew))
}

pub(super) fn rust_add_months_to_jewish_date(
    date: DateTuple,
    months_to_add: i32,
) -> Option<DateTuple> {
    rust_hebrew_date(date)?;

    let mut year = date.year;
    let mut month = date.month;
    for _ in 0..months_to_add.unsigned_abs() {
        if months_to_add > 0 {
            if month == 6 {
                month = 7;
                year += 1;
            } else if (!is_hebrew_leap_year(year) && month == 12)
                || (is_hebrew_leap_year(year) && month == 13)
            {
                month = 1;
            } else {
                month += 1;
            }
        } else if month == 7 {
            month = 6;
            year -= 1;
        } else if (!is_hebrew_leap_year(year) && month == 12)
            || (is_hebrew_leap_year(year) && month == 13)
        {
            month = 11;
        } else {
            if month == 0 {
                return None;
            }
            month -= 1;
        }
    }

    Some(clamped_hebrew_date(year, month, date.day)?)
}

pub(super) fn rust_add_years_to_jewish_date_with_adar(
    date: DateTuple,
    years_to_add: i32,
    use_adar_aleph_for_leap_year: bool,
) -> Option<DateTuple> {
    rust_hebrew_date(date)?;

    let target_year = date.year + years_to_add;
    let month = if date.month == 12
        && !is_hebrew_leap_year(date.year)
        && is_hebrew_leap_year(target_year)
    {
        if use_adar_aleph_for_leap_year {
            12
        } else {
            13
        }
    } else {
        date.month.min(last_month_of_hebrew_year(target_year))
    };

    Some(clamped_hebrew_date(target_year, month, date.day)?)
}

pub(super) fn rust_hebrew_date(date: DateTuple) -> Option<Date<Hebrew>> {
    Date::<Hebrew>::from_hebrew_date(date.year, HebrewMonth::try_from(date.month).ok()?, date.day)
}

pub(super) fn rust_jewish_calendar_snapshot(
    date: DateTuple,
    in_israel: bool,
    use_modern_holidays: bool,
) -> Option<JewishCalendarSnapshot> {
    let hebrew = rust_hebrew_date(date)?;
    let rust_holidays = hebrew
        .holidays(in_israel, use_modern_holidays)
        .copied()
        .collect::<Vec<Holiday>>();

    Some(JewishCalendarSnapshot {
        is_yom_tov: rust_is_yom_tov(&rust_holidays),
        is_yom_tov_assur_bemelacha: rust_is_yom_tov_assur_bemelacha(
            &rust_holidays,
            hebrew.chrono_day_of_week(),
        ),
        is_erev_yom_tov: rust_is_erev_yom_tov(&hebrew, in_israel),
        is_erev_yom_tov_sheni: rust_is_erev_yom_tov_sheni(&hebrew, in_israel),
        is_chol_hamoed: rust_holidays.contains(&Holiday::CholHamoedPesach)
            || rust_holidays.contains(&Holiday::CholHamoedSuccos)
            || rust_holidays.contains(&Holiday::HoshanaRabbah),
        is_chol_hamoed_pesach: rust_holidays.contains(&Holiday::CholHamoedPesach),
        is_chol_hamoed_succos: rust_holidays.contains(&Holiday::CholHamoedSuccos)
            || rust_holidays.contains(&Holiday::HoshanaRabbah),
        is_pesach: rust_holidays.contains(&Holiday::Pesach)
            || rust_holidays.contains(&Holiday::CholHamoedPesach),
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
        is_chanukah: rust_holidays.contains(&Holiday::Chanukah),
        is_purim: rust_holidays.contains(&Holiday::Purim),
        is_isru_chag: rust_holidays.contains(&Holiday::IsruChag),
        is_erev_rosh_chodesh: rust_is_erev_rosh_chodesh(&hebrew),
        is_tomorrow_shabbos_or_yom_tov: rust_is_tomorrow_shabbos_or_yom_tov(&hebrew, in_israel),
        is_birkas_hachamah: rust_holidays.contains(&Holiday::BirchasHachamah),
        parshah: hebrew.todays_parsha(in_israel).map(|parsha| parsha as i32),
        upcoming_parshah: hebrew.upcoming_parsha(in_israel) as i32,
        special_shabbos: hebrew.special_parsha(in_israel).map(|parsha| parsha as i32),
        yom_tov_index: java_holiday_index_from_rust(&rust_holidays),
        is_assur_bemelacha: hebrew.is_assur_bemelacha(in_israel),
        has_candle_lighting: hebrew.has_candle_lighting(in_israel),
        is_aseres_yemei_teshuva: hebrew.is_aseres_yemei_teshuva(),
        is_yom_kippur_katan: rust_holidays.contains(&Holiday::YomKippurKatan),
        is_be_ha_b: rust_holidays.contains(&Holiday::Behab),
        is_taanis_bechoros: rust_holidays.contains(&Holiday::FastOfTheFirstborn),
        day_of_chanukah: hebrew.day_of_chanukah(),
        is_rosh_chodesh: rust_holidays.contains(&Holiday::RoshChodesh),
        is_machar_chodesh: rust_holidays.contains(&Holiday::MacharHachodesh),
        is_shabbos_mevorchim: rust_holidays.contains(&Holiday::ShabbosMevarchim),
        day_of_omer: hebrew.day_of_the_omer(),
    })
}

fn rust_date_tuple_from_hebrew(date: Date<Hebrew>) -> DateTuple {
    DateTuple::new(
        date.year().extended_year(),
        date.hebrew_month().into(),
        date.day_of_month().0,
    )
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

fn rust_cheshvan_kislev_kviah(year: i32) -> i32 {
    if Date::<Hebrew>::is_cheshvan_long(year) && !Date::<Hebrew>::is_kislev_short(year) {
        2
    } else if !Date::<Hebrew>::is_cheshvan_long(year) && Date::<Hebrew>::is_kislev_short(year) {
        0
    } else {
        1
    }
}

fn rust_jewish_calendar_elapsed_days(year: i32) -> i32 {
    (1..year)
        .map(Date::<Hebrew>::days_in_hebrew_year)
        .sum::<i32>()
        + 1
}

fn clamped_hebrew_date(year: i32, month: u8, day: u8) -> Option<DateTuple> {
    let month = HebrewMonth::try_from(month).ok()?;
    let day = day.min(Date::<Hebrew>::days_in_hebrew_month(year, month));
    let date = Date::<Hebrew>::from_hebrew_date(year, month, day)?;
    Some(rust_date_tuple_from_hebrew(date))
}

fn is_hebrew_leap_year(year: i32) -> bool {
    Date::<Hebrew>::is_hebrew_leap_year(year)
}

fn last_month_of_hebrew_year(year: i32) -> u8 {
    if is_hebrew_leap_year(year) {
        13
    } else {
        12
    }
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
                | Holiday::Chanukah
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
    let today_holidays = date
        .holidays(in_israel, false)
        .copied()
        .collect::<Vec<Holiday>>();
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
    let next_holidays = next_day
        .holidays(in_israel, false)
        .copied()
        .collect::<Vec<Holiday>>();
    today_holidays.contains(&Holiday::CholHamoedPesach) && next_holidays.contains(&Holiday::Pesach)
}

fn rust_is_erev_yom_tov_sheni(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let today_holidays = date
        .holidays(in_israel, false)
        .copied()
        .collect::<Vec<Holiday>>();
    let next_holidays = next_day
        .holidays(in_israel, false)
        .copied()
        .collect::<Vec<Holiday>>();
    rust_is_yom_tov(&next_holidays)
        && rust_is_yom_tov_assur_bemelacha(&next_holidays, next_day.chrono_day_of_week())
        && today_holidays
            .iter()
            .any(|holiday| holiday.is_assur_bemelacha())
}

fn rust_is_yom_tov_assur_bemelacha(holidays: &[Holiday], _weekday: chrono::Weekday) -> bool {
    holidays.iter().any(|holiday| holiday.is_assur_bemelacha())
}

fn rust_is_erev_rosh_chodesh(date: &Date<Hebrew>) -> bool {
    if date
        .holidays(false, false)
        .any(|holiday| holiday == &Holiday::RoshChodesh)
    {
        return false;
    }
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    let mut next_holidays = next_day.holidays(false, false);
    let is_next_rosh_chodesh = next_holidays.any(|holiday| holiday == &Holiday::RoshChodesh);
    is_next_rosh_chodesh
        && !(date.hebrew_month() == HebrewMonth::Elul && date.day_of_month().0 == 29)
}

fn rust_is_tomorrow_shabbos_or_yom_tov(date: &Date<Hebrew>, in_israel: bool) -> bool {
    let next_day = date
        .try_added_with_options(DateDuration::for_days(1), constrained_date_add_options())
        .expect("adding one day to valid Hebrew date should work");
    next_day.is_assur_bemelacha(in_israel)
}
