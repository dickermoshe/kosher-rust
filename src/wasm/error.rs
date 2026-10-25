use wasm_bindgen::JsValue;

use crate::LocationError;
use crate::calendar::DateOutOfRange;

/// Why a call was rejected. JavaScript receives it as a `RangeError` or `TypeError`, or as the
/// exception a time zone threw.
pub enum Error {
    /// An argument outside what the calculations accept.
    Range(&'static str),
    /// An argument of the wrong kind.
    Type(String),
    /// An exception from JavaScript, passed on.
    Thrown(JsValue),
}

impl From<DateOutOfRange> for Error {
    fn from(_: DateOutOfRange) -> Self {
        Self::Range("the date is outside the range a HebrewDate covers, 1 January of year 1 through 29 Elul 9999")
    }
}

/// Date arithmetic fails only past the ends of the range jiff holds.
impl From<jiff::Error> for Error {
    fn from(_: jiff::Error) -> Self {
        DateOutOfRange.into()
    }
}

impl From<LocationError> for Error {
    fn from(error: LocationError) -> Self {
        Self::Range(match error {
            LocationError::InvalidLatitude => "latitude must be between -90 and 90 degrees",
            LocationError::InvalidLongitude => "longitude must be between -180 and 180 degrees",
            LocationError::InvalidElevation => "elevation must be a finite number of meters, 0 or more",
        })
    }
}

impl From<tsify::Error> for Error {
    fn from(error: tsify::Error) -> Self {
        Self::Type(error.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(thrown: JsValue) -> Self {
        Self::Thrown(thrown)
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        match error {
            Error::Range(message) => js_sys::RangeError::new(message).into(),
            Error::Type(message) => js_sys::TypeError::new(&message).into(),
            Error::Thrown(thrown) => thrown,
        }
    }
}
