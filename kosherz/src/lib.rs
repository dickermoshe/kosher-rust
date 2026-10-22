//! A C ABI for [kosher-rust](https://docs.rs/kosher-rust), and through
//! [WeaveFFI](https://weaveffi.com) bindings for Node, Python, Swift, Kotlin, Dart, .NET, Go, Ruby,
//! C and C++.
//!
//! [`api`] restates kosher-rust's public API in shapes that cross a C ABI: numbers,
//! strings, records, enums and reference-counted objects.
//!
//! - Instants are milliseconds since the Unix epoch; durations are milliseconds.
//! - Gregorian dates are [`GregorianDate`](api::calendar::GregorianDate) records.
//! - Time zones are supplied by the caller as a
//!   [`TimeZoneOffset`](api::times::TimeZoneOffset) callback that gives the UTC offset
//!   at an instant, so no time zone database is bundled.

pub mod api;
#[cfg(test)]
mod codegen;
mod convert;

weaveffi::export_runtime!();
