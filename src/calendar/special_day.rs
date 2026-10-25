use core::fmt;
use core::iter::FusedIterator;
use core::ops::RangeInclusive;
use core::slice::Iter;

use icu_calendar::types::{Month, Weekday};
use icu_calendar::{Date, cal::Hebrew};

use super::month::*;
use super::year::{HebrewYear, elapsed_days};

/// Yields the holidays on one date by testing it against every rule in order.
pub(crate) struct SpecialDayIterator {
    iter: Iter<'static, SpecialDayRule>,
    date: Date<Hebrew>,
    in_israel: bool,
}

impl SpecialDayIterator {
    pub(crate) fn new(date: Date<Hebrew>, in_israel: bool) -> Self {
        Self {
            iter: HOLIDAY_RULES.iter(),
            date,
            in_israel,
        }
    }
}

impl Iterator for SpecialDayIterator {
    type Item = SpecialDay;

    fn next(&mut self) -> Option<Self::Item> {
        let (date, in_israel) = (self.date, self.in_israel);
        self.iter.find_map(|rule| rule.for_date(&date, in_israel))
    }
}

impl FusedIterator for SpecialDayIterator {}

/// A holiday, fast, or other day the calendar marks.
///
/// [`HebrewDate::special_days`](super::HebrewDate::special_days) lists the ones on a date. Some are not
/// holidays in the halachic sense (Rosh Chodesh, Shabbos Mevarchim, the Omer count) but are
/// listed because calendars show them. Dates below are for the diaspora unless noted.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, serde::Deserialize, tsify::Tsify),
    serde(tag = "id", content = "day", rename_all = "camelCase")
)]
pub enum SpecialDay {
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
    /// Rosh Chodesh, the 1st of every month but Tishrei, and the 30th of a 30-day month.
    RoshChodesh,
    /// Yom HaShoah, around 27 Nisan. Modern.
    YomHaShoah,
    /// Yom HaZikaron, around 4 Iyar. Modern.
    YomHaZikaron,
    /// Yom HaAtzmaut, around 5 Iyar. Modern.
    YomHaAtzmaut,
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
    /// Shabbos Machar Chodesh, a Shabbos on the 29th or 30th of the month, when Sunday is Rosh
    /// Chodesh. On the 30th that Shabbos is also Rosh Chodesh.
    MacharChodesh,
    /// Shabbos Mevarchim, the Shabbos when the coming month is announced (not before Tishrei).
    ShabbosMevarchim,
}

