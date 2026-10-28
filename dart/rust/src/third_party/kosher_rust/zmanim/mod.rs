use flutter_rust_bridge::frb;
use kosher_rust::zmanim::Zman;

// Generic over its time zone. `crate::api::zmanim` has the one Dart uses.
#[frb(ignore)]
pub struct Zmanim;

// Its offsets are `core::time::Duration`, which flutter_rust_bridge takes for chrono's, and one of
// its variants holds a unit struct. `crate::api::zmanim` has the versions Dart uses.
#[frb(ignore)]
pub struct Config;
#[frb(ignore)]
pub enum ZmanimError {}

// Some variants hold data, which flutter_rust_bridge makes opaque by default for another crate's enum.
#[frb(non_opaque)]
pub enum AstronomicalCalculator {}

// Marked `#[non_exhaustive]`, so it can't be built from Dart. Its fields are readable.
#[frb(opaque)]
pub struct SolarPosition;

#[frb(external)]
impl AstronomicalCalculator {
    #[frb(ignore)]
    pub fn name() {}
}

#[frb(external)]
impl Zman {
    #[frb(ignore)]
    pub fn name() {}
    // Takes kosher-rust's `Config`. `describe` in `crate::api::zmanim` takes Dart's.
    #[frb(ignore)]
    pub fn description() {}
}
