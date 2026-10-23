# kosher-rust

Jewish calendar, *zmanim* and daily learning schedules for Rust. A port of
[KosherJava](https://github.com/KosherJava/zmanim), checked against it on every commit.

- **Calendar:** Hebrew dates, holidays, fasts, the weekly *parsha*, the molad.
- **Zmanim:** sunrise, sof zman shma, candle lighting and over 160 other halachic times for any
  place on earth.
- **Limudim:** Daf Yomi, Mishna Yomis, Yerushalmi, Tehillim, Pirkei Avos and more.

`no_std` compatible: it builds for microcontrollers, with or without an allocator.

```bash
cargo add kosher-rust jiff
```

## Other languages

If you're looking for this library in another language, see
[kosherz](https://github.com/dickermoshe/kosher-rust/tree/main/kosherz), its bindings for Node,
WebAssembly, Python, Swift, Kotlin, Dart, .NET, Go, Ruby, C and C++.

## The Hebrew calendar

A `jiff::civil::Date` (or ICU date) converts to a `HebrewDate`, which answers questions about
that day. Pass a `Region` wherever Israel and the diaspora differ. A `HebrewDate` covers 1 January
of year 1 through the end of Hebrew year 9999, and the conversion fails outside that range.

```rust
use jiff::civil::date;
use kosher_rust::{HebrewDate, Region};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = HebrewDate::try_from(date(2024, 4, 23))?;
    assert_eq!(day.to_string(), "15 Nisan 5784");

    for special_day in day.special_days(Region::Diaspora) {
        println!("{special_day} ({})", special_day.hebrew());   // Pesach (פסח)
    }
    assert!(day.is_assur_bemelacha(Region::Diaspora));
    assert!(day.year().is_leap());

    let shabbos = HebrewDate::try_from(date(2024, 5, 4))?;
    println!("{}", shabbos.parsha(Region::Israel).unwrap());   // Achrei Mos
    Ok(())
}
```

## Zmanim

Give a `Location` and a date to `Zmanim`, then ask for any `Zman`. Times come back as
`jiff::Timestamp`, so convert them to the local time zone to display them.

```rust
use jiff::{civil::date, tz::TimeZone};
use kosher_rust::{Location, Zman, Zmanim};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tz = TimeZone::get("America/New_York")?;
    let lakewood = Location::new(40.09, -74.22)?.with_elevation(30.0)?;
    let zmanim = Zmanim::new(lakewood, date(2024, 4, 23), Some(tz));

    let shma = zmanim.get_zoned(Zman::SofZmanShmaGra)?;
    println!("{}", shma.strftime("%H:%M")); // 09:31

    for zman in Zman::ALL {
        if let Ok(time) = zmanim.get_zoned(zman) {
            println!("{:<45} {}", zman.name(), time.strftime("%H:%M:%S"));
        }
    }
    Ok(())
}
```

`get` returns an error when a time doesn't exist that day. For example, in the far north in
summer the sun never sinks 16.1° below the horizon, so there is no *alos* at 16.1°.

To pick the solar algorithm (NOAA, Meeus or SPA), use elevation everywhere, or change the
candle-lighting offset, build the calculator with `Zmanim::with_config` and a `zmanim::Config`.

### Time zones

You can give `Zmanim` and `MyZmanim` a time zone when you create them. With a time zone every
time works. You can also read every time in that time zone with `get_zoned`.

You can also leave the time zone out by passing `None`. Most times still work without one. These
do not work without a time zone:

- The four Kiddush Levana times return `ZmanimError::TimeZoneRequired`. These are the four times:
  - `TchilasZmanKidushLevana3Days`
  - `TchilasZmanKidushLevana7Days`
  - `SofZmanKidushLevanaBetweenMoldos`
  - `SofZmanKidushLevana15Days`
- `get_zoned` returns `ZmanimError::TimeZoneRequired`. You can still read the time as a
  `Timestamp` with `get`.
- A location more than 150° from Greenwich needs a time zone for every time. Without one every
  time returns `ZmanimError::TimeZoneRequired`.

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

`kosher_rust::myzmanim` calculates zmanim the way [MyZmanim](https://www.myzmanim.com/) does.
Use it when your users compare your times with that site.

To match the site, you need the same inputs MyZmanim uses for the location. These are its
latitude and longitude, the observer's elevation, and the elevations of the eastern and western
horizons. MyZmanim doesn't publish these values, but you can usually get them from its API, app or
website. With them, 92% of times match the site to within one second and 99.8% to within two.

Keep in mind when doing a comparison, MyZmanim.com does not calculate times for the exact point you search for.
It groups places into areas, finds the area your point is in, and shows the times for that area. 
For example, two points on opposite ends of Lakewood have slightly different sunrises, but the website
shows the same sunrise for both, to the second. If you use the coordinates you searched with, your times will
differ slightly from the site. Use MyZmanim's coordinates for the area instead.

```rust
use std::time::Duration;

use jiff::{civil::date, tz::TimeZone};
use kosher_rust::Location;
use kosher_rust::myzmanim::{MyZman, MyZmanim};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jerusalem = Location::new(31.778, 35.235)?
        .with_elevation(754.0)?
        .with_horizons(754.0, 0.0)?;
    let times = MyZmanim::new(jerusalem, date(2026, 11, 6), Some(TimeZone::get("Asia/Jerusalem")?))
        .candle_lighting(Duration::from_mins(40));
    println!("Candle lighting: {}", times.get_zoned(MyZman::CandleLighting)?.strftime("%H:%M"));
    Ok(())
}
```

## Port status

This port is tracked against KosherJava commit
[ffb0fef](https://github.com/KosherJava/zmanim/commit/ffb0fefdbc6c70fd1b4e708444da7967f5f95bef),
which is master shortly after the 3.0.0 release. CI compares it with that version on every
commit, using random dates and locations for every zman, the Hebrew calendar and Daf Yomi.

It also includes a fix that KosherJava hasn't merged yet,
[KosherJava/zmanim#296](https://github.com/KosherJava/zmanim/pull/296), which keeps a sunrise or
sunset near the date line on its own date.

The zmanim, the three astronomical calculators, the Hebrew calendar and both Daf Yomi
calculators are ported. Not ported yet:

- `HebrewDateFormatter`
- `ZmanimFormatter`
- `TefilaRules`
- The *tekufos*
- `SunTimesCalculator`
- `getZmanMolad`, `getPolarPlagHaminchaBenIshChai`, `getPolarStartOfDayTeshuvosVehanhagos` and
  `getPolarPlagHaminchaTeshuvosVehanhagos`
- `getLocalMeanTime`, `getTimeAtAzimuth90Or270`, `getSunriseSolarDipFromOffset`,
  `getSunsetSolarDipFromOffset` and `getPercentOfShaahZmanisFromDegrees`
- The distance and bearing methods of `GeoLocation`
- `isErevRoshChodesh`, `isErevYomTovSheni`, `isTomorrowShabbosOrYomTov` and
  `isTonightMutarBemelacha`

Open an issue if you need one.

## Features

| Feature | Default | |
|---|---|---|
| `alloc` | yes | `Zman::description()`, a paragraph explaining each zman to your users |
| `defmt` | no | `defmt::Format` on the public types, for embedded logging |

## License

LGPL-2.1-or-later. Based on [KosherJava](https://github.com/KosherJava/zmanim) by Eliyahu
Hershfeld.
