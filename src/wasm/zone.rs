use std::cell::RefCell;

use jiff::{Timestamp, tz::Offset};
use js_sys::{Function, Reflect};
use wasm_bindgen::prelude::*;

use super::error::Error;
use crate::Zone;

#[wasm_bindgen(typescript_custom_section)]
const TIME_ZONE: &str = r#"
/**
 * A time zone, described by its UTC offset at any instant.
 *
 * Any date library can supply one. `offsetAt` receives Unix milliseconds and returns the offset in
 * minutes east of UTC, so New York in summer is `-240` and Jerusalem in summer is `180`.
 */
export interface TimeZone {
  /** The UTC offset at `epochMilliseconds`, in minutes east of UTC. */
  offsetAt(epochMilliseconds: number): number;
  /** The zone's name, such as `"America/New_York"`, when it has one. */
  readonly name?: string;
}

/**
 * Anything that names or describes a time zone. It is an IANA name such as `"America/New_York"`,
 * looked up in the runtime's own time zone data, a {@link TimeZone}, or a function from Unix
 * milliseconds to the offset in minutes east of UTC.
 */
export type TimeZoneInput = string | TimeZone | ((epochMilliseconds: number) => number);
"#;

thread_local! {
    /// The JavaScript function that turns an IANA name into a [`TimeZone`] object.
    static RESOLVER: RefCell<Option<Function>> = const { RefCell::new(None) };
}

/// Sets the function that turns an IANA name into a time zone. The package's entry points pass
/// the one built on `Intl.DateTimeFormat`.
#[wasm_bindgen(js_name = setTimeZoneResolver)]
pub fn set_time_zone_resolver(resolve: Function) {
    RESOLVER.with(|resolver| *resolver.borrow_mut() = Some(resolve));
}

/// A time zone that asks JavaScript for its offset.
///
/// [`Zone::offset`] can't fail, so when JavaScript throws or gives something that isn't an offset,
/// the error is kept and UTC is used. Call [`check`](Self::check) after calculating.
#[derive(Clone, Debug)]
pub struct JsZone {
    /// The time zone as given, or as resolved from its name.
    zone: JsValue,
    /// The function that gives the offset in minutes, and the `this` it is called with.
    offset_at: Function,
    this: JsValue,
    error: RefCell<Option<JsValue>>,
}

impl JsZone {
    /// The time zone `input` names or describes, or `None` when it is `undefined` or `null`.
    pub(crate) fn from_input(input: JsValue) -> Result<Option<Self>, Error> {
        if input.is_undefined() || input.is_null() {
            return Ok(None);
        }
        let zone = match input.as_string() {
            Some(name) => {
                let resolver = RESOLVER.with(|resolver| resolver.borrow().clone());
                let resolver = resolver.ok_or(Error::Range("time zone names need the package's entry point"))?;
                resolver.call1(&JsValue::NULL, &JsValue::from_str(&name))?
            }
            None => input,
        };
        let (offset_at, this) = match zone.dyn_ref::<Function>() {
            Some(function) => (function.clone(), JsValue::NULL),
            None => {
                let method = if zone.is_object() {
                    Reflect::get(&zone, &"offsetAt".into())?
                } else {
                    JsValue::UNDEFINED
                };
                let Ok(method) = method.dyn_into::<Function>() else {
                    return Err(Error::Type(
                        "a time zone is an IANA name, an object with offsetAt, or a function".into(),
                    ));
                };
                (method, zone.clone())
            }
        };
        Ok(Some(Self {
            zone,
            offset_at,
            this,
            error: RefCell::new(None),
        }))
    }

    /// The time zone as given, or as resolved from its name.
    pub(crate) fn js_value(&self) -> JsValue {
        self.zone.clone()
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
        let returned = self.offset_at.call1(&self.this, &millis)?;
        let Some(minutes) = returned.as_f64().filter(|minutes| minutes.is_finite()) else {
            return Err(Error::Type(format!(
                "the time zone's offsetAt returned {}, not a number of minutes",
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
