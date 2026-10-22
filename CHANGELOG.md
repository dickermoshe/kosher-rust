# Changelog

## 0.3.0 (2026-10-22)

0.3 rebuilds the public API around what users do. The calculations are unchanged and still
checked against KosherJava and hebcal.

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
