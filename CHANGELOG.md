# Changelog

## Unreleased

- **The time zone moves from `Location` to `Zmanim` and `MyZmanim`.** `Location::new(lat, lon)`
  no longer takes a time zone. You pass the time zone to `Zmanim::new`, `Zmanim::with_config`
  or `MyZmanim::new` instead. A location more than 150° from Greenwich still needs a time zone.
  Without one each time now returns `ZmanimError::TimeZoneRequired`. Before this change
  `Location::new` returned `LocationError::TimeZoneRequired`. That error no longer exists.
- **`get_zoned`.** `Zmanim` and `MyZmanim` have a new `get_zoned` method. It gives a time in the
  time zone of the *zmanim*. Use it to show the time to someone.
- **Custom time zones.** `Zmanim::with_custom_timezone` and `MyZmanim::with_custom_timezone` take
  any `TimeZoneLike`. A `TimeZoneLike` is any type that gives the UTC offset at an instant.
- **No panics at the ends of the date range.** Near the first and last days a `Timestamp` holds,
  times built by adding to or subtracting from another time panicked when the result fell
  outside the range, even with the default `Config`; they now return `ZmanimError::Internal`,
  as do candle-lighting and Ateret Torah offsets too large to add.
- **Days a time zone skipped match KosherJava.** On a date whose midnight a zone skipped
  entirely (Samoa and Tokelau on 30 December 2011, Kiritimati on 31 December 1994, Kwajalein
  on 21 August 1993), the date-line adjustment used the offset from before the jump; it now
  uses the one after, as KosherJava does.
- **`PirkeiAvos` past Hebrew year 9999.** It panicked for every date from the summer of 9999
  (Gregorian 6239) on; that summer now has its portions, and from 10000 on there are none.

## 0.3.0 (2026-10-22)

0.3 rebuilds the public API around what users do. Apart from the fixes below, the calculations
are unchanged, and they are still checked against KosherJava and hebcal.

### Fixes

