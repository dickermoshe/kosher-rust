use jiff::civil;
use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use super::error::Error;
use crate::calendar::DateOutOfRange;

#[wasm_bindgen(typescript_custom_section)]
const DATE_INPUT: &str = r#"
/**
 * A Gregorian day, given as an object with `year`, `month` (from 1) and `day` such as a
 * `Temporal.PlainDate`, a `HebrewDate` for its Gregorian day, or a JavaScript `Date` for its day in
 * the runtime's local time zone. A `Date` made from an ISO string such as `new Date("2024-04-23")` is
 * midnight UTC, which is the evening before west of Greenwich.
 */
export type DateInput = Date | GregorianDate | { readonly gregorian: GregorianDate };
"#;

/// A day on the Gregorian calendar.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
pub struct GregorianDate {
    year: i16,
    /// 1 for January to 12 for December.
    month: i8,
    day: i8,
}

impl From<civil::Date> for GregorianDate {
    fn from(date: civil::Date) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

const NO_SUCH_DATE: Error = Error::Range("there is no such Gregorian date");

/// The number `object` has at `key`.
fn number(object: &JsValue, key: &str) -> Option<f64> {
    Reflect::get(object, &JsValue::from_str(key)).ok()?.as_f64()
}

/// The Gregorian day `input` gives. See `DateInput`.
pub(crate) fn gregorian(input: &JsValue) -> Result<civil::Date, Error> {
    if let Some(date) = input.dyn_ref::<js_sys::Date>() {
        if date.get_time().is_nan() {
            return Err(Error::Range("the Date is invalid"));
        }
        // The year arrives as a signed 32-bit number. A `Date` holds years up to 275760.
        let year = i16::try_from(date.get_full_year() as i32).ok().ok_or(DateOutOfRange)?;
        return Ok(civil::Date::new(
            year,
            date.get_month() as i8 + 1,
            date.get_date() as i8,
        )?);
    }
    if input.is_object() {
        let inner = Reflect::get(input, &JsValue::from_str("gregorian"))?;
        if inner.is_object() {
            return gregorian(&inner);
        }
        if let (Some(year), Some(month), Some(day)) =
            (number(input, "year"), number(input, "month"), number(input, "day"))
        {
            let whole = |n: f64| (n.fract() == 0.0).then_some(n);
            let (Some(year), Some(month), Some(day)) = (whole(year), whole(month), whole(day)) else {
                return Err(NO_SUCH_DATE);
            };
            if !(-9999.0..=9999.0).contains(&year) || !(1.0..=12.0).contains(&month) || !(1.0..=31.0).contains(&day) {
                return Err(NO_SUCH_DATE);
            }
            return civil::Date::new(year as i16, month as i8, day as i8)
                .ok()
                .ok_or(NO_SUCH_DATE);
        }
    }
    Err(Error::Type(
        "a date is a Date, or an object with year, month and day".into(),
    ))
}
