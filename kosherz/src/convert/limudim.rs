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
            chapter: i32::from(mishna.chapter),
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

/// `first` and `last` as kosher-rust's narrower type, if both are in `range` and `first` comes no
/// later than `last`.
fn valid_span<T: TryFrom<i32>>(first: i32, last: i32, range: RangeInclusive<i32>) -> Result<(T, T), LimudimError> {
    if first > last {
        return Err(LimudimError::InvalidUnit);
    }
    Ok((
        valid_number(first, range.clone()).ok_or(LimudimError::InvalidUnit)?,
        valid_number(last, range).ok_or(LimudimError::InvalidUnit)?,
    ))
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

impl TryFrom<TehillimUnit> for k::TehillimUnit {
    type Error = LimudimError;

    fn try_from(unit: TehillimUnit) -> Result<Self, LimudimError> {
        Ok(match unit {
            TehillimUnit::Psalms { start, end } => {
                let (start, end) = valid_span(start, end, PSALMS)?;
                Self::Psalms { start, end }
            }
            TehillimUnit::PsalmVerses {
                psalm,
                start_verse,
                end_verse,
            } => {
                let (start_verse, end_verse) = valid_span(start_verse, end_verse, VERSES)?;
                Self::PsalmVerses {
                    psalm: valid_number(psalm, PSALMS).ok_or(LimudimError::InvalidUnit)?,
                    start_verse,
                    end_verse,
                }
            }
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
            PirkeiAvosUnit::Combined { first, second } if first < second => {
                let (first, second) = valid_span(first, second, CHAPTERS_OF_AVOS)?;
                Self::Combined(first, second)
            }
            PirkeiAvosUnit::Combined { .. } => return Err(LimudimError::InvalidUnit),
        })
    }
}