- **Meeus and NOAA no longer miss events near the threshold.** On days when the sun only just
  reaches an angle (sunrise and sunset in the far north and south, twilight angles from about
  50° latitude), they reported no time because the first estimate used the sun's position at
  noon. They now retry at the solar midnight next to the event, as KosherJava does since
  [0bd7893](https://github.com/KosherJava/zmanim/commit/0bd78932fe9b42e38ecd5ceab036c90d96fc3d99).
- **SPA is a port of Klaus Brunner's solarpositioning 3.0.3.** Sunrise and sunset are searched
  in the window belonging to the date, so polar and grazing days no longer return a time from
  the wrong day or a time when the sun never reaches the angle. Results agree with KosherJava's
  `SPACalculator` to within a few milliseconds.
- **MyZmanim** uses the Meeus fix too: 14 more times in the site data now have a value. The
  worst difference from the site is 13.6 seconds, a *misheyakir* on a day the sun only just
  reaches 11.5°.

### Migrating from 0.2

#### Calendar

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::calendar::prelude::*` | `use kosher_rust::{HebrewDate, Region}` |
| `date.holidays(in_israel, use_modern)` on any date | `HebrewDate::from(date).holidays(Region::Israel)`; modern holidays are always included, filter with `Holiday::is_modern()` |
| `date.is_assur_bemelacha(in_israel)`, `has_candle_lighting(in_israel)` | same names on `HebrewDate`, taking a `Region` |
| `date.todays_parsha(..)`, `upcoming_parsha(..)` | `HebrewDate::parsha(..)`, `next_parsha(..)` |
| `date.hebrew_date()` (`HebrewCalendarDate`) | `HebrewDate::from(date)` |
| `Hebrew::is_hebrew_leap_year(y)`, `days_in_hebrew_year(y)`, `days_in_hebrew_month(y, m)` | `HebrewYear::new(y).is_leap()`, `.days()`, `.days_in(month)` |
| `Hebrew::cheshvan_kislev_kviah(y)` → `YearLengthType` | `HebrewYear::new(y).length()` → `YearLength` |
| `calendar::month::NISAN` and the other ICU `Month` constants | `HebrewMonth::Nisan` |
| `month.he(year)` (`HebrewMonthExt`) | `HebrewMonth::hebrew()` |
| `Holiday::he()`, `Parsha::he()` | `hebrew()` |
| `Holiday::is_modern_holiday()` | `Holiday::is_modern()` |
| `zmanim::molad::MoladCalendar::molad(tz)` | `HebrewDate::molad()` → `Molad`, the molad of the date's month |

`Display` on `Holiday` and `Parsha` now prints the English name ("Pesach", "Bereshis"); use
`hebrew()` for the Hebrew one.

`HebrewYear::days()`, `is_cheshvan_long()` and `is_kislev_short()` no longer return `Option`.

#### Location and errors

| 0.2 | 0.3 |
|---|---|
| `zmanim::types::location::Location` | `kosher_rust::Location` |
| `Location::new(lat, lon, elevation, tz)` | `Location::new(lat, lon, tz)?.with_elevation(elevation)?` |
| `Location::with_horizon_elevations(lat, lon, elevation, east, west, tz)` | `Location::new(lat, lon, tz)?.with_elevation(elevation)?.with_horizons(east, west)?` |
| `location.latitude` and the other public fields | `location.latitude()` and the other getters |
| `location.sunrise_elevation()`, `sunset_elevation()` | removed |
| `zmanim::types::config::{CalculatorConfig, AstronomicalCalculator}` | `zmanim::{CalculatorConfig, AstronomicalCalculator}` |
| `zmanim::types::error::ZmanimError` | `zmanim::ZmanimError` |
| `ZmanimError::InvalidLatitude`, `InvalidLongitude`, `InvalidElevation` | `LocationError::*`, returned when the `Location` is built |
| `ZmanimError::TimeZoneRequired` from `Location::new` | `LocationError::TimeZoneRequired`; `ZmanimError::TimeZoneRequired` is now only for the Kiddush Levana times |
| `ZmanimError::InvalidForDate` | `ZmanimError::NotApplicable`, which the polar Ben Ish Chai times now also return on ordinary days |
| `ZmanimError::CalculationError`, `TimeConversionError`, `InvalidHours` | `ZmanimError::Internal` |

Elevations must now be finite; an infinite observer elevation used to be accepted.

#### Zmanim

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::zmanim::prelude::*` | `use kosher_rust::{Location, Zman, Zmanim}` |
| `ZmanimCalculator::new(location, date, config)` | `Zmanim::with_config(location, date, config)`, or `Zmanim::new(location, date)` for the defaults |
| `calc.calculate(&presets::SOF_ZMAN_SHMA_GRA)` | `zmanim.get(Zman::SofZmanShmaGra)` |
| `presets::ELEVATION_ADJUSTED_SUNRISE` and the other 166 statics | `Zman::Sunrise` and the other variants, named after the KosherJava getter without `get` |
| `presets::ALL_ZMANIM` | `Zman::ALL` |
| `preset.method_name`, `.name`, `.zman_type`, `.deprecated` | `zman.java_name()`, `.name()`, `.category()`, `.is_deprecated()`; `Zman::from_java_name(..)` goes the other way |
| `preset.description(&calc)` | `zman.description(&config)` |
| `ZmanType` | `Category` |
| `CalculatorConfig` | `Config` |
| `ZmanPrimitive`, `ZmanLike`, `ZmanPreset` | private. For a custom depression angle use `Zmanim::sunrise_offset_by_degrees` / `sunset_offset_by_degrees`; for proportional hours, `zmanim::shaah_zmanis` |
| `kosher_rust::for_all_zman_presets!` | removed; iterate `Zman::ALL` |

The 24 zmanim KosherJava deprecates are `#[deprecated]` variants: they still calculate, with a
warning where you name them.

#### Limudim

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::limudim::prelude::*`, `kosher_rust::prelude::*` | `use kosher_rust::limudim::{DafYomiBavli, Schedule, ..}`; the preludes are gone |
| `date.limud(DafYomiBavli::default())` (`LimudCalendar`) | `DafYomiBavli.on(date)` (`Schedule`) |
| `Limud<T>` | `Schedule`, with the unit as `Schedule::Unit` |
| `AmudYomiBavliDirshu` | `AmudYomiDirshu` |
| `DafYomiYerushalmiVilna` | `DafYomiYerushalmi` |
| `PirkeiAvos { in_israel }`, `PirkeiAvos::new(in_israel)` | `PirkeiAvos(Region::Israel)` / `PirkeiAvos(Region::Diaspora)` |
| `Mishnas(first, second)` | `MishnaPair { first, second }` |
| `Daf::page`, `Amud::page` | `Daf::daf`, `Amud::daf` |
| `Amud::amudim_to`, `Amud::at_offset` | removed |

`Daf`, `Amud`, `Mishna`, `MishnaPair`, `TehillimUnit`, `PirkeiAvosUnit` and `Tractate` implement
`Display` ("Bava Metzia 55", "Nazir 5:5-6", "Tehillim 119:1-96"); `Tractate::hebrew()` gives the
Hebrew name.

#### MyZmanim

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::myzmanim::prelude::*` | `use kosher_rust::myzmanim::{MyZman, MyZmanim}` |
| `MyZmanimCalendar::new(location, date)` | `MyZmanim::new(location, date)` |
| `.with_candle_lighting_minutes(m)` | `.candle_lighting(m)` |
| `calendar.calculate(MyZman::X)`, `MyZman::X.calculate(&calendar)` | `times.get(MyZman::X)` |
| `set_date`, `set_candle_lighting_minutes` | build a new `MyZmanim` |

`MyZman` implements `Display` with the site's label.

### Also new in 0.3

- The README is the crate's front page and its examples are tested; `examples/today.rs` prints
  today's date, holidays, learning and zmanim.
- `no_std` builds are checked in CI for `thumbv7em-none-eabihf`, with and without `alloc`.
- `cargo bench` (divan) replaces the 10 ms timing assertions that ran as tests.
- Every public item is documented, including every `Holiday`, `Parsha`, `Tractate` and `Zman`.
