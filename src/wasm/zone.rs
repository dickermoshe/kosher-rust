use std::cell::RefCell;

use jiff::{Timestamp, tz::Offset};
use js_sys::Function;
use wasm_bindgen::prelude::*;

use super::error::Error;
use crate::Zone;

thread_local! {
    /// The JavaScript function that turns an IANA name into a function from Unix milliseconds to
    /// the UTC offset in minutes.
    static RESOLVER: RefCell<Option<Function>> = const { RefCell::new(None) };
}

/// Sets the function that looks up time zone names. The package's entry points pass the one
/// built on `Intl.DateTimeFormat`.
#[wasm_bindgen(js_name = setTimeZoneResolver)]
pub fn set_time_zone_resolver(resolve: Function) {
    RESOLVER.with(|resolver| *resolver.borrow_mut() = Some(resolve));
}

/// A named time zone whose offsets come from the runtime's own time zone data.
///
/// [`Zone::offset`] can't fail, so when JavaScript throws or gives something that isn't an offset,
/// the error is kept and UTC is used. Call [`check`](Self::check) after calculating.
#[derive(Clone, Debug)]
pub struct JsZone {
    name: String,
    /// The function that gives the offset in minutes at a Unix millisecond.
    offset_at: Function,
    error: RefCell<Option<JsValue>>,
}

impl JsZone {
    /// The time zone `name`, such as `"America/New_York"`, or `None` without one.
    pub(crate) fn from_name(name: Option<String>) -> Result<Option<Self>, Error> {
        let Some(name) = name else {
            return Ok(None);
        };
        let resolver = RESOLVER.with(|resolver| resolver.borrow().clone());
        let resolver = resolver.ok_or(Error::Range("time zone names need the package's entry point"))?;
        let offset_at = resolver.call1(&JsValue::NULL, &JsValue::from_str(&name))?.dyn_into::<Function>()?;
        Ok(Some(Self {
            name,
            offset_at,
            error: RefCell::new(None),
        }))
    }

    /// The name the time zone was given by.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// The first error since the last check, if any.
    pub(crate) fn check(&self) -> Result<(), Error> {
        self.error
            .borrow_mut()
            .take()
            .map_or(Ok(()), |thrown| Err(Error::Thrown(thrown)))
    }

    fn try_offset(&self, instant: Timestamp) -> Result<Offset, Error> {
        let millis = JsValue::from_f64(instant.as_millisecond() as f64);
        let returned = self.offset_at.call1(&JsValue::NULL, &millis)?;
        let Some(minutes) = returned.as_f64().filter(|minutes| minutes.is_finite()) else {
            return Err(Error::Type(format!(
                "the time zone gave {}, not a number of minutes",
                String::from(js_sys::JsString::from(returned))
            )));
        };
        let Ok(offset) = Offset::from_seconds((minutes * 60.0).round() as i32) else {
            return Err(Error::Range(
                "the time zone gave an offset of more than 25:59:59 from UTC",
            ));
        };
        Ok(offset)
    }
}

impl Zone for JsZone {
    fn offset(&self, instant: Timestamp) -> Offset {
        self.try_offset(instant).unwrap_or_else(|error| {
            let mut kept = self.error.borrow_mut();
            if kept.is_none() {
                *kept = Some(error.into());
            }
            Offset::UTC
        })
    }
}

/// `zone`, checked by reading its offset at the start of `date`.
pub(crate) fn checked(zone: Option<JsZone>, date: jiff::civil::Date) -> Result<Option<JsZone>, Error> {
    if let Some(zone) = &zone {
        zone.offset(zone.start_of_day(date)?);
        zone.check()?;
    }
    Ok(zone)
}
