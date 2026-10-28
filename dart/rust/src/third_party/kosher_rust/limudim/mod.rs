use flutter_rust_bridge::frb;
use kosher_rust::limudim::{BavliTractate, MishnaTractate, YerushalmiTractate};

// The schedules are reached through the functions in `crate::api::limudim`. The unit structs are
// marked opaque because flutter_rust_bridge fails on a unit struct a trait is implemented for. Their
// `Schedule` trait has no method Dart can call, so it is declared in `lib/src/convert.dart`.
#[frb(opaque)]
pub struct AmudYomiDirshu;
#[frb(opaque)]
pub struct DafHashavuaBavli;
#[frb(opaque)]
pub struct DafYomiBavli;
#[frb(opaque)]
pub struct DafYomiYerushalmi;
#[frb(opaque)]
pub struct MishnaYomis;
#[frb(opaque)]
pub struct TehillimMonthly;
#[frb(ignore)]
pub struct PirkeiAvos;
#[frb(ignore)]
pub struct Daf;

#[frb(non_opaque)]
pub enum TehillimUnit {}
#[frb(non_opaque)]
pub enum PirkeiAvosUnit {}

#[frb(external)]
impl MishnaTractate {
    #[frb(ignore)]
    pub fn name() {}
    #[frb(ignore)]
    pub fn hebrew() {}
}

#[frb(external)]
impl BavliTractate {
    #[frb(ignore)]
    pub fn name() {}
    #[frb(ignore)]
    pub fn hebrew() {}
}

#[frb(external)]
impl YerushalmiTractate {
    #[frb(ignore)]
    pub fn name() {}
    #[frb(ignore)]
    pub fn hebrew() {}
}
