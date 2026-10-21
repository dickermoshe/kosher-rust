# kosher-rust: the 0.3 rewrite of the public surface

Written 2026-10-21 against `east-west-horizon` (3595cc1).

## The short version

The maths is good. Three conformance suites (KosherJava over JNI, hebcal over WASM, the
MyZmanim site fixture) prove it. Nothing in this plan touches a formula.

What's wrong is everything a user touches: the API, the docs, and the file layout. Too much of
the inside of the crate is public. It's documented as if the reader were maintaining it.

The fix is to **design the crate from the README inward**:

1. Write the README and crate docs a new user would want, as doctests.
2. Change the public API until those doctests compile.
3. Make everything they don't mention `pub(crate)`.
4. Arrange files around what's left.

The result should be a crate with about a third of today's public items, where every public
item has a short doc written for someone *using* it.

---

## 1. What's wrong, concretely

**Too much is public, so too much needs docs.**

- `ZmanPrimitive` is a 50-variant internal expression language. It has `&'static` children,
  so nobody outside the crate can build a useful one. It's public anyway.
- `ZmanPreset` exposes its fields (`event`, `zman_type`, `method_name`).
- `for_all_zman_presets!` is `#[macro_export]`ed only so the Java tests can use it.
- `HebrewHolidayCalendar::input_month`, `type HolidayIter`, `HolidayIterator`,
  `Amud::amudim_to`, `Amud::at_offset`, the `TRACTATES` arrays and `MoladCalendar` are public
  plumbing.
- Five preludes, one per module plus one at the root.

**ICU leaks everywhere.** Users meet `icu_calendar::Date<Hebrew>`, ICU `Month`, a trait on
ICU's unit type (`Hebrew::days_in_hebrew_year(5784)`), and month constants like `ḤESHVAN`,
which nobody can type. A major ICU release then becomes a major kosher-rust release.

**Calls read badly.**

- `date.holidays(false, false)`
- `Location::with_horizon_elevations(lat, lon, elev, east, west, tz)`
- `date.limud(DafYomiBavli::default())`
- `zmanim::types::config::CalculatorConfig`
- `MyZman::X.calculate(&cal)` *and* `cal.calculate(MyZman::X)`
- `ZmanimError` lumps "your latitude is 200" together with "the sun doesn't set today".

**The docs are written at the wrong reader.**

- Every preset carries five paragraphs of end-user display copy pasted in as rustdoc. The
  index shows near-identical text 167 times.
- `Holiday`, `Parsha` and `Tractate` variants have no docs at all (`#[allow(missing_docs)]`).
- `Limud` has one line. The README has no example of the three things the crate does, but it
  does have the maintainer's subtree command.
- Module docs explain internals ("`ZmanPrimitive` is the internal expression language…") where
  a user needs "here's how to get sof zman shma".

**The layout follows history, not concepts.** Molad lives in `zmanim` but is calendar
arithmetic. Astronomy lives in `zmanim` but `myzmanim` reaches into it. `limudim/mod.rs`
holds schedule constants that belong to individual schedules. Presets are 3,620 generated lines
from a 2,459-line Python DSL, with each definition 1,700 lines away from its docs.

---

## 2. Principles for the new version

1. **The README is the spec.** If a task isn't in the README or a module guide, it doesn't
   need a public API yet.
2. **Own the vocabulary.** Public signatures use our types (`HebrewDate`, `HebrewMonth`,
   `Location`) plus `jiff`, which is the crate's chosen date-time vocabulary. They don't use
   ICU.
3. **Closed sets are enums.** Zmanim, MyZmanim rows, holidays, parshiyos and tractates are all
   enums with `ALL`, `name()` and `Display`. Same shape everywhere.
4. **One way to do each thing.** No duplicate entry points and no shims.
5. **`///` is for users and `//` is for maintainers.** Public docs say what something is,
   when to use it, and when it fails. Implementation notes go in `//` comments next to the
   code they explain.
6. **Files hold one idea, not one item.** Split a file when it has two jobs, not by rule.
   Twenty-five well-named files beat seventy tiny ones.
7. **Test through the front door.** Behaviour tests go through the public API in `tests/`.
   `#[cfg(test)]` modules are for private helpers only.

---

## 3. The API we're aiming for

This is the draft README. Every block becomes a doctest.

