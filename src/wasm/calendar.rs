use jiff::civil;
use serde::Serialize;
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use super::date::{self, GregorianDate};
use super::error::Error;
use super::{js_date, whole};
use crate::calendar::{HebrewMonth, HebrewYear, Parsha, SpecialDay, YearLength};
use crate::{HebrewDate, Region};

/// A month of the Hebrew calendar.
#[derive(Serialize, Tsify)]
pub struct HebrewMonthInfo {
    /// `"adar"` is Adar II in a leap year.
    id: HebrewMonth,
    name: &'static str,
    hebrew: &'static str,
}

impl From<HebrewMonth> for HebrewMonthInfo {
    fn from(month: HebrewMonth) -> Self {
        Self {
            id: month,
            name: month.name(),
            hebrew: month.hebrew(),
        }
    }
}

/// A Torah reading for Shabbos. It is a weekly *parsha*, a pair read together, or a special Shabbos.
#[derive(Serialize, Tsify)]
pub struct ParshaInfo {
    id: Parsha,
    name: &'static str,
    hebrew: &'static str,
}

impl From<Parsha> for ParshaInfo {
    fn from(parsha: Parsha) -> Self {
        Self {
            id: parsha,
            name: parsha.name(),
            hebrew: parsha.hebrew(),
        }
    }
}

/// A holiday, fast or other notable day.
#[derive(Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct SpecialDayInfo {
    #[tsify(type = "SpecialDay[\"id\"]")]
    id: String,
    /// The day of Chanukah (1 to 8) or of the Omer (1 to 49). Absent on other days.
    #[tsify(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    day: Option<u8>,
    name: &'static str,
    hebrew: &'static str,
    /// Whether work is forbidden, as on Yom Tov.
    is_assur_bemelacha: bool,
    /// Whether it is a public fast.
    is_fast_day: bool,
    /// Whether it is one of the modern Israeli days.
    is_modern: bool,
}

impl From<SpecialDay> for SpecialDayInfo {
    fn from(day: SpecialDay) -> Self {
        Self {
            id: special_day_id(day),
            day: match day {
                SpecialDay::Chanukah(n) | SpecialDay::CountOfTheOmer(n) => Some(n),
                _ => None,
            },
            name: day.name(),
            hebrew: day.hebrew(),
            is_assur_bemelacha: day.is_assur_bemelacha(),
            is_fast_day: day.is_fast_day(),
            is_modern: day.is_modern(),
        }
    }
}

/// The molad, the mean new moon that begins a Hebrew month. The weekday, hours, minutes and
/// *chalakim* are in Jerusalem mean time, as announced in shul.
#[derive(Serialize, Tsify)]
pub struct Molad {
    /// The moment of the molad.
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "Date")]
    time: JsValue,
    /// From 0 for Sunday to 6 for Shabbos, as `Date.getDay` counts.
    weekday: i8,
    hours: u8,
    minutes: u8,
    /// From 0 to 17. A *chelek* is 1/1080 of an hour.
    chalakim: u8,
}

/// A day on the Hebrew calendar, and everything the calendar says about it.
///
/// A Hebrew day begins at nightfall, but a `HebrewDate` is the civil day it mostly overlaps, so the
/// Hebrew date of 23 April 2024 is 15 Nisan, the first day of Pesach. Dates from 1 January of year 1
/// (18 Teves 3761) through 29 Elul 9999 are covered.
#[wasm_bindgen(js_name = HebrewDate)]
#[derive(Clone, Copy)]
pub struct JsHebrewDate {
    date: HebrewDate,
    gregorian: civil::Date,
}

impl From<HebrewDate> for JsHebrewDate {
    fn from(date: HebrewDate) -> Self {
        Self {
            date,
            gregorian: date.to_gregorian(),
        }
    }
}

#[wasm_bindgen(js_class = HebrewDate)]
impl JsHebrewDate {
    /// The Hebrew date of a Gregorian day.
    pub fn from(#[wasm_bindgen(unchecked_param_type = "DateInput")] date: JsValue) -> Result<JsHebrewDate, Error> {
        Ok(HebrewDate::try_from(date::gregorian(&date)?)?.into())
    }

    /// The Hebrew date `day` `month` `year`, such as `HebrewDate.of(5785, "nisan", 15)`.
    pub fn of(year: f64, month: Ts<HebrewMonth>, day: f64) -> Result<JsHebrewDate, Error> {
        const MESSAGE: &str = "there is no such Hebrew date";
        let date = HebrewDate::new(whole(year, MESSAGE)?, month.to_rust()?, whole(day, MESSAGE)?);
        Ok(date.ok_or(Error::Range(MESSAGE))?.into())
    }

    /// The Hebrew year, such as 5784.
    #[wasm_bindgen(getter)]
    pub fn year(&self) -> i32 {
        self.date.year().get()
    }

    /// The month.
    #[wasm_bindgen(getter)]
    pub fn month(&self) -> Result<Ts<HebrewMonthInfo>, Error> {
        Ok(Ts::from_rust(&self.date.month().into())?)
    }

