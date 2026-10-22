use core::fmt;

use icu_calendar::types::Month;

use super::year::HebrewYear;

/// A month of the Hebrew calendar, in order from Tishrei.
///
/// A leap year has two Adars. [`AdarI`](Self::AdarI) is the added month and exists only in leap
/// years; [`Adar`](Self::Adar) is Adar in a common year and Adar II in a leap year. Purim is
/// always in `Adar`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HebrewMonth {
    /// Tishrei (תשרי), the month of Rosh Hashana, Yom Kippur and Succos.
    Tishrei,
    /// Cheshvan (חשון), 29 or 30 days depending on the year.
    Cheshvan,
    /// Kislev (כסלו), 29 or 30 days depending on the year. Chanukah begins on the 25th.
    Kislev,
    /// Teves (טבת).
    Teves,
    /// Shevat (שבט).
    Shevat,
    /// Adar I (אדר א), the month added in a leap year.
    AdarI,
    /// Adar (אדר) in a common year, Adar II (אדר ב) in a leap year.
    Adar,
    /// Nisan (ניסן), the month of Pesach.
    Nisan,
    /// Iyar (אייר).
    Iyar,
    /// Sivan (סיון), the month of Shavuos.
    Sivan,
    /// Tammuz (תמוז).
    Tammuz,
    /// Av (אב).
    Av,
    /// Elul (אלול).
    Elul,
}

impl HebrewMonth {
    /// The months of a common year, from Tishrei.
    pub const COMMON_YEAR: [HebrewMonth; 12] = [
        Self::Tishrei,
        Self::Cheshvan,
        Self::Kislev,
        Self::Teves,
        Self::Shevat,
        Self::Adar,
        Self::Nisan,
        Self::Iyar,
        Self::Sivan,
        Self::Tammuz,
        Self::Av,
        Self::Elul,
    ];

    /// The months of a leap year, from Tishrei.
    pub const LEAP_YEAR: [HebrewMonth; 13] = [
        Self::Tishrei,
        Self::Cheshvan,
        Self::Kislev,
        Self::Teves,
        Self::Shevat,
        Self::AdarI,
        Self::Adar,
        Self::Nisan,
        Self::Iyar,
        Self::Sivan,
        Self::Tammuz,
        Self::Av,
        Self::Elul,
    ];

    /// The month's name in Hebrew, for example `"ניסן"`.
    ///
    /// [`Adar`](Self::Adar) is `"אדר"` whatever the year; [`HebrewDate`](super::HebrewDate)'s
    /// formatting says Adar II when it needs to.
    pub const fn hebrew(self) -> &'static str {
        match self {
            Self::Tishrei => "תשרי",
            Self::Cheshvan => "חשון",
            Self::Kislev => "כסלו",
            Self::Teves => "טבת",
            Self::Shevat => "שבט",
            Self::AdarI => "אדר א",
            Self::Adar => "אדר",
            Self::Nisan => "ניסן",
            Self::Iyar => "אייר",
            Self::Sivan => "סיון",
            Self::Tammuz => "תמוז",
            Self::Av => "אב",
            Self::Elul => "אלול",
        }
    }

    const fn english(self) -> &'static str {
        match self {
            Self::Tishrei => "Tishrei",
            Self::Cheshvan => "Cheshvan",
            Self::Kislev => "Kislev",
            Self::Teves => "Teves",
            Self::Shevat => "Shevat",
            Self::AdarI => "Adar I",
            Self::Adar => "Adar",
            Self::Nisan => "Nisan",
            Self::Iyar => "Iyar",
            Self::Sivan => "Sivan",
            Self::Tammuz => "Tammuz",
            Self::Av => "Av",
            Self::Elul => "Elul",
        }
    }

    /// The month's English name in `year`: "Adar II" for [`Adar`](Self::Adar) in a leap year.
    pub(crate) fn english_in(self, year: HebrewYear) -> &'static str {
        if self == Self::Adar && year.is_leap() {
            "Adar II"
        } else {
            self.english()
        }
    }

    pub(crate) const fn to_icu(self) -> Month {
        match self {
            Self::Tishrei => TISHREI,
            Self::Cheshvan => CHESHVAN,
            Self::Kislev => KISLEV,
            Self::Teves => TEVET,
            Self::Shevat => SHEVAT,
            Self::AdarI => ADARI,
            Self::Adar => ADAR,
            Self::Nisan => NISAN,
            Self::Iyar => IYYAR,
            Self::Sivan => SIVAN,
            Self::Tammuz => TAMMUZ,
            Self::Av => AV,
            Self::Elul => ELUL,
        }
    }

    pub(crate) fn from_icu(month: Month) -> Option<Self> {
        Self::LEAP_YEAR
            .into_iter()
            .find(|candidate| candidate.to_icu() == month)
    }
}

/// The English name, for example "Nisan". [`Adar`](HebrewMonth::Adar) prints as "Adar"; format a
/// [`HebrewDate`](super::HebrewDate) to get "Adar II" in a leap year.
impl fmt::Display for HebrewMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.english())
    }
}

// ICU's codes for the Hebrew months. The calendar internals work in these; the public API uses
// `HebrewMonth`.
pub(crate) const TISHREI: Month = Month::new(1);
pub(crate) const CHESHVAN: Month = Month::new(2);
pub(crate) const KISLEV: Month = Month::new(3);
pub(crate) const TEVET: Month = Month::new(4);
pub(crate) const SHEVAT: Month = Month::new(5);
pub(crate) const ADARI: Month = Month::leap(5);
pub(crate) const ADAR: Month = Month::new(6);
pub(crate) const NISAN: Month = Month::new(7);
pub(crate) const IYYAR: Month = Month::new(8);
pub(crate) const SIVAN: Month = Month::new(9);
pub(crate) const TAMMUZ: Month = Month::new(10);
pub(crate) const AV: Month = Month::new(11);
pub(crate) const ELUL: Month = Month::new(12);

/// This month's 1-based position in `year`, counting from Tishrei, or `None` if `month` is not
/// in that year (Adar I in a common year).
pub(crate) fn month_of_year(month: Month, year: i32) -> Option<u8> {
    let months: &[HebrewMonth] = if HebrewYear::unchecked(year).is_leap() {
        &HebrewMonth::LEAP_YEAR
    } else {
        &HebrewMonth::COMMON_YEAR
    };
    months
        .iter()
        .position(|candidate| candidate.to_icu() == month)
        .map(|index| index as u8 + 1)
}
