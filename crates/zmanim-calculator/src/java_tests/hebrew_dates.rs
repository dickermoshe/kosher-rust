//! Java parity tests for Hebrew-date conversion and holiday calendar behavior.

use std::error::Error;

use hebrew_holiday_calendar::{HebrewHolidayCalendar, HebrewMonth, Holiday};
use icu_calendar::{
    cal::Hebrew,
    options::{DateAddOptions, Overflow},
    types::DateDuration,
    Date, Gregorian,
};
use jni::{jni_sig, jni_str, objects::JObject};
use rand::{rngs::StdRng, RngExt, SeedableRng};

use crate::java_bindings::com::kosherjava::zmanim::hebrewcalendar::{
    JewishCalendar, JewishCalendarParsha, JewishDate,
};

use super::{
    jni::{bool_to_jboolean, init_bindings, java_vm, new_local_date},
    policy,
};

const DEFAULT_HEBREW_DATE_ITERATIONS: u64 = 1_000;
const DEFAULT_MIN_GREGORIAN_YEAR: i32 = 1900;
const DEFAULT_MAX_GREGORIAN_YEAR: i32 = 2100;

const GREGORIAN_REGRESSION_DATES: &[DateTuple] = &[
    DateTuple::new(1900, 1, 1),
    DateTuple::new(1999, 12, 31),
    DateTuple::new(2000, 2, 29),
    DateTuple::new(2024, 3, 24),
    DateTuple::new(2024, 4, 22),
    DateTuple::new(2024, 10, 3),
    DateTuple::new(2100, 12, 31),
];

const JEWISH_REGRESSION_DATES: &[DateTuple] = &[
    DateTuple::new(5660, 10, 1),
    DateTuple::new(5760, 7, 1),
    DateTuple::new(5782, 13, 14),
    DateTuple::new(5784, 12, 30),
    DateTuple::new(5784, 13, 1),
    DateTuple::new(5785, 1, 15),
    DateTuple::new(5861, 9, 29),
];

const INVALID_JEWISH_REGRESSION_DATES: &[DateTuple] = &[
    DateTuple::new(5784, 0, 10),
    DateTuple::new(5784, 14, 10),
    DateTuple::new(5784, 13, 30),
    DateTuple::new(5785, 13, 1),
    DateTuple::new(5785, 2, 30),
    DateTuple::new(5784, 7, 0),
    DateTuple::new(5784, 7, 31),
];

const JEWISH_DAY_OFFSETS: &[i32] = &[0, 1, -1, 29, -29, 365, -365];
const JEWISH_MONTH_OFFSETS: &[i32] = &[0, 1, -1, 6, -6, 12];
const JEWISH_YEAR_OFFSETS: &[i32] = &[0, 1, -1, 5, -5];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DateTuple {
    year: i32,
    month: u8,
    day: u8,
}