// How a holiday's date is found. `Chutz` rules apply only in the diaspora.
enum SpecialDayRule {
    ExactDate(SpecialDay, u8, Month),
    ExactDates(SpecialDay, RangeInclusive<u8>, Month),
    ExactDateChutz(SpecialDay, u8, Month),
    ExactDateIsrael(SpecialDay, u8, Month),
    Group(&'static [SpecialDayRule]),
    Custom(fn(&Date<Hebrew>, bool) -> Option<SpecialDay>),
}

impl SpecialDayRule {
    fn for_date(&self, date: &Date<Hebrew>, in_israel: bool) -> Option<SpecialDay> {
        match self {
            SpecialDayRule::ExactDate(special_day, day, month) => {
                if date.day_of_month().0 == *day && date.month().to_input() == *month {
                    Some(*special_day)
                } else {
                    None
                }
            }
            SpecialDayRule::ExactDates(special_day, range, month) => {
                if range.contains(&date.day_of_month().0) && date.month().to_input() == *month {
                    Some(*special_day)
                } else {
                    None
                }
            }
            SpecialDayRule::ExactDateChutz(special_day, day, month) => {
                if date.day_of_month().0 == *day && date.month().to_input() == *month && !in_israel {
                    Some(*special_day)
                } else {
                    None
                }
            }
            SpecialDayRule::ExactDateIsrael(special_day, day, month) => {
                if date.day_of_month().0 == *day && date.month().to_input() == *month && in_israel {
                    Some(*special_day)
                } else {
                    None
                }
            }
            SpecialDayRule::Group(rules) => rules.iter().find_map(|rule| rule.for_date(date, in_israel)),
            SpecialDayRule::Custom(func) => func(date, in_israel),
        }
    }
}

static HOLIDAY_RULES: [SpecialDayRule; 42] = [
    SpecialDayRule::ExactDate(SpecialDay::ErevPesach, 14, NISAN),
    SpecialDayRule::Group(&[
        SpecialDayRule::ExactDate(SpecialDay::Pesach, 15, NISAN),
        SpecialDayRule::ExactDate(SpecialDay::Pesach, 21, NISAN),
        SpecialDayRule::ExactDateChutz(SpecialDay::Pesach, 16, NISAN),
        SpecialDayRule::ExactDateChutz(SpecialDay::Pesach, 22, NISAN),
    ]),
    SpecialDayRule::Group(&[
        SpecialDayRule::ExactDateIsrael(SpecialDay::CholHamoedPesach, 16, NISAN),
        SpecialDayRule::ExactDates(SpecialDay::CholHamoedPesach, 17..=20, NISAN),
    ]),
    SpecialDayRule::ExactDate(SpecialDay::PesachSheni, 14, IYYAR),
    SpecialDayRule::ExactDate(SpecialDay::ErevShavuos, 5, SIVAN),
    SpecialDayRule::Group(&[
        SpecialDayRule::ExactDate(SpecialDay::Shavuos, 6, SIVAN),
        SpecialDayRule::ExactDateChutz(SpecialDay::Shavuos, 7, SIVAN),
    ]),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != TAMMUZ {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 17 && day_of_week != Weekday::Saturday) || (day == 18 && day_of_week == Weekday::Sunday) {
            Some(SpecialDay::SeventeenthOfTammuz)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != AV {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day_of_week == Weekday::Sunday && day == 10) || (day_of_week != Weekday::Saturday && day == 9) {
            Some(SpecialDay::TishahBav)
        } else {
            None
        }
    }),
    SpecialDayRule::ExactDate(SpecialDay::TuBav, 15, AV),
    SpecialDayRule::ExactDate(SpecialDay::ErevRoshHashana, 29, ELUL),
    SpecialDayRule::ExactDates(SpecialDay::RoshHashana, 1..=2, TISHREI),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != TISHREI {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 3 && day_of_week != Weekday::Saturday) || (day == 4 && day_of_week == Weekday::Sunday) {
            Some(SpecialDay::FastOfGedalyah)
        } else {
            None
        }
    }),
    SpecialDayRule::ExactDate(SpecialDay::ErevYomKippur, 9, TISHREI),
    SpecialDayRule::ExactDate(SpecialDay::YomKippur, 10, TISHREI),
    SpecialDayRule::ExactDate(SpecialDay::ErevSuccos, 14, TISHREI),
    SpecialDayRule::Group(&[
        SpecialDayRule::ExactDate(SpecialDay::Succos, 15, TISHREI),
        SpecialDayRule::ExactDateChutz(SpecialDay::Succos, 16, TISHREI),
    ]),
    SpecialDayRule::Group(&[
        SpecialDayRule::ExactDateIsrael(SpecialDay::CholHamoedSuccos, 16, TISHREI),
        SpecialDayRule::ExactDates(SpecialDay::CholHamoedSuccos, 17..=20, TISHREI),
    ]),
    SpecialDayRule::ExactDate(SpecialDay::HoshanaRabbah, 21, TISHREI),
    SpecialDayRule::ExactDate(SpecialDay::SheminiAtzeres, 22, TISHREI),
    SpecialDayRule::ExactDateChutz(SpecialDay::SimchasTorah, 23, TISHREI),
    SpecialDayRule::Custom(|date, _in_israel| {
        let month = date.month().to_input();
        let day = date.day_of_month().0;

        let day_of_chanukah = if month == KISLEV && day >= 25 {
            day - 24
        } else if month == TEVET {
            let is_kislev_short = HebrewYear::unchecked(date.year().extended_year()).is_kislev_short();
            let max_teves_day = if is_kislev_short { 3 } else { 2 };
            if day <= max_teves_day {
                if is_kislev_short { day + 5 } else { day + 6 }
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some(SpecialDay::Chanukah(day_of_chanukah))
    }),
    SpecialDayRule::ExactDate(SpecialDay::TenthOfTeves, 10, TEVET),
    SpecialDayRule::ExactDate(SpecialDay::TuBshvat, 15, SHEVAT),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != ADAR {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if ((day == 11 || day == 12) && day_of_week == Weekday::Thursday)
            || (day == 13 && !(day_of_week == Weekday::Friday || day_of_week == Weekday::Saturday))
        {
            Some(SpecialDay::FastOfEsther)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        let month = date.month().to_input();
        let day = date.day_of_month().0;
        if month == ADAR && day == 14 {
            Some(SpecialDay::Purim)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        let month = date.month().to_input();
        let day = date.day_of_month().0;
        if month == ADAR && day == 15 {
            Some(SpecialDay::ShushanPurim)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 14 {
            Some(SpecialDay::PurimKatan)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if (date.day_of_month().0 == 1 && date.month().to_input() != TISHREI) || date.day_of_month().0 == 30 {
            Some(SpecialDay::RoshChodesh)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != NISAN {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 26 && day_of_week == Weekday::Thursday)
            || (day == 28 && day_of_week == Weekday::Monday)
            || (day == 27 && day_of_week != Weekday::Sunday && day_of_week != Weekday::Friday)
        {
            Some(SpecialDay::YomHaShoah)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != IYYAR {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 4 && day_of_week == Weekday::Tuesday)
            || ((day == 3 || day == 2) && day_of_week == Weekday::Wednesday)
            || (day == 5 && day_of_week == Weekday::Monday)
        {
            Some(SpecialDay::YomHaZikaron)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() != IYYAR {
            return None;
        }
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if (day == 5 && day_of_week == Weekday::Wednesday)
            || ((day == 4 || day == 3) && day_of_week == Weekday::Thursday)
            || (day == 6 && day_of_week == Weekday::Tuesday)
        {
            Some(SpecialDay::YomHaAtzmaut)
        } else {
            None
        }
    }),
    SpecialDayRule::ExactDate(SpecialDay::YomYerushalayim, 28, IYYAR),
    SpecialDayRule::ExactDate(SpecialDay::LagBomer, 18, IYYAR),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.month().to_input() == ADARI && date.is_in_leap_year() && date.day_of_month().0 == 15 {
            Some(SpecialDay::ShushanPurimKatan)
        } else {
            None
        }
    }),
    SpecialDayRule::Group(&[
        SpecialDayRule::ExactDateIsrael(SpecialDay::IsruChag, 22, NISAN),
        SpecialDayRule::ExactDateChutz(SpecialDay::IsruChag, 23, NISAN),
        SpecialDayRule::ExactDateIsrael(SpecialDay::IsruChag, 7, SIVAN),
        SpecialDayRule::ExactDateChutz(SpecialDay::IsruChag, 8, SIVAN),
        SpecialDayRule::ExactDateIsrael(SpecialDay::IsruChag, 23, TISHREI),
        SpecialDayRule::ExactDateChutz(SpecialDay::IsruChag, 24, TISHREI),
    ]),
    SpecialDayRule::Custom(|date, _in_israel| {
        let day_of_week = date.weekday();
        let month = date.month().to_input();
        let day = date.day_of_month().0;

        if matches!(month, ELUL | TISHREI | KISLEV | NISAN) {
            return None;
        }

        if day == 29 && day_of_week != Weekday::Friday && day_of_week != Weekday::Saturday {
            return Some(SpecialDay::YomKippurKatan);
        }

        if (day == 27 || day == 28) && day_of_week == Weekday::Thursday {
            Some(SpecialDay::YomKippurKatan)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        let day_of_week = date.weekday();
        let month = date.month().to_input();
        let day = date.day_of_month().0;

        if (month == CHESHVAN || month == IYYAR)
            && ((day_of_week == Weekday::Monday && day > 4 && day < 18)
                || (day_of_week == Weekday::Thursday && day > 7 && day < 14))
        {
            return Some(SpecialDay::Behab);
        }
        None
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        let month = date.month().to_input();
        let day = date.day_of_month().0;
        let day_of_week = date.weekday();
        if month == NISAN
            && ((day == 14 && day_of_week != Weekday::Saturday) || (day == 12 && day_of_week == Weekday::Thursday))
        {
            Some(SpecialDay::FastOfTheFirstborn)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
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

        Some(SpecialDay::CountOfTheOmer(day_of_omer))
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        let elapsed_days = elapsed_days(date.year().extended_year() as i64) + date.day_of_year().0 as i64;
        let cycle_length = 10227;
        if (elapsed_days % cycle_length) == 172 {
            Some(SpecialDay::BirchasHachamah)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.weekday() == Weekday::Saturday && (date.day_of_month().0 == 30 || date.day_of_month().0 == 29) {
            Some(SpecialDay::MacharChodesh)
        } else {
            None
        }
    }),
    SpecialDayRule::Custom(|date, _in_israel| {
        if date.weekday() == Weekday::Saturday
            && date.day_of_month().0 >= 23
            && date.day_of_month().0 <= 29
            && date.month().to_input() != ELUL
        {
            Some(SpecialDay::ShabbosMevarchim)
        } else {
            None
        }
    }),
];

impl SpecialDay {
    /// Every kind of day. Chanukah and the Omer are listed once, as their first day.
    pub const ALL: [SpecialDay; 42] = [
        SpecialDay::ErevPesach,
        SpecialDay::Pesach,
        SpecialDay::CholHamoedPesach,
        SpecialDay::PesachSheni,
        SpecialDay::ErevShavuos,
        SpecialDay::Shavuos,
        SpecialDay::SeventeenthOfTammuz,
        SpecialDay::TishahBav,
        SpecialDay::TuBav,
        SpecialDay::ErevRoshHashana,
        SpecialDay::RoshHashana,
        SpecialDay::FastOfGedalyah,
        SpecialDay::ErevYomKippur,
        SpecialDay::YomKippur,
        SpecialDay::ErevSuccos,
        SpecialDay::Succos,
        SpecialDay::CholHamoedSuccos,
        SpecialDay::HoshanaRabbah,
        SpecialDay::SheminiAtzeres,
        SpecialDay::SimchasTorah,
        SpecialDay::Chanukah(1),
        SpecialDay::TenthOfTeves,
        SpecialDay::TuBshvat,
        SpecialDay::FastOfEsther,
        SpecialDay::Purim,
        SpecialDay::ShushanPurim,
        SpecialDay::PurimKatan,
        SpecialDay::RoshChodesh,
        SpecialDay::YomHaShoah,
        SpecialDay::YomHaZikaron,
        SpecialDay::YomHaAtzmaut,
        SpecialDay::YomYerushalayim,
        SpecialDay::LagBomer,
        SpecialDay::ShushanPurimKatan,
        SpecialDay::IsruChag,
        SpecialDay::YomKippurKatan,
        SpecialDay::Behab,
        SpecialDay::FastOfTheFirstborn,
        SpecialDay::CountOfTheOmer(1),
        SpecialDay::BirchasHachamah,
        SpecialDay::MacharChodesh,
        SpecialDay::ShabbosMevarchim,
    ];

    /// Whether work is forbidden. It is on the days of Yom Tov, Rosh Hashana and Yom Kippur.
    pub const fn is_assur_bemelacha(self) -> bool {
        matches!(
            self,
            SpecialDay::Pesach
                | SpecialDay::Shavuos
                | SpecialDay::Succos
                | SpecialDay::SheminiAtzeres
                | SpecialDay::SimchasTorah
                | SpecialDay::RoshHashana
                | SpecialDay::YomKippur
        )
    }

    /// Whether this is one of the modern Israeli days, which are Yom HaShoah, Yom HaZikaron, Yom
    /// HaAtzmaut and Yom Yerushalayim. They are marked in every year, including the years before
    /// they were established.
    pub const fn is_modern(self) -> bool {
        matches!(
            self,
            SpecialDay::YomHaShoah | SpecialDay::YomHaZikaron | SpecialDay::YomHaAtzmaut | SpecialDay::YomYerushalayim
        )
    }

    /// Whether this is a public fast day. The Fast of the Firstborn is not one, because it binds
    /// only firstborn sons.
    pub const fn is_fast_day(self) -> bool {
        matches!(
            self,
            SpecialDay::FastOfGedalyah
                | SpecialDay::FastOfEsther
                | SpecialDay::TishahBav
                | SpecialDay::TenthOfTeves
                | SpecialDay::SeventeenthOfTammuz
                | SpecialDay::YomKippur
        )
    }

    /// The name in Hebrew, for example `"פסח"`. The day of Chanukah or of the Omer is not
    /// included.
    pub const fn hebrew(self) -> &'static str {
        match self {
            SpecialDay::ErevPesach => "ערב פסח",
            SpecialDay::Pesach => "פסח",
            SpecialDay::CholHamoedPesach => "חול המועד פסח",
            SpecialDay::PesachSheni => "פסח שני",
            SpecialDay::ErevShavuos => "ערב שבועות",
            SpecialDay::Shavuos => "שבועות",
            SpecialDay::SeventeenthOfTammuz => "שבעה עשר בתמוז",
            SpecialDay::TishahBav => "תשעה באב",
            SpecialDay::TuBav => "ט\"ו באב",
            SpecialDay::ErevRoshHashana => "ערב ראש השנה",
            SpecialDay::RoshHashana => "ראש השנה",
            SpecialDay::FastOfGedalyah => "צום גדליה",
            SpecialDay::ErevYomKippur => "ערב יום כיפור",
            SpecialDay::YomKippur => "יום כיפור",
            SpecialDay::ErevSuccos => "ערב סוכות",
            SpecialDay::Succos => "סוכות",
            SpecialDay::CholHamoedSuccos => "חול המועד סוכות",
            SpecialDay::HoshanaRabbah => "הושענא רבה",
            SpecialDay::SheminiAtzeres => "שמיני עצרת",
            SpecialDay::SimchasTorah => "שמחת תורה",
            SpecialDay::Chanukah(_) => "חנוכה",
            SpecialDay::TenthOfTeves => "עשרה בטבת",
            SpecialDay::TuBshvat => "ט\"ו בשבט",
            SpecialDay::FastOfEsther => "תענית אסתר",
            SpecialDay::Purim => "פורים",
            SpecialDay::ShushanPurim => "שושן פורים",
            SpecialDay::PurimKatan => "פורים קטן",
            SpecialDay::RoshChodesh => "ראש חודש",
            SpecialDay::YomHaShoah => "יום השואה",
            SpecialDay::YomHaZikaron => "יום הזיכרון",
            SpecialDay::YomHaAtzmaut => "יום העצמאות",
            SpecialDay::YomYerushalayim => "יום ירושלים",
            SpecialDay::LagBomer => "ל\"ג בעומר",
            SpecialDay::ShushanPurimKatan => "שושן פורים קטן",
            SpecialDay::IsruChag => "אסרו חג",
            SpecialDay::YomKippurKatan => "יום כיפור קטן",
            SpecialDay::Behab => "בה'ב",
            SpecialDay::FastOfTheFirstborn => "תענית בכורות",
            SpecialDay::CountOfTheOmer(_) => "ספירת העומר",
            SpecialDay::BirchasHachamah => "ברכת החמה",
            SpecialDay::MacharChodesh => "מחר החדש",
            SpecialDay::ShabbosMevarchim => "שבת מברכים",
        }
    }
}

impl SpecialDay {
    /// The name in English, for example `"Pesach"`. The day of Chanukah or of the Omer is not
    /// included.
    pub const fn name(self) -> &'static str {
        match self {
            SpecialDay::ErevPesach => "Erev Pesach",
            SpecialDay::Pesach => "Pesach",
            SpecialDay::CholHamoedPesach => "Chol Hamoed Pesach",
            SpecialDay::PesachSheni => "Pesach Sheni",
            SpecialDay::ErevShavuos => "Erev Shavuos",
            SpecialDay::Shavuos => "Shavuos",
            SpecialDay::SeventeenthOfTammuz => "Seventeenth of Tammuz",
            SpecialDay::TishahBav => "Tishah B'Av",
            SpecialDay::TuBav => "Tu B'Av",
            SpecialDay::ErevRoshHashana => "Erev Rosh Hashana",
            SpecialDay::RoshHashana => "Rosh Hashana",
            SpecialDay::FastOfGedalyah => "Fast of Gedalyah",
            SpecialDay::ErevYomKippur => "Erev Yom Kippur",
            SpecialDay::YomKippur => "Yom Kippur",
            SpecialDay::ErevSuccos => "Erev Succos",
            SpecialDay::Succos => "Succos",
            SpecialDay::CholHamoedSuccos => "Chol Hamoed Succos",
            SpecialDay::HoshanaRabbah => "Hoshana Rabbah",
            SpecialDay::SheminiAtzeres => "Shemini Atzeres",
            SpecialDay::SimchasTorah => "Simchas Torah",
            SpecialDay::Chanukah(_) => "Chanukah",
            SpecialDay::TenthOfTeves => "Tenth of Teves",
            SpecialDay::TuBshvat => "Tu B'Shvat",
            SpecialDay::FastOfEsther => "Fast of Esther",
            SpecialDay::Purim => "Purim",
            SpecialDay::ShushanPurim => "Shushan Purim",
            SpecialDay::PurimKatan => "Purim Katan",
            SpecialDay::RoshChodesh => "Rosh Chodesh",
            SpecialDay::YomHaShoah => "Yom HaShoah",
            SpecialDay::YomHaZikaron => "Yom HaZikaron",
            SpecialDay::YomHaAtzmaut => "Yom HaAtzmaut",
            SpecialDay::YomYerushalayim => "Yom Yerushalayim",
            SpecialDay::LagBomer => "Lag B'Omer",
            SpecialDay::ShushanPurimKatan => "Shushan Purim Katan",
            SpecialDay::IsruChag => "Isru Chag",
            SpecialDay::YomKippurKatan => "Yom Kippur Katan",
            SpecialDay::Behab => "BeHaB",
            SpecialDay::FastOfTheFirstborn => "Fast of the Firstborn",
            SpecialDay::CountOfTheOmer(_) => "Count of the Omer",
            SpecialDay::BirchasHachamah => "Birchas Hachamah",
            SpecialDay::MacharChodesh => "Machar Chodesh",
            SpecialDay::ShabbosMevarchim => "Shabbos Mevarchim",
        }
    }
}

/// The name in English, for example "Pesach". The day of Chanukah or of the Omer is not
/// included.
impl fmt::Display for SpecialDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