    /// The day of the month, from 1.
    #[wasm_bindgen(getter)]
    pub fn day(&self) -> u8 {
        self.date.day()
    }

    /// The day of the year, from 1 on Rosh Hashana.
    #[wasm_bindgen(getter, js_name = dayOfYear)]
    pub fn day_of_year(&self) -> u16 {
        self.date.day_of_year()
    }

    /// The day of the week, from 0 for Sunday to 6 for Shabbos, as `Date.getDay` counts.
    #[wasm_bindgen(getter)]
    pub fn weekday(&self) -> i8 {
        self.date.weekday().to_sunday_zero_offset()
    }

    /// The same day on the Gregorian calendar.
    #[wasm_bindgen(getter)]
    pub fn gregorian(&self) -> Result<Ts<GregorianDate>, Error> {
        Ok(Ts::from_rust(&self.gregorian.into())?)
    }

    /// The year, with its length and months.
    #[wasm_bindgen(getter, js_name = hebrewYear)]
    pub fn hebrew_year(&self) -> JsHebrewYear {
        JsHebrewYear(self.date.year())
    }

    /// The date `days` days later, or earlier when `days` is negative.
    #[wasm_bindgen(js_name = addDays)]
    pub fn add_days(&self, days: f64) -> Result<JsHebrewDate, Error> {
        let days = whole(days, "the number of days must be a whole number")?;
        Ok(self.date.add_days(days).ok_or(crate::calendar::DateOutOfRange)?.into())
    }

    /// The holidays, fasts and other notable days on this date, in calendar order. The modern
    /// Israeli days are included; filter on `isModern` to leave them out.
    #[wasm_bindgen(js_name = specialDays)]
    pub fn special_days(&self, region: Ts<Region>) -> Result<Vec<Ts<SpecialDayInfo>>, Error> {
        let region = region.to_rust()?;
        let days = self.date.special_days(region);
        Ok(days.map(|day| Ts::from_rust(&day.into())).collect::<Result<_, _>>()?)
    }

    /// Whether this date is the special day `id`, such as `"purim"`.
    pub fn is(
        &self,
        #[wasm_bindgen(unchecked_param_type = "SpecialDay[\"id\"]")] id: String,
        region: Ts<Region>,
    ) -> Result<bool, Error> {
        let region = region.to_rust()?;
        let kind = SpecialDay::ALL
            .into_iter()
            .find(|day| special_day_id(*day) == id)
            .ok_or_else(|| Error::Type(format!("there is no special day \"{id}\"")))?;
        let kind = core::mem::discriminant(&kind);
        Ok(self
            .date
            .special_days(region)
            .any(|day| core::mem::discriminant(&day) == kind))
    }

    /// Whether work is forbidden: Shabbos, or a day of Yom Tov.
    #[wasm_bindgen(js_name = isAssurBemelacha)]
    pub fn is_assur_bemelacha(&self, region: Ts<Region>) -> Result<bool, Error> {
        Ok(self.date.is_assur_bemelacha(region.to_rust()?))
    }

    /// Whether candles are lit this evening, because tomorrow is Shabbos or Yom Tov.
    #[wasm_bindgen(js_name = hasCandleLighting)]
    pub fn has_candle_lighting(&self, region: Ts<Region>) -> Result<bool, Error> {
        Ok(self.date.has_candle_lighting(region.to_rust()?))
    }

    /// Whether this is one of the Ten Days of Repentance, 1 to 10 Tishrei.
    #[wasm_bindgen(js_name = isAseresYemeiTeshuva)]
    pub fn is_aseres_yemei_teshuva(&self) -> bool {
        self.date.is_aseres_yemei_teshuva()
    }

    /// The weekly *parsha* read on this Shabbos. `undefined` on a weekday, and when Yom Tov replaces
    /// it.
    pub fn parsha(&self, region: Ts<Region>) -> Result<Option<Ts<ParshaInfo>>, Error> {
        parsha(self.date.parsha(region.to_rust()?))
    }

    /// The special Shabbos this is, such as Shekalim or Hagadol. `undefined` on other days.
    #[wasm_bindgen(js_name = specialParsha)]
    pub fn special_parsha(&self, region: Ts<Region>) -> Result<Option<Ts<ParshaInfo>>, Error> {
        parsha(self.date.special_parsha(region.to_rust()?))
    }

    /// The *parsha* of the next Shabbos that has one. On Shabbos this is next week's.
    #[wasm_bindgen(js_name = nextParsha)]
    pub fn next_parsha(&self, region: Ts<Region>) -> Result<Option<Ts<ParshaInfo>>, Error> {
        parsha(self.date.next_parsha(region.to_rust()?))
    }

    /// The molad of this date's month.
    pub fn molad(&self) -> Result<Ts<Molad>, Error> {
        let molad = self.date.molad();
        Ok(Ts::from_rust(&Molad {
            time: js_date(molad.timestamp()),
            weekday: molad.weekday().to_sunday_zero_offset(),
            hours: molad.hours(),
            minutes: molad.minutes(),
            chalakim: molad.chalakim(),
        })?)
    }

    /// Whether `other` is the same day.
    pub fn equals(&self, other: &JsHebrewDate) -> bool {
        self.date == other.date
    }

