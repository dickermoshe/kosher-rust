//! The JavaScript API of the kosher-rust npm package, built with wasm-bindgen.
//!
//! The classes and functions here are what `bindings/kosher-rust-js` publishes. Their TypeScript
//! types come from wasm-bindgen and from the `Tsify` derives on the crate's own types, so the ids
//! JavaScript uses, such as `"sofZmanShmaGra"`, are the enums' variant names in camel case.

mod calendar;
mod date;
mod error;
mod limudim;
mod zmanim;
mod zone;

use jiff::Timestamp;
use wasm_bindgen::JsValue;

use error::Error;

/// `timestamp` as a JavaScript `Date`.
fn js_date(timestamp: Timestamp) -> JsValue {
    js_sys::Date::new(&JsValue::from_f64(timestamp.as_millisecond() as f64)).into()
}

/// `number` as a `T`, or a `RangeError` with `message` when it isn't a whole number that fits.
fn whole<T: TryFrom<i64>>(number: f64, message: &'static str) -> Result<T, Error> {
    let fits = number.fract() == 0.0 && number.abs() < 2f64.powi(53);
    fits.then(|| T::try_from(number as i64).ok())
        .flatten()
        .ok_or(Error::Range(message))
}
