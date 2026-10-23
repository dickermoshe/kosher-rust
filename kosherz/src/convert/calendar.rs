use jiff::civil;
use kosher_rust::calendar::{self as k, HebrewYear};

use super::valid_number;
use crate::api::calendar::{
    CalendarError, GregorianDate, HebrewMonth, Parsha, Region, SpecialDay, Weekday, YearInfo, YearLength,
};

fn hebrew_year(year: i32) -> Result<HebrewYear, CalendarError> {
    HebrewYear::new(year).ok_or(CalendarError::InvalidDate)
}

pub(crate) fn hebrew_date(year: i32, month: HebrewMonth, day: i32) -> Result<k::HebrewDate, CalendarError> {
    let day = u8::try_from(day).map_err(|_| CalendarError::InvalidDate)?;
    k::HebrewDate::new(hebrew_year(year)?.get(), month.into(), day).ok_or(CalendarError::InvalidDate)
}

pub(crate) fn from_gregorian(date: &GregorianDate) -> Result<k::HebrewDate, CalendarError> {
    let date = super::civil_date(date).ok_or(CalendarError::InvalidDate)?;
    Ok(k::HebrewDate::try_from(date)?)
}

pub(crate) fn add_days(date: k::HebrewDate, days: i32) -> Result<k::HebrewDate, CalendarError> {
    date.add_days(days).ok_or(CalendarError::InvalidDate)
}

pub(crate) fn year_info(year: i32) -> Result<YearInfo, CalendarError> {
    let year = hebrew_year(year)?;
    Ok(YearInfo {
        year: year.get(),
        is_leap: year.is_leap(),
        days: i32::from(year.days()),
        length: year.length().into(),
        is_cheshvan_long: year.is_cheshvan_long(),
        is_kislev_short: year.is_kislev_short(),
        months: year.months().iter().map(|&month| month.into()).collect(),
    })
}

pub(crate) fn days_in_month(year: i32, month: HebrewMonth) -> Option<i32> {
    hebrew_year(year).ok()?.days_in(month.into()).map(i32::from)
}

pub(crate) fn rosh_hashana(year: i32) -> Result<k::HebrewDate, CalendarError> {
    hebrew_year(year)?.rosh_hashana().ok_or(CalendarError::InvalidDate)
}

impl From<Region> for k::Region {
    fn from(region: Region) -> Self {
        match region {
            Region::Israel => Self::Israel,
            Region::Diaspora => Self::Diaspora,
        }
    }
}

impl From<HebrewMonth> for k::HebrewMonth {
    fn from(month: HebrewMonth) -> Self {
        match month {
            HebrewMonth::Tishrei => Self::Tishrei,
            HebrewMonth::Cheshvan => Self::Cheshvan,
            HebrewMonth::Kislev => Self::Kislev,
            HebrewMonth::Teves => Self::Teves,
            HebrewMonth::Shevat => Self::Shevat,
            HebrewMonth::AdarI => Self::AdarI,
            HebrewMonth::Adar => Self::Adar,
            HebrewMonth::Nisan => Self::Nisan,
            HebrewMonth::Iyar => Self::Iyar,
            HebrewMonth::Sivan => Self::Sivan,
            HebrewMonth::Tammuz => Self::Tammuz,
            HebrewMonth::Av => Self::Av,
            HebrewMonth::Elul => Self::Elul,
        }
    }
}

impl From<k::HebrewMonth> for HebrewMonth {
    fn from(month: k::HebrewMonth) -> Self {
        match month {
            k::HebrewMonth::Tishrei => Self::Tishrei,
            k::HebrewMonth::Cheshvan => Self::Cheshvan,
            k::HebrewMonth::Kislev => Self::Kislev,
            k::HebrewMonth::Teves => Self::Teves,
            k::HebrewMonth::Shevat => Self::Shevat,
            k::HebrewMonth::AdarI => Self::AdarI,
            k::HebrewMonth::Adar => Self::Adar,
            k::HebrewMonth::Nisan => Self::Nisan,
            k::HebrewMonth::Iyar => Self::Iyar,
            k::HebrewMonth::Sivan => Self::Sivan,
            k::HebrewMonth::Tammuz => Self::Tammuz,
            k::HebrewMonth::Av => Self::Av,
            k::HebrewMonth::Elul => Self::Elul,
        }
    }
}

impl From<civil::Weekday> for Weekday {
    fn from(weekday: civil::Weekday) -> Self {
        match weekday {
            civil::Weekday::Sunday => Self::Sunday,
            civil::Weekday::Monday => Self::Monday,
            civil::Weekday::Tuesday => Self::Tuesday,
            civil::Weekday::Wednesday => Self::Wednesday,
            civil::Weekday::Thursday => Self::Thursday,
            civil::Weekday::Friday => Self::Friday,
            civil::Weekday::Saturday => Self::Shabbos,
        }
    }
}

