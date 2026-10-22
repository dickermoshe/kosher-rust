use kosher_rust::limudim::{self as k, Schedule};

use super::calendar::mirror;
use crate::api::kosher::{
    calendar::HebrewDate,
    limudim::{Amud, Daf, Mishna, MishnaPair, PirkeiAvosUnit, Side, TehillimUnit, Tractate},
};

/// `schedule`'s portion on `date`.
pub(crate) fn on<S: Schedule, T: From<S::Unit>>(schedule: S, date: &HebrewDate) -> Option<T> {
    schedule.on(date.inner).map(T::from)
}

mirror!(
    Tractate,
    k::Tractate,
    [
        Berachos,
        Peah,
        Demai,
        Kilayim,
        Sheviis,
        Terumos,
        Maasros,
        MaaserSheni,
        Chalah,
        Orlah,
        Bikurim,
        Shabbos,
        Eruvin,
        Pesachim,
        Shekalim,
        Yoma,
        Sukkah,
        Beitzah,
        RoshHashanah,
        Taanis,
        Megillah,
        MoedKatan,
        Chagigah,
        Yevamos,
        Kesubos,
        Nedarim,
        Nazir,
        Sotah,
        Gitin,
        Kiddushin,
        BavaKamma,
        BavaMetzia,
        BavaBasra,
        Sanhedrin,
        Makkos,
        Shevuos,
        Eduyos,
        AvodahZarah,
        Avos,
        Horiyos,
        Zevachim,
        Menachos,
        Chullin,
        Bechoros,
        Arachin,
        Temurah,
        Kerisos,
        Meilah,
        Tamid,
        Midos,
        Kinnim,
        Keilim,
        Ohalos,
        Negaim,
        Parah,
        Taharos,
        Mikvaos,
        Niddah,
        Machshirin,
        Zavim,
        TevulYom,
        Yadayim,
        Uktzin,
    ]
);

mirror!(Side, k::Side, [Aleph, Bet]);

impl From<k::Daf> for Daf {
    fn from(daf: k::Daf) -> Self {
        Self {
            tractate: daf.tractate.into(),
            daf: i32::from(daf.daf),
            display: daf.to_string(),
        }
    }
}

impl From<k::Amud> for Amud {
    fn from(amud: k::Amud) -> Self {
        Self {
            tractate: amud.tractate.into(),
            daf: i32::from(amud.daf),
            side: amud.side.into(),
            display: amud.to_string(),
        }
    }
}

impl From<k::Mishna> for Mishna {
    fn from(mishna: k::Mishna) -> Self {
        Self {
            tractate: mishna.tractate.into(),
            // Chapters number at most 30.
            chapter: i32::try_from(mishna.chapter).unwrap_or(i32::MAX),
            mishna: i32::from(mishna.mishna),
            display: mishna.to_string(),
        }
    }
}

impl From<k::MishnaPair> for MishnaPair {
    fn from(pair: k::MishnaPair) -> Self {
        Self {
            first: pair.first.into(),
            second: pair.second.into(),
            display: pair.to_string(),
        }
    }
}

/// A chapter or verse number from outside, saturated into kosher-rust's narrower type.
fn narrow<T: TryFrom<i32> + Bounded>(value: i32) -> T {
    T::try_from(value).unwrap_or(if value < 0 { T::MIN } else { T::MAX })
}

trait Bounded {
    const MIN: Self;
    const MAX: Self;
}

impl Bounded for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl Bounded for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl From<k::TehillimUnit> for TehillimUnit {
    fn from(unit: k::TehillimUnit) -> Self {
        match unit {
            k::TehillimUnit::Psalms { start, end } => Self::Psalms {
                start: i32::from(start),
                end: i32::from(end),
            },
            k::TehillimUnit::PsalmVerses {
                psalm,
                start_verse,
                end_verse,
            } => Self::PsalmVerses {
                psalm: i32::from(psalm),
                start_verse: i32::from(start_verse),
                end_verse: i32::from(end_verse),
            },
        }
    }
}

impl From<TehillimUnit> for k::TehillimUnit {
    fn from(unit: TehillimUnit) -> Self {
        match unit {
            TehillimUnit::Psalms { start, end } => Self::Psalms {
                start: narrow(start),
                end: narrow(end),
            },
            TehillimUnit::PsalmVerses {
                psalm,
                start_verse,
                end_verse,
            } => Self::PsalmVerses {
                psalm: narrow(psalm),
                start_verse: narrow(start_verse),
                end_verse: narrow(end_verse),
            },
        }
    }
}

impl From<k::PirkeiAvosUnit> for PirkeiAvosUnit {
    fn from(unit: k::PirkeiAvosUnit) -> Self {
        match unit {
            k::PirkeiAvosUnit::Single(chapter) => Self::Single {
                chapter: i32::from(chapter),
            },
            k::PirkeiAvosUnit::Combined(first, second) => Self::Combined {
                first: i32::from(first),
                second: i32::from(second),
            },
        }
    }
}

impl From<PirkeiAvosUnit> for k::PirkeiAvosUnit {
    fn from(unit: PirkeiAvosUnit) -> Self {
        match unit {
            PirkeiAvosUnit::Single { chapter } => Self::Single(narrow(chapter)),
            PirkeiAvosUnit::Combined { first, second } => Self::Combined(narrow(first), narrow(second)),
        }
    }
}