    /// Negative when this date is before `other`, positive after, 0 on the same day.
    pub fn compare(&self, other: &JsHebrewDate) -> i32 {
        self.date.cmp(&other.date) as i32
    }

    /// Like `"15 Nisan 5784"`, with Adar I and Adar II in a leap year.
    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.date.to_string()
    }

    /// The Gregorian day as `"YYYY-MM-DD"`.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn json(&self) -> String {
        self.gregorian.to_string()
    }
}

/// The id JavaScript uses for `day`, such as `"purim"`.
fn special_day_id(day: SpecialDay) -> String {
    serde_wasm_bindgen::to_value(&day)
        .ok()
        .and_then(|value| js_sys::Reflect::get(&value, &"id".into()).ok())
        .and_then(|id| id.as_string())
        .unwrap_or_default()
}

fn parsha(parsha: Option<Parsha>) -> Result<Option<Ts<ParshaInfo>>, Error> {
    Ok(parsha.map(|p| Ts::from_rust(&p.into())).transpose()?)
}

/// A year of the Hebrew calendar, from 3761 to 9999.
#[wasm_bindgen(js_name = HebrewYear)]
pub struct JsHebrewYear(HebrewYear);

#[wasm_bindgen(js_class = HebrewYear)]
impl JsHebrewYear {
    /// The Hebrew year `year`.
    pub fn of(year: f64) -> Result<JsHebrewYear, Error> {
        const MESSAGE: &str = "the Hebrew year must be from 3761 to 9999";
        HebrewYear::new(whole(year, MESSAGE)?)
            .map(Self)
            .ok_or(Error::Range(MESSAGE))
    }

    /// The year, such as 5784.
    #[wasm_bindgen(getter)]
    pub fn year(&self) -> i32 {
        self.0.get()
    }

    /// Whether it has two Adars.
    #[wasm_bindgen(getter, js_name = isLeap)]
    pub fn is_leap(&self) -> bool {
        self.0.is_leap()
    }

    /// 353 to 355 days in a common year, 383 to 385 in a leap year.
    #[wasm_bindgen(getter)]
    pub fn days(&self) -> u16 {
        self.0.days()
    }

    /// Whether Cheshvan and Kislev are short, regular or long.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> Result<Ts<YearLength>, Error> {
        Ok(Ts::from_rust(&self.0.length())?)
    }

    /// Whether Cheshvan has 30 days.
    #[wasm_bindgen(getter, js_name = isCheshvanLong)]
    pub fn is_cheshvan_long(&self) -> bool {
        self.0.is_cheshvan_long()
    }

    /// Whether Kislev has 29 days.
    #[wasm_bindgen(getter, js_name = isKislevShort)]
    pub fn is_kislev_short(&self) -> bool {
        self.0.is_kislev_short()
    }

    /// The months of the year from Tishrei, with Adar I in a leap year.
    #[wasm_bindgen(getter)]
    pub fn months(&self) -> Result<Vec<Ts<HebrewMonthInfo>>, Error> {
        let months = self.0.months().iter();
        Ok(months
            .map(|&month| Ts::from_rust(&month.into()))
            .collect::<Result<_, _>>()?)
    }

    /// The days in `month`, or `undefined` when the year has no such month.
    #[wasm_bindgen(js_name = daysIn)]
    pub fn days_in(&self, month: Ts<HebrewMonth>) -> Result<Option<u8>, Error> {
        Ok(self.0.days_in(month.to_rust()?))
    }

    /// 1 Tishrei.
    #[wasm_bindgen(getter, js_name = roshHashana)]
    pub fn rosh_hashana(&self) -> Result<JsHebrewDate, Error> {
        Ok(self.0.rosh_hashana().ok_or(crate::calendar::DateOutOfRange)?.into())
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn display(&self) -> String {
        self.0.to_string()
    }
}

/// Every Hebrew month from Tishrei, as in a leap year.
#[wasm_bindgen(js_name = allHebrewMonths)]
pub fn all_hebrew_months() -> Result<Vec<Ts<HebrewMonthInfo>>, Error> {
    let months = HebrewMonth::ALL.into_iter();
    Ok(months
        .map(|month| Ts::from_rust(&month.into()))
        .collect::<Result<_, _>>()?)
}

/// Every parsha: the weekly ones, the pairs, then the special Shabbosos.
#[wasm_bindgen(js_name = allParshiyos)]
pub fn all_parshiyos() -> Result<Vec<Ts<ParshaInfo>>, Error> {
    let parshiyos = Parsha::ALL.into_iter();
    Ok(parshiyos
        .map(|parsha| Ts::from_rust(&parsha.into()))
        .collect::<Result<_, _>>()?)
}

/// Every kind of special day. Chanukah and the Omer are listed once, as their first day.
#[wasm_bindgen(js_name = allSpecialDays)]
pub fn all_special_days() -> Result<Vec<Ts<SpecialDayInfo>>, Error> {
    let days = SpecialDay::ALL.into_iter();
    Ok(days.map(|day| Ts::from_rust(&day.into())).collect::<Result<_, _>>()?)
}
