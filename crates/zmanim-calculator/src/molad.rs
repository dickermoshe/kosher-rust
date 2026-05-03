use hebrew_holiday_calendar::{HebrewHolidayCalendar, HebrewMonth};
use icu_calendar::{
    cal::Hebrew,
    options::{DateAddOptions, Overflow},
    types::DateDuration,
    Date, Gregorian,
};
use jiff::{civil::Date as JiffDate, tz, tz::TimeZone, SignedDuration, Timestamp};
use jiff_icu::ConvertTryFrom;
/// Number of chalakim (parts) from the molad tohu (theoretical first new moon)
static CHALAKIM_MOLAD_TOHU: i64 = 31524;
/// Number of chalakim in a lunar month
static CHALAKIM_PER_MONTH: i64 = 765433;
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
pub(crate) trait MoladCalendar {
    /// Returns the latest time of _Kiddush Levana_ calculated as 15 days after the molad.
    ///
    /// # Returns
    /// - `Some((DateTime, HebrewMonth))` - The time representing 15 days after the molad and the Hebrew month
    /// - `None` - If the zman will not occur on this day
    //
    fn sof_zman_kidush_levana_15_days(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)>;

    /// Returns the earliest time of _Kiddush Levana_ according to
    /// [Rabbeinu Yonah](https://en.wikipedia.org/wiki/Yonah_Gerondi)'s opinion that it can be
    /// said 3 days after the molad.
    ///
    /// # Returns
    /// - `Some((DateTime, HebrewMonth))` - The time representing 3 days after the molad and the Hebrew month
    /// - `None` - If the zman will not occur on this day
    fn tchilas_zman_kidush_levana_3_days(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)>;

    /// Returns the earliest time of _Kiddush Levana_ according to the opinions that it should
    /// not be said until 7 days after the molad.
    ///
    /// # Returns
    /// - `Some((DateTime, HebrewMonth))` - The time representing 7 days after the molad and the Hebrew month
    /// - `None` - If the zman will not occur on this day
    fn tchilas_zman_kidush_levana_7_days(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)>;

    /// Returns the latest time of Kiddush Levana according to the
    /// [Maharil](https://en.wikipedia.org/wiki/Yaakov_ben_Moshe_Levi_Moelin)'s opinion that it
    /// is calculated as halfway between molad and molad.
    ///
    /// This adds half the 29 days, 12 hours and 793 chalakim time between molad and molad
    /// (14 days, 18 hours, 22 minutes and 666 milliseconds) to the month's molad.
    ///
    /// # Returns
    /// - `Some((DateTime, HebrewMonth))` - The time representing halfway between molad and molad and the Hebrew month
    /// - `None` - If the zman will not occur on this day
    fn sof_zman_kidush_levana_between_moldos(
        &self,
        tz: &TimeZone,
    ) -> Option<(Timestamp, HebrewMonth)>;

    /// Returns the time of the molad (new moon) for the current date's Hebrew month.
    ///
    /// The molad is the precise moment of the conjunction of the sun and moon.
    ///
    /// # Returns
    /// - `Some((DateTime, HebrewMonth))` - The time of the molad and the Hebrew month
    /// - `None` - If the zman will not occur on this day
    fn molad(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)>;
}

impl MoladCalendar for Date<Gregorian> {
    fn sof_zman_kidush_levana_15_days(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)> {
        let hebrew = self.to_calendar(Hebrew);

        if hebrew.day_of_month().0 < 11 || hebrew.day_of_month().0 > 17 {
            return None;
        }
        let molad = months_molad(&hebrew)? + SignedDuration::from_hours(24 * 15);
        if is_same_gregorian_day(&hebrew, &molad, tz) {
            return Some((molad, hebrew.hebrew_month()));
        }
        None
    }

