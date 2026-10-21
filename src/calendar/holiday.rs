use core::fmt;
use core::iter::FusedIterator;
use core::ops::RangeInclusive;
use core::slice::Iter;

use icu_calendar::types::{Month, Weekday};
use icu_calendar::{Date, cal::Hebrew};

use super::month::*;
use super::year::{HebrewYear, elapsed_days};

/// Yields the holidays on one date by testing it against every rule in order.
pub(crate) struct HolidayIterator {
    iter: Iter<'static, HolidayRule>,
    date: Date<Hebrew>,
    in_israel: bool,
}

impl HolidayIterator {
    pub(crate) fn new(date: Date<Hebrew>, in_israel: bool) -> Self {
        Self {
            iter: HOLIDAY_RULES.iter(),
            date,
            in_israel,
        }
    }
}

impl Iterator for HolidayIterator {
    type Item = Holiday;

    fn next(&mut self) -> Option<Self::Item> {
        let (date, in_israel) = (self.date, self.in_israel);
        self.iter.find_map(|rule| rule.for_date(&date, in_israel))
    }
}

impl FusedIterator for HolidayIterator {}

/// A holiday, fast, or other day the calendar marks.
///
/// [`HebrewDate::holidays`](super::HebrewDate::holidays) lists the ones on a date. Some are not
/// holidays in the halachic sense (Rosh Chodesh, Shabbos Mevarchim, the Omer count) but are
/// listed because calendars show them. Dates below are for the diaspora unless noted.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Holiday {
    /// Erev Pesach, 14 Nisan.
    ErevPesach,
    /// Pesach: 15 and 21 Nisan, plus 16 and 22 Nisan in the diaspora.
    Pesach,
    /// Chol Hamoed Pesach, the intermediate days of Pesach.
    CholHamoedPesach,
    /// Pesach Sheni, 14 Iyar.
    PesachSheni,
    /// Erev Shavuos, 5 Sivan.
    ErevShavuos,
    /// Shavuos: 6 Sivan, plus 7 Sivan in the diaspora.
    Shavuos,
    /// The fast of 17 Tammuz, postponed to Sunday when it falls on Shabbos.
    SeventeenthOfTammuz,
    /// Tishah B'Av, 9 Av, postponed to Sunday when it falls on Shabbos.
    TishahBav,
    /// Tu B'Av, 15 Av.
    TuBav,
    /// Erev Rosh Hashana, 29 Elul.
    ErevRoshHashana,
    /// Rosh Hashana, 1 and 2 Tishrei.
    RoshHashana,
    /// The Fast of Gedalyah, 3 Tishrei, postponed to Sunday when it falls on Shabbos.
    FastOfGedalyah,
    /// Erev Yom Kippur, 9 Tishrei.
    ErevYomKippur,
    /// Yom Kippur, 10 Tishrei.
    YomKippur,
    /// Erev Succos, 14 Tishrei.
    ErevSuccos,
    /// Succos: 15 Tishrei, plus 16 Tishrei in the diaspora.
    Succos,
    /// Chol Hamoed Succos, the intermediate days of Succos before Hoshana Rabbah.
    CholHamoedSuccos,
    /// Hoshana Rabbah, 21 Tishrei.
    HoshanaRabbah,
    /// Shemini Atzeres, 22 Tishrei. In Israel it is also Simchas Torah.
    SheminiAtzeres,
    /// Simchas Torah, 23 Tishrei, in the diaspora only.
    SimchasTorah,
    /// Chanukah, from 25 Kislev. The number is the day of Chanukah, 1 to 8.
    Chanukah(u8),
    /// The fast of 10 Teves.
    TenthOfTeves,
    /// Tu B'Shvat, 15 Shevat.
    TuBshvat,
    /// The Fast of Esther, 13 Adar, moved to the Thursday before when it falls on Shabbos.
    FastOfEsther,
    /// Purim, 14 Adar (Adar II in a leap year).
    Purim,
    /// Shushan Purim, 15 Adar (Adar II in a leap year).
    ShushanPurim,
    /// Purim Katan, 14 Adar I in a leap year.
    PurimKatan,
    /// Rosh Chodesh: the 1st of every month but Tishrei, and the 30th of a 30-day month.
    RoshChodesh,
    /// Yom HaShoah, around 27 Nisan. Modern.
    YomHaShoah,
    /// Yom HaZikaron, around 4 Iyar. Modern.
    YomHazikaron,
    /// Yom HaAtzmaut, around 5 Iyar. Modern.
    YomHaatzmaut,
    /// Yom Yerushalayim, 28 Iyar. Modern.
    YomYerushalayim,
    /// Lag B'Omer, 18 Iyar.
    LagBomer,
    /// Shushan Purim Katan, 15 Adar I in a leap year.
    ShushanPurimKatan,
    /// Isru Chag, the day after Pesach, Shavuos and Succos.
    IsruChag,
    /// Yom Kippur Katan, the eve of Rosh Chodesh, moved to Thursday when it falls on Friday or
    /// Shabbos. Not marked before Rosh Chodesh Tishrei, Cheshvan, Teves or Iyar.
    YomKippurKatan,
    /// BeHaB, the Monday, Thursday and Monday fasts in Cheshvan and Iyar.
    Behab,
    /// The Fast of the Firstborn, 14 Nisan, moved to Thursday when it falls on Shabbos.
    FastOfTheFirstborn,
    /// The Omer count, 16 Nisan to 5 Sivan. The number is the day of the count, 1 to 49.
    CountOfTheOmer(u8),
    /// Birchas Hachamah, the blessing of the sun, once every 28 years.
    BirchasHachamah,
    /// Shabbos Machar Chodesh, the Shabbos before Rosh Chodesh.
    MacharHachodesh,
    /// Shabbos Mevarchim, the Shabbos when the coming month is announced (not before Tishrei).
    ShabbosMevarchim,
}

