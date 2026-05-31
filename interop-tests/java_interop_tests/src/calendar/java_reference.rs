//! Java-side reference path for Hebrew-date Java parity tests.

use std::error::Error;

use jni::{
    Env, jni_sig, jni_str,
    objects::{JObject, JValue},
    sys::jboolean,
};

use crate::{init_bindings, java_vm};

use kosher_rust::calendar::prelude::*;

use crate::java_bindings::com::kosherjava::zmanim::hebrewcalendar::{JewishCalendar, JewishCalendarParsha, JewishDate};

use super::types::{DateTuple, JewishCalendarSnapshot, JewishDateSnapshot, JewishYearSnapshot};

pub(super) fn java_gregorian_date_to_jewish_date(date: DateTuple) -> Result<Option<DateTuple>, Box<dyn Error>> {
    with_java_optional_date_result(|env| {
        let local_date = new_local_date(env, date.year, i32::from(date.month), i32::from(date.day))?;
        let jewish_date = JewishDate::new_local_date(env, &local_date)?;
        jewish_date_tuple(env, &jewish_date)
    })
}

pub(super) fn java_jewish_date_to_gregorian_date(date: DateTuple) -> Result<Option<DateTuple>, Box<dyn Error>> {
    with_java_optional_date_result(|env| {
        let jewish_date = new_java_jewish_date(env, date)?;
        let local_date = jewish_date.get_local_date(env)?;
        if local_date.is_null() {
            return Err(jni::errors::Error::NullPtr("getLocalDate"));
        }
        local_date_tuple(env, &local_date)
    })
}

pub(super) fn java_add_days_to_jewish_date(
    date: DateTuple,
    days_to_add: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    if days_to_add == 0 {
        return java_jewish_identity(date);
    }
    java_shift_jewish_date(
        date,
        |env, jewish_date| jewish_date.plus_days(env, days_to_add),
        "plusDays",
    )
}

pub(super) fn java_minus_days_to_jewish_date(
    date: DateTuple,
    days_to_subtract: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    java_shift_jewish_date(
        date,
        |env, jewish_date| jewish_date.minus_days(env, days_to_subtract),
        "minusDays",
    )
}

pub(super) fn java_add_months_to_jewish_date(
    date: DateTuple,
    months_to_add: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    if months_to_add == 0 {
        java_jewish_identity(date)
    } else if months_to_add > 0 {
        java_shift_jewish_date(
            date,
            |env, jewish_date| jewish_date.plus_months(env, months_to_add),
            "plusMonths",
        )
    } else {
        java_shift_jewish_date(
            date,
            |env, jewish_date| jewish_date.minus_months(env, -months_to_add),
            "minusMonths",
        )
    }
}

pub(super) fn java_add_years_to_jewish_date(
    date: DateTuple,
    years_to_add: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    java_add_years_to_jewish_date_with_adar(date, years_to_add, false)
}

pub(super) fn java_add_years_to_jewish_date_with_adar(
    date: DateTuple,
    years_to_add: i32,
    use_adar_aleph_for_leap_year: bool,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    if years_to_add == 0 {
        java_jewish_identity(date)
    } else if years_to_add > 0 {
        java_shift_jewish_date(
            date,
            |env, jewish_date| {
                jewish_date.plus_years(env, years_to_add, bool_to_jboolean(use_adar_aleph_for_leap_year))
            },
            "plusYears",
        )
    } else {
        java_shift_jewish_date(
            date,
            |env, jewish_date| {
                jewish_date.minus_years(env, -years_to_add, bool_to_jboolean(use_adar_aleph_for_leap_year))
            },
            "minusYears",
        )
    }
}

