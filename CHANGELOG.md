# Changelog

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
- **Kiddush Levana checks the day the way KosherJava does.** A Kiddush Levana time counts for a
  date when it falls between the local midnight that starts the date and the local midnight that
  ends it. A time exactly at the midnight that ends the date counts for that date, as it does in
  KosherJava.
- **Days a time zone skipped match KosherJava.** On a date whose midnight a zone skipped
  entirely (Samoa and Tokelau on 30 December 2011, Kiritimati on 31 December 1994, Kwajalein
  on 21 August 1993), the date-line adjustment used the offset from before the jump. It now
  uses the one after, as KosherJava does.
- **No panics at the ends of the date range.** Near the first and last days a `Timestamp` holds,
  times built by adding to or subtracting from another time panicked when the result fell
  outside the range, even with the default `Config`. They now return `ZmanimError::Internal`,
  as do candle-lighting and Ateret Torah offsets too large to add.
- **`HebrewDate` has a range.** It covers 1 January of year 1, which is 18 Teves 3761, through
  29 Elul 9999. Converting a date outside that range fails with `DateOutOfRange`, where before it
  gave wrong leap years, parshiyos and moladot, or a panic in `molad`. The times that need a
  Hebrew date (Kiddush Levana and the chametz times) return `ZmanimError::Internal` outside it,
  and the schedules return `None`.
- **Pirkei Avos in the week after Pesach.** On the weekdays of Pesach whose week ends in a
  Shabbos with a reading, `PirkeiAvos` returned nothing. It now gives that Shabbos's chapter, as
  it does for every other week, and agrees with hebcal.
- **`PirkeiAvos` past Hebrew year 9999.** It panicked for every date from the summer of 9999
  (Gregorian 6239) on. That summer now has its portions, and from 10000 on there are none.
- **`PirkeiAvos` on the weekdays of Pesach.** A weekday of Pesach week returned `None`, even when
  that week's Shabbos is after Pesach and has a reading. It now gives that Shabbos's chapter, as
  every other weekday does.
- **Sunrise and sunset near the date line.** Within a few degrees of 180°, in polar winter or
  summer, a sunrise or sunset near local noon could be reported a day early or late, and the times
  built on it with it. They now fall on their own date.
- **Custom angles are checked.** `Zmanim::sunrise_offset_by_degrees` and
  `sunset_offset_by_degrees` return the new `ZmanimError::InvalidDegrees` for an angle that is not
  a number from -90 to 90. Before, NaN gave `ZmanimError::Internal` and a huge angle gave a
  meaningless time.
- **MyZmanim above 44 km.** A horizon higher than about 44 km gave `ZmanimError::Internal` for
  sunrise and the times built on it. Refraction there is now zero.
