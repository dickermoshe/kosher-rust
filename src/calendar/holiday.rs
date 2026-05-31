//! Jewish holidays, fast days, and related calendar events.
//!
//! Use [`HebrewHolidayCalendar::holidays`] on any [`HebrewCalendarDate`] to see
//! which observances fall on a date. Israel vs. diaspora customs and modern
//! Israeli holidays are controlled by the method arguments.

use core::ops::RangeInclusive;
use core::slice::Iter;
use icu_calendar::types::{Month, Weekday};
use icu_calendar::{Date, cal::Hebrew};

use super::month::*;
use super::{HebrewCalendar, HebrewHolidayCalendar, get_hebrew_elapsed_days};

/// Iterator over [`Holiday`] values that occur on a given Hebrew date.
pub struct HolidayIterator {
    pub(super) iter: Iter<'static, HolidayRule>,
    pub(super) date: Date<Hebrew>,
    pub(super) in_israel: bool,
    pub(super) use_modern_holidays: bool,
}

impl Iterator for HolidayIterator {
    type Item = Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let rule = self.iter.next()?;
            if let Some(holiday) = rule.for_date(&self.date, self.in_israel)
                && (self.use_modern_holidays || !holiday.is_modern_holiday())
            {
                return Some(holiday);
            }
        }
    }
}
impl core::iter::FusedIterator for HolidayIterator {}

/// Represents Jewish holidays and special days in the Hebrew calendar.
///
/// There are many holidays which wouldn't really be considered "holidays" in halacha.
/// However, we kept the naming here for backwards compatibility reasons.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[allow(missing_docs)]
pub enum Holiday {
    ErevPesach,
    Pesach,
    CholHamoedPesach,
    PesachSheni,
    ErevShavuos,
    Shavuos,
    SeventeenthOfTammuz,
    TishahBav,
    TuBav,
    ErevRoshHashana,
    RoshHashana,
    FastOfGedalyah,
    ErevYomKippur,
    YomKippur,
    ErevSuccos,
    Succos,
    CholHamoedSuccos,
    HoshanaRabbah,
    SheminiAtzeres,
    SimchasTorah,
    /// Chanukah. The inner value is the day of Chanukah, from 1 through 8.
    Chanukah(u8),
    TenthOfTeves,
    TuBshvat,
    FastOfEsther,
    Purim,
    ShushanPurim,
    PurimKatan,
    RoshChodesh,
    YomHaShoah,
    YomHazikaron,
    YomHaatzmaut,
    YomYerushalayim,
    LagBomer,
    ShushanPurimKatan,
    IsruChag,
    YomKippurKatan,
    Behab,
    FastOfTheFirstborn,
    /// Counting the Omer. The inner value is the day of the Omer count, from 1 through 49.
    CountOfTheOmer(u8),
    BirchasHachamah,
    MacharHachodesh,
    ShabbosMevarchim,
}