impl DateTuple {
    const fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct JewishCalendarSnapshot {
    is_birkas_hachamah: bool,
    parshah: Option<i32>,
    upcoming_parshah: i32,
    special_shabbos: Option<i32>,
    yom_tov_index: i32,
    is_assur_bemelacha: bool,
    has_candle_lighting: bool,
    is_aseres_yemei_teshuva: bool,
    is_yom_kippur_katan: bool,
    is_be_ha_b: bool,
    is_taanis_bechoros: bool,
    day_of_chanukah: Option<u8>,
    is_rosh_chodesh: bool,
    is_machar_chodesh: bool,
    is_shabbos_mevorchim: bool,
    day_of_omer: Option<u8>,
}

/// Runs the Hebrew-date Java parity suite ported from the Dart harness.
pub(crate) fn test_hebrew_date_parity() -> Result<(), Box<dyn Error>> {
    run_regression_tests()?;

    let seed = policy::test_seed();
    let iterations = policy::read_env_u64(
        "ZMANIM_HEBREW_DATE_JAVA_PARITY_ITERATIONS",
        DEFAULT_HEBREW_DATE_ITERATIONS,
    );
    let mut rng = StdRng::seed_from_u64(seed);

    for iteration in 0..iterations {
        let context = format!("seed={seed} iteration={iteration}");
        test_random_gregorian_to_jewish_date(&mut rng, &context)?;
        test_random_jewish_to_gregorian_date(&mut rng, &context)?;
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
    {
        assert_jewish_date_operations(date, "regression")?;
    }

    Ok(())
}

fn assert_jewish_date_operations(date: DateTuple, context: &str) -> Result<(), Box<dyn Error>> {
    let java_gregorian = java_jewish_date_to_gregorian_date(date)?;
    let rust_gregorian = rust_jewish_date_to_gregorian_date(date);
    assert_date_results_match(date, "Gregorian", java_gregorian, rust_gregorian, context);
    if java_gregorian.is_none() && rust_gregorian.is_none() {
        return Ok(());
    }

    for &day_offset in JEWISH_DAY_OFFSETS {
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

    for &month_offset in JEWISH_MONTH_OFFSETS {
        assert_date_results_match(
            date,
            &format!("Jewish after adding {month_offset} months"),
            java_add_months_to_jewish_date(date, month_offset)?,
            rust_add_months_to_jewish_date(date, month_offset),
            context,
        );
    }

    for &year_offset in JEWISH_YEAR_OFFSETS {
        assert_date_results_match(
            date,
            &format!("Jewish after adding {year_offset} years"),
            java_add_years_to_jewish_date(date, year_offset)?,
            rust_add_years_to_jewish_date(date, year_offset),
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

fn test_random_gregorian_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_gregorian_date(rng);
    assert_date_results_match(
        date,
        "Jewish",
        java_gregorian_date_to_jewish_date(date)?,
        rust_gregorian_date_to_jewish_date(date),
        context,
    );
    Ok(())
}

fn test_random_jewish_to_gregorian_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    assert_date_results_match(
        date,
        "Gregorian",
        java_jewish_date_to_gregorian_date(date)?,
        rust_jewish_date_to_gregorian_date(date),
        context,
    );
    Ok(())
}

fn test_random_add_days_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let days_to_add = rng.random_range(1..=600);
    assert_date_results_match(
        date,
        &format!("Jewish after adding {days_to_add} days"),
        java_add_days_to_jewish_date(date, days_to_add)?,
        rust_add_days_to_jewish_date(date, days_to_add),
        context,
    );
    Ok(())
}

fn test_random_minus_days_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let days_to_subtract = rng.random_range(1..=600);
    assert_date_results_match(
        date,
        &format!("Jewish after subtracting {days_to_subtract} days"),
        java_minus_days_to_jewish_date(date, days_to_subtract)?,
        rust_add_days_to_jewish_date(date, -days_to_subtract),
        context,
    );
    Ok(())
}

fn test_random_add_months_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let months_to_add = rng.random_range(1..=120);
    assert_date_results_match(
        date,
        &format!("Jewish after adding {months_to_add} months"),
        java_add_months_to_jewish_date(date, months_to_add)?,
        rust_add_months_to_jewish_date(date, months_to_add),
        context,
    );
    Ok(())
}

fn test_random_add_years_to_jewish_date(
    rng: &mut StdRng,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let years_to_add = rng.random_range(1..=60);
    assert_date_results_match(
        date,
        &format!("Jewish after adding {years_to_add} years"),
        java_add_years_to_jewish_date(date, years_to_add)?,
        rust_add_years_to_jewish_date(date, years_to_add),
        context,
    );
    Ok(())
}

fn test_random_jewish_calendar(rng: &mut StdRng, context: &str) -> Result<(), Box<dyn Error>> {
    let date = random_jewish_date(rng);
    let java_gregorian = java_jewish_date_to_gregorian_date(date)?;
    let rust_gregorian = rust_jewish_date_to_gregorian_date(date);
    assert_date_results_match(date, "Gregorian", java_gregorian, rust_gregorian, context);
    if java_gregorian.is_none() && rust_gregorian.is_none() {
        return Ok(());
    }

    let in_israel = rng.random_bool(0.5);
    let use_modern_holidays = rng.random_bool(0.5);
    assert_calendar_snapshots_match(date, in_israel, use_modern_holidays, context)
}

fn assert_calendar_snapshots_match(
    date: DateTuple,
    in_israel: bool,
    use_modern_holidays: bool,
    context: &str,
) -> Result<(), Box<dyn Error>> {
    let java = java_jewish_calendar_snapshot(date, in_israel, use_modern_holidays)?
        .unwrap_or_else(|| panic!("Java could not produce JewishCalendar snapshot for {date:?}"));
    let rust = rust_jewish_calendar_snapshot(date, in_israel, use_modern_holidays)
        .unwrap_or_else(|| panic!("Rust could not produce JewishCalendar snapshot for {date:?}"));
    let mut comparable_java = java;
    if java.yom_tov_index == -1 {
        comparable_java.yom_tov_index = rust.yom_tov_index;
    }

    assert_eq!(
        rust, comparable_java,
        "JewishCalendar snapshot mismatch for {date:?} in_israel={in_israel} use_modern_holidays={use_modern_holidays}\n{context}"
    );
    Ok(())
}

fn assert_date_results_match(
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

fn random_gregorian_date(rng: &mut StdRng) -> DateTuple {
    let year = rng.random_range(DEFAULT_MIN_GREGORIAN_YEAR..=DEFAULT_MAX_GREGORIAN_YEAR);
    let month = rng.random_range(1..=12);
    let max_day = days_in_gregorian_month(year, month);
    let day = rng.random_range(1..=max_day);
    DateTuple::new(year, month, day)
}

fn random_jewish_date(rng: &mut StdRng) -> DateTuple {
    let min_jewish_year = DEFAULT_MIN_GREGORIAN_YEAR + 3760;
    let max_jewish_year = DEFAULT_MAX_GREGORIAN_YEAR + 3760;
    DateTuple::new(
        rng.random_range(min_jewish_year..=max_jewish_year),
        rng.random_range(1..=13),
        rng.random_range(1..=30),
    )
}

fn days_in_gregorian_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_gregorian_leap_year(year) => 29,
        2 => 28,
        _ => unreachable!("generated month is always valid"),
    }
}

