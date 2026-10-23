//! A C ABI for [kosher-rust](https://docs.rs/kosher-rust), and through
//! [WeaveFFI](https://weaveffi.com) bindings for Node, WebAssembly, Python, Swift, Kotlin, Dart,
//! .NET, Go, Ruby, C and C++.
//!
//! [`api`] restates kosher-rust's public API in shapes that cross a C ABI: numbers,
//! strings, records, enums and reference-counted objects.
//!
//! - Instants are milliseconds since the Unix epoch; durations are milliseconds.
//! - Gregorian dates are [`GregorianDate`](api::calendar::GregorianDate) records.
//! - The caller describes the date in its time zone with a [`LocalDay`](api::times::LocalDay):
//!   its two local midnights and their offsets. No time zone database is bundled.

pub mod api;
#[cfg(test)]
mod codegen;
mod convert;

weaveffi::export_runtime!();
