//! What Dart calls that kosher-rust doesn't provide as it is. Everything else is scanned straight
//! from kosher-rust.

pub mod calendar;
pub mod convert;
pub mod limudim;
pub mod zmanim;

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