impl From<k::YearLength> for YearLength {
    fn from(length: k::YearLength) -> Self {
        match length {
            k::YearLength::Chaserim => Self::Chaserim,
            k::YearLength::Kesidran => Self::Kesidran,
            k::YearLength::Shelaimim => Self::Shelaimim,
        }
    }
}

/// Writes both `From` impls between the binding enum and kosher-rust's for enums whose variants
/// carry no data and have the same names.
macro_rules! mirror {
    ($ours:ty, $theirs:ty, [$($variant:ident),* $(,)?]) => {
        impl From<$ours> for $theirs {
            fn from(value: $ours) -> Self {
                type O = $ours;
                match value {
                    $(O::$variant => Self::$variant,)*
                }
            }
        }

        impl From<$theirs> for $ours {
            fn from(value: $theirs) -> Self {
                type T = $theirs;
                match value {
                    $(T::$variant => Self::$variant,)*
                }
            }
        }
    };
    // For a `#[non_exhaustive]` kosher-rust enum: `TryFrom` back, failing for a new variant.
    ($ours:ty, $theirs:ty, non_exhaustive, [$($variant:ident),* $(,)?]) => {
        impl From<$ours> for $theirs {
            fn from(value: $ours) -> Self {
                type O = $ours;
                match value {
                    $(O::$variant => Self::$variant,)*
                }
            }
        }

        impl TryFrom<$theirs> for $ours {
            type Error = ();

            fn try_from(value: $theirs) -> Result<Self, ()> {
                type T = $theirs;
                Ok(match value {
                    $(T::$variant => Self::$variant,)*
                    _ => return Err(()),
                })
            }
        }
    };
}
pub(crate) use mirror;

mirror!(
    Parsha,
    k::Parsha,
    [
        Bereshis,
        Noach,
        LechLecha,
        Vayera,
        ChayeiSara,
        Toldos,
        Vayetzei,
        Vayishlach,
        Vayeshev,
        Miketz,
        Vayigash,
        Vayechi,
        Shemos,
        Vaera,
        Bo,
        Beshalach,
        Yisro,
        Mishpatim,
        Terumah,
        Tetzaveh,
        KiSisa,
        Vayakhel,
        Pekudei,
        Vayikra,
        Tzav,
        Shmini,
        Tazria,
        Metzora,
        AchreiMos,
        Kedoshim,
        Emor,
        Behar,
        Bechukosai,
        Bamidbar,
        Nasso,
        Behaaloscha,
        Shlach,
        Korach,
        Chukas,
        Balak,
        Pinchas,
        Matos,
        Masei,
        Devarim,
        Vaeschanan,
        Eikev,
        Reeh,
        Shoftim,
        KiSeitzei,
        KiSavo,
        Nitzavim,
        Vayeilech,
        HaAzinu,
        VezosHabracha,
        VayakhelPekudei,
        TazriaMetzora,
        AchreiMosKedoshim,
        BeharBechukosai,
        ChukasBalak,
        MatosMasei,
        NitzavimVayeilech,
        Shekalim,
        Zachor,
        Parah,
        Hachodesh,
        Shuva,
        Shira,
        Hagadol,
        Chazon,
        Nachamu,
    ]
);

/// The conversions both ways for [`SpecialDay`], whose numbered variants become named fields, each
/// with the range its day must be in.
macro_rules! special_days {
    ([$($variant:ident),* $(,)?], [$($numbered:ident: $days:expr),* $(,)?]) => {
        impl TryFrom<SpecialDay> for k::SpecialDay {
            type Error = CalendarError;

            fn try_from(special_day: SpecialDay) -> Result<Self, CalendarError> {
                Ok(match special_day {
                    $(SpecialDay::$variant => Self::$variant,)*
                    $(SpecialDay::$numbered { day } => Self::$numbered(valid_number(day, $days).ok_or(CalendarError::InvalidSpecialDay)?),)*
                })
            }
        }

        /// Fails for a holiday kosher-rust added that the bindings don't have yet.
        impl TryFrom<k::SpecialDay> for SpecialDay {
            type Error = ();

            fn try_from(special_day: k::SpecialDay) -> Result<Self, ()> {
                Ok(match special_day {
                    $(k::SpecialDay::$variant => Self::$variant,)*
                    $(k::SpecialDay::$numbered(day) => Self::$numbered { day: i32::from(day) },)*
                    _ => return Err(()),
                })
            }
        }
    };
}

special_days!(
    [
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
        TenthOfTeves,
        TuBshvat,
        FastOfEsther,
        Purim,
        ShushanPurim,
        PurimKatan,
        RoshChodesh,
        YomHaShoah,
        YomHaZikaron,
        YomHaAtzmaut,
        YomYerushalayim,
        LagBomer,
        ShushanPurimKatan,
        IsruChag,
        YomKippurKatan,
        Behab,
        FastOfTheFirstborn,
        BirchasHachamah,
        MacharChodesh,
        ShabbosMevarchim,
    ],
    [Chanukah: 1..=8, CountOfTheOmer: 1..=49]
);
