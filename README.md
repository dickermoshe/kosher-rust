# kosher-rust

Jewish calendar, *zmanim* and daily learning schedules for Rust. A port of
[KosherJava](https://github.com/KosherJava/zmanim), checked against it on every commit.

- **Calendar:** Hebrew dates, holidays, fasts, the weekly *parsha*, the molad.
- **Zmanim:** sunrise, sof zman shma, candle lighting and over 160 other halachic times for any
  place on earth.
- **Limudim:** Daf Yomi, Mishna Yomis, Yerushalmi, Tehillim, Pirkei Avos and more.

`no_std` compatible.

```bash
cargo add kosher-rust jiff
```

## The Hebrew calendar

Any `jiff::civil::Date` (or ICU date) converts to a `HebrewDate`, which answers questions about
that day. Pass a `Region` wherever Israel and the diaspora differ.

```rust
use jiff::civil::date;
use kosher_rust::{HebrewDate, Region};

let day = HebrewDate::from(date(2024, 4, 23));
assert_eq!(day.to_string(), "15 Nisan 5784");

for holiday in day.holidays(Region::Diaspora) {
    println!("{holiday} ({})", holiday.hebrew());   // Pesach (פסח)
}
assert!(day.is_assur_bemelacha(Region::Diaspora));
assert!(day.year().is_leap());

let shabbos = HebrewDate::from(date(2024, 5, 4));
println!("{}", shabbos.parsha(Region::Israel).unwrap());   // Achrei Mos
```

## Zmanim

Give a `Location` and a date to `Zmanim`, then ask for any `Zman`. Times come back as
`jiff::Timestamp`, so convert them to the local time zone to display them.

```rust
use jiff::{civil::date, tz::TimeZone};
use kosher_rust::{Location, Zman, Zmanim};

let tz = TimeZone::get("America/New_York")?;
let lakewood = Location::new(40.09, -74.22, Some(tz.clone()))?.with_elevation(30.0)?;
let zmanim = Zmanim::new(lakewood, date(2024, 4, 23));

let shma = zmanim.get(Zman::SofZmanShmaGra)?;
println!("{}", shma.to_zoned(tz.clone()).strftime("%H:%M"));   // 09:31

for zman in Zman::ALL {
    if let Ok(time) = zmanim.get(zman) {
        println!("{:<45} {}", zman.name(), time.to_zoned(tz.clone()).strftime("%H:%M:%S"));
    }
}
```

`get` returns an error when a time doesn't exist that day. For example, in the far north in
summer the sun never sinks 16.1° below the horizon, so there is no *alos* at 16.1°.

To pick the solar algorithm (NOAA, Meeus or SPA), use elevation everywhere, or change the
candle-lighting offset, build the calculator with `Zmanim::with_config` and a `zmanim::Config`.

## Learning schedules

Each schedule answers `on(date)` with that day's portion, or `None` when nothing is scheduled.

```rust
use jiff::civil::date;
use kosher_rust::limudim::{DafYomiBavli, MishnaYomis, Schedule};

let day = date(2024, 4, 23);
println!("{}", DafYomiBavli.on(day).unwrap());   // Bava Metzia 55
println!("{}", MishnaYomis.on(day).unwrap());    // Nazir 5:5-6
```

| Schedule | Gives | |
|---|---|---|
| `DafYomiBavli` | `Daf` | one daf a day, since 1923 |
| `DafHashavuaBavli` | `Daf` | one daf a week, since 2005 |
| `AmudYomiDirshu` | `Amud` | one amud a day, since 2023 |
| `DafYomiYerushalmi` | `Daf` | Vilna edition, since 1980 |
| `MishnaYomis` | `MishnaPair` | two mishnayos a day, since 1947 |
| `TehillimMonthly` | `TehillimUnit` | the book of Tehillim each Hebrew month |
| `PirkeiAvos(Region)` | `PirkeiAvosUnit` | summer Shabbos afternoons |

## MyZmanim

`kosher_rust::myzmanim` reproduces the times on [MyZmanim](https://www.myzmanim.com/) to the
second. Use it when your users compare against that site.

```rust
use jiff::{civil::date, tz::TimeZone};
use kosher_rust::Location;
use kosher_rust::myzmanim::{MyZman, MyZmanim};

let jerusalem = Location::new(31.778, 35.235, Some(TimeZone::get("Asia/Jerusalem")?))?
    .with_elevation(754.0)?
    .with_horizons(754.0, 0.0)?;
let times = MyZmanim::new(jerusalem, date(2026, 11, 6)).candle_lighting(40.0);
let candles = times.get(MyZman::CandleLighting)?;
```

## Coming from KosherJava

| KosherJava | kosher-rust |
|---|---|
| `JewishCalendar` | `HebrewDate` |
| `GeoLocation` | `Location` |
| `ComprehensiveZmanimCalendar.getSofZmanShmaGRA()` | `zmanim.get(Zman::SofZmanShmaGra)` |
| `NOAACalculator`, `MeeusCalculator`, `SPACalculator` | `zmanim::Config::astronomical_calculator` |
| `YomiCalculator.getDafYomiBavli(cal)` | `DafYomiBavli.on(date)` |
| `YerushalmiYomiCalculator.getDafYomiYerushalmi(cal)` | `DafYomiYerushalmi.on(date)` |

Every zman getter is a `Zman` variant with the same name minus `get`. Look one up by its
KosherJava name with `Zman::from_java_name("getSofZmanShmaGRA")`.

Not ported yet: `HebrewDateFormatter`, `ZmanimFormatter`, `TefilaRules` and the *tekufos*.
Open an issue if you need one.

## Features

| Feature | Default | |
|---|---|---|
| `alloc` | yes | `Zman::description()`, a paragraph explaining each zman to your users |
| `defmt` | no | `defmt::Format` on the public types, for embedded logging |

## License

LGPL-2.1-or-later. Based on [KosherJava](https://github.com/KosherJava/zmanim) by Eliyahu
Hershfeld.