pub(super) fn java_jewish_date_snapshot(date: DateTuple) -> Result<Option<JewishDateSnapshot>, Box<dyn Error>> {
    with_java_optional_result(|env| {
        let jewish_date = new_java_jewish_date(env, date)?;
        let year = jewish_date.get_jewish_year(env)?;
        Ok(Some(JewishDateSnapshot {
            day_of_week: jewish_date.get_day_of_week(env)?,
            abs_date: jewish_date.get_abs_date(env)?,
            days_in_jewish_month: jewish_date.get_days_in_jewish_month(env)?,
            days_in_jewish_year: jewish_date.get_days_in_jewish_year(env)?,
            static_days_in_jewish_year: JewishDate::get_days_in_jewish_year_int(env, year)?,
            days_since_start_of_jewish_year: jewish_date.get_days_since_start_of_jewish_year(env)?,
            jewish_calendar_elapsed_days: JewishDate::get_jewish_calendar_elapsed_days(env, year)?,
            is_jewish_leap_year: jewish_date.is_jewish_leap_year(env)?,
            is_cheshvan_long: jewish_date.is_cheshvan_long(env)?,
            is_kislev_short: jewish_date.is_kislev_short(env)?,
            cheshvan_kislev_kviah: jewish_date.get_cheshvan_kislev_kviah(env)?,
        }))
    })
}

pub(super) fn java_jewish_year_snapshot(year: i32) -> Result<Option<JewishYearSnapshot>, Box<dyn Error>> {
    with_java_required_result("Jewish year snapshot", |env| {
        let jewish_date = JewishDate::new3(env, year, 7, 1)?;
        Ok(JewishYearSnapshot {
            days_in_jewish_year: JewishDate::get_days_in_jewish_year_int(env, year)?,
            jewish_calendar_elapsed_days: JewishDate::get_jewish_calendar_elapsed_days(env, year)?,
            is_jewish_leap_year: jewish_date.is_jewish_leap_year(env)?,
            is_cheshvan_long: jewish_date.is_cheshvan_long(env)?,
            is_kislev_short: jewish_date.is_kislev_short(env)?,
        })
    })
    .map(Some)
}

pub(super) fn java_jewish_calendar_snapshot(
    date: DateTuple,
    in_israel: bool,
    use_modern_holidays: bool,
) -> Result<Option<JewishCalendarSnapshot>, Box<dyn Error>> {
    with_java_optional_result(|env| {
        let calendar = JewishCalendar::new4(
            env,
            date.year,
            i32::from(date.month),
            i32::from(date.day),
            bool_to_jboolean(in_israel),
        )?;
        calendar.set_use_modern_holidays(env, bool_to_jboolean(use_modern_holidays))?;

        let day_of_chanukah = optional_positive_u8(calendar.get_day_of_chanukah(env)?);
        let day_of_omer = optional_positive_u8(calendar.get_day_of_omer(env)?);
        let parshah = calendar.get_parshah(env)?;
        let upcoming_parshah = calendar.get_upcoming_parshah(env)?;
        let special_shabbos = calendar.get_special_shabbos(env)?;

        Ok(Some(JewishCalendarSnapshot {
            is_yom_tov: calendar.is_yom_tov(env)?,
            is_yom_tov_assur_bemelacha: calendar.is_yom_tov_assur_bemelacha(env)?,
            is_erev_yom_tov: calendar.is_erev_yom_tov(env)?,
            is_erev_yom_tov_sheni: calendar.is_erev_yom_tov_sheni(env)?,
            is_chol_hamoed: calendar.is_chol_hamoed(env)?,
            is_chol_hamoed_pesach: calendar.is_chol_hamoed_pesach(env)?,
            is_chol_hamoed_succos: calendar.is_chol_hamoed_succos(env)?,
            is_pesach: calendar.is_pesach(env)?,
            is_shavuos: calendar.is_shavuos(env)?,
            is_succos: calendar.is_succos(env)?,
            is_shmini_atzeres: calendar.is_shmini_atzeres(env)?,
            is_simchas_torah: calendar.is_simchas_torah(env)?,
            is_rosh_hashana: calendar.is_rosh_hashana(env)?,
            is_yom_kippur: calendar.is_yom_kippur(env)?,
            is_taanis: calendar.is_taanis(env)?,
            is_tisha_bav: calendar.is_tisha_bav(env)?,
            is_chanukah: calendar.is_chanukah(env)?,
            is_purim: calendar.is_purim(env)?,
            is_isru_chag: calendar.is_isru_chag(env)?,
            is_erev_rosh_chodesh: calendar.is_erev_rosh_chodesh(env)?,
            is_tomorrow_shabbos_or_yom_tov: calendar.is_tomorrow_shabbos_or_yom_tov(env)?,
            is_birkas_hachamah: calendar.is_birkas_hachamah(env)?,
            parshah: java_parsha_index(env, parshah)?,
            upcoming_parshah: java_parsha_index(env, upcoming_parshah)?
                .expect("Java upcoming parshah should not be NONE"),
            special_shabbos: java_parsha_index(env, special_shabbos)?,
            yom_tov_index: calendar.get_yom_tov_index(env)?,
            is_assur_bemelacha: calendar.is_assur_bemelacha(env)?,
            has_candle_lighting: calendar.has_candle_lighting(env)?,
            is_aseres_yemei_teshuva: calendar.is_aseres_yemei_teshuva(env)?,
            is_yom_kippur_katan: calendar.is_yom_kippur_katan(env)?,
            is_be_ha_b: calendar.is_be_ha_b(env)?,
            is_taanis_bechoros: calendar.is_taanis_bechoros(env)?,
            day_of_chanukah,
            is_rosh_chodesh: calendar.is_rosh_chodesh(env)?,
            is_machar_chodesh: calendar.is_machar_chodesh(env)?,
            is_shabbos_mevorchim: calendar.is_shabbos_mevorchim(env)?,
            day_of_omer,
        }))
    })
}