/// Internal type representing different rules for when holidays occur.
pub(super) enum HolidayRule {
    ExactDate(Holiday, u8, Month),
    ExactDates(Holiday, RangeInclusive<u8>, Month),
    ExactDateChutz(Holiday, u8, Month),
    ExactDateIsrael(Holiday, u8, Month),
    Group(&'static [HolidayRule]),
    Custom(fn(&Date<Hebrew>, bool) -> Option<Holiday>),
}

impl HolidayRule {
    fn for_date(&self, date: &Date<Hebrew>, in_israel: bool) -> Option<Holiday> {
        match self {
            HolidayRule::ExactDate(holiday, day, month) => {
                if date.day_of_month().0 == *day && date.input_month() == *month {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::ExactDates(holiday, range, month) => {
                if range.contains(&date.day_of_month().0) && date.input_month() == *month {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::ExactDateChutz(holiday, day, month) => {
                if date.day_of_month().0 == *day && date.input_month() == *month && !in_israel {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::ExactDateIsrael(holiday, day, month) => {
                if date.day_of_month().0 == *day && date.input_month() == *month && in_israel {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::Group(rules) => rules.iter().find_map(|rule| rule.for_date(date, in_israel)),
            HolidayRule::Custom(func) => func(date, in_israel),
        }
    }
}

static HOLIDAY_RULES: [HolidayRule; 42] = [
    HolidayRule::ExactDate(Holiday::ErevPesach, 14, NISAN),
    HolidayRule::Group(&[
        HolidayRule::ExactDate(Holiday::Pesach, 15, NISAN),
        HolidayRule::ExactDate(Holiday::Pesach, 21, NISAN),
        HolidayRule::ExactDateChutz(Holiday::Pesach, 16, NISAN),
        HolidayRule::ExactDateChutz(Holiday::Pesach, 22, NISAN),
    ]),
    HolidayRule::Group(&[
        HolidayRule::ExactDateIsrael(Holiday::CholHamoedPesach, 16, NISAN),
        HolidayRule::ExactDates(Holiday::CholHamoedPesach, 17..=20, NISAN),
    ]),
    HolidayRule::ExactDate(Holiday::PesachSheni, 14, IYYAR),
    HolidayRule::ExactDate(Holiday::ErevShavuos, 5, SIVAN),
    HolidayRule::Group(&[
        HolidayRule::ExactDate(Holiday::Shavuos, 6, SIVAN),
        HolidayRule::ExactDateChutz(Holiday::Shavuos, 7, SIVAN),
    ]),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != TAMMUZ {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 17 && day_of_week != Weekday::Saturday) || (day == 18 && day_of_week == Weekday::Sunday) {
            Some(Holiday::SeventeenthOfTammuz)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != AV {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day_of_week == Weekday::Sunday && day == 10) || (day_of_week != Weekday::Saturday && day == 9) {
            Some(Holiday::TishahBav)
        } else {
            None
        }
    }),
    HolidayRule::ExactDate(Holiday::TuBav, 15, AV),
    HolidayRule::ExactDate(Holiday::ErevRoshHashana, 29, ELUL),
    HolidayRule::ExactDates(Holiday::RoshHashana, 1..=2, TISHREI),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != TISHREI {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 3 && day_of_week != Weekday::Saturday) || (day == 4 && day_of_week == Weekday::Sunday) {
            Some(Holiday::FastOfGedalyah)
        } else {
            None
        }
    }),
    HolidayRule::ExactDate(Holiday::ErevYomKippur, 9, TISHREI),
    HolidayRule::ExactDate(Holiday::YomKippur, 10, TISHREI),
    HolidayRule::ExactDate(Holiday::ErevSuccos, 14, TISHREI),
    HolidayRule::Group(&[
        HolidayRule::ExactDate(Holiday::Succos, 15, TISHREI),
        HolidayRule::ExactDateChutz(Holiday::Succos, 16, TISHREI),
    ]),
    HolidayRule::Group(&[
        HolidayRule::ExactDateIsrael(Holiday::CholHamoedSuccos, 16, TISHREI),
        HolidayRule::ExactDates(Holiday::CholHamoedSuccos, 17..=20, TISHREI),
    ]),
    HolidayRule::ExactDate(Holiday::HoshanaRabbah, 21, TISHREI),
    HolidayRule::ExactDate(Holiday::SheminiAtzeres, 22, TISHREI),
    HolidayRule::ExactDateChutz(Holiday::SimchasTorah, 23, TISHREI),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.input_month();
        let day = date.day_of_month().0;

        let day_of_chanukah = if month == KISLEV && day >= 25 {
            day - 24
        } else if month == TEVET {
            let is_kislev_short = Hebrew::is_kislev_short(date.year().extended_year())?;
            let max_teves_day = if is_kislev_short { 3 } else { 2 };
            if day <= max_teves_day {
                if is_kislev_short { day + 5 } else { day + 6 }
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some(Holiday::Chanukah(day_of_chanukah))
    }),
    HolidayRule::ExactDate(Holiday::TenthOfTeves, 10, TEVET),
    HolidayRule::ExactDate(Holiday::TuBshvat, 15, SHEVAT),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != ADAR {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if ((day == 11 || day == 12) && day_of_week == Weekday::Thursday)
            || (day == 13 && !(day_of_week == Weekday::Friday || day_of_week == Weekday::Saturday))
        {
            Some(Holiday::FastOfEsther)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.input_month();
        let day = date.day_of_month().0;
        if month == ADAR && day == 14 {
            Some(Holiday::Purim)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.input_month();
        let day = date.day_of_month().0;
        if month == ADAR && day == 15 {
            Some(Holiday::ShushanPurim)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 14 {
            Some(Holiday::PurimKatan)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if (date.day_of_month().0 == 1 && date.input_month() != TISHREI) || date.day_of_month().0 == 30 {
            Some(Holiday::RoshChodesh)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != NISAN {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 26 && day_of_week == Weekday::Thursday)
            || (day == 28 && day_of_week == Weekday::Monday)
            || (day == 27 && day_of_week != Weekday::Sunday && day_of_week != Weekday::Friday)
        {
            Some(Holiday::YomHaShoah)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != IYYAR {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 4 && day_of_week == Weekday::Tuesday)
            || ((day == 3 || day == 2) && day_of_week == Weekday::Wednesday)
            || (day == 5 && day_of_week == Weekday::Monday)
        {
            Some(Holiday::YomHazikaron)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() != IYYAR {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 5 && day_of_week == Weekday::Wednesday)
            || ((day == 4 || day == 3) && day_of_week == Weekday::Thursday)
            || (day == 6 && day_of_week == Weekday::Tuesday)
        {
            Some(Holiday::YomHaatzmaut)
        } else {
            None
        }
    }),
    HolidayRule::ExactDate(Holiday::YomYerushalayim, 28, IYYAR),
    HolidayRule::ExactDate(Holiday::LagBomer, 18, IYYAR),
    HolidayRule::Custom(|date, _in_israel| {
        if date.input_month() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 15 {
            Some(Holiday::ShushanPurimKatan)
        } else {
            None
        }
    }),
    HolidayRule::Group(&[
        HolidayRule::ExactDateIsrael(Holiday::IsruChag, 22, NISAN),
        HolidayRule::ExactDateChutz(Holiday::IsruChag, 23, NISAN),
        HolidayRule::ExactDateIsrael(Holiday::IsruChag, 7, SIVAN),
        HolidayRule::ExactDateChutz(Holiday::IsruChag, 8, SIVAN),
        HolidayRule::ExactDateIsrael(Holiday::IsruChag, 23, TISHREI),
        HolidayRule::ExactDateChutz(Holiday::IsruChag, 24, TISHREI),
    ]),
    HolidayRule::Custom(|date, _in_israel| {
        let day_of_week = date.weekday();
        let month = date.input_month();
        let day = date.day_of_month().0;

        if matches!(month, ELUL | TISHREI | KISLEV | NISAN) {
            return None;
        }

        if day == 29 && day_of_week != Weekday::Friday && day_of_week != Weekday::Saturday {
            return Some(Holiday::YomKippurKatan);
        }

        if (day == 27 || day == 28) && day_of_week == Weekday::Thursday {
            Some(Holiday::YomKippurKatan)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let day_of_week = date.weekday();
        let month = date.input_month();
        let day = date.day_of_month().0;

        if (month == ḤESHVAN || month == IYYAR)
            && ((day_of_week == Weekday::Monday && day > 4 && day < 18)
                || (day_of_week == Weekday::Thursday && day > 7 && day < 14))
        {
            return Some(Holiday::Behab);
        }
        None
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.input_month();
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if month == NISAN
            && ((day == 14 && day_of_week != Weekday::Saturday) || (day == 12 && day_of_week == Weekday::Thursday))
        {
            Some(Holiday::FastOfTheFirstborn)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.input_month();
        let day = date.day_of_month().0;

        let day_of_omer = if month == NISAN && day >= 16 {
            day - 15
        } else if month == IYYAR {
            day + 15
        } else if month == SIVAN && day < 6 {
            day + 44
        } else {
            return None;
        };

        Some(Holiday::CountOfTheOmer(day_of_omer))
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let elapsed_days = get_hebrew_elapsed_days(date.year().extended_year())?;
        let elapsed_days = elapsed_days + date.day_of_year().0 as i32;
        let cycle_length = 10227i32;
        if (elapsed_days % cycle_length) == 172 {
            Some(Holiday::BirchasHachamah)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.weekday() == Weekday::Saturday && (date.day_of_month().0 == 30 || date.day_of_month().0 == 29) {
            Some(Holiday::MacharHachodesh)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.weekday() == Weekday::Saturday
            && date.day_of_month().0 >= 23
            && date.day_of_month().0 <= 29
            && date.input_month() != ELUL
        {
            Some(Holiday::ShabbosMevarchim)
        } else {
            None
        }
    }),
];

/// Returns all holiday rules used to evaluate which holidays fall on a date.
pub(super) const fn all_rules() -> &'static [HolidayRule; 42] {
    &HOLIDAY_RULES
}

impl Holiday {
    /// Returns whether work is forbidden (assur bemelacha) on this holiday.
    pub fn is_assur_bemelacha(&self) -> bool {
        matches!(
            self,
            Holiday::Pesach
                | Holiday::Shavuos
                | Holiday::Succos
                | Holiday::SheminiAtzeres
                | Holiday::SimchasTorah
                | Holiday::RoshHashana
                | Holiday::YomKippur
        )
    }

    /// Returns whether this is a modern Israeli holiday.
    pub fn is_modern_holiday(&self) -> bool {
        matches!(
            self,
            Holiday::YomHaShoah | Holiday::YomHazikaron | Holiday::YomHaatzmaut | Holiday::YomYerushalayim
        )
    }

    /// Returns whether this holiday is a communal fast day.
    ///
    /// [`Holiday::FastOfTheFirstborn`] is excluded: it applies only to
    /// firstborn males, not to the community at large.
    pub fn is_fast_day(&self) -> bool {
        matches!(
            self,
            Holiday::FastOfGedalyah
                | Holiday::FastOfEsther
                | Holiday::TishahBav
                | Holiday::TenthOfTeves
                | Holiday::SeventeenthOfTammuz
                | Holiday::YomKippur
        )
    }

    /// Returns the Hebrew name of the holiday.
    ///
    /// The day of Chanukah and the day of the Omer are not included in this
    /// name (e.g. [`Holiday::Chanukah`] returns `"חנוכה"`, not `"חנוכה א׳"`).
    pub fn he(&self) -> &str {
        match self {
            Holiday::ErevPesach => "ערב פסח",
            Holiday::Pesach => "פסח",
            Holiday::CholHamoedPesach => "חול המועד פסח",
            Holiday::PesachSheni => "פסח שני",
            Holiday::ErevShavuos => "ערב שבועות",
            Holiday::Shavuos => "שבועות",
            Holiday::SeventeenthOfTammuz => "שבעה עשר בתמוז",
            Holiday::TishahBav => "תשעה באב",
            Holiday::TuBav => "ט\"ו באב",
            Holiday::ErevRoshHashana => "ערב ראש השנה",
            Holiday::RoshHashana => "ראש השנה",
            Holiday::FastOfGedalyah => "צום גדליה",
            Holiday::ErevYomKippur => "ערב יום כיפור",
            Holiday::YomKippur => "יום כיפור",
            Holiday::ErevSuccos => "ערב סוכות",
            Holiday::Succos => "סוכות",
            Holiday::CholHamoedSuccos => "חול המועד סוכות",
            Holiday::HoshanaRabbah => "הושענא רבה",
            Holiday::SheminiAtzeres => "שמיני עצרת",
            Holiday::SimchasTorah => "שמחת תורה",
            Holiday::Chanukah(_) => "חנוכה",
            Holiday::TenthOfTeves => "עשרה בטבת",
            Holiday::TuBshvat => "ט\"ו בשבט",
            Holiday::FastOfEsther => "תענית אסתר",
            Holiday::Purim => "פורים",
            Holiday::ShushanPurim => "שושן פורים",
            Holiday::PurimKatan => "פורים קטן",
            Holiday::RoshChodesh => "ראש חודש",
            Holiday::YomHaShoah => "יום השואה",
            Holiday::YomHazikaron => "יום הזיכרון",
            Holiday::YomHaatzmaut => "יום העצמאות",
            Holiday::YomYerushalayim => "יום ירושלים",
            Holiday::LagBomer => "ל\"ג בעומר",
            Holiday::ShushanPurimKatan => "שושן פורים קטן",
            Holiday::IsruChag => "אסרו חג",
            Holiday::YomKippurKatan => "יום כיפור קטן",
            Holiday::Behab => "בה'ב",
            Holiday::FastOfTheFirstborn => "תענית בכורות",
            Holiday::CountOfTheOmer(_) => "ספירת העומר",
            Holiday::BirchasHachamah => "ברכת החמה",
            Holiday::MacharHachodesh => "מחר החדש",
            Holiday::ShabbosMevarchim => "שבת מברכים",
        }
    }
}

impl core::fmt::Display for Holiday {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.he())
    }
}
