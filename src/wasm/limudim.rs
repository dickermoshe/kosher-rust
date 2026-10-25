use serde::Serialize;
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use super::date;
use super::error::Error;
use crate::Region;
use crate::limudim::{
    Amud, AmudYomiDirshu, Daf, DafHashavuaBavli, DafYomiBavli, DafYomiYerushalmi, Mishna, MishnaPair, MishnaTractate,
    MishnaYomis, PirkeiAvos, PirkeiAvosUnit, Schedule, Side, TehillimMonthly, TehillimUnit,
};

/// A tractate of the Mishnah or Talmud. Its `id` is the tractate of the Mishnah with the same name.
#[derive(Serialize, Tsify)]
pub struct TractateInfo {
    id: MishnaTractate,
    name: &'static str,
    hebrew: &'static str,
}

impl From<MishnaTractate> for TractateInfo {
    fn from(tractate: MishnaTractate) -> Self {
        Self {
            id: tractate,
            name: tractate.name(),
            hebrew: tractate.hebrew(),
        }
    }
}

fn tractate(tractate: impl Into<MishnaTractate>) -> Result<Ts<TractateInfo>, Error> {
    Ok(Ts::from_rust(&TractateInfo::from(tractate.into()))?)
}

/// A page (*daf*) of the Talmud, both sides of one leaf. Prints as "Bava Metzia 55".
#[wasm_bindgen(js_name = Daf)]
pub struct JsDaf(Daf<MishnaTractate>);

impl<T: Into<MishnaTractate>> From<Daf<T>> for JsDaf {
    fn from(daf: Daf<T>) -> Self {
        Self(Daf {
            tractate: daf.tractate.into(),
            daf: daf.daf,
        })
    }
}

#[wasm_bindgen(js_class = Daf)]
impl JsDaf {
    #[wasm_bindgen(getter)]
    pub fn tractate(&self) -> Result<Ts<TractateInfo>, Error> {
        tractate(self.0.tractate)
    }