pub(super) fn java_holiday_index_from_rust(holidays: &[Holiday]) -> i32 {
    let Some(holiday) = holidays
        .iter()
        .find(|holiday| java_holiday_index_for(**holiday).is_some())
    else {
        return -1;
    };
    java_holiday_index_for(*holiday).expect("holiday index was checked above")
}

pub(super) fn java_holiday_index_for(holiday: Holiday) -> Option<i32> {
    match holiday {
        Holiday::ErevPesach => Some(0),
        Holiday::Pesach => Some(1),
        Holiday::CholHamoedPesach => Some(2),
        Holiday::PesachSheni => Some(3),
        Holiday::ErevShavuos => Some(4),
        Holiday::Shavuos => Some(5),
        Holiday::SeventeenthOfTammuz => Some(6),
        Holiday::TishahBav => Some(7),
        Holiday::TuBav => Some(8),
        Holiday::ErevRoshHashana => Some(9),
        Holiday::RoshHashana => Some(10),
        Holiday::FastOfGedalyah => Some(11),
        Holiday::ErevYomKippur => Some(12),
        Holiday::YomKippur => Some(13),
        Holiday::ErevSuccos => Some(14),
        Holiday::Succos => Some(15),
        Holiday::CholHamoedSuccos => Some(16),
        Holiday::HoshanaRabbah => Some(17),
        Holiday::SheminiAtzeres => Some(18),
        Holiday::SimchasTorah => Some(19),
        Holiday::Chanukah(_) => Some(21),
        Holiday::TenthOfTeves => Some(22),
        Holiday::TuBshvat => Some(23),
        Holiday::FastOfEsther => Some(24),
        Holiday::Purim => Some(25),
        Holiday::ShushanPurim => Some(26),
        Holiday::PurimKatan => Some(27),
        Holiday::RoshChodesh => None,
        Holiday::YomHaShoah => Some(29),
        Holiday::YomHazikaron => Some(30),
        Holiday::YomHaatzmaut => Some(31),
        Holiday::YomYerushalayim => Some(32),
        Holiday::LagBomer => Some(33),
        Holiday::ShushanPurimKatan => Some(34),
        Holiday::IsruChag => Some(35),
        Holiday::YomKippurKatan => None,
        Holiday::Behab => None,
        Holiday::BirchasHachamah
        | Holiday::MacharHachodesh
        | Holiday::ShabbosMevarchim
        | Holiday::FastOfTheFirstborn
        | Holiday::CountOfTheOmer(_) => None,
    }
}

fn java_shift_jewish_date<F>(
    date: DateTuple,
    shift: F,
    operation: &'static str,
) -> Result<Option<DateTuple>, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>, &JewishDate<'_>) -> jni::errors::Result<()>,
{
    java_vm().attach_current_thread(|env| {
        let jewish_date = match new_java_jewish_date(env, date) {
            Ok(jewish_date) => jewish_date,
            Err(_) => {
                env.exception_catch().ok();
                return Ok(None);
            }
        };

        if let Err(err) = shift(env, &jewish_date) {
            return java_required_error(env, operation, err);
        }

        match jewish_date_tuple(env, &jewish_date) {
            Ok(date) => Ok(Some(date)),
            Err(err) => java_required_error(env, operation, err),
        }
    })
}

