//! How jiff's types cross to Dart.

// Under its own name, `Duration` is taken to be chrono's.
use core::time::Duration as StdDuration;

use flutter_rust_bridge::frb;
use jiff::{
    civil::{Date, Weekday},
    tz::Offset,
    SignedDuration, Timestamp, Unit,
};

// A value from Dart outside the range jiff holds is a mistake in the caller, and decoding it
// panics. Dart receives the panic as an exception.

/// An instant crosses as whole seconds since the Unix epoch, rounded to the nearest second.
#[frb(rust2dart(dart_type = "int", dart_code = "{}"))]
pub fn encode_timestamp(raw: Timestamp) -> i64 {
    raw.round(Unit::Second).unwrap_or(raw).as_second()
}

#[frb(dart2rust(dart_type = "int", dart_code = "{}"))]
pub fn decode_timestamp(raw: i64) -> Timestamp {
    Timestamp::from_second(raw).expect("seconds between the years -9999 and 9999")
}

/// A civil date crosses as the Dart record `GregorianDate`, its year, month and day.
#[frb(rust2dart(dart_type = "GregorianDate", dart_code = "dateToDart({})"))]
pub fn encode_date(raw: Date) -> (i16, i8, i8) {
    (raw.year(), raw.month(), raw.day())
}

#[frb(dart2rust(dart_type = "GregorianDate", dart_code = "dateFromDart({})"))]
pub fn decode_date(raw: (i16, i8, i8)) -> Date {
    Date::new(raw.0, raw.1, raw.2).expect("a date that exists, between the years -9999 and 9999")
}

#[frb(rust2dart(dart_type = "Duration", dart_code = "Duration(microseconds: {})"))]
pub fn encode_duration(raw: StdDuration) -> i64 {
    raw.as_micros().min(i64::MAX as u128) as i64
}

#[frb(dart2rust(dart_type = "Duration", dart_code = "{}.inMicroseconds"))]
pub fn decode_duration(raw: i64) -> StdDuration {
    StdDuration::from_micros(raw.try_into().expect("a Duration that is not negative"))
}

#[frb(rust2dart(dart_type = "Duration", dart_code = "Duration(microseconds: {})"))]
pub fn encode_signed_duration(raw: SignedDuration) -> i64 {
    raw.as_micros() as i64
}

#[frb(dart2rust(dart_type = "Duration", dart_code = "{}.inMicroseconds"))]
pub fn decode_signed_duration(raw: i64) -> SignedDuration {
    SignedDuration::from_micros(raw)
}

#[frb(rust2dart(dart_type = "Duration", dart_code = "Duration(seconds: {})"))]
pub fn encode_offset(raw: Offset) -> i32 {
    raw.seconds()
}

#[frb(dart2rust(dart_type = "Duration", dart_code = "{}.inSeconds"))]
pub fn decode_offset(raw: i32) -> Offset {
    Offset::from_seconds(raw).expect("an offset of less than 26 hours")
}

/// A weekday crosses as a number, 1 for Monday through 7 for Sunday.
#[frb(rust2dart(dart_type = "int", dart_code = "{}"))]
pub fn encode_weekday(raw: Weekday) -> i8 {
    raw.to_monday_one_offset()
}

#[frb(dart2rust(dart_type = "int", dart_code = "{}"))]
pub fn decode_weekday(raw: i8) -> Weekday {
    Weekday::from_monday_one_offset(raw).expect("a weekday from 1 to 7")
}
