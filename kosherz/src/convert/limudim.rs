use std::ops::RangeInclusive;

use kosher_rust::limudim::{self as k, Schedule};

use super::{calendar::mirror, valid_number};
use crate::api::calendar::{
    HebrewDate,
    limudim::{Amud, Daf, LimudimError, Mishna, MishnaPair, PirkeiAvosUnit, Side, TehillimUnit, Tractate},
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
            number: i32::from(daf.daf),
            display: daf.to_string(),
        }
    }
}

impl From<k::Amud> for Amud {
    fn from(amud: k::Amud) -> Self {
        Self {
            tractate: amud.tractate.into(),
            number: i32::from(amud.daf),
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
            number: i32::from(mishna.mishna),
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

const PSALMS: RangeInclusive<i32> = 1..=150;
const VERSES: RangeInclusive<i32> = 1..=176;
const CHAPTERS_OF_AVOS: RangeInclusive<i32> = 1..=6;

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

impl TryFrom<TehillimUnit> for k::TehillimUnit {
    type Error = LimudimError;

    fn try_from(unit: TehillimUnit) -> Result<Self, LimudimError> {
        Ok(match unit {
            TehillimUnit::Psalms { start, end } => Self::Psalms {
                start: valid_number(start, PSALMS).ok_or(LimudimError::InvalidUnit)?,
                end: valid_number(end, PSALMS).ok_or(LimudimError::InvalidUnit)?,
            },
            TehillimUnit::PsalmVerses {
                psalm,
                start_verse,
                end_verse,
            } => Self::PsalmVerses {
                psalm: valid_number(psalm, PSALMS).ok_or(LimudimError::InvalidUnit)?,
                start_verse: valid_number(start_verse, VERSES).ok_or(LimudimError::InvalidUnit)?,
                end_verse: valid_number(end_verse, VERSES).ok_or(LimudimError::InvalidUnit)?,
            },
        })
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

impl TryFrom<PirkeiAvosUnit> for k::PirkeiAvosUnit {
    type Error = LimudimError;

    fn try_from(unit: PirkeiAvosUnit) -> Result<Self, LimudimError> {
        Ok(match unit {
            PirkeiAvosUnit::Single { chapter } => {
                Self::Single(valid_number(chapter, CHAPTERS_OF_AVOS).ok_or(LimudimError::InvalidUnit)?)
            }
            PirkeiAvosUnit::Combined { first, second } => Self::Combined(
                valid_number(first, CHAPTERS_OF_AVOS).ok_or(LimudimError::InvalidUnit)?,
                valid_number(second, CHAPTERS_OF_AVOS).ok_or(LimudimError::InvalidUnit)?,
            ),
        })
    }
}
