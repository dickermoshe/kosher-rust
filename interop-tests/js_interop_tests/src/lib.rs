use icu_calendar::{Date, cal::Hebrew};

use serde::Serialize;
use wasm_bindgen::prelude::*;

use kosher_rust::Region;
use kosher_rust::limudim::*;

#[derive(Serialize)]
pub struct SerializableTractate(String);

impl From<Tractate> for SerializableTractate {
    fn from(t: Tractate) -> Self {
        SerializableTractate(format!("{:?}", t))
    }
}

#[derive(Serialize)]
pub struct SerializableSide(String);

impl From<Side> for SerializableSide {
    fn from(s: Side) -> Self {
        SerializableSide(match s {
            Side::Aleph => "a".to_string(),
            Side::Bet => "b".to_string(),
        })
    }
}

#[derive(Serialize)]
pub struct SerializableDaf {
    tractate: SerializableTractate,
    page: u16,
}

impl From<Daf> for SerializableDaf {
    fn from(d: Daf) -> Self {
        SerializableDaf {
            tractate: d.tractate.into(),
            page: d.daf,
        }
    }
}

#[derive(Serialize)]
pub struct SerializableAmud {
    tractate: SerializableTractate,
    page: u16,
    side: SerializableSide,
}

impl From<Amud> for SerializableAmud {
    fn from(a: Amud) -> Self {
        SerializableAmud {
            tractate: a.tractate.into(),
            page: a.daf,
            side: a.side.into(),
        }
    }
}

#[derive(Serialize)]
pub struct SerializableMishna {
    tractate: SerializableTractate,
    chapter: u16,
    mishna: u16,
}

impl From<Mishna> for SerializableMishna {
    fn from(m: Mishna) -> Self {
        SerializableMishna {
            tractate: m.tractate.into(),
            chapter: m.chapter,
            mishna: m.mishna,
        }
    }
}

#[derive(Serialize)]
pub struct SerializableMishnas {
    start: SerializableMishna,
    end: SerializableMishna,
}

impl From<MishnaPair> for SerializableMishnas {
    fn from(m: MishnaPair) -> Self {
        SerializableMishnas {
            start: m.first.into(),
            end: m.second.into(),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SerializablePirkeiAvosUnit {
    Single { perek: u8 },
    Combined { perek1: u8, perek2: u8 },
}

impl From<PirkeiAvosUnit> for SerializablePirkeiAvosUnit {
    fn from(p: PirkeiAvosUnit) -> Self {
        match p {
            PirkeiAvosUnit::Single(perek) => SerializablePirkeiAvosUnit::Single { perek },
            PirkeiAvosUnit::Combined(p1, p2) => SerializablePirkeiAvosUnit::Combined { perek1: p1, perek2: p2 },
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SerializableTehillimUnit {
    Psalms {
        start: u8,
        end: u8,
    },
    PsalmVerses {
        psalm: u8,
        start_verse: u16,
        end_verse: u16,
    },
}

impl From<TehillimUnit> for SerializableTehillimUnit {
    fn from(t: TehillimUnit) -> Self {
        match t {
            TehillimUnit::Psalms { start, end } => SerializableTehillimUnit::Psalms { start, end },
            TehillimUnit::PsalmVerses {
                psalm,
                start_verse,
                end_verse,
            } => SerializableTehillimUnit::PsalmVerses {
                psalm,
                start_verse,
                end_verse,
            },
        }
    }
}

// Helper to create a Hebrew date from Gregorian year/month/day
fn hebrew_date_from_gregorian(year: i32, month: u8, day: u8) -> Option<Date<Hebrew>> {
    let iso = Date::try_new_iso(year, month, day).ok()?;
    Some(iso.to_calendar(Hebrew))
}

/// Get Daf Yomi Bavli for a given Gregorian date
/// Returns JSON: { "tractate": "Berachos", "page": 2 } or null
#[wasm_bindgen]
pub fn daf_yomi_bavli(year: i32, month: u8, day: u8) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = DafYomiBavli.on(date);
    match result {
        Some(daf) => {
            let serializable: SerializableDaf = daf.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}

/// Get Daf Yomi Yerushalmi for a given Gregorian date
#[wasm_bindgen]
pub fn daf_yomi_yerushalmi(year: i32, month: u8, day: u8) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = DafYomiYerushalmi.on(date);
    match result {
        Some(daf) => {
            let serializable: SerializableDaf = daf.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}

/// Get Daf Hashavua Bavli for a given Gregorian date
#[wasm_bindgen]
pub fn daf_hashavua_bavli(year: i32, month: u8, day: u8) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = DafHashavuaBavli.on(date);
    match result {
        Some(daf) => {
            let serializable: SerializableDaf = daf.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}

/// Get Amud Yomi Bavli Dirshu for a given Gregorian date
#[wasm_bindgen]
pub fn amud_yomi_bavli_dirshu(year: i32, month: u8, day: u8) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = AmudYomiDirshu.on(date);
    match result {
        Some(amud) => {
            let serializable: SerializableAmud = amud.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}

/// Get Mishna Yomis for a given Gregorian date
#[wasm_bindgen]
pub fn mishna_yomis(year: i32, month: u8, day: u8) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = MishnaYomis.on(date);
    match result {
        Some(mishnas) => {
            let serializable: SerializableMishnas = mishnas.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}

/// Get Pirkei Avos for a given Gregorian date (outside Israel)
#[wasm_bindgen]
pub fn pirkei_avos(year: i32, month: u8, day: u8, in_israel: bool) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = PirkeiAvos(if in_israel { Region::Israel } else { Region::Diaspora }).on(date);
    match result {
        Some(unit) => {
            let serializable: SerializablePirkeiAvosUnit = unit.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}

/// Get Tehillim Monthly for a given Gregorian date
#[wasm_bindgen]
pub fn tehillim_monthly(year: i32, month: u8, day: u8) -> JsValue {
    let date = match hebrew_date_from_gregorian(year, month, day) {
        Some(d) => d,
        None => return JsValue::NULL,
    };

    let result = TehillimMonthly.on(date);
    match result {
        Some(unit) => {
            let serializable: SerializableTehillimUnit = unit.into();
            serde_wasm_bindgen::to_value(&serializable).unwrap_or(JsValue::NULL)
        }
        None => JsValue::NULL,
    }
}
