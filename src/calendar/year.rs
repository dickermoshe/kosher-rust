use core::fmt;

use super::date::HebrewDate;
use super::molad::{CHALAKIM_PER_DAY, chalakim_at_month_index};
use super::month::HebrewMonth;

/// The first Hebrew year a [`HebrewDate`] covers. Its first day is 1 January of year 1, which is
/// 18 Teves 3761.
pub(crate) const FIRST_YEAR: i32 = 3761;
/// The last Hebrew year a [`HebrewDate`] covers, the last the calendar library builds dates for.
pub(crate) const LAST_YEAR: i32 = 9999;

/// A year of the Hebrew calendar, counted from creation (5784 began in September 2023).
///
/// Get one from [`HebrewDate::year`] or [`HebrewYear::new`]. It covers the years 3761 to 9999, the
/// same range as [`HebrewDate`].
///
/// ```
/// use kosher_rust::calendar::{HebrewYear, YearLength};
///
/// let year = HebrewYear::new(5784).unwrap();
/// assert!(year.is_leap());
/// assert_eq!(year.days(), 383);
/// assert_eq!(year.length(), YearLength::Chaserim);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct HebrewYear(i32);

/// Whether Cheshvan and Kislev are long or short in a year, which sets its length.
///
/// Called the year's *kviah* along with the weekday of Rosh Hashana.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum YearLength {
    /// Deficient: Cheshvan and Kislev both have 29 days (353 or 383 days in the year).
    Chaserim,
    /// Regular: Cheshvan has 29 days and Kislev 30 (354 or 384 days).
    Kesidran,
    /// Complete: Cheshvan and Kislev both have 30 days (355 or 385 days).
    Shelaimim,
}

impl HebrewYear {
    /// The Hebrew year `year`, for example `HebrewYear::new(5785)`.
    ///
    /// Returns `None` outside the years 3761 to 9999.
    pub const fn new(year: i32) -> Option<Self> {
        if year >= FIRST_YEAR && year <= LAST_YEAR {
            Some(Self(year))
        } else {
            None
        }
    }

    /// The year `year`, without checking the range. The arithmetic here holds for any year from 1.
    pub(crate) const fn unchecked(year: i32) -> Self {
        Self(year)
    }

    /// The year as a number.
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Whether this year has thirteen months (years 3, 6, 8, 11, 14, 17 and 19 of the 19-year cycle).
    pub const fn is_leap(self) -> bool {
        let year_in_cycle = ((self.0 as i64 - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }

    /// The number of days in the year: 353, 354 or 355, or 383, 384 or 385 in a leap year.
    pub fn days(self) -> u16 {
        (elapsed_days(self.0 as i64 + 1) - elapsed_days(self.0 as i64)) as u16
    }

    /// Whether Cheshvan and Kislev are long or short this year.
    pub fn length(self) -> YearLength {
        match (self.is_cheshvan_long(), self.is_kislev_short()) {
            (true, false) => YearLength::Shelaimim,
            (false, true) => YearLength::Chaserim,
            _ => YearLength::Kesidran,
        }
    }

    /// Whether Cheshvan has 30 days this year.
    pub fn is_cheshvan_long(self) -> bool {
        self.days() % 10 == 5
    }

    /// Whether Kislev has 29 days this year.
    pub fn is_kislev_short(self) -> bool {
        self.days() % 10 == 3
    }

    /// The months of this year in order from Tishrei: 12, or 13 in a leap year.
    pub const fn months(self) -> &'static [HebrewMonth] {
        if self.is_leap() {
            &HebrewMonth::LEAP_YEAR
        } else {
            &HebrewMonth::COMMON_YEAR
        }
    }

    /// The number of days in `month` this year.
    ///
    /// Returns `None` for [`HebrewMonth::AdarI`] in a common year.
    pub fn days_in(self, month: HebrewMonth) -> Option<u8> {
        Some(match month {
            HebrewMonth::AdarI if !self.is_leap() => return None,
            HebrewMonth::Cheshvan if self.is_cheshvan_long() => 30,
            HebrewMonth::Kislev if self.is_kislev_short() => 29,
            HebrewMonth::Iyar
            | HebrewMonth::Tammuz
            | HebrewMonth::Elul
            | HebrewMonth::Teves
            | HebrewMonth::Adar
            | HebrewMonth::Cheshvan => 29,
            HebrewMonth::Tishrei
            | HebrewMonth::Kislev
            | HebrewMonth::Shevat
            | HebrewMonth::AdarI
            | HebrewMonth::Nisan
            | HebrewMonth::Sivan
            | HebrewMonth::Av => 30,
        })
    }

    /// The first day of the year, 1 Tishrei.
    ///
    /// Returns `None` for 3761, whose Rosh Hashana is before the first day a [`HebrewDate`] covers.
    pub fn rosh_hashana(self) -> Option<HebrewDate> {
        HebrewDate::new(self.0, HebrewMonth::Tishrei, 1)
    }
}

impl From<HebrewYear> for i32 {
    fn from(year: HebrewYear) -> Self {
        year.0
    }
}

impl fmt::Display for HebrewYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// Days from the epoch to Rosh Hashana of `year`. Rosh Hashana is the day of the molad of
/// Tishrei, postponed by the four *dechiyos*.
pub(crate) fn elapsed_days(year: i64) -> i64 {
    let chalakim_since = chalakim_at_month_index(year, 1);
    let molad_day = chalakim_since / CHALAKIM_PER_DAY;
    let molad_parts = chalakim_since - molad_day * CHALAKIM_PER_DAY;
    let mut rosh_hashana_day = molad_day;

    let is_leap = |year: i64| HebrewYear::unchecked(year as i32).is_leap();
    // Molad zaken, and GaTaRaD / BeTUTaKPaT.
    if (molad_parts >= 19440)
        || (((molad_day % 7) == 2) && (molad_parts >= 9924) && !is_leap(year))
        || (((molad_day % 7) == 1) && (molad_parts >= 16789) && is_leap(year - 1))
    {
        rosh_hashana_day += 1;
    }

    // Lo ADU Rosh: never on Sunday, Wednesday or Friday.
    if ((rosh_hashana_day % 7) == 0) || ((rosh_hashana_day % 7) == 3) || ((rosh_hashana_day % 7) == 5) {
        rosh_hashana_day += 1;
    }

    rosh_hashana_day
}
