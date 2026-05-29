use icu_calendar::cal::Hebrew;
use icu_calendar::types::Month;

use super::HebrewCalendar;

#[allow(missing_docs)]
pub const TISHREI: Month = Month::new(1);
#[allow(missing_docs)]
pub const ḤESHVAN: Month = Month::new(2);
#[allow(missing_docs)]
pub const KISLEV: Month = Month::new(3);
#[allow(missing_docs)]
pub const TEVET: Month = Month::new(4);
#[allow(missing_docs)]
pub const SHEVAT: Month = Month::new(5);
#[allow(missing_docs)]
pub const ADARI: Month = Month::leap(5);
#[allow(missing_docs)]
pub const ADAR: Month = Month::new(6);
#[allow(missing_docs)]
pub const NISAN: Month = Month::new(7);
#[allow(missing_docs)]
pub const IYYAR: Month = Month::new(8);
#[allow(missing_docs)]
pub const SIVAN: Month = Month::new(9);
#[allow(missing_docs)]
pub const TAMMUZ: Month = Month::new(10);
#[allow(missing_docs)]
pub const AV: Month = Month::new(11);
#[allow(missing_docs)]
pub const ELUL: Month = Month::new(12);

const COMMON_HEBREW_MONTHS: [Month; 12] = [
    TISHREI, ḤESHVAN, KISLEV, TEVET, SHEVAT, ADAR, NISAN, IYYAR, SIVAN, TAMMUZ, AV, ELUL,
];

const LEAP_HEBREW_MONTHS: [Month; 13] = [
    TISHREI, ḤESHVAN, KISLEV, TEVET, SHEVAT, ADARI, ADAR, NISAN, IYYAR, SIVAN, TAMMUZ, AV, ELUL,
];

/// Hebrew calendar month helpers for ICU [`Month`] values.
pub trait HebrewMonthExt {
    /// Returns the Hebrew name of the month.
    ///
    /// Will return None if the month is not valid for the given year
    /// or if the given month is not a Hebrew month
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
    /// Returns the Hebrew months in the given year starting from Tishrei.
    fn hebrew_months_in_year(year: i32) -> &'static [Month];

    /// Returns the Hebrew month number within the year, indexed from Tishrei.
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
