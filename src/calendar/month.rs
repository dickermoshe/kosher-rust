//! Hebrew month constants and helpers for ICU [`Month`] values.
//!
//! Each month is a stable [`Month`] constant (for example [`NISAN`] or [`ADARI`])
//! that can be passed to ICU date constructors and calendar logic throughout this
//! crate. Use [`HebrewMonthExt::he`] for the Hebrew month name in a given year.

use icu_calendar::cal::Hebrew;
use icu_calendar::types::Month;

use super::HebrewCalendar;

/// Tishrei (month 1).
pub const TISHREI: Month = Month::new(1);
/// Cheshvan (month 2).
pub const ḤESHVAN: Month = Month::new(2);
/// Kislev (month 3).
pub const KISLEV: Month = Month::new(3);
/// Tevet (month 4).
pub const TEVET: Month = Month::new(4);
/// Shevat (month 5).
pub const SHEVAT: Month = Month::new(5);
/// Adar I — present only in leap years (month 5, leap variant).
pub const ADARI: Month = Month::leap(5);
/// Adar — Adar II in leap years, plain Adar otherwise (month 6).
pub const ADAR: Month = Month::new(6);
/// Nisan (month 7).
pub const NISAN: Month = Month::new(7);
/// Iyyar (month 8).
pub const IYYAR: Month = Month::new(8);
/// Sivan (month 9).
pub const SIVAN: Month = Month::new(9);
/// Tammuz (month 10).
pub const TAMMUZ: Month = Month::new(10);
/// Av (month 11).
pub const AV: Month = Month::new(11);
/// Elul (month 12).
pub const ELUL: Month = Month::new(12);

const COMMON_HEBREW_MONTHS: [Month; 12] = [
    TISHREI, ḤESHVAN, KISLEV, TEVET, SHEVAT, ADAR, NISAN, IYYAR, SIVAN, TAMMUZ, AV, ELUL,
];

const LEAP_HEBREW_MONTHS: [Month; 13] = [
    TISHREI, ḤESHVAN, KISLEV, TEVET, SHEVAT, ADARI, ADAR, NISAN, IYYAR, SIVAN, TAMMUZ, AV, ELUL,
];

/// Hebrew names and year-relative validity for ICU [`Month`] values.
pub trait HebrewMonthExt {
    /// Returns the Hebrew name of this month in `year`.
    ///
    /// Returns `None` if `self` is not a Hebrew month or is not valid for `year`
    /// (for example [`ADARI`] in a non-leap year).
    fn he(&self, year: i32) -> Option<&'static str>;
}

impl HebrewMonthExt for Month {
    fn he(&self, year: i32) -> Option<&'static str> {
        if !Self::hebrew_months_in_year(year).contains(self) {
            return None;
        }

        let result = match *self {
            NISAN => "ניסן",
            IYYAR => "אייר",
            SIVAN => "סיון",
            TAMMUZ => "תמוז",
            AV => "אב",
            ELUL => "אלול",
            TISHREI => "תשרי",
            ḤESHVAN => "חשון",
            KISLEV => "כסלו",
            TEVET => "טבת",
            SHEVAT => "שבט",
            ADARI => "אדר א",
            ADAR => {
                if Hebrew::is_hebrew_leap_year(year) {
                    "אדר ב"
                } else {
                    "אדר"
                }
            }
            _ => return None,
        };
        Some(result)
    }
}

pub(crate) trait HebrewMonthHelpers {
    /// Returns the ordered list of months in `year`, starting from Tishrei.
    fn hebrew_months_in_year(year: i32) -> &'static [Month];

    /// Returns this month's 1-based index within `year`, counting from Tishrei.
    fn hebrew_month_of_year(self, year: i32) -> Option<u8>;
}

impl HebrewMonthHelpers for Month {
    fn hebrew_months_in_year(year: i32) -> &'static [Month] {
        if Hebrew::is_hebrew_leap_year(year) {
            &LEAP_HEBREW_MONTHS
        } else {
            &COMMON_HEBREW_MONTHS
        }
    }

    fn hebrew_month_of_year(self, year: i32) -> Option<u8> {
        Self::hebrew_months_in_year(year)
            .iter()
            .position(|candidate| *candidate == self)
            .map(|index| index as u8 + 1)
    }
}
