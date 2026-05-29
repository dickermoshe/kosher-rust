use core::ops::RangeInclusive;
use core::slice::Iter;
use icu_calendar::types::{Month, Weekday};
use icu_calendar::{Date, cal::Hebrew};

use super::month::*;
use super::{HebrewCalendar, HebrewHolidayCalendar, get_hebrew_elapsed_days};

/// Iterator that yields holidays occurring on a specific Hebrew date.
///
/// This iterator filters through all possible holidays and returns only those
/// that occur on the given date, respecting both location (Israel vs. Diaspora)
/// and whether to include modern holidays.
pub struct HolidayIterator {
    pub(super) iter: Iter<'static, Holiday>,
    pub(super) date: Date<Hebrew>,
    pub(super) in_israel: bool,
    pub(super) use_modern_holidays: bool,
}

impl Iterator for HolidayIterator {
    type Item = &'static Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let holiday = self.iter.next()?;
            if holiday.rule().is_today(&self.date, self.in_israel)
                && (self.use_modern_holidays || !holiday.is_modern_holiday())
            {
                return Some(holiday);
            }
        }
    }
}

/// Returns whether `date` falls during Chanukah
/// (25 Kislev through 2 Tevet, or 3 Tevet in a short Kislev year)
fn is_chanukah(date: &Date<Hebrew>) -> bool {
    let month = date.input_month();
    let day = date.day_of_month().0;
    if month == KISLEV && day >= 25 {
        return true;
    }
    if month == TEVET {
        let Some(is_kislev_short) = Hebrew::is_kislev_short(date.year().extended_year()) else {
            return false;
        };
        let max_teves_day = if is_kislev_short { 3 } else { 2 };
        return day <= max_teves_day;
    }
    false
}

/// Returns whether `date` falls during the counting of the Omer
/// (16 Nissan through 6 Sivan)
fn is_counting_omer(date: &Date<Hebrew>) -> bool {
    let month = date.input_month();
    let day = date.day_of_month().0;

    (month == NISAN && day >= 16) || month == IYYAR || (month == SIVAN && day < 6)
}
/// Returns the day of Chanukah. None if not Chanukah.
pub(super) fn day_of_chanukah(date: &Date<Hebrew>) -> Option<u8> {
    if !is_chanukah(date) {
        return None;
    }

    let month = date.input_month();
    let day = date.day_of_month().0;

    if month == KISLEV {
        Some(day - 24)
    } else if Hebrew::is_kislev_short(date.year().extended_year())? {
        Some(day + 5)
    } else {
        Some(day + 6)
    }
}

/// Returns the day of the Omer. None if not counting the Omer.
pub(super) fn day_of_the_omer(date: &Date<Hebrew>) -> Option<u8> {
    if !is_counting_omer(date) {
        return None;
    }

    let month = date.input_month();
    let day = date.day_of_month().0;

    if month == NISAN {
        Some(day - 15)
    } else if month == IYYAR {
        Some(day + 15)
    } else if month == SIVAN && day < 6 {
        Some(day + 44)
    } else {
        None
    }
}

/// Internal type representing different rules for when holidays occur.
enum HolidayRule<'a> {
    ExactDate(u8, Month),
    ExactDates(RangeInclusive<u8>, Month),
    ExactDateChutz(u8, Month),
    ExactDateIsrael(u8, Month),
    ExactDates2([&'a HolidayRule<'a>; 2]),
    ExactDates4([&'a HolidayRule<'a>; 4]),
    ExactDates6([&'a HolidayRule<'a>; 6]),
    Custom(fn(&Date<Hebrew>, bool) -> bool),
}
impl HolidayRule<'_> {
    fn is_today(&self, date: &Date<Hebrew>, in_israel: bool) -> bool {
        match self {
            HolidayRule::ExactDate(day, month) => date.day_of_month().0 == *day && date.input_month() == *month,
            HolidayRule::ExactDates(range, month) => {
                range.contains(&date.day_of_month().0) && date.input_month() == *month
            }
            HolidayRule::ExactDateChutz(day, month) => {
                date.day_of_month().0 == *day && date.input_month() == *month && !in_israel
            }
            HolidayRule::ExactDateIsrael(day, month) => {
                date.day_of_month().0 == *day && date.input_month() == *month && in_israel
            }
            HolidayRule::ExactDates2(rules) => rules.iter().any(|rule| rule.is_today(date, in_israel)),

            HolidayRule::ExactDates4(rules) => rules.iter().any(|rule| rule.is_today(date, in_israel)),

            HolidayRule::ExactDates6(rules) => rules.iter().any(|rule| rule.is_today(date, in_israel)),
            HolidayRule::Custom(func) => func(date, in_israel),
        }
    }
}

