mod calendar;
mod limudim;
mod myzmanim;
mod zmanim;

use flutter_rust_bridge::frb;
use kosher_rust::Location;

// Dart supplies the time zone as a `TimeZone`. See `crate::api::zmanim`.
#[frb(ignore)]
pub trait Zone {}

#[frb(external)]
impl Location {
    #[frb(getter)]
    pub fn latitude() {}
    #[frb(getter)]
    pub fn longitude() {}
    #[frb(getter)]
    pub fn elevation() {}
    #[frb(getter)]
    pub fn eastern_horizon_elevation() {}
    #[frb(getter)]
    pub fn western_horizon_elevation() {}
}