    fn tchilas_zman_kidush_levana_3_days(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)> {
        let hebrew = self.to_calendar(Hebrew);

        if hebrew.day_of_month().0 > 5 && hebrew.day_of_month().0 < 30 {
            return None;
        }
        let molad = months_molad(&hebrew)? + SignedDuration::from_hours(72);

        if is_same_gregorian_day(&hebrew, &molad, tz) {
            return Some((molad, hebrew.hebrew_month()));
        }

        if hebrew.day_of_month().0 == 30 {
            let mut add_option = DateAddOptions::default();
            add_option.overflow = Some(Overflow::Constrain);

            let new = hebrew
                .try_added_with_options(DateDuration::for_months(1), add_option)
                .ok()?;
            let molad = months_molad(&new)? + SignedDuration::from_hours(72);
            if is_same_gregorian_day(&hebrew, &molad, tz) {
                return Some((molad, new.hebrew_month()));
            }
        }
        None
    }

    fn tchilas_zman_kidush_levana_7_days(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)> {
        let hebrew = self.to_calendar(Hebrew);

        if hebrew.day_of_month().0 < 4 || hebrew.day_of_month().0 > 9 {
            return None;
        }
        let molad = months_molad(&hebrew)? + SignedDuration::from_hours(168);
        if is_same_gregorian_day(&hebrew, &molad, tz) {
            return Some((molad, hebrew.hebrew_month()));
        }
        None
    }

    fn sof_zman_kidush_levana_between_moldos(
        &self,
        tz: &TimeZone,
    ) -> Option<(Timestamp, HebrewMonth)> {
        let hebrew = self.to_calendar(Hebrew);

        if hebrew.day_of_month().0 < 11 || hebrew.day_of_month().0 > 16 {
            return None;
        }
        let molad = months_molad(&hebrew)?
            + SignedDuration::from_hours(24 * 14 + 18)
            + SignedDuration::from_mins(22)
            + SignedDuration::from_secs(1)
            + SignedDuration::from_millis(666);
        if is_same_gregorian_day(&hebrew, &molad, tz) {
            return Some((molad, hebrew.hebrew_month()));
        }
        None
    }

    fn molad(&self, tz: &TimeZone) -> Option<(Timestamp, HebrewMonth)> {
        let hebrew = self.to_calendar(Hebrew);
        let day = hebrew.day_of_month().0;
        if day > 2 && day < 27 {
            return None;
        }
        let molad = months_molad(&hebrew)?;
        if !is_same_gregorian_day(&hebrew, &molad, tz) {
            let mut add_option = DateAddOptions::default();
            add_option.overflow = Some(Overflow::Constrain);
            if day > 26 {
                // Next month molad can fall on the current Gregorian day near month boundaries.
                let new = hebrew
                    .try_added_with_options(DateDuration::for_months(1), add_option)
                    .ok()?;
                let molad = months_molad(&new.to_calendar(Hebrew))?;
                if is_same_gregorian_day(&hebrew, &molad, tz) {
                    return Some((molad, new.hebrew_month()));
                }
            }
            return None;
        }
        Some((molad, hebrew.hebrew_month()))
    }
}

