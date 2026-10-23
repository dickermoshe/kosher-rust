use icu_calendar::{
    Date, Gregorian,
    cal::Hebrew,
    options::DateAddOptions,
    types::{DateDuration, Month},
};
use jiff::{
    SignedDuration, Timestamp,
    civil::{Date as JiffDate, Weekday},
    tz,
    tz::TimeZone,
};
use jiff_icu::ConvertTryFrom;

use super::month::month_of_year;

/// The molad, the mean new moon that begins a Hebrew month, as the calendar reckons it.
///
/// Get one from [`HebrewDate::molad`](super::HebrewDate::molad). The weekday, hours, minutes
/// and *chalakim* are in Jerusalem mean time, the way the molad is announced in shul.
///
/// ```
/// use jiff::civil::{Weekday, date};
/// use kosher_rust::HebrewDate;
///
/// // The molad of Iyar 5784.
/// let molad = HebrewDate::try_from(date(2024, 5, 9))?.molad();
/// assert_eq!(molad.weekday(), Weekday::Wednesday);
/// assert_eq!((molad.hours(), molad.minutes(), molad.chalakim()), (11, 41, 8));
/// # Ok::<(), kosher_rust::calendar::DateOutOfRange>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Molad {
    timestamp: Timestamp,
    weekday: Weekday,
    hours: u8,
    minutes: u8,
    chalakim: u8,
}

impl Molad {
    /// The moment of the molad, converted from Jerusalem mean time.
    pub const fn timestamp(self) -> Timestamp {
        self.timestamp
    }

    /// The day of the week, counting from midnight.
    pub const fn weekday(self) -> Weekday {
        self.weekday
    }

    /// The hour, from 0 to 23.
    pub const fn hours(self) -> u8 {
        self.hours
    }

    /// The minutes past the hour, from 0 to 59.
    pub const fn minutes(self) -> u8 {
        self.minutes
    }

    /// The *chalakim* past the minute, from 0 to 17. A *chelek* is 1/1080 of an hour, or 3⅓
    /// seconds.
    pub const fn chalakim(self) -> u8 {
        self.chalakim
    }
}

/// Chalakim (parts, 1/1080 of an hour) in a minute.
const CHALAKIM_PER_MINUTE: i64 = 18;
/// Chalakim in an hour.
const CHALAKIM_PER_HOUR: i64 = 1080;
/// Chalakim in a day.
pub(crate) const CHALAKIM_PER_DAY: i64 = 25920;
/// Chalakim in a mean lunar month (29 days, 12 hours, 793 chalakim).
pub(crate) const CHALAKIM_PER_MONTH: i64 = 765433;
/// Chalakim from the epoch to the molad tohu, the first molad of year 1.
const CHALAKIM_MOLAD_TOHU: i64 = 31524;
/// Days from the start of the Gregorian calendar to the Hebrew calendar epoch, as in KosherJava.
const JEWISH_EPOCH: i64 = -1373429;

/// Chalakim from the epoch to the molad of the `month_of_year`-th month (1 = Tishrei) of `year`.
pub(crate) fn chalakim_at_month_index(year: i64, month_of_year: u8) -> i64 {
    let months_elapsed = (235 * ((year - 1) / 19))
        + (12 * ((year - 1) % 19))
        + ((7 * ((year - 1) % 19) + 1) / 19)
        + (month_of_year as i64 - 1);
    CHALAKIM_MOLAD_TOHU + CHALAKIM_PER_MONTH * months_elapsed
}

/// Chalakim from the epoch to the molad of `month` in `year`, or `None` if the month is not in
/// that year.
pub(crate) fn chalakim_since_molad_tohu(year: i32, month: Month) -> Option<i64> {
    Some(chalakim_at_month_index(year as i64, month_of_year(month, year)?))
}

/// The moment of the molad of `date`'s month.
pub(crate) fn months_molad(date: &Date<Hebrew>) -> Option<Timestamp> {
    molad_of_month(date).map(Molad::timestamp)
}