/// Represents Jewish holidays and special days in the Hebrew calendar.
///
/// This enum covers traditional holidays, fast days, modern Israeli holidays,
/// and other significant dates in the Jewish calendar.
///
///
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
    Chanukah,
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
    CountOfTheOmer,
    BirchasHachamah,
    MacharHachodesh,
    ShabbosMevarchim,
}

impl Holiday {
    /// Returns the internal rule that determines when this holiday occurs.
    fn rule(&self) -> HolidayRule<'_> {
        match self {
            Holiday::ErevPesach => HolidayRule::ExactDate(14, NISAN),
            Holiday::Pesach => HolidayRule::ExactDates4([
                &HolidayRule::ExactDate(15, NISAN),
                &HolidayRule::ExactDate(21, NISAN),
                &HolidayRule::ExactDateChutz(16, NISAN),
                &HolidayRule::ExactDateChutz(22, NISAN),
            ]),
            Holiday::CholHamoedPesach => HolidayRule::ExactDates2([
                &HolidayRule::ExactDateIsrael(16, NISAN),
                &HolidayRule::ExactDates(17..=20, NISAN),
            ]),
            Holiday::PesachSheni => HolidayRule::ExactDate(14, IYYAR),
            Holiday::ErevShavuos => HolidayRule::ExactDate(5, SIVAN),
            Holiday::Shavuos => HolidayRule::ExactDates2([
                &HolidayRule::ExactDate(6, SIVAN),
                &HolidayRule::ExactDateChutz(7, SIVAN),
            ]),
            Holiday::SeventeenthOfTammuz => HolidayRule::Custom(|date, _in_israel| {
                if date.input_month() != TAMMUZ {
                    return false;
                }
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                (day == 17 && day_of_week != Weekday::Saturday) || (day == 18 && day_of_week == Weekday::Sunday)
            }),
            Holiday::TishahBav => HolidayRule::Custom(|date, _in_israel| {
                if date.input_month() != AV {
                    return false;
                }
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                (day_of_week == Weekday::Sunday && day == 10) || (day_of_week != Weekday::Saturday && day == 9)
            }),
            Holiday::TuBav => HolidayRule::ExactDate(15, AV),
            Holiday::ErevRoshHashana => HolidayRule::ExactDate(29, ELUL),
            Holiday::RoshHashana => HolidayRule::ExactDates(1..=2, TISHREI),
            Holiday::FastOfGedalyah => HolidayRule::Custom(|date, _in_israel| {
                if date.input_month() != TISHREI {
                    return false;
                }
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                (day == 3 && day_of_week != Weekday::Saturday) || (day == 4 && day_of_week == Weekday::Sunday)
            }),
            Holiday::ErevYomKippur => HolidayRule::ExactDate(9, TISHREI),
            Holiday::YomKippur => HolidayRule::ExactDate(10, TISHREI),
            Holiday::ErevSuccos => HolidayRule::ExactDate(14, TISHREI),
            Holiday::Succos => HolidayRule::ExactDates2([
                &HolidayRule::ExactDate(15, TISHREI),
                &HolidayRule::ExactDateChutz(16, TISHREI),
            ]),
            Holiday::CholHamoedSuccos => HolidayRule::ExactDates2([
                &HolidayRule::ExactDateIsrael(16, TISHREI),
                &HolidayRule::ExactDates(17..=20, TISHREI),
            ]),
            Holiday::HoshanaRabbah => HolidayRule::ExactDate(21, TISHREI),
            Holiday::SheminiAtzeres => HolidayRule::ExactDate(22, TISHREI),
            Holiday::SimchasTorah => HolidayRule::ExactDateChutz(23, TISHREI),
            Holiday::Chanukah => HolidayRule::Custom(|date, _in_israel| is_chanukah(date)),
            Holiday::TenthOfTeves => HolidayRule::ExactDate(10, TEVET),
            Holiday::TuBshvat => HolidayRule::ExactDate(15, SHEVAT),
            Holiday::FastOfEsther => HolidayRule::Custom(|date, _in_israel| {
                let month = date.input_month();
                if month == ADAR {
                    let day = date.day_of_month().0;
                    let day_of_week = date.weekday();
                    ((day == 11 || day == 12) && day_of_week == Weekday::Thursday)
                        || (day == 13 && !(day_of_week == Weekday::Friday || day_of_week == Weekday::Saturday))
                } else {
                    false
                }
            }),
            Holiday::Purim => HolidayRule::Custom(|date, _in_israel| {
                let month = date.input_month();
                let day = date.day_of_month().0;
                month == ADAR && day == 14
            }),
            Holiday::ShushanPurim => HolidayRule::Custom(|date, _in_israel| {
                let month = date.input_month();
                let day = date.day_of_month().0;
                month == ADAR && day == 15
            }),
            Holiday::PurimKatan => HolidayRule::Custom(|date, _in_israel| {
                date.input_month() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 14
            }),
            Holiday::RoshChodesh => HolidayRule::Custom(|date, _in_israel| {
                (date.day_of_month().0 == 1 && date.input_month() != TISHREI) || date.day_of_month().0 == 30
            }),
            Holiday::YomHaShoah => HolidayRule::Custom(|date, _in_israel| {
                if date.input_month() != NISAN {
                    return false;
                }
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                (day == 26 && day_of_week == Weekday::Thursday)
                    || (day == 28 && day_of_week == Weekday::Monday)
                    || (day == 27 && day_of_week != Weekday::Sunday && day_of_week != Weekday::Friday)
            }),
            Holiday::YomHazikaron => HolidayRule::Custom(|date, _in_israel| {
                if date.input_month() != IYYAR {
                    return false;
                }
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                (day == 4 && day_of_week == Weekday::Tuesday)
                    || ((day == 3 || day == 2) && day_of_week == Weekday::Wednesday)
                    || (day == 5 && day_of_week == Weekday::Monday)
            }),
            Holiday::YomHaatzmaut => HolidayRule::Custom(|date, _in_israel| {
                if date.input_month() != IYYAR {
                    return false;
                }
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                (day == 5 && day_of_week == Weekday::Wednesday)
                    || ((day == 4 || day == 3) && day_of_week == Weekday::Thursday)
                    || (day == 6 && day_of_week == Weekday::Tuesday)
            }),
            Holiday::YomYerushalayim => HolidayRule::ExactDate(28, IYYAR),
            Holiday::LagBomer => HolidayRule::ExactDate(18, IYYAR),
            Holiday::ShushanPurimKatan => HolidayRule::Custom(|date, _in_israel| {
                date.input_month() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 15
            }),
            Holiday::IsruChag => HolidayRule::ExactDates6([
                &HolidayRule::ExactDateIsrael(22, NISAN),
                &HolidayRule::ExactDateChutz(23, NISAN),
                &HolidayRule::ExactDateIsrael(7, SIVAN),
                &HolidayRule::ExactDateChutz(8, SIVAN),
                &HolidayRule::ExactDateIsrael(23, TISHREI),
                &HolidayRule::ExactDateChutz(24, TISHREI),
            ]),
            Holiday::YomKippurKatan => HolidayRule::Custom(|date, _in_israel| {
                let day_of_week = date.weekday();
                let month = date.input_month();
                let day = date.day_of_month().0;

                // Not observed in Elul, Tishrei, Kislev, or Nissan
                if matches!(month, ELUL | TISHREI | KISLEV | NISAN) {
                    return false;
                }

                // On 29th if not Friday or Shabbos
                if day == 29 && day_of_week != Weekday::Friday && day_of_week != Weekday::Saturday {
                    return true;
                }

                // On 27th or 28th if Thursday (moved back from Friday/Shabbos)
                (day == 27 || day == 28) && day_of_week == Weekday::Thursday
            }),
            Holiday::Behab => HolidayRule::Custom(|date, _in_israel| {
                let day_of_week = date.weekday();
                let month = date.input_month();
                let day = date.day_of_month().0;

                // BeHaB is only in Cheshvan and Iyar
                if month == ḤESHVAN || month == IYYAR {
                    // Monday between 5-17 or Thursday between 8-13
                    return (day_of_week == Weekday::Monday && day > 4 && day < 18)
                        || (day_of_week == Weekday::Thursday && day > 7 && day < 14);
                }
                false
            }),
            Holiday::FastOfTheFirstborn => HolidayRule::Custom(|date, _in_israel| {
                let month = date.input_month();
                let day = date.day_of_month().0;
                let day_of_week = date.weekday();
                month == NISAN
                    && ((day == 14 && day_of_week != Weekday::Saturday)
                        || (day == 12 && day_of_week == Weekday::Thursday))
            }),
            Holiday::CountOfTheOmer => HolidayRule::Custom(|date, _in_israel| is_counting_omer(date)),
            Holiday::BirchasHachamah => HolidayRule::Custom(|date, _in_israel| {
                let Some(elapsed_days) = get_hebrew_elapsed_days(date.year().extended_year()) else {
                    return false;
                };
                let elapsed_days = elapsed_days + date.day_of_year().0 as i32;
                let cycle_length = 10227i32;
                (elapsed_days % cycle_length) == 172
            }),
            Holiday::MacharHachodesh => HolidayRule::Custom(|date, _in_israel| {
                date.weekday() == Weekday::Saturday && (date.day_of_month().0 == 30 || date.day_of_month().0 == 29)
            }),
            Holiday::ShabbosMevarchim => HolidayRule::Custom(|date, _in_israel| {
                date.weekday() == Weekday::Saturday
                    && date.day_of_month().0 >= 23
                    && date.day_of_month().0 <= 29
                    && date.input_month() != ELUL
            }),
        }
    }

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

    /// Returns whether this holiday is a fast day.
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

    /// Returns a slice of all possible holidays.
    pub const fn all() -> &'static [Holiday; 42] {
        &HOLIDAYS
    }
    /// Returns the Hebrew name of the holiday.
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
            Holiday::TuBav => "ט״ו באב",
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
            Holiday::Chanukah => "חנוכה",
            Holiday::TenthOfTeves => "עשרה בטבת",
            Holiday::TuBshvat => "ט״ו בשבט",
            Holiday::FastOfEsther => "תענית אסתר",
            Holiday::Purim => "פורים",
            Holiday::ShushanPurim => "שושן פורים",
            Holiday::PurimKatan => "פורים קטן",
            Holiday::RoshChodesh => "ראש חודש",
            Holiday::YomHaShoah => "יום השואה",
            Holiday::YomHazikaron => "יום הזיכרון",
            Holiday::YomHaatzmaut => "יום העצמאות",
            Holiday::YomYerushalayim => "יום ירושלים",
            Holiday::LagBomer => "ל״ג בעומר",
            Holiday::ShushanPurimKatan => "שושן פורים קטן",
            Holiday::IsruChag => "אסרו חג",
            Holiday::YomKippurKatan => "יום כיפור קטן",
            Holiday::Behab => "בה'ב",
            Holiday::FastOfTheFirstborn => "תענית בכורות",
            Holiday::CountOfTheOmer => "ספירת העומר",
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

