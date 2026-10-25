//! Builds the WebAssembly of the kr4js npm package.
//!
//! The JavaScript API is kosher-rust's `wasm` feature. This crate turns that feature on and is a
//! `cdylib`, which kosher-rust itself can't be without breaking the crates that depend on it.

pub use kosher_rust;
