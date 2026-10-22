use jiff::civil;
use kosher_rust::calendar::{self as k, HebrewYear};

use crate::api::kosher::{
    KosherError,
    calendar::{HebrewMonth, Holiday, Parsha, Region, Weekday, YearInfo, YearLength},
};

pub(crate) fn hebrew_date(year: i32, month: HebrewMonth, day: i32) -> Result<k::HebrewDate, KosherError> {
    let day = u8::try_from(day).map_err(|_| KosherError::InvalidDate)?;
    k::HebrewDate::new(year, month.into(), day).ok_or(KosherError::InvalidDate)
}

pub(crate) fn year_info(year: i32) -> YearInfo {
    let year = HebrewYear::new(year);
    YearInfo {
        year: year.get(),
        is_leap: year.is_leap(),
        days: i32::from(year.days()),
        length: year.length().into(),
        is_cheshvan_long: year.is_cheshvan_long(),
        is_kislev_short: year.is_kislev_short(),
        months: year.months().iter().map(|&month| month.into()).collect(),
    }
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

/// Both `From` impls for [`Holiday`], whose numbered variants become named fields.
macro_rules! holidays {
    ([$($variant:ident),* $(,)?], [$($numbered:ident),* $(,)?]) => {
        impl From<Holiday> for k::Holiday {
            fn from(holiday: Holiday) -> Self {
                match holiday {
                    $(Holiday::$variant => Self::$variant,)*
                    // The day is 1 to 8 or 1 to 49; anything else came from outside and saturates.
                    $(Holiday::$numbered { day } => Self::$numbered(u8::try_from(day).unwrap_or(u8::MAX)),)*
                }
            }
        }

        impl From<k::Holiday> for Holiday {
            fn from(holiday: k::Holiday) -> Self {
                match holiday {
                    $(k::Holiday::$variant => Self::$variant,)*
                    $(k::Holiday::$numbered(day) => Self::$numbered { day: i32::from(day) },)*
                }
            }
        }
    };
}

holidays!(
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
        YomHazikaron,
        YomHaatzmaut,
        YomYerushalayim,
        LagBomer,
        ShushanPurimKatan,
        IsruChag,
        YomKippurKatan,
        Behab,
        FastOfTheFirstborn,
        BirchasHachamah,
        MacharHachodesh,
        ShabbosMevarchim,
    ],
    [Chanukah, CountOfTheOmer]
);