fn java_jewish_identity(date: DateTuple) -> Result<Option<DateTuple>, Box<dyn Error>> {
    with_java_optional_date_result(|env| {
        let jewish_date = new_java_jewish_date(env, date)?;
        jewish_date_tuple(env, &jewish_date)
    })
}

fn new_java_jewish_date<'local>(
    env: &mut jni::Env<'local>,
    date: DateTuple,
) -> jni::errors::Result<JewishDate<'local>> {
    JewishDate::new3(env, date.year, i32::from(date.month), i32::from(date.day))
}

fn jewish_date_tuple(env: &mut jni::Env<'_>, jewish_date: &JewishDate<'_>) -> jni::errors::Result<DateTuple> {
    Ok(DateTuple::new(
        jewish_date.get_jewish_year(env)?,
        jewish_date.get_jewish_month(env)? as u8,
        jewish_date.get_jewish_day_of_month(env)? as u8,
    ))
}

fn local_date_tuple(env: &mut jni::Env<'_>, local_date: &JObject<'_>) -> jni::errors::Result<DateTuple> {
    Ok(DateTuple::new(
        local_date_int_method(env, local_date, jni_str!("getYear"))?,
        local_date_int_method(env, local_date, jni_str!("getMonthValue"))? as u8,
        local_date_int_method(env, local_date, jni_str!("getDayOfMonth"))? as u8,
    ))
}

fn local_date_int_method(
    env: &mut jni::Env<'_>,
    local_date: &JObject<'_>,
    method_name: &'static jni::strings::JNIStr,
) -> jni::errors::Result<i32> {
    env.call_method(local_date, method_name, jni_sig!("()I"), &[])?.i()
}

fn java_parsha_index(env: &mut jni::Env<'_>, parsha: JewishCalendarParsha<'_>) -> jni::errors::Result<Option<i32>> {
    let ordinal = env
        .call_method(&parsha, jni_str!("ordinal"), jni_sig!("()I"), &[])?
        .i()?;
    if ordinal == 0 { Ok(None) } else { Ok(Some(ordinal - 1)) }
}

fn optional_positive_u8(value: i32) -> Option<u8> {
    if value == -1 { None } else { Some(value as u8) }
}

fn bool_to_jboolean(value: bool) -> jboolean {
    value.into()
}

pub(crate) fn new_local_date<'local>(
    env: &mut Env<'local>,
    year: i32,
    month: i32,
    day: i32,
) -> jni::errors::Result<JObject<'local>> {
    env.call_static_method(
        jni_str!("java/time/LocalDate"),
        jni_str!("of"),
        jni_sig!("(III)Ljava/time/LocalDate;"),
        &[JValue::Int(year), JValue::Int(month), JValue::Int(day)],
    )?
    .l()
}

fn with_java_optional_date_result<F>(f: F) -> Result<Option<DateTuple>, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>) -> jni::errors::Result<DateTuple>,
{
    with_java_optional_result(|env| f(env).map(Some))
}

fn with_java_optional_result<T, F>(f: F) -> Result<Option<T>, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>) -> jni::errors::Result<Option<T>>,
{
    java_vm().attach_current_thread(|env| match f(env) {
        Ok(value) => Ok(value),
        Err(_) => {
            env.exception_catch().ok();
            Ok(None)
        }
    })
}

fn with_java_required_result<T, F>(operation: &'static str, f: F) -> Result<T, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>) -> jni::errors::Result<T>,
{
    java_vm().attach_current_thread(|env| match f(env) {
        Ok(value) => Ok(value),
        Err(err) => java_required_error(env, operation, err),
    })
}

fn java_required_error<T>(
    env: &mut Env<'_>,
    operation: &'static str,
    err: jni::errors::Error,
) -> Result<T, Box<dyn Error>> {
    if let Err(exception) = env.exception_catch() {
        Err(format!("Java exception during {operation}: {exception}").into())
    } else {
        Err(format!("JNI error during {operation}: {err}").into())
    }
}
