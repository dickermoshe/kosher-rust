use icu_calendar::{cal::Hebrew, types::Month};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::molad::{CHALAKIM_PER_DAY, chalakim_since_molad_tohu};
use super::month::*;

/// Represents the length type of a Hebrew year based on Cheshvan and Kislev.
///
/// - `Chaserim`: Both months are short (Cheshvan 29 days, Kislev 29 days)
/// - `Kesidran`: Normal length (Cheshvan 29 days, Kislev 30 days)
/// - `Shelaimim`: Both months are long (Cheshvan 30 days, Kislev 30 days)
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum YearLengthType {
    /// Cheshvan and Kislev are both short (29 days).
    Chaserim = 0,
    /// Cheshvan is short and Kislev is long (typical year).
    Kesidran = 1,
    /// Cheshvan and Kislev are both long (30 days).
    Shelaimim = 2,
}

/// Returns elapsed days through Rosh Hashana of `year`, including the standard
/// molad postponement rules.
pub(crate) fn get_hebrew_elapsed_days(year: i32) -> Option<i32> {
    let chalakim_since = chalakim_since_molad_tohu(year, TISHREI)?;
    let molad_day = chalakim_since / CHALAKIM_PER_DAY;
    let molad_parts = chalakim_since - molad_day * CHALAKIM_PER_DAY;
    let mut rosh_hashana_day = molad_day;

    if (molad_parts >= 19440)
        || (((molad_day % 7) == 2) && (molad_parts >= 9924) && !Hebrew::is_hebrew_leap_year(year))
        || (((molad_day % 7) == 1) && (molad_parts >= 16789) && (Hebrew::is_hebrew_leap_year(year - 1)))
    {
        rosh_hashana_day += 1;
    }

    if ((rosh_hashana_day % 7) == 0) || ((rosh_hashana_day % 7) == 3) || ((rosh_hashana_day % 7) == 5) {
        rosh_hashana_day += 1;
    }

    Some(rosh_hashana_day as i32)
}

/// Year-level Hebrew calendar calculations for ICU [`Hebrew`].
///
/// Import this trait to call `Hebrew::days_in_hebrew_year(...)` and related helpers.
pub trait HebrewCalendar {
    /// Returns the number of days in the given Hebrew year.
    fn days_in_hebrew_year(year: i32) -> Option<i32>;

    /// Returns the number of days in the given Hebrew month for a specific year.
    ///
    /// Returns `None` if `month` is not valid for `year`.
    fn days_in_hebrew_month(year: i32, month: Month) -> Option<u8>;

    /// Returns whether Cheshvan has 30 days (long) in the given year.
    fn is_cheshvan_long(year: i32) -> Option<bool>;

    /// Returns whether Kislev has 29 days (short) in the given year.
    fn is_kislev_short(year: i32) -> Option<bool>;

    /// Returns whether the given year is a Hebrew leap year.
    fn is_hebrew_leap_year(year: i32) -> bool;

    /// Returns the year type based on the lengths of Cheshvan and Kislev.
    fn cheshvan_kislev_kviah(year: i32) -> Option<YearLengthType>;
}

impl HebrewCalendar for Hebrew {
    #[inline]
    fn days_in_hebrew_year(year: i32) -> Option<i32> {
        Some(get_hebrew_elapsed_days(year + 1)? - get_hebrew_elapsed_days(year)?)
    }

    #[inline]
    fn days_in_hebrew_month(year: i32, month: Month) -> Option<u8> {
        month.hebrew_month_of_year(year)?;

        Some(match month {
            IYYAR | TAMMUZ | ELUL | TEVET => 29,
            ḤESHVAN if Self::is_cheshvan_long(year)? => 30,
            ḤESHVAN => 29,
            KISLEV if Self::is_kislev_short(year)? => 29,
            KISLEV => 30,
            ADARI => 30,
            ADAR => 29,
            TISHREI | SHEVAT | NISAN | SIVAN | AV => 30,
            _ => return None,
        })
    }

    #[inline]
    fn is_cheshvan_long(year: i32) -> Option<bool> {
        Some(Self::days_in_hebrew_year(year)? % 10 == 5)
    }

    #[inline]
    fn is_kislev_short(year: i32) -> Option<bool> {
        Some(Self::days_in_hebrew_year(year)? % 10 == 3)
    }

    #[inline]
    fn is_hebrew_leap_year(year: i32) -> bool {
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }

    #[inline]
    fn cheshvan_kislev_kviah(year: i32) -> Option<YearLengthType> {
        Some(if Self::is_cheshvan_long(year)? && !Self::is_kislev_short(year)? {
            YearLengthType::Shelaimim
        } else if !Self::is_cheshvan_long(year)? && Self::is_kislev_short(year)? {
            YearLengthType::Chaserim
        } else {
            YearLengthType::Kesidran
        })
    }
}