fn is_same_gregorian_day(hdate: &Date<Hebrew>, gdate: &Timestamp, tz: &TimeZone) -> bool {
    let gdate_local = gdate.to_zoned(tz.clone()).date();
    let gregorian_date = hdate.gregorian_date();
    let hdate_greg = JiffDate::convert_try_from(gregorian_date).ok();

    Some(gdate_local) == hdate_greg
}
#[derive(Debug, Clone)]
struct MoladData {
    hours: i64,
    minutes: i64,
    chalakim: i64,
    date: Date<Gregorian>,
}
fn _get_molad(date: &Date<Hebrew>) -> Option<MoladData> {
    let chalakim_since_molad_tohu =
        chalakim_since_molad_tohu(date.year().extended_year(), date.hebrew_month());
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
    Some(MoladData {
        date: gregorian_date,
        hours,
        minutes,
        chalakim,
    })
}
// Molad and Kiddush Levana
fn months_molad(date: &Date<Hebrew>) -> Option<Timestamp> {
    let molad_data = _get_molad(date)?;

    let molad_seconds = molad_data.chalakim as f64 * 10.0 / 3.0;
    let seconds = molad_seconds as i8;
    let nanos = ((molad_seconds - seconds as f64) * 1_000_000_000.0) as i32;

    let datetime = JiffDate::convert_try_from(molad_data.date).ok()?.at(
        molad_data.hours as i8,
        molad_data.minutes as i8,
        seconds,
        nanos,
    );

    // Molad is in Jerusalem standard time (GMT+2)
    let jerusalem_offset = TimeZone::fixed(tz::offset(2));
    let datetime_jerusalem = datetime.to_zoned(jerusalem_offset).ok()?;

    // Subtract local mean time offset (20.94 minutes = 1256.4 seconds)
    // Longitude of Har Habayis: 35.2354°
    // 35.2354° away from 35° (GMT+2 +  20 minutes) = 0.2354° = ~0.94 minutes
    // Total: 20 minutes 56.496 seconds ≈ 1256.496 seconds
    Some(datetime_jerusalem.timestamp() - SignedDuration::from_millis(1_256_496))
}

/// Returns the number of chalakim from the original hypothetical Molad Tohu
fn chalakim_since_molad_tohu(year: i32, month: HebrewMonth) -> i64 {
    let month_of_year = hebrew_month_of_year(year, month);
    let months_elapsed = (235 * ((year - 1) / 19))
        + (12 * ((year - 1) % 19))
        + ((7 * ((year - 1) % 19) + 1) / 19)
        + (month_of_year as i32 - 1);

    CHALAKIM_MOLAD_TOHU + (CHALAKIM_PER_MONTH * months_elapsed as i64)
}
/// Returns the `HebrewMonth` as a `u8` which is indexed starting from Tishrei
/// instead of Nissan.
fn hebrew_month_of_year(year: i32, month: HebrewMonth) -> u8 {
    let is_leap_year = Date::<Hebrew>::is_hebrew_leap_year(year);
    (month as u8 + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
}

fn abs_date_to_gregorian_date(abs_date: i64) -> Option<Date<Gregorian>> {
    let mut year = (abs_date / 366) as i32;
    while abs_date >= gregorian_date_to_abs_date(year + 1, 1, 1)? {
        year += 1;
    }
    let mut month: u8 = 1;
    while abs_date
        > gregorian_date_to_abs_date(year, month, get_last_day_of_gregorian_month(month, year)?)?
    {
        month += 1;
    }
    let day_of_month: u8 = (abs_date - gregorian_date_to_abs_date(year, month, 1)? + 1) as u8;
    Date::try_new_gregorian(year, month, day_of_month).ok()
}

fn gregorian_date_to_abs_date(year: i32, month: u8, day_of_month: u8) -> Option<i64> {
    let mut abs_date = day_of_month as i64;
    for m in (1..month).rev() {
        abs_date += get_last_day_of_gregorian_month(m, year)? as i64;
    }
    let year: i64 = year as i64;
    Some(abs_date + 365 * (year - 1) + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400)
}

fn get_last_day_of_gregorian_month(month: u8, year: i32) -> Option<u8> {
    let day = Date::<Gregorian>::try_new_gregorian(year, month, 1).ok()?;
    Some(day.days_in_month())
}

#[test]
fn test_chalakim_calculations() {
    // Test that chalakim are calculated consistently
    let chalakim1 = chalakim_since_molad_tohu(5784, HebrewMonth::Tishrei);
    let chalakim2 = chalakim_since_molad_tohu(5784, HebrewMonth::Cheshvan);

    // Next month should have more chalakim
    assert!(chalakim2 > chalakim1);

    // Difference should be approximately one month
    let diff = chalakim2 - chalakim1;
    assert!((diff - CHALAKIM_PER_MONTH).abs() < 100);
}