static HOLIDAYS: [Holiday; 42] = [
    Holiday::ErevPesach,
    Holiday::Pesach,
    Holiday::CholHamoedPesach,
    Holiday::PesachSheni,
    Holiday::ErevShavuos,
    Holiday::Shavuos,
    Holiday::SeventeenthOfTammuz,
    Holiday::TishahBav,
    Holiday::TuBav,
    Holiday::ErevRoshHashana,
    Holiday::RoshHashana,
    Holiday::FastOfGedalyah,
    Holiday::ErevYomKippur,
    Holiday::YomKippur,
    Holiday::ErevSuccos,
    Holiday::Succos,
    Holiday::CholHamoedSuccos,
    Holiday::HoshanaRabbah,
    Holiday::SheminiAtzeres,
    Holiday::SimchasTorah,
    Holiday::Chanukah,
    Holiday::TenthOfTeves,
    Holiday::TuBshvat,
    Holiday::FastOfEsther,
    Holiday::Purim,
    Holiday::ShushanPurim,
    Holiday::PurimKatan,
    Holiday::RoshChodesh,
    Holiday::YomHaShoah,
    Holiday::YomHazikaron,
    Holiday::YomHaatzmaut,
    Holiday::YomYerushalayim,
    Holiday::LagBomer,
    Holiday::ShushanPurimKatan,
    Holiday::IsruChag,
    Holiday::YomKippurKatan,
    Holiday::Behab,
    Holiday::FastOfTheFirstborn,
    Holiday::CountOfTheOmer,
    Holiday::BirchasHachamah,
    Holiday::MacharHachodesh,
    Holiday::ShabbosMevarchim,
];
