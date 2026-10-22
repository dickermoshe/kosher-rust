//! Today on the Hebrew calendar and today's zmanim for a place.
//!
//! ```bash
//! cargo run --example today                      # Jerusalem
//! cargo run --example today -- 40.09 -74.22 America/New_York
//! ```

use std::{env, error::Error};

use jiff::{Zoned, tz::TimeZone};
use kosher_rust::limudim::{DafYomiBavli, MishnaYomis, PirkeiAvos, Schedule, TehillimMonthly};
use kosher_rust::zmanim::Category;
use kosher_rust::{HebrewDate, Location, Region, Zman, Zmanim};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let (latitude, longitude, tz_name) = match args.as_slice() {
        [lat, lon, tz] => (lat.parse()?, lon.parse()?, tz.as_str()),
        _ => (31.778, 35.235, "Asia/Jerusalem"),
    };
    let tz = TimeZone::get(tz_name)?;
    let region = if tz_name == "Asia/Jerusalem" {
        Region::Israel
    } else {
        Region::Diaspora
    };

    let today = Zoned::now().with_time_zone(tz.clone()).date();
    let hebrew = HebrewDate::from(today);
    println!("{today}  {hebrew}");

    for holiday in hebrew.holidays(region) {
        println!("  {holiday} ({})", holiday.hebrew());
    }
    if let Some(parsha) = hebrew.next_parsha(region) {
        println!("  This week's parsha: {parsha}");
    }

    println!();
    println!("Learning");
    if let Some(daf) = DafYomiBavli.on(today) {
        println!("  Daf Yomi:     {daf}");
    }
    if let Some(mishnayos) = MishnaYomis.on(today) {
        println!("  Mishna Yomis: {mishnayos}");
    }
    if let Some(tehillim) = TehillimMonthly.on(today) {
        println!("  Tehillim:     {tehillim}");
    }
    if let Some(avos) = PirkeiAvos(region).on(today) {
        println!("  Pirkei Avos:  {avos}");
    }

    println!();
    println!("Zmanim at {latitude}, {longitude}");
    let zmanim = Zmanim::new(Location::new(latitude, longitude)?, today, Some(tz));
    let headline = [
        Zman::Alos72Minutes,
        Zman::Sunrise,
        Zman::SofZmanShmaGra,
        Zman::SofZmanTfilaGra,
        Zman::ChatzosHayom,
        Zman::MinchaGedolaGra,
        Zman::PlagHaminchaGra,
        Zman::Sunset,
        Zman::TzaisGeonim8Point5Degrees,
        Zman::Tzais72Minutes,
    ];
    for zman in headline {
        match zmanim.get_zoned(zman) {
            Ok(time) => println!("  {:<32} {}", zman.name(), time.strftime("%H:%M:%S")),
            Err(error) => println!("  {:<32} ({error})", zman.name()),
        }
    }
    if hebrew.has_candle_lighting(region) {
        let candles = zmanim.get_zoned(Zman::CandleLighting)?;
        println!("  {:<32} {}", Zman::CandleLighting.name(), candles.strftime("%H:%M"));
    }

    let alos_opinions = Zman::ALL
        .into_iter()
        .filter(|zman| zman.category() == Category::Alos)
        .count();
    println!();
    println!(
        "({} zmanim in all, {alos_opinions} of them for alos; see `Zman::ALL`.)",
        Zman::ALL.len()
    );
    Ok(())
}
