use icu_calendar::{
    Date, Gregorian,
    cal::Hebrew,
    options::{DateAddOptions, Overflow},
    types::{DateDuration, Month},
};
use jiff::{SignedDuration, Timestamp, civil::Date as JiffDate, tz, tz::TimeZone};
use jiff_icu::ConvertTryFrom;

use crate::{
    calendar::{chalakim_since_molad_tohu, prelude::*},
    zmanim::types::error::ZmanimError,
};

/// Number of chalakim in a day
static CHALAKIM_PER_DAY: i64 = 25920;
/// Number of chalakim in a minute
static CHALAKIM_PER_MINUTE: i64 = 18;
/// Number of chalakim in an hour
static CHALAKIM_PER_HOUR: i64 = 1080;
/// Number of chalakim in a day
static JEWISH_EPOCH: i64 = -1373429;

/// A trait for calculating times related to the molad (new moon) and Kiddush Levana (blessing of the new moon).
///
/// This trait provides methods for calculating various times related to the Jewish lunar month,
/// specifically for the mitzvah of Kiddush Levana.
pub trait MoladCalendar {
    /// Returns the time of the molad (new moon) for the current date's Hebrew month.
    ///
    /// The molad is the precise moment of the conjunction of the sun and moon.
    ///
    /// # Returns
    /// - `Ok((DateTime, Month))` - The time of the molad and the Hebrew month
    /// - `Err(ZmanimError::InvalidForDate)` - If the zman will not occur on this day
    /// - `Err(ZmanimError::TimeConversionError)` - If any time conversion fails
    fn molad(&self, tz: &TimeZone) -> Result<(Timestamp, Month), ZmanimError>;
}

impl<T> MoladCalendar for T
where
    T: HebrewCalendarDate,
{
    fn molad(&self, tz: &TimeZone) -> Result<(Timestamp, Month), ZmanimError> {
        let hebrew = self.hebrew_date();
        let day = hebrew.day_of_month().0;
        if day > 2 && day < 27 {
            return Err(ZmanimError::InvalidForDate);
        }
        let molad = months_molad(&hebrew)?;
        if !is_same_gregorian_day(&hebrew, &molad, tz) {
            let mut add_option = DateAddOptions::default();
            add_option.overflow = Some(Overflow::Constrain);
            if day > 26 {
                // Next month molad can fall on the current Gregorian day near month boundaries.
                let new = hebrew
                    .try_added_with_options(DateDuration::for_months(1), add_option)
                    .map_err(|_| ZmanimError::TimeConversionError)?;
                let molad = months_molad(&new.to_calendar(Hebrew))?;
                if is_same_gregorian_day(&hebrew, &molad, tz) {
                    return Ok((molad, new.input_month()));
                }
            }
            return Err(ZmanimError::InvalidForDate);
        }
        Ok((molad, hebrew.input_month()))
    }
}

pub(crate) fn is_same_gregorian_day(hdate: &Date<Hebrew>, gdate: &Timestamp, tz: &TimeZone) -> bool {
    let gdate_local = gdate.to_zoned(tz.clone()).date();
    let hdate_greg = JiffDate::convert_try_from(hdate.to_calendar(Gregorian)).ok();

    Some(gdate_local) == hdate_greg
}

pub(crate) fn months_molad(date: &Date<Hebrew>) -> Result<Timestamp, ZmanimError> {
    let chalakim_since_molad_tohu = chalakim_since_molad_tohu(date.year().extended_year(), date.input_month())
        .ok_or(ZmanimError::TimeConversionError)?;
    let abs_date = JEWISH_EPOCH + (chalakim_since_molad_tohu / CHALAKIM_PER_DAY);
    let mut gregorian_date = abs_date_to_gregorian_date(abs_date).ok_or(ZmanimError::TimeConversionError)?;
    let conjunction_day = chalakim_since_molad_tohu / CHALAKIM_PER_DAY;
    let conjunction_parts = chalakim_since_molad_tohu - conjunction_day * CHALAKIM_PER_DAY;
    let mut hours = conjunction_parts / CHALAKIM_PER_HOUR;
    let adjusted_conjunction_parts = conjunction_parts - (hours * CHALAKIM_PER_HOUR);
    let minutes = adjusted_conjunction_parts / CHALAKIM_PER_MINUTE;
    let chalakim = adjusted_conjunction_parts - (minutes * CHALAKIM_PER_MINUTE);
    if hours >= 6 {
        gregorian_date
            .try_add_with_options(DateDuration::for_days(1), DateAddOptions::default())
            .map_err(|_| ZmanimError::TimeConversionError)?;
    }
    hours = (hours + 18) % 24;

    let molad_seconds = chalakim as f64 * 10.0 / 3.0;
    let seconds = molad_seconds as i8;
    let nanos = ((molad_seconds - seconds as f64) * 1_000_000_000.0) as i32;

    let datetime = JiffDate::convert_try_from(gregorian_date)
        .map_err(|_| ZmanimError::TimeConversionError)?
        .at(hours as i8, minutes as i8, seconds, nanos);

    // Molad is in Jerusalem standard time (GMT+2)
    let jerusalem_offset = TimeZone::fixed(tz::offset(2));
    let datetime_jerusalem = datetime
        .to_zoned(jerusalem_offset)
        .map_err(|_| ZmanimError::TimeConversionError)?;

    // Subtract local mean time offset (20.94 minutes = 1256.4 seconds)
    // Longitude of Har Habayis: 35.2354°
    // 35.2354° away from 35° (GMT+2 +  20 minutes) = 0.2354° = ~0.94 minutes
    // Total: 20 minutes 56.496 seconds ≈ 1256.496 seconds
    Ok(datetime_jerusalem.timestamp() - SignedDuration::from_millis(1_256_496))
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