```rust
use jiff::{civil::date, tz::TimeZone};
use kosher_rust::{HebrewDate, Location, Region, Zman, Zmanim};
use kosher_rust::limudim::{DafYomiBavli, Schedule};

// The Hebrew calendar
let day = HebrewDate::from(date(2024, 4, 23));
println!("{day}");                                      // 15 Nisan 5784
for holiday in day.holidays(Region::Diaspora) {         // impl Iterator<Item = Holiday>
    println!("{holiday} / {}", holiday.hebrew());       // Pesach / פסח
}
day.is_yom_tov(Region::Diaspora);                       // true
day.parsha(Region::Israel);                             // Option<Parsha>
day.year().is_leap();                                   // true

// Zmanim
let lakewood = Location::new(40.09, -74.22, Some(TimeZone::get("America/New_York")?))?
    .with_elevation(30.0)?;
let zmanim = Zmanim::new(lakewood, date(2024, 4, 23));  // default Config
let shma = zmanim.get(Zman::SofZmanShmaGra)?;           // Result<Timestamp, ZmanimError>
for z in Zman::ALL {
    println!("{:<40} {:?}", z.name(), zmanim.get(z));
}

// Learning schedules
let daf = DafYomiBavli.on(day);                         // Option<Daf>
println!("{}", daf.unwrap());                           // e.g. "Shevuos 30"
```

### calendar

| Today | After |
|---|---|
| ext traits `HebrewCalendarDate` + `HebrewHolidayCalendar` on any date | `HebrewDate`, our own `Copy` type wrapping ICU privately. `From<jiff::civil::Date>` and `From<icu Date<C>>`. `to_gregorian() -> civil::Date` |
| `in_israel: bool` and `use_modern_holidays: bool` on every call | `Region::{Israel, Diaspora}`. `holidays()` always yields modern holidays, and `Holiday::is_modern()` filters them |
| `HebrewCalendar` trait on ICU's `Hebrew` unit type | `HebrewYear` value: `is_leap()`, `days()`, `length() -> YearLength`, `rosh_hashana()` |
| ICU `Month` + `month::ḤESHVAN` constants + `HebrewMonthExt` | `HebrewMonth` enum (`Tishrei … Elul`, `AdarI`, `AdarII`) with `Display` and `hebrew()` |
| `todays_parsha` / `special_parsha` / `upcoming_parsha` | `parsha` / `special_parsha` / `next_parsha` |
| `MoladCalendar` trait in `zmanim` returning `(Timestamp, Month)` | `HebrewDate::molad() -> Molad` in `calendar`. The "molad falls today" check moves into the Kiddush Levana zmanim, the only place that needs it |

### location (top level, shared)

```rust
Location::new(lat, lon, tz: Option<TimeZone>) -> Result<Location, LocationError>
    .with_elevation(m)                        -> Result<Location, LocationError>
    .with_horizons(east, west)                -> Result<Location, LocationError>
```

The timezone stays an `Option` argument. `Location::new` fails with
`LocationError::TimeZoneRequired` only when it's `None` and `|longitude| > 150°`. Elevation
moves out of `new` into `with_elevation`. The six-argument `with_horizon_elevations` becomes
`with_horizons`.

`LocationError` covers validation: latitude, longitude, elevation, and the anti-meridian
timezone. `ZmanimError` covers:

- `AllDay` and `AllNight`
- `NotApplicable` (was `InvalidForDate`)
- `TimeZoneRequired`, which only the Kiddush Levana zmanim return when the location has no
  timezone, as they do today
- `Internal`

`InvalidHours` and `TimeConversionError` go.

### zmanim

- `Zmanim::new(location, date)` and `Zmanim::with_config(location, date, config)`.
- `zmanim::Config` (was `CalculatorConfig`) keeps its fields and `Default`.
- **`Zman` enum, 167 variants**, replacing 167 `static ZmanPreset`s. Each variant has a
  one-line doc that names the opinion. KosherJava-deprecated ones get `#[deprecated]`.
  - Variant names are the KosherJava getter minus `get`, with acronyms title-cased:
    `getSofZmanShmaGRA` → `SofZmanShmaGra`, `getSunrise` → `Sunrise`.
  - Methods: `ALL`, `name()`, `category() -> Category`, `java_name()`, `from_java_name()`, and
    `description(&Config) -> String` (the end-user display text, behind `alloc`).
  - Users can match on it, store it, and round-trip it through `java_name()`. It mirrors
    `MyZman`, which already works this way.
- Custom times get three small public methods instead of a public expression language:
  `sun_below_horizon(degrees, Morning | Evening)`, `shaah_zmanis(start, end)` and
  `fraction_of_day(start, end, hours)`. That covers the realistic "my shul uses 8.5°" case.
- `ZmanPrimitive` becomes private `Formula`. `ZmanLike`, `ZmanPreset` and
  `for_all_zman_presets!` go. The Java suite iterates `Zman::ALL` and uses `java_name()`.

### limudim

