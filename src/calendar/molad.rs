use icu_calendar::{
    Date, Gregorian,
    cal::Hebrew,
    options::DateAddOptions,
    types::{DateDuration, Month},
};
use jiff::{SignedDuration, Timestamp, civil::Date as JiffDate, tz, tz::TimeZone};
use jiff_icu::ConvertTryFrom;

use super::month::month_of_year;
use crate::TimeZoneLike;

/// The molad: the mean new moon that begins a Hebrew month, as the calendar reckons it.
///
/// Get one from [`HebrewDate::molad`](super::HebrewDate::molad).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Molad {
    timestamp: Timestamp,
}

impl Molad {
    /// The moment of the molad, converted from Jerusalem mean time.
    pub const fn timestamp(self) -> Timestamp {
        self.timestamp
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

/// The molad of `date`'s month.
pub(crate) fn molad_of_month(date: &Date<Hebrew>) -> Option<Molad> {
    months_molad(date).map(|timestamp| Molad { timestamp })
}

pub(crate) fn is_same_gregorian_day(hdate: &Date<Hebrew>, gdate: &Timestamp, tz: &impl TimeZoneLike) -> bool {
    let gdate_local = gdate.to_zoned(TimeZone::fixed(tz.offset_at(*gdate))).date();
    let hdate_greg = JiffDate::convert_try_from(hdate.to_calendar(Gregorian)).ok();

    Some(gdate_local) == hdate_greg
}

pub(crate) fn months_molad(date: &Date<Hebrew>) -> Option<Timestamp> {
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

    let datetime = JiffDate::convert_try_from(gregorian_date)
        .ok()?
        .at(hours as i8, minutes as i8, seconds, nanos);

    // Molad is in Jerusalem standard time (GMT+2)
    let jerusalem_offset = TimeZone::fixed(tz::offset(2));
    let datetime_jerusalem = datetime.to_zoned(jerusalem_offset).ok()?;

    // Subtract local mean time offset (20.94 minutes = 1256.4 seconds)
    // Longitude of Har Habayis: 35.2354°
    // 35.2354° away from 35° (GMT+2 +  20 minutes) = 0.2354° = ~0.94 minutes
    // Total: 20 minutes 56.496 seconds ≈ 1256.496 seconds
    datetime_jerusalem
        .timestamp()
        .checked_sub(SignedDuration::from_millis(1_256_496))
        .ok()
}

fn abs_date_to_gregorian_date(abs_date: i64) -> Option<Date<Gregorian>> {
    let mut year = (abs_date / 366) as i32;
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

fn gregorian_date_to_abs_date(year: i32, month: u8, day_of_month: u8) -> Option<i64> {
    let mut abs_date = day_of_month as i64;
    for m in (1..month).rev() {
        abs_date += Date::try_new_gregorian(year, m, 1).ok()?.days_in_month() as i64;
    }
    let year: i64 = year as i64;
    Some(abs_date + 365 * (year - 1) + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400)
}