/// The molad of `date`'s month.
pub(crate) fn molad_of_month(date: &Date<Hebrew>) -> Option<Molad> {
    // The arithmetic below assumes a positive count of chalakim since the molad tohu.
    if date.year().extended_year() < 1 {
        return None;
    }
    let chalakim_since_molad_tohu = chalakim_since_molad_tohu(date.year().extended_year(), date.month().to_input())?;
    let abs_date = JEWISH_EPOCH + (chalakim_since_molad_tohu / CHALAKIM_PER_DAY);
    let mut gregorian_date = abs_date_to_gregorian_date(abs_date)?;
    let conjunction_day = chalakim_since_molad_tohu / CHALAKIM_PER_DAY;
    let conjunction_parts = chalakim_since_molad_tohu - conjunction_day * CHALAKIM_PER_DAY;
    let mut hours = conjunction_parts / CHALAKIM_PER_HOUR;
    let adjusted_conjunction_parts = conjunction_parts - (hours * CHALAKIM_PER_HOUR);
    let minutes = adjusted_conjunction_parts / CHALAKIM_PER_MINUTE;
    let chalakim = adjusted_conjunction_parts - (minutes * CHALAKIM_PER_MINUTE);
    if hours >= 6 {
        gregorian_date
            .try_add_with_options(DateDuration::for_days(1), DateAddOptions::default())
            .ok()?;
    }
    hours = (hours + 18) % 24;

    let molad_seconds = chalakim as f64 * 10.0 / 3.0;
    let seconds = molad_seconds as i8;
    let nanos = ((molad_seconds - seconds as f64) * 1_000_000_000.0) as i32;

    let civil_date = JiffDate::convert_try_from(gregorian_date).ok()?;
    let datetime = civil_date.at(hours as i8, minutes as i8, seconds, nanos);

    // Molad is in Jerusalem standard time (GMT+2)
    let jerusalem_offset = TimeZone::fixed(tz::offset(2));
    let datetime_jerusalem = datetime.to_zoned(jerusalem_offset).ok()?;

    // Subtract local mean time offset (20.94 minutes = 1256.4 seconds)
    // Longitude of Har Habayis: 35.2354°
    // 35.2354° away from 35° (GMT+2 +  20 minutes) = 0.2354° = ~0.94 minutes
    // Total: 20 minutes 56.496 seconds ≈ 1256.496 seconds
    let timestamp = datetime_jerusalem
        .timestamp()
        .checked_sub(SignedDuration::from_millis(1_256_496))
        .ok()?;
    Some(Molad {
        timestamp,
        weekday: civil_date.weekday(),
        hours: hours as u8,
        minutes: minutes as u8,
        chalakim: chalakim as u8,
    })
}

fn abs_date_to_gregorian_date(abs_date: i64) -> Option<Date<Gregorian>> {
    let mut year = abs_date.div_euclid(366) as i32;
    while abs_date >= gregorian_date_to_abs_date(year + 1, 1, 1)? {
        year += 1;
    }
    let mut month: u8 = 1;
    while abs_date
        > gregorian_date_to_abs_date(
            year,
            month,
            Date::try_new_gregorian(year, month, 1).ok()?.days_in_month(),
        )?
    {
        month += 1;
    }
    let day_of_month: u8 = (abs_date - gregorian_date_to_abs_date(year, month, 1)? + 1) as u8;
    Date::try_new_gregorian(year, month, day_of_month).ok()
}

/// Days from 31 December of year 0 to the date, so 1 January of year 1 is day 1. The leap days
/// before `year` are counted with floor division, which also holds for year 0 and earlier.
fn gregorian_date_to_abs_date(year: i32, month: u8, day_of_month: u8) -> Option<i64> {
    let mut abs_date = day_of_month as i64;
    for m in (1..month).rev() {
        abs_date += Date::try_new_gregorian(year, m, 1).ok()?.days_in_month() as i64;
    }
    let year: i64 = year as i64;
    Some(
        abs_date + 365 * (year - 1) + (year - 1).div_euclid(4) - (year - 1).div_euclid(100)
            + (year - 1).div_euclid(400),
    )
}
