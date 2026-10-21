//! Hebrew calendar extensions built on ICU4X.
//!
//! This module adds Jewish-calendar logic on top of [`icu_calendar::Date`] internally:
//! holidays, weekly Torah readings (*parshiyot*), month constants, and year-length
//! helpers. Callers typically use [`jiff::civil::Date`]; ICU and other date types also
//! work via [`HebrewCalendarDate`].
//!
//! # Submodules
//!
//! - [`month`] — named [`Month`] constants and Hebrew month names
//! - [`holiday`] — Yom Tov, fast days, and other observances for a Hebrew date
//! - [`parsha`] — [`Parsha`] enum and annual reading schedules
//!
//! # Quick start
//!
//! ```
//! use jiff::civil;
//! use kosher_rust::calendar::prelude::*;
//!
//! let date = civil::date(2023, 9, 25);
//! assert!(date.is_assur_bemelacha(false));
//! assert!(date.holidays(false, false).any(|h| h == Holiday::YomKippur));
//! ```

/// Named ICU [`Month`] values for every Hebrew month, including Adar I in leap years.
pub mod month;

/// Jewish holidays, fast days, and related calendar events.
pub mod holiday;

/// Weekly Torah portions (*parshiyot*) and special Shabbat designations.
pub mod parsha;

pub(crate) mod molad;
mod parsha_tables;
mod year;

/// Convenience re-exports for calendar traits, types, and month constants.
///
/// ```
/// use kosher_rust::calendar::prelude::*;
/// ```
pub mod prelude {
    pub use super::holiday::{Holiday, HolidayIterator};
    pub use super::month;
    pub use super::month::HebrewMonthExt;
    pub use super::parsha::Parsha;
    pub use super::{HebrewCalendar, HebrewCalendarDate, HebrewHolidayCalendar, YearLengthType};
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests;

use holiday::{Holiday, HolidayIterator};
use icu_calendar::options::DateAddOptions;
use icu_calendar::types::{DateDuration, Month, Weekday};
use icu_calendar::{AsCalendar, Date, Iso, cal::Hebrew};
use jiff_icu::ConvertFrom;
use month::*;
pub use parsha::Parsha;
use parsha_tables::get_parsha_list;
use year::get_hebrew_elapsed_days;
pub use year::{HebrewCalendar, YearLengthType};

/// Any date type that can be converted to a Hebrew calendar date.
///
/// Implemented for [`icu_calendar::Date`] (any calendar) and [`jiff::civil::Date`].
/// Other wrappers can implement this trait to gain [`HebrewHolidayCalendar`] for free.
pub trait HebrewCalendarDate {
    /// Returns this date converted to a [`icu_calendar::Date<Hebrew>`] date.
    fn hebrew_date(&self) -> Date<Hebrew>;
}

impl<C> HebrewCalendarDate for Date<C>
where
    C: AsCalendar,
{
    #[inline]
    fn hebrew_date(&self) -> Date<Hebrew> {
        self.to_calendar(Hebrew)
    }
}

impl HebrewCalendarDate for jiff::civil::Date {
    #[inline]
    fn hebrew_date(&self) -> Date<Hebrew> {
        let iso_date = Date::<Iso>::convert_from(*self);
        iso_date.to_calendar(Hebrew)
    }
}

/// Hebrew calendar queries for a date: holidays, parsha, and related observances.
///
/// Blanket-implemented for every [`HebrewCalendarDate`]. Most methods take
/// `in_israel` because diaspora communities observe an extra day of Yom Tov.
pub trait HebrewHolidayCalendar: HebrewCalendarDate {
    /// Iterator over holidays occurring on a specific date.
    type HolidayIter: Iterator<Item = Holiday>;

    /// Returns this date's month as an ICU [`Month`] (Tishrei-based numbering).
    fn input_month(&self) -> Month;

    /// Returns an iterator over holidays occurring on this date.
    ///
    /// # Arguments
    ///
    /// * `in_israel` - Whether to use Israeli customs (affects second day observances)
    /// * `use_modern_holidays` - Whether to include modern Israeli holidays
    fn holidays(&self, in_israel: bool, use_modern_holidays: bool) -> Self::HolidayIter;

    /// Returns whether work is forbidden (assur bemelacha) on this date.
    fn is_assur_bemelacha(&self, in_israel: bool) -> bool;

    /// Returns whether candle lighting should occur on this date (day before Yom Tov/Shabbat).
    fn has_candle_lighting(&self, in_israel: bool) -> bool;

    /// Returns whether this date falls during the Ten Days of Repentance.
    fn is_aseres_yemei_teshuva(&self) -> bool;

    /// Returns the weekly Torah reading (parsha) if this is Shabbat and not a holiday.
    fn todays_parsha(&self, in_israel: bool) -> Option<Parsha>;

    /// Returns the special Shabbat designation if applicable (e.g., Shekalim, Zachor).
    fn special_parsha(&self, in_israel: bool) -> Option<Parsha>;