    /// The page number. The Bavli starts at 2, the Yerushalmi at 1.
    #[wasm_bindgen(getter)]
    pub fn daf(&self) -> u16 {
        self.0.daf
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// One side of a page (*amud*). Prints as "Bava Metzia 55b".
#[wasm_bindgen(js_name = Amud)]
pub struct JsAmud(Amud);

#[wasm_bindgen(js_class = Amud)]
impl JsAmud {
    #[wasm_bindgen(getter)]
    pub fn tractate(&self) -> Result<Ts<TractateInfo>, Error> {
        tractate(self.0.tractate)
    }

    /// The page number. The Bavli starts at 2.
    #[wasm_bindgen(getter)]
    pub fn daf(&self) -> u16 {
        self.0.daf
    }

    /// `"a"` for *amud aleph*, the front, and `"b"` for *amud beis*, the back.
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> Result<Ts<Side>, Error> {
        Ok(Ts::from_rust(&self.0.side)?)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// One *mishnah*. Prints as "Nazir 5:5".
#[wasm_bindgen(js_name = Mishna)]
pub struct JsMishna(Mishna);

#[wasm_bindgen(js_class = Mishna)]
impl JsMishna {
    #[wasm_bindgen(getter)]
    pub fn tractate(&self) -> Result<Ts<TractateInfo>, Error> {
        tractate(self.0.tractate)
    }

    /// The chapter (*perek*), from 1.
    #[wasm_bindgen(getter)]
    pub fn chapter(&self) -> u16 {
        self.0.chapter
    }

    /// The *mishnah* within the chapter, from 1.
    #[wasm_bindgen(getter)]
    pub fn mishna(&self) -> u16 {
        self.0.mishna
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// The two *mishnayos* of a day. Prints as "Nazir 5:5-6", "Nazir 5:10-6:1" or "Nazir 9:5-Sotah 1:1".
#[wasm_bindgen(js_name = MishnaPair)]
pub struct JsMishnaPair(MishnaPair);

#[wasm_bindgen(js_class = MishnaPair)]
impl JsMishnaPair {
    #[wasm_bindgen(getter)]
    pub fn first(&self) -> JsMishna {
        JsMishna(self.0.first)
    }

    #[wasm_bindgen(getter)]
    pub fn second(&self) -> JsMishna {
        JsMishna(self.0.second)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// The verses of a chapter.
#[derive(Serialize, Tsify)]
pub struct Verses {
    start: u16,
    end: u16,
}

/// A day's Tehillim, either whole chapters `start` to `end` or verses of one long chapter. Prints as
/// "Tehillim 1-9" or "Tehillim 119:1-96".
#[wasm_bindgen(js_name = TehillimPortion)]
pub struct JsTehillimPortion(TehillimUnit);

#[wasm_bindgen(js_class = TehillimPortion)]
impl JsTehillimPortion {
    /// The first chapter.
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> u8 {
        match self.0 {
            TehillimUnit::Psalms { start, .. } => start,
            TehillimUnit::PsalmVerses { psalm, .. } => psalm,
        }
    }

    /// The last chapter.
    #[wasm_bindgen(getter)]
    pub fn end(&self) -> u8 {
        match self.0 {
            TehillimUnit::Psalms { end, .. } => end,
            TehillimUnit::PsalmVerses { psalm, .. } => psalm,
        }
    }

    /// The verses, when the day has only part of one chapter.
    #[wasm_bindgen(getter)]
    pub fn verses(&self) -> Result<Option<Ts<Verses>>, Error> {
        Ok(match self.0 {
            TehillimUnit::Psalms { .. } => None,
            TehillimUnit::PsalmVerses {
                start_verse, end_verse, ..
            } => Some(Ts::from_rust(&Verses {
                start: start_verse,
                end: end_verse,
            })?),
        })
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// The Pirkei Avos read on a Shabbos afternoon. Prints as "Pirkei Avos 3" or "Pirkei Avos 5-6".
#[wasm_bindgen(js_name = PirkeiAvosPortion)]
pub struct JsPirkeiAvosPortion(PirkeiAvosUnit);

#[wasm_bindgen(js_class = PirkeiAvosPortion)]
impl JsPirkeiAvosPortion {
    /// One chapter, or two read together at the end of the summer.
    #[wasm_bindgen(getter, unchecked_return_type = "number[]")]
    pub fn chapters(&self) -> js_sys::Array {
        match self.0 {
            PirkeiAvosUnit::Single(chapter) => js_sys::Array::of1(&chapter.into()),
            PirkeiAvosUnit::Combined(first, second) => js_sys::Array::of2(&first.into(), &second.into()),
        }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// Daf Yomi, one daf of the Bavli a day since 1923. `undefined` before it began.
#[wasm_bindgen(js_name = dafYomiBavli)]
pub fn daf_yomi_bavli(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
) -> Result<Option<JsDaf>, Error> {
    Ok(DafYomiBavli.on(date::gregorian(&date)?).map(JsDaf::from))
}

/// One daf of the Bavli a week, from Sunday to Shabbos, since 2005.
#[wasm_bindgen(js_name = dafHashavuaBavli)]
pub fn daf_hashavua_bavli(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
) -> Result<Option<JsDaf>, Error> {
    Ok(DafHashavuaBavli.on(date::gregorian(&date)?).map(JsDaf::from))
}

/// Daf Yomi of the Vilna Yerushalmi, since 1980. `undefined` on Yom Kippur and Tishah B'Av.
#[wasm_bindgen(js_name = dafYomiYerushalmi)]
pub fn daf_yomi_yerushalmi(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
) -> Result<Option<JsDaf>, Error> {
    Ok(DafYomiYerushalmi.on(date::gregorian(&date)?).map(JsDaf::from))
}

/// Dirshu's Amud Yomi, one amud a day since 2023.
#[wasm_bindgen(js_name = amudYomiDirshu)]
pub fn amud_yomi_dirshu(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
) -> Result<Option<JsAmud>, Error> {
    Ok(AmudYomiDirshu.on(date::gregorian(&date)?).map(JsAmud))
}

/// Mishna Yomis, two *mishnayos* a day since 1947.
#[wasm_bindgen(js_name = mishnaYomis)]
pub fn mishna_yomis(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
) -> Result<Option<JsMishnaPair>, Error> {
    Ok(MishnaYomis.on(date::gregorian(&date)?).map(JsMishnaPair))
}

/// The book of Tehillim divided over the days of the Hebrew month.
#[wasm_bindgen(js_name = tehillimMonthly)]
pub fn tehillim_monthly(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
) -> Result<Option<JsTehillimPortion>, Error> {
    Ok(TehillimMonthly.on(date::gregorian(&date)?).map(JsTehillimPortion))
}

/// The Pirkei Avos of the week, read on Shabbos afternoons from Pesach to Rosh Hashana. Every day
/// from Sunday to Shabbos gives that Shabbos's chapter. `undefined` for a week whose Shabbos has
/// none.
#[wasm_bindgen(js_name = pirkeiAvos)]
pub fn pirkei_avos(
    #[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue,
    region: Ts<Region>,
) -> Result<Option<JsPirkeiAvosPortion>, Error> {
    let region = region.to_rust()?;
    Ok(PirkeiAvos(region).on(date::gregorian(&date)?).map(JsPirkeiAvosPortion))
}

/// Every tractate, in the order of the Mishnah.
#[wasm_bindgen(js_name = allTractates)]
pub fn all_tractates() -> Result<Vec<Ts<TractateInfo>>, Error> {
    MishnaTractate::ALL.into_iter().map(tractate).collect()
}