```rust
pub trait Schedule: sealed::Sealed {
    type Unit;
    fn on(&self, date: impl Into<HebrewDate>) -> Option<Self::Unit>;
}
```

- The schedules become unit structs: `DafYomiBavli`, `DafHashavuaBavli`, `AmudYomiDirshu`,
  `DafYomiYerushalmi`, `MishnaYomis`, `TehillimMonthly`, and `PirkeiAvos(Region)`.
- `on()` returns `None` when nothing is scheduled. Every schedule documents when that happens.
- The engine hooks (`cycle_finder`, `interval_end`, `is_skip`, …) become a private
  `engine::Cycle` trait with one blanket impl. That kills the `InternalLimud` +
  `#[allow(private_bounds)]` sealing hack.
- Unit types get `Display`: `Daf` prints "Shevuos 30" and `Amud` prints "Shevuos 30b". The
  field is renamed from `page` to `daf`. `Mishnas(pub Mishna, pub Mishna)` becomes
  `MishnaPair { first, second }`.

### myzmanim

Same shape as `zmanim`: `MyZmanim::new(location, date)`, `.candle_lighting(minutes)` and
`.get(MyZman::X)`. Drop `MyZman::calculate` and the setters. Its measurement notes stay on the
module, since that is user-relevant ("why does this differ from KosherJava?").

### Gone from the root

The five preludes. The root re-exports `HebrewDate`, `Region`, `Location`, `Zmanim` and `Zman`,
which covers 90% of uses. The rest is one path segment away.

---

## 4. Layout

```
src/
├── lib.rs                  #![doc = include_str!("../README.md")] + module map + re-exports
├── location.rs             Location, LocationError
├── calendar/
│   ├── mod.rs              module guide, re-exports
│   ├── date.rs             HebrewDate (+ conversions, Display)
│   ├── month.rs            HebrewMonth
│   ├── year.rs             HebrewYear, YearLength, elapsed-days / dechiyos arithmetic
│   ├── molad.rs            Molad, chalakim arithmetic
│   ├── holiday.rs          Holiday + its rule table
│   ├── parsha.rs           Parsha, parsha(), special_parsha()
│   └── parsha_tables.rs    the year-type tables, data only
├── zmanim/
│   ├── mod.rs              module guide
│   ├── zmanim.rs           Zmanim, Config, the three custom-time helpers
│   ├── zman.rs             the zman_catalog! macro → enum Zman + metadata
│   ├── catalog.rs          one macro invocation: all 167 entries, grouped by Category
│   ├── formula.rs          private Formula + evaluator
│   └── error.rs            ZmanimError
├── limudim/
│   ├── mod.rs              module guide, Schedule trait
│   ├── units.rs            Tractate, Daf, Amud, Side, Mishna, MishnaPair
│   ├── engine.rs           private cycle/interval walker (was cycle.rs + interval.rs + limud.rs)
│   └── daf_yomi_bavli.rs … one file per schedule, owning its own constants and tables
├── myzmanim/
│   ├── mod.rs              module guide + measured constants
│   └── myzman.rs           MyZmanim, MyZman
└── astronomy/              pub(crate). Moved out of zmanim/ because myzmanim uses it too
    ├── mod.rs  noaa.rs  meeus.rs  spa.rs  vsop87.rs  solar_radius_table.rs
tests/
├── calendar.rs  zmanim.rs  limudim.rs  myzmanim.rs   public-API behaviour tests
├── catalog_snapshot.rs + catalog_snapshot.txt         see phase 0
└── myzmanim_site.rs + fixtures/
examples/
└── today.rs                prints today's date, holidays, zmanim and daf for a location
```

### The preset catalog: a macro instead of Python

One entry per zman, definition and docs together:

```rust
zman_catalog! {
    /// Dawn: 72 fixed minutes before sunrise (4 mil at 18 minutes each).
    Alos72Minutes {
        java: "getAlos72Minutes",
        name: "Alos (72 Minutes)",
        category: Alos,
        formula: Offset(&ConfiguredSunrise, mins(-72)),
        about: "Dawn calculated as 72 minutes before sunrise, the time to walk 4 mil …",
    }
}
```

The `///` line is the rustdoc: short and written for the developer. `about` is the display copy
for *their* users, returned by `Zman::description()`, and never shown in rustdoc. That one split
fixes the "167 identical essays" problem. `tools/` (Python, uv, pydantic) and the codegen CI job
are deleted.

---

## 5. How docs are written

Put these rules in `CONTRIBUTING.md` and keep them short enough that people follow them:

- **Crate root:** the README (install, the three examples above, features, "coming from
  KosherJava" table, what's not ported). This is the getting-started guide.
- **Each module:** two or three sentences on what it's for, one runnable example, and a short
  list of the types to reach for. No architecture.
- **Entry-point types and their main methods** (`HebrewDate`, `Zmanim`, `Schedule`,
  `MyZmanim`, `Location`): what it does, a short example, and when it fails ("Returns `None`
  when…" or `# Errors`).
- **Enum variants:** one line. For `Zman`, name the opinion (Gra, Magen Avraham, Baal
  HaTanya…). For `Holiday`, `Parsha` and `Tractate`, give the transliteration and the Hebrew.
- **Constants:** value, unit, and where it comes from.
- **Never in `///`:** how it's computed internally, which private helper it calls, or
  historical notes about the port. Put those in `//`, or delete them.
- **Lints:** `missing_docs = "deny"`, `rustdoc::broken_intra_doc_links = "deny"`,
  `clippy::missing_errors_doc`. The `#[allow(missing_docs)]` exemptions get removed. With a
  third of the public items, this is cheap to keep up.

Maintainer material (toolchains, subtree update, tracked KosherJava sha, how to add a zman,
release steps) moves from the README to `CONTRIBUTING.md`.

---

## 6. Order of work

Each phase is one PR. Conformance stays green at every step, and the conformance crates are
updated in the same PR as the API they use. They are the first real consumer: if they get
uglier, the API is wrong.

| # | Phase | What lands |
|---|---|---|
| 0 | **Safety net** | `tests/catalog_snapshot.rs`: a sorted dump of `(java name, display name, category, deprecated, formula Debug)` for every preset, committed. Confirm all three conformance suites run from one documented command. |
| 1 | **README first** | Rewrite the README against the target API. It is the spec, so review it before writing code. Its examples use real values computed with the current crate. Module guides and `examples/today.rs` land with the phase that makes them compile; `lib.rs` includes the README as a doctest in phase 8. |
| 2 | **Move internals** | `astronomy/` out of `zmanim/`, molad into `calendar/`, schedule constants into their schedule files, `cycle` + `interval` + `limud` → `engine.rs`. No public change. |
| 3 | **Calendar** | `HebrewDate`, `HebrewMonth`, `HebrewYear`, `Region`, `Molad`, plus `Display`/`hebrew()` on `Holiday` and `Parsha`. Remove the ext traits and ICU from signatures. |
| 4 | **Location + errors** | `new(lat, lon, Option<TimeZone>)`, `with_elevation`, `with_horizons`, `LocationError`, slimmer `ZmanimError`. |
| 5 | **Zmanim catalog** | `zman_catalog!`, `enum Zman`, `Zmanim`, `Config`, the custom-time helpers, private `Formula`. Snapshot test must match byte for byte. Delete `tools/` and the codegen job. |
| 6 | **Limudim** | `Schedule`, unit-struct schedules, private engine trait, `Display` on units. |
| 7 | **MyZmanim** | Align with `Zmanim`'s shape. |
| 8 | **Docs + hygiene + release** | Doc pass to §5, deny lints, `CONTRIBUTING.md`, `CHANGELOG.md` with a 0.2 → 0.3 migration table, repo cleanups below. Publish 0.3.0. |

Phases 3, 4 and 6 touch different files and can run in parallel once phase 2 is in.

**Repo cleanups for phase 8:**

- CI's PR trigger is `branches: [master]`, but the default branch is `main`.
- `Cargo.toml` `repository` points at `rust-zmanim-project`, but the remote is `kosher-rust`.
- `exclude` lists `/crates/` and `/REVIEW.md`, which don't exist.
- `rust-toolchain.toml` installs 12 cross targets.
- `.gitignore` is 392 lines.
- `tests/benchmarks.rs` asserts 10 ms on shared CI runners. Turn it into a `benches/` target.
- Add a `no_std` build check (`--no-default-features --target thumbv7em-none-eabihf`) so the
  claim is tested.

**No deprecation shims.** It's 0.x. A clean break plus a migration table costs users ten
minutes. Shims would keep the old mess alive for another release.

---

## 7. Decisions (settled 2026-10-21)

1. **Own `HebrewDate` type** instead of extension traits on any date. It gives one rustdoc
   page listing everything you can ask about a day, and it gets ICU out of the public API.
2. **`Zman` enum** instead of 167 statics or 167 methods.
3. **`ZmanPrimitive` becomes private**, with three custom-time helpers. It can be made public
   later if someone asks.
4. **The timezone is an `Option` argument to `Location::new`.** It's required only near the
   anti-meridian (checked when the `Location` is built) and for Kiddush Levana (checked when
   that zman is calculated).
5. **Modern holidays are always yielded** and filtered with `Holiday::is_modern()`.