    /// Returns the Torah reading for the next Shabbat on or after this date, skipping
    /// Shabbatot when Yom Tov replaces the regular weekly portion.
    ///
    /// If today is Shabbat, this targets the *next* Shabbat (seven days ahead), not
    /// today's reading; use [`Self::todays_parsha`] for the current Shabbat.
    ///
    /// This function will return `None` if the date is out of the bounds that
    /// `icu_calendar::Date<Hebrew>` can handle. Otherwise it will return the next Shabbat's parsha.
    fn upcoming_parsha(&self, in_israel: bool) -> Option<Parsha>;
}

impl<T> HebrewHolidayCalendar for T
where
    T: HebrewCalendarDate,
{
    type HolidayIter = HolidayIterator;

    #[inline]
    fn is_assur_bemelacha(&self, in_israel: bool) -> bool {
        self.hebrew_date().weekday() == Weekday::Saturday
            || self.holidays(in_israel, false).any(|i| i.is_assur_bemelacha())
    }
    #[inline]
    fn has_candle_lighting(&self, in_israel: bool) -> bool {
        self.hebrew_date()
            .try_added_with_options(DateDuration::for_days(1), DateAddOptions::default())
            .map(|next_day| next_day.is_assur_bemelacha(in_israel))
            .unwrap_or(false)
    }

    #[inline]
    fn input_month(&self) -> Month {
        self.hebrew_date().month().to_input()
    }

    #[inline]
    fn holidays(&self, in_israel: bool, use_modern_holidays: bool) -> Self::HolidayIter {
        HolidayIterator {
            iter: holiday::all_rules().iter(),
            date: self.hebrew_date(),
            in_israel,
            use_modern_holidays,
        }
    }

    fn is_aseres_yemei_teshuva(&self) -> bool {
        let date = self.hebrew_date();
        date.input_month() == TISHREI && date.day_of_month().0 <= 10
    }

    fn todays_parsha(&self, in_israel: bool) -> Option<Parsha> {
        let date = self.hebrew_date();
        if date.weekday() != Weekday::Saturday {
            return None;
        }

        let parsha_list = get_parsha_list(&date, in_israel)?;

        let rosh_hashana_day_of_week = get_hebrew_elapsed_days(date.year().extended_year())? % 7;
        let day = rosh_hashana_day_of_week + date.day_of_year().0 as i32;
        let week_index = usize::try_from(day / 7).ok()?;
        parsha_list.get(week_index).copied().flatten()
    }

    fn special_parsha(&self, in_israel: bool) -> Option<Parsha> {
        let date = self.hebrew_date();
        if date.weekday() != Weekday::Saturday {
            return None;
        }

        let month = date.input_month();
        let day = date.day_of_month().0;
        let is_leap = Hebrew::is_hebrew_leap_year(date.year().extended_year());

        // Shkalim
        if ((month == SHEVAT && !is_leap) || (month == ADARI && is_leap)) && (day == 25 || day == 27 || day == 29) {
            return Some(Parsha::Shekalim);
        }

        if month == ADAR {
            if day == 1 {
                return Some(Parsha::Shekalim);
            }
            // Zachor
            if day == 8 || day == 9 || day == 11 || day == 13 {
                return Some(Parsha::Zachor);
            }
            // Para
            if day == 18 || day == 20 || day == 22 || day == 23 {
                return Some(Parsha::Parah);
            }
            // Hachodesh
            if day == 25 || day == 27 || day == 29 {
                return Some(Parsha::Hachodesh);
            }
        }

        if month == NISAN {
            if day == 1 {
                return Some(Parsha::Hachodesh);
            }
            // Hagadol
            if (8..=14).contains(&day) {
                return Some(Parsha::Hagadol);
            }
        }

        if month == AV {
            // Chazon
            if (4..=9).contains(&day) {
                return Some(Parsha::Chazon);
            }
            // Nachamu
            if (10..=16).contains(&day) {
                return Some(Parsha::Nachamu);
            }
        }

        if month == TISHREI {
            // Shuva
            if (3..=8).contains(&day) {
                return Some(Parsha::Shuva);
            }
        }

        // Shira
        if self.todays_parsha(in_israel) == Some(Parsha::Beshalach) {
            return Some(Parsha::Shira);
        }

        None
    }

    fn upcoming_parsha(&self, in_israel: bool) -> Option<Parsha> {
        let days_to_shabbos = match self.hebrew_date().weekday() {
            Weekday::Monday => 5,
            Weekday::Tuesday => 4,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 2,
            Weekday::Friday => 1,
            Weekday::Saturday => 7,
            Weekday::Sunday => 6,
        };

        let mut date = self
            .hebrew_date()
            .try_added_with_options(DateDuration::for_days(days_to_shabbos), DateAddOptions::default())
            .ok()?;

        // Avoid an unbounded search near ICU's supported date limits.
        for _ in 0..60 {
            if let Some(parshah) = date.todays_parsha(in_israel) {
                return Some(parshah);
            }

            date = date
                .try_added_with_options(DateDuration::for_days(7), DateAddOptions::default())
                .ok()?;
        }

        None
    }
}