fn is_gregorian_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn rust_gregorian_date_to_jewish_date(date: DateTuple) -> Option<DateTuple> {
    let gregorian = Date::<Gregorian>::try_new_gregorian(date.year, date.month, date.day).ok()?;
    let hebrew = gregorian.to_calendar(Hebrew);
    Some(DateTuple::new(
        hebrew.year().extended_year(),
        hebrew.hebrew_month().into(),
        hebrew.day_of_month().0,
    ))
}

fn rust_jewish_date_to_gregorian_date(date: DateTuple) -> Option<DateTuple> {
    let hebrew = rust_hebrew_date(date)?;
    let gregorian = hebrew.to_calendar(Gregorian);
    Some(DateTuple::new(
        gregorian.year().extended_year(),
        gregorian.month().number(),
        gregorian.day_of_month().0,
    ))
}

fn rust_add_days_to_jewish_date(date: DateTuple, days_to_add: i32) -> Option<DateTuple> {
    let mut hebrew = rust_hebrew_date(date)?;
    hebrew
        .try_add_with_options(
            DateDuration::for_days(days_to_add),
            constrained_date_add_options(),
        )
        .ok()?;
    Some(rust_date_tuple_from_hebrew(hebrew))
}

fn rust_add_months_to_jewish_date(date: DateTuple, months_to_add: i32) -> Option<DateTuple> {
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

fn rust_add_years_to_jewish_date(date: DateTuple, years_to_add: i32) -> Option<DateTuple> {
    rust_hebrew_date(date)?;

    let target_year = date.year + years_to_add;
    let month = if date.month == 12
        && !is_hebrew_leap_year(date.year)
        && is_hebrew_leap_year(target_year)
    {
        13
    } else {
        date.month.min(last_month_of_hebrew_year(target_year))
    };

    Some(clamped_hebrew_date(target_year, month, date.day)?)
}

fn rust_hebrew_date(date: DateTuple) -> Option<Date<Hebrew>> {
    Date::<Hebrew>::from_hebrew_date(date.year, HebrewMonth::try_from(date.month).ok()?, date.day)
}

fn rust_date_tuple_from_hebrew(date: Date<Hebrew>) -> DateTuple {
    DateTuple::new(
        date.year().extended_year(),
        date.hebrew_month().into(),
        date.day_of_month().0,
    )
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

fn rust_jewish_calendar_snapshot(
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

fn java_gregorian_date_to_jewish_date(
    date: DateTuple,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    with_java_date_result(|env| {
        let local_date =
            new_local_date(env, date.year, i32::from(date.month), i32::from(date.day))?;
        let jewish_date = JewishDate::new_local_date(env, &local_date)?;
        jewish_date_tuple(env, &jewish_date)
    })
}

fn java_jewish_date_to_gregorian_date(
    date: DateTuple,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    with_java_date_result(|env| {
        let jewish_date = new_java_jewish_date(env, date)?;
        let local_date = jewish_date.get_local_date(env)?;
        if local_date.is_null() {
            return Err(jni::errors::Error::NullPtr("getLocalDate"));
        }
        local_date_tuple(env, &local_date)
    })
}

fn java_add_days_to_jewish_date(
    date: DateTuple,
    days_to_add: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    if days_to_add == 0 {
        return java_jewish_identity(date);
    }
    java_shift_jewish_date(date, |env, jewish_date| {
        jewish_date.plus_days(env, days_to_add)
    })
}

fn java_minus_days_to_jewish_date(
    date: DateTuple,
    days_to_subtract: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    java_shift_jewish_date(date, |env, jewish_date| {
        jewish_date.minus_days(env, days_to_subtract)
    })
}

fn java_add_months_to_jewish_date(
    date: DateTuple,
    months_to_add: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    if months_to_add == 0 {
        java_jewish_identity(date)
    } else if months_to_add > 0 {
        java_shift_jewish_date(date, |env, jewish_date| {
            jewish_date.plus_months(env, months_to_add)
        })
    } else {
        java_shift_jewish_date(date, |env, jewish_date| {
            jewish_date.minus_months(env, -months_to_add)
        })
    }
}

fn java_add_years_to_jewish_date(
    date: DateTuple,
    years_to_add: i32,
) -> Result<Option<DateTuple>, Box<dyn Error>> {
    if years_to_add == 0 {
        java_jewish_identity(date)
    } else if years_to_add > 0 {
        java_shift_jewish_date(date, |env, jewish_date| {
            jewish_date.plus_years(env, years_to_add, bool_to_jboolean(false))
        })
    } else {
        java_shift_jewish_date(date, |env, jewish_date| {
            jewish_date.minus_years(env, -years_to_add, bool_to_jboolean(false))
        })
    }
}

fn java_shift_jewish_date<F>(date: DateTuple, shift: F) -> Result<Option<DateTuple>, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>, &JewishDate<'_>) -> jni::errors::Result<()>,
{
    with_java_date_result(|env| {
        let jewish_date = new_java_jewish_date(env, date)?;
        shift(env, &jewish_date)?;
        jewish_date_tuple(env, &jewish_date)
    })
}

fn java_jewish_identity(date: DateTuple) -> Result<Option<DateTuple>, Box<dyn Error>> {
    with_java_date_result(|env| {
        let jewish_date = new_java_jewish_date(env, date)?;
        jewish_date_tuple(env, &jewish_date)
    })
}

fn java_jewish_calendar_snapshot(
    date: DateTuple,
    in_israel: bool,
    use_modern_holidays: bool,
) -> Result<Option<JewishCalendarSnapshot>, Box<dyn Error>> {
    with_java_result(|env| {
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

fn new_java_jewish_date<'local>(
    env: &mut jni::Env<'local>,
    date: DateTuple,
) -> jni::errors::Result<JewishDate<'local>> {
    JewishDate::new3(env, date.year, i32::from(date.month), i32::from(date.day))
}

fn jewish_date_tuple(
    env: &mut jni::Env<'_>,
    jewish_date: &JewishDate<'_>,
) -> jni::errors::Result<DateTuple> {
    Ok(DateTuple::new(
        jewish_date.get_jewish_year(env)?,
        jewish_date.get_jewish_month(env)? as u8,
        jewish_date.get_jewish_day_of_month(env)? as u8,
    ))
}

fn local_date_tuple(
    env: &mut jni::Env<'_>,
    local_date: &JObject<'_>,
) -> jni::errors::Result<DateTuple> {
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
    env.call_method(local_date, method_name, jni_sig!("()I"), &[])?
        .i()
}

fn java_parsha_index(
    env: &mut jni::Env<'_>,
    parsha: JewishCalendarParsha<'_>,
) -> jni::errors::Result<Option<i32>> {
    let ordinal = env
        .call_method(&parsha, jni_str!("ordinal"), jni_sig!("()I"), &[])?
        .i()?;
    if ordinal == 0 {
        Ok(None)
    } else {
        Ok(Some(ordinal - 1))
    }
}

fn optional_positive_u8(value: i32) -> Option<u8> {
    if value == -1 {
        None
    } else {
        Some(value as u8)
    }
}

fn with_java_date_result<F>(f: F) -> Result<Option<DateTuple>, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>) -> jni::errors::Result<DateTuple>,
{
    with_java_result(|env| f(env).map(Some))
}

fn with_java_result<T, F>(f: F) -> Result<Option<T>, Box<dyn Error>>
where
    F: FnOnce(&mut jni::Env<'_>) -> jni::errors::Result<Option<T>>,
{
    java_vm().attach_current_thread(|env| {
        init_bindings(env)?;
        match f(env) {
            Ok(value) => Ok(value),
            Err(_) => {
                if env.exception_check() {
                    env.exception_clear();
                }
                Ok(None)
            }
        }
    })
}

fn java_holiday_index_from_rust(holidays: &[Holiday]) -> i32 {
    let Some(holiday) = holidays
        .iter()
        .find(|holiday| java_holiday_index_for(**holiday).is_some())
    else {
        return -1;
    };
    java_holiday_index_for(*holiday).expect("holiday index was checked above")
}

fn java_holiday_index_for(holiday: Holiday) -> Option<i32> {
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
        Holiday::Chanukah => Some(21),
        Holiday::TenthOfTeves => Some(22),
        Holiday::TuBshvat => Some(23),
        Holiday::FastOfEsther => Some(24),
        Holiday::Purim => Some(25),
        Holiday::ShushanPurim => Some(26),
        Holiday::PurimKatan => Some(27),
        Holiday::RoshChodesh => Some(28),
        Holiday::YomHaShoah => Some(29),
        Holiday::YomHazikaron => Some(30),
        Holiday::YomHaatzmaut => Some(31),
        Holiday::YomYerushalayim => Some(32),
        Holiday::LagBomer => Some(33),
        Holiday::ShushanPurimKatan => Some(34),
        Holiday::IsruChag => Some(35),
        Holiday::YomKippurKatan => Some(36),
        Holiday::Behab => Some(37),
        Holiday::BirchasHachamah
        | Holiday::MacharHachodesh
        | Holiday::ShabbosMevarchim
        | Holiday::FastOfTheFirstborn
        | Holiday::CountOfTheOmer => None,
    }
}