- **Zman descriptions.** Offsets are written with their seconds ("18 minutes 45 seconds"). Morning
  times such as Shema say they divide the morning when astronomical chatzos is used, and Mincha
  Gedola (Ahavat Shalom) and Mincha Gedola (GR'A, greater than 30) mention the chatzos setting.

### Migrating from 0.2

#### Calendar

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::calendar::prelude::*` | `use kosher_rust::{HebrewDate, Region}` |
| `Holiday` | `SpecialDay`, since it also covers fasts, Rosh Chodesh, the Omer and other marked days |
| `date.holidays(in_israel, use_modern)` on any date | `HebrewDate::try_from(date)?.special_days(Region::Israel)`; modern holidays are always included, filter with `SpecialDay::is_modern()` |
| `date.is_assur_bemelacha(in_israel)`, `has_candle_lighting(in_israel)` | same names on `HebrewDate`, taking a `Region` |
| `date.todays_parsha(..)`, `upcoming_parsha(..)` | `HebrewDate::parsha(..)`, `next_parsha(..)` |
| `date.hebrew_date()` (`HebrewCalendarDate`) | `HebrewDate::try_from(date)`, which fails outside 1 January of year 1 to 29 Elul 9999 |
| `Hebrew::is_hebrew_leap_year(y)`, `days_in_hebrew_year(y)`, `days_in_hebrew_month(y, m)` | `HebrewYear::new(y)?.is_leap()`, `.days()`, `.days_in(month)`. `new` returns `None` outside the years 3761 to 9999 |
| `Hebrew::cheshvan_kislev_kviah(y)` → `YearLengthType` | `HebrewYear::new(y)?.length()` → `YearLength` |
| `calendar::month::NISAN` and the other ICU `Month` constants | `HebrewMonth::Nisan` |
| `month.he(year)` (`HebrewMonthExt`) | `HebrewMonth::hebrew()` |
| `Holiday::he()`, `Parsha::he()` | `hebrew()` |
| `Holiday::is_modern_holiday()` | `SpecialDay::is_modern()` |
| `Holiday::YomHazikaron`, `YomHaatzmaut`, `MacharHachodesh` | `SpecialDay::YomHaZikaron`, `YomHaAtzmaut`, `MacharChodesh` |
| `zmanim::molad::MoladCalendar::molad(tz)` | `HebrewDate::molad()` → `Molad`, the molad of the date's month |

`Display` on `SpecialDay` and `Parsha` now prints the English name ("Pesach", "Bereshis"); use
`hebrew()` for the Hebrew one.

`HebrewYear::days()`, `is_cheshvan_long()` and `is_kislev_short()` no longer return `Option`.

#### Location and errors

| 0.2 | 0.3 |
|---|---|
| `zmanim::types::location::Location` | `kosher_rust::Location` |
| `Location::new(lat, lon, elevation, tz)` | `Location::new(lat, lon)?.with_elevation(elevation)?`. The time zone goes to `Zmanim::new` or `MyZmanim::new` |
| `Location::with_horizon_elevations(lat, lon, elevation, east, west, tz)` | `Location::new(lat, lon)?.with_elevation(elevation)?.with_horizons(east, west)?` |
| `location.latitude` and the other public fields | `location.latitude()` and the other getters |
| `location.sunrise_elevation()`, `sunset_elevation()` | removed |
| `zmanim::types::config::{CalculatorConfig, AstronomicalCalculator}` | `zmanim::{Config, AstronomicalCalculator}` |
| `zmanim::types::error::ZmanimError` | `zmanim::ZmanimError` |
| `ZmanimError::InvalidLatitude`, `InvalidLongitude`, `InvalidElevation` | `LocationError::*`, returned when the `Location` is built |
| `ZmanimError::TimeZoneRequired` from `Location::new` | `ZmanimError::TimeZoneRequired` from `get`. Without a time zone it is returned by the four Kiddush Levana times, by `get_zoned`, and by every time more than 150° from Greenwich |
| `ZmanimError::InvalidForDate` | `ZmanimError::NotApplicable`, which the polar Ben Ish Chai times now also return on ordinary days |
| `ZmanimError::CalculationError`, `TimeConversionError`, `InvalidHours` | `ZmanimError::Internal` |

Elevations must now be finite; an infinite observer elevation used to be accepted.

#### Zmanim

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::zmanim::prelude::*` | `use kosher_rust::{Location, Zman, Zmanim}` |
| `ZmanimCalculator::new(location, date, config)` | `Zmanim::with_config(location, date, Some(tz), config)`, or `Zmanim::new(location, date, Some(tz))` for the defaults |
| `calc.calculate(&presets::SOF_ZMAN_SHMA_GRA)` | `zmanim.get(Zman::SofZmanShmaGra)` |
| `presets::ELEVATION_ADJUSTED_SUNRISE` and the other 166 statics | `Zman::Sunrise` and the other variants, named after the KosherJava getter without `get` |
| `presets::ALL_ZMANIM` | `Zman::ALL` |
| `preset.method_name`, `.name`, `.zman_type`, `.deprecated` | `zman.java_name()`, `.name()`, `.category()`, `.is_deprecated()` |
| `preset.description(&calc)` | `zman.description(&config)` |
| `ZmanType` | `Category` |
| `CalculatorConfig` | `Config`. Its `candle_lighting_offset` is a `core::time::Duration` before sunset |
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
| `Mishna::chapter` as a `usize` | a `u16`, like `Mishna::mishna` |

`Daf`, `Amud`, `Mishna`, `MishnaPair`, `TehillimUnit`, `PirkeiAvosUnit` and `Tractate` implement
`Display` ("Bava Metzia 55", "Nazir 5:5-6", "Tehillim 119:1-96"); `Tractate::hebrew()` gives the
Hebrew name.

#### MyZmanim

| 0.2 | 0.3 |
|---|---|
| `use kosher_rust::myzmanim::prelude::*` | `use kosher_rust::myzmanim::{MyZman, MyZmanim}` |
| `MyZmanimCalendar::new(location, date)` | `MyZmanim::new(location, date, Some(tz))` |
| `.with_candle_lighting_minutes(m)` | `.candle_lighting(Duration::from_mins(m))`, a `core::time::Duration` before sunset |
| `calendar.calculate(MyZman::X)`, `MyZman::X.calculate(&calendar)` | `times.get(MyZman::X)` |
| `set_date`, `set_candle_lighting_minutes` | build a new `MyZmanim` |

`MyZman` implements `Display` with the site's label.

### Also new in 0.3

- **Time zones belong to the *zmanim*.** `Location` no longer holds a time zone. Pass it to
  `Zmanim::new`, `Zmanim::with_config` or `MyZmanim::new`, or pass `None`. The README lists the
  times that need one.
- **`get_zoned`.** `Zmanim` and `MyZmanim` give a time as a `Zoned` in their time zone, with
  any time zone.
- **Lists and names.** `SpecialDay`, `Parsha`, `Tractate` and `HebrewMonth` have `ALL` and
  `name()`, as `Zman` and `MyZman` do. `Zman`, `MyZman`, `Parsha`, `Tractate` and `HebrewMonth`
  also have `from_name`, which returns the variant with that name or `None`.
- **The molad as it is announced.** `Molad` has `weekday()`, `hours()`, `minutes()` and
  `chalakim()`, in Jerusalem mean time.
- **Sorting.** `Mishna` and `MishnaPair` sort in the order of the Mishnah, as `Daf` and `Amud`
  do. `Zmanim` implements `PartialEq`.
- **Room to grow.** `Zman`, `MyZman`, `Category`, `SpecialDay` and `AstronomicalCalculator` are
  `#[non_exhaustive]`, so later releases can add to them without breaking your code. A `match` on
  one needs a `_` arm. The error types stay exhaustive.

- The README is the crate's front page and its examples are tested; `examples/today.rs` prints
  today's date, holidays, learning and zmanim.
- `no_std` builds are checked in CI for `thumbv7em-none-eabihf`, with and without `alloc`.
- `cargo bench` (divan) replaces the 10 ms timing assertions that ran as tests.
- Every public item is documented, including every `SpecialDay`, `Parsha`, `Tractate` and `Zman`.