// How a holiday's date is found. `Chutz` rules apply only in the diaspora.
enum HolidayRule {
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
                if date.day_of_month().0 == *day && date.month().to_input() == *month {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::ExactDates(holiday, range, month) => {
                if range.contains(&date.day_of_month().0) && date.month().to_input() == *month {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::ExactDateChutz(holiday, day, month) => {
                if date.day_of_month().0 == *day && date.month().to_input() == *month && !in_israel {
                    Some(*holiday)
                } else {
                    None
                }
            }
            HolidayRule::ExactDateIsrael(holiday, day, month) => {
                if date.day_of_month().0 == *day && date.month().to_input() == *month && in_israel {
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
        if date.month().to_input() != TAMMUZ {
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
        if date.month().to_input() != AV {
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
        if date.month().to_input() != TISHREI {
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
        let month = date.month().to_input();
        let day = date.day_of_month().0;

        let day_of_chanukah = if month == KISLEV && day >= 25 {
            day - 24
        } else if month == TEVET {
            let is_kislev_short = HebrewYear::new(date.year().extended_year()).is_kislev_short();
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
        if date.month().to_input() != ADAR {
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
        let month = date.month().to_input();
        let day = date.day_of_month().0;
        if month == ADAR && day == 14 {
            Some(Holiday::Purim)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.month().to_input();
        let day = date.day_of_month().0;
        if month == ADAR && day == 15 {
            Some(Holiday::ShushanPurim)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.month().to_input() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 14 {
            Some(Holiday::PurimKatan)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if (date.day_of_month().0 == 1 && date.month().to_input() != TISHREI) || date.day_of_month().0 == 30 {
            Some(Holiday::RoshChodesh)
        } else {
            None
        }
    }),
    HolidayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != NISAN {
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
        if date.month().to_input() != IYYAR {
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
        if date.month().to_input() != IYYAR {
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
        if date.month().to_input() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 15 {
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
        let month = date.month().to_input();
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
        let month = date.month().to_input();
        let day = date.day_of_month().0;

        if (month == CHESHVAN || month == IYYAR)
            && ((day_of_week == Weekday::Monday && day > 4 && day < 18)
                || (day_of_week == Weekday::Thursday && day > 7 && day < 14))
        {
            return Some(Holiday::Behab);
        }
        None
    }),
    HolidayRule::Custom(|date, _in_israel| {
        let month = date.month().to_input();
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
        let month = date.month().to_input();
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
        let elapsed_days = elapsed_days(date.year().extended_year() as i64) + date.day_of_year().0 as i64;
        let cycle_length = 10227;
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
            && date.month().to_input() != ELUL
        {
            Some(Holiday::ShabbosMevarchim)
        } else {
            None
        }
    }),
];

impl Holiday {
    /// Whether work is forbidden: the days of Yom Tov, Rosh Hashana and Yom Kippur.
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

    /// Whether this is one of the modern Israeli days: Yom HaShoah, Yom HaZikaron, Yom HaAtzmaut
    /// and Yom Yerushalayim.
    pub fn is_modern(&self) -> bool {
        matches!(
            self,
            Holiday::YomHaShoah | Holiday::YomHazikaron | Holiday::YomHaatzmaut | Holiday::YomYerushalayim
        )
    }

    /// Whether this is a public fast day. The Fast of the Firstborn is not: it binds only
    /// firstborn sons.
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

    /// The name in Hebrew, for example `"פסח"`. The day of Chanukah or of the Omer is not
    /// included.
    pub const fn hebrew(&self) -> &'static str {
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

/// The name in English, for example "Pesach". The day of Chanukah or of the Omer is not
/// included.
impl fmt::Display for Holiday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Holiday::ErevPesach => "Erev Pesach",
            Holiday::Pesach => "Pesach",
            Holiday::CholHamoedPesach => "Chol Hamoed Pesach",
            Holiday::PesachSheni => "Pesach Sheni",
            Holiday::ErevShavuos => "Erev Shavuos",
            Holiday::Shavuos => "Shavuos",
            Holiday::SeventeenthOfTammuz => "Seventeenth of Tammuz",
            Holiday::TishahBav => "Tishah B'Av",
            Holiday::TuBav => "Tu B'Av",
            Holiday::ErevRoshHashana => "Erev Rosh Hashana",
            Holiday::RoshHashana => "Rosh Hashana",
            Holiday::FastOfGedalyah => "Fast of Gedalyah",
            Holiday::ErevYomKippur => "Erev Yom Kippur",
            Holiday::YomKippur => "Yom Kippur",
            Holiday::ErevSuccos => "Erev Succos",
            Holiday::Succos => "Succos",
            Holiday::CholHamoedSuccos => "Chol Hamoed Succos",
            Holiday::HoshanaRabbah => "Hoshana Rabbah",
            Holiday::SheminiAtzeres => "Shemini Atzeres",
            Holiday::SimchasTorah => "Simchas Torah",
            Holiday::Chanukah(_) => "Chanukah",
            Holiday::TenthOfTeves => "Tenth of Teves",
            Holiday::TuBshvat => "Tu B'Shvat",
            Holiday::FastOfEsther => "Fast of Esther",
            Holiday::Purim => "Purim",
            Holiday::ShushanPurim => "Shushan Purim",
            Holiday::PurimKatan => "Purim Katan",
            Holiday::RoshChodesh => "Rosh Chodesh",
            Holiday::YomHaShoah => "Yom HaShoah",
            Holiday::YomHazikaron => "Yom HaZikaron",
            Holiday::YomHaatzmaut => "Yom HaAtzmaut",
            Holiday::YomYerushalayim => "Yom Yerushalayim",
            Holiday::LagBomer => "Lag B'Omer",
            Holiday::ShushanPurimKatan => "Shushan Purim Katan",
            Holiday::IsruChag => "Isru Chag",
            Holiday::YomKippurKatan => "Yom Kippur Katan",
            Holiday::Behab => "BeHaB",
            Holiday::FastOfTheFirstborn => "Fast of the Firstborn",
            Holiday::CountOfTheOmer(_) => "Count of the Omer",
            Holiday::BirchasHachamah => "Birchas Hachamah",
            Holiday::MacharHachodesh => "Machar Chodesh",
            Holiday::ShabbosMevarchim => "Shabbos Mevarchim",
        })
    }
}
