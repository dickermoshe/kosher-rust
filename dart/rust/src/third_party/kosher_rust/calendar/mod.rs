use flutter_rust_bridge::frb;
use kosher_rust::calendar::{HebrewDate, HebrewMonth, HebrewYear, Parsha};

// flutter_rust_bridge can't translate a unit struct.
#[frb(opaque)]
pub struct DateOutOfRange;

// The names are `&'static str`, which flutter_rust_bridge can't return. `crate::api::calendar` adds
// `String` versions.

#[frb(external)]
impl HebrewMonth {
    #[frb(ignore)]
    pub fn name() {}
    #[frb(ignore)]
    pub fn hebrew() {}
}

#[frb(external)]
impl Parsha {
    #[frb(ignore)]
    pub fn name() {}
    #[frb(ignore)]
    pub fn hebrew() {}
}

// Some variants hold data, which flutter_rust_bridge makes opaque by default for another crate's enum.
#[frb(non_opaque)]
pub enum SpecialDay {}

#[frb(external)]
impl SpecialDay {
    #[frb(ignore)]
    pub fn name() {}
    #[frb(ignore)]
    pub fn hebrew() {}
}

// `new` returns an `Option`, which a Dart constructor can't. `of` in `crate::api::calendar` replaces
// it.

#[frb(external)]
impl HebrewDate {
    #[frb(ignore)]
    pub fn new() {}
}

#[frb(external)]
impl HebrewYear {
    #[frb(ignore)]
    pub fn new() {}
    // `value` in `crate::api::calendar`, since `get` is reserved in Dart.
    #[frb(ignore)]
    pub fn get() {}
}
