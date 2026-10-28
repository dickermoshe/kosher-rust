use flutter_rust_bridge::frb;
use kosher_rust::myzmanim::MyZman;

// Generic over its time zone. `crate::api::zmanim` has the one Dart uses.
#[frb(ignore)]
pub struct MyZmanim;

#[frb(external)]
impl MyZman {
    #[frb(ignore)]
    pub fn name() {}
}
