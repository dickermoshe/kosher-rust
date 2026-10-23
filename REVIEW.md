# kosher-rust review

Written 2026-10-22 against `main` at 22c7ddd ("Merge pull request #7 from
dickermoshe/bindings-review-fixes").

This file lists every problem the review found, one checkbox each. Each item is meant to stand on
its own. It says what is wrong, where, how to see it, why it matters and what a fix could look
like, so you can pick any item up later without the conversation that produced it.

## How the review was done

Four reviewers worked in parallel on astronomy, calendar and limudim, zmanim and MyZmanim, and
the `kosherz` bindings. Each compared the Rust code with the vendored KosherJava sources in
`third-party/kosher-java` and ran throwaway programs against the crate to confirm what they
suspected. I checked the CI, the packaging and the docs myself and re-ran the most important
findings.

Each item has a status.

- **Reproduced** means I ran it myself and saw the failure.
- **Confirmed** means a reviewer ran code and saw the failure.
- **From reading** means it comes from reading the code and was not run.

Each item also has a severity. **High** means users get a crash or a wrong answer in normal use,
or the release is broken. **Medium** means a real bug in less common cases, or a gap that lets
such bugs through. **Low** means polish, docs or an unusual edge.

### What passed

All of this passed locally on Windows with Rust 1.99.1.

- `cargo test -p kosher-rust`. That covers 206 unit tests, 21 calendar tests, 6 limudim tests,
  3 MyZmanim site tests and 17 doctests.
- `cargo clippy --workspace --all-targets --all-features`.
- `cargo clippy -p kosher-rust --no-default-features --lib`.
- `cargo build -p kosher-rust --no-default-features --target thumbv7em-none-eabihf`.
- `RUSTDOCFLAGS="-D warnings" cargo doc -p kosher-rust --no-deps --all-features`.
- `cargo fmt --all --check`.
- `cargo test -p kosherz`. That covers 13 parity tests and the generated-code drift test.

The KosherJava (Maven and JNI) and hebcal (Bun) conformance suites were not run. CI runs them.

### What is in good shape

These were checked and found correct, so they don't need another look.

- All 167 `Zman` entries match the KosherJava method bodies. That includes the base (sea level or
  elevation), degrees, minutes, which day bounds the *shaah zmanis*, the Ateret Torah, Baal
  HaTanya, fixed-local-chatzos and Ahavat Shalom families, the Ben Ish Chai polar times and the
  chametz times. `java_name` and `from_java_name` round-trip for all 167. The 24 deprecated
  variants are exactly Java's `@Deprecated` getters.
- The VSOP87 tables, the SPA nutation tables, the obliquity polynomial and the solar radius table
  are numerically identical to Java's. The NOAA and Meeus formulas, the ΔT polynomials, the
  solar-midnight retry and the azimuth loops match. A sweep of 2.16 million comparisons across
  the three solar algorithms found nothing over 10 minutes apart except items C1 and C8.
- Holiday rules match Java's `getYomTovIndex` rule by rule. That covers Israel and the diaspora,
  fast postponements, the modern Israeli days, Purim Katan, Rosh Chodesh, Chanukah, Yom Kippur
  Katan, BeHaB, the Omer, Machar Chodesh, Mevarchim and Birchas Hachamah. `is_assur_bemelacha`,
  `has_candle_lighting`, `special_parsha` and the parsha table choice match too.
- Year arithmetic, *dechiyos* and year lengths agree with ICU for every Hebrew year from 1 to
  13760. The molad matches an independent formula for 3761 to 13760.
- Daf Yomi Bavli (including the Shekalim change in cycle 8), Yerushalmi skips, Mishna Yomis and
  Tehillim month lengths are right.
- The Kiddush Levana windows and the time-zone handling match Java. That includes jiff's
  `compatible` choice in daylight-saving gaps and overlaps, and the date-line adjustment at Apia
  in December 2011. The list in the README of what needs a time zone is accurate.
- About 2.3 million evaluations near `Date::MIN` and `Date::MAX`, at the poles and at ±180°
  found no panic in the zmanim code itself. The panic in B1 is in the calendar code they call.
- There is no `unsafe` code in `src` or `kosherz/src`. WeaveFFI wraps every exported call in
  `catch_unwind`. Regenerating all 11 bindings with weaveffi-cli 0.25.0 gives output byte for
  byte identical to what is committed.

---

## 1. Release blockers

- [x] **B1. `Zmanim::get` and `HebrewDate::molad` panic on valid dates before Hebrew year 1.**
  High. Reproduced.

  **What happens.** The molad code divides with Rust's `/` and `%`, which round toward zero.
  For a date before Hebrew year 1, the count of *chalakim* since the epoch is negative, so the
  hours and minutes it computes are negative. jiff's `Date::at(hour, minute, ..)` then panics
  with "invalid hour" or "invalid minute".

  **Where.** [src/calendar/molad.rs:60-91](src/calendar/molad.rs#L60), in `months_molad`. It is
  called by `HebrewDate::molad` ([src/calendar/date.rs:263](src/calendar/date.rs#L263)) and by
  the four Kiddush Levana zmanim ([src/zmanim/formula.rs:403](src/zmanim/formula.rs#L403), and
  lines 419, 438, 454 and 471).

  **How to see it.**

  ```rust
  use jiff::{civil::date, tz::TimeZone};
  use kosher_rust::{HebrewDate, Location, Zman, Zmanim};

  HebrewDate::from(date(-3760, 5, 12)).molad(); // panics with "invalid hour"

  let zmanim = Zmanim::new(Location::new(0.0, 0.0)?, date(-4151, 8, 10), Some(TimeZone::UTC));
  zmanim.get(Zman::TchilasZmanKidushLevana3Days); // panics with "invalid minute"
  ```

  The calendar reviewer counted 9030 month starts between Hebrew years -6259 and 0 that panic.
  jiff's earliest date, `jiff::civil::Date::MIN`, is in Hebrew year -6239, so many of them can
  be reached straight from a jiff date.

  **Why it matters.** A library call panics on an input the types accept. Through `kosherz` it
  comes back as an `InternalError` ("producer panicked: invalid minute"), and `all()` for that
  day fails as a whole. On wasm32, where a panic aborts, it would probably take down the whole
  module (not run). The changelog says the opposite, at
  [CHANGELOG.md:20](CHANGELOG.md#L20) ("No panics at the ends of the date range").

  **Why the tests miss it.** `extreme_dates_do_not_panic` in
  [tests/calendar.rs:242](tests/calendar.rs#L242) uses years ±20000. ICU only builds dates for
  years -9999 to 9999, so `HebrewDate::new(±20000, ..)` and `Date::try_new_gregorian(±20000, ..)`
  both fail and the body never calls `exercise()`.

  **Fix.** Pick one of these.

  - Compute the molad as a `Timestamp` straight from the day number and the *chalakim*, with
    `div_euclid` and `rem_euclid`, and never call `.at()`. This also fixes C3.
  - Return `None` for Hebrew years before 1. KosherJava refuses any date before 18 Teves 3761
    (1 January 1 CE), so Java never computes these at all.

  Then fix the test. Use years ICU accepts (-9999, 0, 1, 9999), dates built from
  `jiff::civil::Date::MIN` and `MAX`, and call `Zmanim::get` for every `Zman` on those dates.
  The `kosherz` constructors could also reject Gregorian dates whose Hebrew year is outside the
  supported range with `InvalidGregorianDate`
  ([kosherz/src/convert/mod.rs:144](kosherz/src/convert/mod.rs#L144) accepts any year from -9999
  to 9999 today).

- [x] **B2. `cargo test` on the published crate doesn't compile.** High. Reproduced.

  **What happens.** [src/astronomy/spa/tests.rs:48](src/astronomy/spa/tests.rs#L48) reads
  `tests/fixtures/spa_reference_testdata.csv` with `include_str!`. `Cargo.toml` excludes the
  whole `/tests/fixtures/` folder from the package, but it ships `src/astronomy/spa/tests.rs`.

  **How to see it.** Run `cargo package -p kosher-rust --no-verify`, unpack the `.crate`, and run
  `cargo test --no-run` inside it. It fails with "couldn't read
  `src\astronomy\spa\../../../tests/fixtures/spa_reference_testdata.csv`". `cargo publish`
  doesn't catch this, because its verify step builds only the library.

  **Why it matters.** Anyone who runs the tests of the published crate hits a compile error. That
  includes distro packagers, crater runs and people checking a vendored copy.

  **Fix.** Ship the CSV, which is only 33 KB. Exclude only `tests/fixtures/myzmanim-site.bin` and
  `tests/fixtures/zman_catalog.snap`. Or move the SPA reference test into `tests/` and exclude
  it like `tests/myzmanim_site.rs`. The catalog snapshot test already copes with a missing file
  ([src/zmanim/catalog_snapshot.rs:69](src/zmanim/catalog_snapshot.rs#L69)).

- [x] **B3. The changelog, the version and the tags don't match what has been released.** High.
  Reproduced.

  **What is wrong.**

  - crates.io has 0.2.0 as the latest release. `Cargo.toml` says 0.3.0, and
    [CHANGELOG.md](CHANGELOG.md) has a "0.3.0 (2026-10-22)" section, so 0.3.0 has not been
    published.
  - Above it, "Unreleased" lists breaking changes. The time zone moved from `Location` to
    `Zmanim`, `LocationError::TimeZoneRequired` is gone, and there is the new `TimeZoneLike`.
  - Because of that, the 0.3.0 migration tables are now wrong.
    - [CHANGELOG.md:80-81](CHANGELOG.md#L80) tell people to write `Location::new(lat, lon, tz)`.
      It now takes two arguments.
    - [CHANGELOG.md:87](CHANGELOG.md#L87) points to `LocationError::TimeZoneRequired`, which no
      longer exists.
    - [CHANGELOG.md:98](CHANGELOG.md#L98) shows `Zmanim::with_config(location, date, config)`
      and `Zmanim::new(location, date)`. Both now take a time zone too.
    - [CHANGELOG.md:135](CHANGELOG.md#L135) shows `MyZmanim::new(location, date)`, which also
      takes a time zone now.
    - [CHANGELOG.md:84](CHANGELOG.md#L84) names `zmanim::{CalculatorConfig, ..}`. The type is
      `Config`, as line 105 of the same file says.
  - The remote has a tag `v0.5.0`. It points to 6d1507d from 2026-03-03, the old multi-crate
    workspace (`crates/zmanim-calculator` and others). Tagging `v0.3.0` after it, as
    [CONTRIBUTING.md](CONTRIBUTING.md) says to, leaves `v0.5.0` looking like the newest release
    to anything that sorts tags, GitHub's "latest release" included.

  **Fix.** Either fold "Unreleased" into 0.3.0 and correct those rows, or release 0.3.0 as it was
  and call this 0.4.0. Decide what to do with `v0.5.0`. Deleting a pushed tag is not reversible
  for people who fetched it, so renaming or annotating it may be safer.

- [x] **B4. The README says every KosherJava getter is a `Zman`, but four are missing.** Medium.
  Reproduced.

  **What is wrong.** [README.md:144](README.md#L144) says "Every zman getter is a `Zman` variant
  with the same name minus `get`." These four public getters in the vendored KosherJava have no
  variant, and `Zman::from_java_name` returns `None` for them.

  - `getZmanMolad`
  - `getPolarPlagHaminchaBenIshChai`
  - `getPolarStartOfDayTeshuvosVehanhagos`
  - `getPolarPlagHaminchaTeshuvosVehanhagos`

  The Java parity tests build their list from `Zman::ALL`, so they can't notice a missing
  getter. The parity policy even lists `getZmanMolad`
  ([interop-tests/java_interop_tests/src/zmanim/policy.rs:48](interop-tests/java_interop_tests/src/zmanim/policy.rs#L48)),
  but nothing runs it.

  **Fix.** Port the four, or list them under "Not ported yet" in the README. A test that lists the
  public getters of `ComprehensiveZmanimCalendar` through JNI and compares them with `Zman::ALL`
  would catch this from now on. Read B5 first, because adding variants after release is a
  breaking change today.

- [x] **B5. Decide on `#[non_exhaustive]` before publishing.** Medium. From reading.

  **What is wrong.** No public enum or struct in the crate is `#[non_exhaustive]`. Under semver,
  adding a variant to `Zman`, `MyZman`, `SpecialDay`, `Parsha`, `Category`, `ZmanimError`,
  `LocationError` or `AstronomicalCalculator` then breaks anyone who matches on it exhaustively.
  So does adding a field to `Config`, which has all-public fields, for anyone who writes the
  struct out in full. KosherJava adds zmanim regularly, and B4 already needs four new variants.

  **Fix.** This is a design choice, so it's yours to make. Marking the error enums and the
  catalog enums `#[non_exhaustive]` lets you add variants in minor releases. The cost is that
  users must write a `_` arm when they match.

  `Config` is harder. A `#[non_exhaustive]` struct can't be built with a struct expression
  outside the crate, even with `..Config::default()`. That would break the README and doc
  examples that do exactly that. Users could still write `let mut config = Config::default();`
  and then set fields, or you could add builder methods. Leaving `Config` as it is means a new
  field only breaks people who list every field, which is rare.

  Adding `#[non_exhaustive]` after 0.3.0 is itself a breaking change, so decide before publishing.

## 2. Wrong results in the library

- [x] **C1. Near the date line, some sunrises and sunsets land on the wrong day.** Medium.
  Reproduced.

  **Fixed in both.** The vendored KosherJava and the Rust port now add the two missing wraps: a sunrise at -6 hours local mean time or earlier moves forward a day, and a sunset at +30 hours or later moves back a day. A sweep of 759,577 sunrises and sunsets near the date line above 60° found 10 out of their windows before and none after. The Java change is proposed upstream as [KosherJava/zmanim#296](https://github.com/KosherJava/zmanim/pull/296).

  **What happens.** At Cape Schmidt (68.88, -179.37) in the `Asia/Anadyr` time zone, with the
  default config, on 2021-01-10 these come back.

  | Zman | Returned (local time) | Expected |
  |---|---|---|
  | `SeaLevelSunrise` | 2021-01-09 12:00:23 | 2021-01-10, around 12:00 |
  | `SeaLevelSunset` | 2021-01-10 12:09:57 | correct |
  | `SofZmanShmaGra` | 2021-01-09 18:02:47 | 2021-01-10, a few minutes after 12:00 |

  This is a day of about 10 minutes of sun, between a day with none (2021-01-09) and a day of
  about an hour (2021-01-11). Sunrise is reported a day early, so the "day" runs for more than 24
  hours, and every proportional zman built on it is nonsense. All three
  solar algorithms do the same. The astronomy reviewer found 16 such cases in a sweep, for
  example also at 68.5°N on 2024-01-08 and 70.5°N on 2025-01-19.

  **Why.** [src/astronomy/mod.rs:374-402](src/astronomy/mod.rs#L374), in
  `instant_from_utc_hours`. The solar code returns a UTC hour folded into 0 to 24. This
  function adds `longitude / 15` to get local mean time and picks the date from that. It moves a
  sunrise back a day when local mean time is after 18:00, and a sunset forward a day when it is
  before 06:00. It never handles the opposite wrap. West of about -176°, a sunrise near local
  midday has a true UTC hour of 24 or more, which folds to just after 0 and lands a day early.
  East of about +176°, a sunset near local midday has a true UTC hour below 0, which folds to
  just before 24 and lands a day late. This only happens in polar winter, when sunrise and sunset
  are near midday, close to the date line.

  **Why the tests miss it.** KosherJava's `AstronomicalCalendar.getInstantFromTime` has the same
  logic, so Rust and Java agree and the parity suite passes. The comment at
  [src/astronomy/spa/mod.rs:278-282](src/astronomy/spa/mod.rs#L278) says a result in the SPA
  window "always maps back to this date", which is not true in these cases.

  **Fix.** Add a sunrise case that moves the date forward a day when local mean time is -6 or
  less, and a sunset case that moves it back a day when local mean time is 30 or more. That makes
  the mapping the exact inverse of the SPA search windows. In local mean time those are (-6, 18]
  for a sunrise, and (6, 30] for a sunset, which should become [6, 30) (C8 (a)). This deliberately departs from KosherJava, so the Java
  parity suite needs a documented exception for these cases. Consider reporting it upstream.

- [x] **C2. Calendar arithmetic is wrong for Hebrew year 0 and earlier.** Low. Reproduced
  (`is_leap`) and confirmed (the rest).

  **What happens.** Several places use `%` and `/`, which round toward zero, on values that are
  negative for those years.

  - [src/calendar/year.rs:53](src/calendar/year.rs#L53), `is_leap` computes
    `((year - 1) % 19) + 1`. `HebrewYear::new(0).is_leap()` is `false`. Java's formula
    `((7 * year) + 1) % 19 < 7`, written with `rem_euclid`, gives `true`, and ICU agrees.
  - [src/calendar/molad.rs:42-45](src/calendar/molad.rs#L42), in `chalakim_at_month_index`.
  - [src/calendar/year.rs:147-152](src/calendar/year.rs#L147), the `% 7` weekday checks in
    `elapsed_days`.
  - [src/calendar/parsha_tables.rs:1012](src/calendar/parsha_tables.rs#L1012) casts a negative
    `i64` to `usize` before `% 7`, which picks the wrong weekday.

  **What users see.** `HebrewDate::from(jiff::civil::Date::MIN)` is Hebrew year -6239, so this is
  reachable. The calendar reviewer found these results for years 0 and below.

  - 3685 leap-year answers and 6482 year-length answers differ from ICU.
  - Year 0 is a leap year in ICU, but a date in its last Adar prints as "14 Adar 0" instead of
    Adar II.
  - `HebrewDate::new(0, HebrewMonth::AdarI, 1)` exists, yet `year().months()` lists 12 months
    and `days_in(AdarI)` is `None`.
  - In 2447 years Chanukah has 9 days, `[1, 2, 3, 4, 5, 6, 6, 7, 8]`.
  - `parsha()` is `None` on every Shabbos in 9984 of 10000 such years, so `next_parsha()` is
    `None` too. Its doc says that only happens "near the limits".

  Every year from 1 to 13760 is correct.

  **Fix.** Pick one of these.

  - Use `rem_euclid` and `div_euclid` throughout, or take leap years, year lengths and Kislev
    length from ICU.
  - Declare that Hebrew years before 1 are unsupported, and say so on `HebrewDate`, not only on
    `HebrewYear`. Java rejects them.

- [x] **C3. `molad()` is wrong or `None` for Hebrew years 1 to 3760.** Low. Confirmed.

  **What happens.** `abs_date_to_gregorian_date` at
  [src/calendar/molad.rs:99-117](src/calendar/molad.rs#L99) is a hand-written day-number to
  Gregorian conversion. It divides with `/`, which rounds toward zero, and only searches forward
  for the year, so it goes wrong for dates before 1 CE. Of the 46,505 months in Hebrew years 1 to
  3760, 722 come out right, 38,485 return `None` and 7,298 are wrong. For example, 1 Sivan 3001
  (Gregorian -759-05-07) gives a molad on -758-01-15 instead of -759-05-05.

  The doc at [src/calendar/date.rs:262](src/calendar/date.rs#L262) says it "Returns `None` near
  the limits of the supported date range", which understates this.

  **Fix.** Replace the hand-written conversion with ICU's `Date::from_rata_die`, or compute the
  `Timestamp` directly from the day number. The first fix for B1 covers this too.

- [x] **C4. Some functions stop at Hebrew year 9999, though dates go on to 13760.** Low.
  Confirmed.

  **What happens.** ICU's `Date::try_new_hebrew_v2` only accepts years -9999 to 9999. A
  `HebrewDate` built from a jiff date can be as late as 28 Cheshvan 13760 (jiff's
  `9999-12-31`), and `add_days` goes further. Everything that builds a date from a year number
  then fails.

  - `HebrewDate::new(13760, HebrewMonth::Cheshvan, 28)` returns `None`, though that date exists
    ([src/calendar/date.rs:62](src/calendar/date.rs#L62)).
  - `HebrewYear::rosh_hashana()` returns `None` for years from 10000
    ([src/calendar/year.rs:117](src/calendar/year.rs#L117)). Its doc says "outside the range the
    underlying calendar supports", which reads as if the date itself were unsupported.
  - `TehillimMonthly` returns `None` from 1 Tishrei 10000, which is Gregorian 6239
    ([src/limudim/tehillim_monthly.rs:62](src/limudim/tehillim_monthly.rs#L62) and line 66).
  - `DafYomiYerushalmi` returns `None` after Gregorian 6236-10-12
    ([src/limudim/daf_yomi_yerushalmi.rs:108](src/limudim/daf_yomi_yerushalmi.rs#L108) and line
    122).
  - `PirkeiAvos` returns `None` from year 10000. That one is intended and documented
    ([src/limudim/pirkei_avos.rs:135](src/limudim/pirkei_avos.rs#L135)).

  Daf Yomi Bavli, Mishna Yomis, Amud Yomi, Daf HaShavua, the parsha and the holidays keep
  working.

  **Fix.** Build these dates without a year-based constructor. For the first of the current
  month, use `date.add_days(1 - day)`. The calendar reviewer reports that ICU's
  `Date::try_from_fields` accepts the wider range, which is worth checking. If you keep the
  limit, document it on each function.

- [x] **C5. `sunrise_offset_by_degrees` and `sunset_offset_by_degrees` jump at 0° and accept any
  number.** Low. Confirmed.

  **What happens.**

  - At exactly `0.0` the zenith is 90°, the value `adjust_zenith` treats as true sunrise and
    sunset. It then adds refraction, the solar radius and the elevation dip
    ([src/astronomy/mod.rs:433-439](src/astronomy/mod.rs#L433)). So `sunrise_offset_by_degrees(0.0)`
    equals sea-level sunrise, while `sunrise_offset_by_degrees(1e-9)` is 4 min 33 s later
    (Lakewood, 2024-04-23). The docs at
    [src/zmanim/calculator.rs:108-128](src/zmanim/calculator.rs#L108) say the angle is "measured
    from the geometric horizon, without elevation". KosherJava behaves the same way.
  - NaN or infinity returns `ZmanimError::Internal`, whose message asks users to report a bug.
  - Values like `-1000.0` or `1e300` return made-up times.
  - SPA returns `Internal` for `95.0`, where NOAA and Meeus return `AllDay`
    ([src/astronomy/spa/mod.rs:428-431](src/astronomy/spa/mod.rs#L428)).

  **Fix.** Document the 0° case, or special-case it so it really is the geometric horizon. Reject
  angles that are not finite or are outside a sensible range (0 to 90, or -90 to 90 if you want to
  allow the sun above the horizon), with an error that is not `Internal`. Adding an error variant
  is a breaking change unless B5 is done first.

- [x] **C6. MyZmanim misbehaves for very high horizons and NaN candle lighting.** Low. Confirmed.

  **What happens.**

  - `refraction` at [src/myzmanim/myzman.rs:488-490](src/myzmanim/myzman.rs#L488) computes
    `(1 - 2.25577e-5 * h).powf(5.25588)`. Above about 44,330 m the base is negative and the result
    is NaN, so `MyZman::SunriseLevel` and the times built on it return `Internal`. `Location`
    accepts any finite elevation.
  - `MyZmanim::candle_lighting(f64::NAN)` silently gives sunset itself as candle lighting
    ([src/myzmanim/myzman.rs:286](src/myzmanim/myzman.rs#L286)).

  **Fix.** Give `Location` an upper elevation limit (Everest is under 9 km), or clamp in
  `refraction`. Take candle lighting as a `SignedDuration`, which also fixes D5, or ignore
  values that aren't finite.

- [x] **C7. `Zman::description` says the wrong thing in three cases.** Low. Reproduced (a) and
  confirmed (b, c).

  **(a) Offsets are rounded and lose their sign.** `minutes` at
  [src/zmanim/zman.rs:217-219](src/zmanim/zman.rs#L217) uses `as_mins()`, which truncates, and
  `unsigned_abs()`, which drops the sign. An 18 min 45 s candle-lighting offset reads "18
  minutes". A negative offset still reads "before sunset".

  **(b) Two zmanim never mention the chatzos setting.** `uses_astronomical_chatzos_from_config`
  at [src/zmanim/formula.rs:652-699](src/zmanim/formula.rs#L652) returns `false` for
  `MinchaGedolaAhavatShalom` and `MinchaGedolaGraGreaterThan30`. Both are computed from
  `ChatzosHayom`, so `Config::use_astronomical_chatzos` changes them, but their descriptions
  don't say so.

  **(c) Morning zmanim are said to "divide the afternoon".** The sentence added at
  [src/zmanim/zman.rs:200](src/zmanim/zman.rs#L200) when
  `use_astronomical_chatzos_for_other_zmanim` is on says "This zman divides the afternoon from
  astronomical chatzos". It is also added to Shema, Tefila and the chametz times, which divide
  the morning.

  **Fix.** Print minutes and seconds with the sign, or "after sunset" when negative. Return `true`
  for those two formulas. Pick the morning or afternoon wording from the formula. Then
  regenerate `tests/fixtures/zman_catalog.snap` with `UPDATE_SNAPSHOT=1`.

- [x] **C8. Smaller astronomy issues.** Low. Confirmed (a, c) and from reading (b, d).

  **(a) SPA's sunset window has the boundary on the wrong side.**
  [src/astronomy/spa/mod.rs:414](src/astronomy/spa/mod.rs#L414) accepts a sunset when
  `t1 > window.0 && t1 <= window.1`. A result at exactly 06:00 local mean time the next morning is
  accepted, and `instant_from_utc_hours` then maps it to the next date. Java's secant search uses
  `start <= t < end`. Only one exact value is affected. Fix it with C1.

  **(b) SPA's transit search can quietly return an estimate.**
  [src/astronomy/spa/mod.rs:328-329](src/astronomy/spa/mod.rs#L328) returns local mean noon
  when the search finds nothing. The module docs say searches report "no event rather than a
  near miss". In practice the search always succeeds, so a `debug_assert!` or returning
  `Internal` would make that explicit.

  **(c) NOAA and Meeus don't settle near the poles.** This is inherited from KosherJava, not a
  porting slip. Within about 1.75° of a pole NOAA differs from SPA by up to 54 minutes. In 5 cases
  a deeper angle gave a later sunrise, which can't happen. Rise and set disagree on the error
  kind. At latitude -90 on 2025-05-04, NOAA returns `AllDay` for a 16.1° rise and `AllNight` for
  a 16.1° set, while SPA finds a real setting at 09:58 UTC. Consider documenting that SPA is the
  algorithm to use near the poles.

  **Won't fix.** The two fixed passes are NOAA's published algorithm, and other libraries keep
  them. NOAA's own calculator and astral make two passes, and suncalc refines twice. NOAA states
  its results are within a minute only between ±72° latitude. SPA is the accurate choice near the
  poles, and the `AstronomicalCalculator` docs now say so.

  **(d) The barycenter option is described as doing more than it does.**
  [src/astronomy/vsop87.rs:19-27](src/astronomy/vsop87.rs#L19) and
  [src/astronomy/meeus.rs:32-34](src/astronomy/meeus.rs#L32) say the option turns the Earth's
  orbit into the Earth-Moon barycenter's. It only removes the synodic-month terms. The 0.58″
  Moon-driven B0 term stays in. The effect on times is negligible.

## 3. Tests that check less than they seem to

- [x] **T1. The MyZmanim site test can't catch an error of whole hours, and ~10k fixture cells are
  11 to 12 hours off.** Medium. Confirmed, and I checked the logic by reading.

  **What is wrong.** At [tests/myzmanim_site.rs:379](tests/myzmanim_site.rs#L379) the test
  subtracts the nearest whole number of hours from every difference, to allow for MyZmanim's
  daylight-saving hour.

  ```rust
  seconds -= (seconds / 3600.0).round() * 3600.0;
  ```

  After that, `seconds` is never more than 1800 from zero. So the `seconds.abs() > 1800.0` branch
  on line 381 never runs, and `assert!(far_off.is_empty())` on line 435 always passes. A real bug
  of a whole number of hours, such as a daylight-saving mistake, a wrong offset sign or a 24-hour
  rollover, would pass apart from its leftover seconds.

  **What it hides.** The zmanim reviewer counted the hours being removed across the fixture.

  | Hours removed | Cells |
  |---|---|
  | -12 | 9,777 |
  | -1 | 447,694 |
  | 0 | 486,648 |
  | +11 | 296 |
  | +12 | 188 |

  The 0 and -1 rows are the daylight-saving hour. Each place-day uses only one of them, and 0 of
  the 24,140 place-days mix them. The ±11 and ±12 rows, about 10,261 cells, look like AM/PM
  mistakes in the scraped data around 12:00. For example, a midday of 11:50:15 is stored as
  23:50:15, a midnight of 23:57 on 4 December is stored as 5 December 11:57, and Anchorage's
  nightfall at 23:10 in July is stored as 12:10.

  **Fix.** For each place-day, take the most common hour correction and require it to be 0 or -1
  for every column. Fix or drop the cells that are 11 to 12 hours off in the fixture, or handle
  them explicitly. Then compare the raw difference, so that `far_off` can actually fail.

- [x] **T2. Java parity only compares "has a time" with "has no time".** Medium. From reading.

  **What is wrong.** On the Rust side of the parity harness,
  [interop-tests/java_interop_tests/src/zmanim/rust_reference.rs:29](interop-tests/java_interop_tests/src/zmanim/rust_reference.rs#L29)
  turns every error into `None`. Java returns `null`, which is also `None`. So an
  `Err(ZmanimError::Internal)` ("please report") passes as long as Java returns `null`, and
  `TimeZoneRequired` would pass too. Every case passes a time zone, so the documented behaviour
  without one is never checked against anything.

  **Fix.** Fail the test when Rust returns `Internal` or `TimeZoneRequired` with a time zone
  given. Add a check that, for the zmanim the docs say work without a time zone, `None` and
  `Some(tz)` give the same instant (for |longitude| ≤ 150°).

- [x] **T3. The conformance suites cover narrow date ranges, and the molad isn't compared
  directly.** Low. From reading.

  **What is wrong.**

  - Calendar parity only uses Gregorian 1900 to 2100
    ([interop-tests/java_interop_tests/src/calendar/policy.rs:12-13](interop-tests/java_interop_tests/src/calendar/policy.rs#L12)).
  - Zmanim parity uses 1900 to 2300. The Kiddush Levana zmanim use 1990 to 2030, to avoid time
    zone database differences
    ([policy.rs:41](interop-tests/java_interop_tests/src/zmanim/policy.rs#L41)).
  - The molad is never compared with Java's `JewishCalendar.getMolad`. It is checked only through
    the Kiddush Levana zmanim, and `getZmanMolad` isn't ported (B4).
  - The hebcal limudim tests use random dates from 1900 to 2099, and Pirkei Avos is tested on
    Shabbos only.

  KosherJava accepts dates from 1 CE, so most of B1, C2 and C3 sit outside every tested range.

  **Fix.** Widen the random ranges to what Java accepts. Use fixed time zones for the early years
  where the time zone databases disagree. Add a direct molad comparison, and Pirkei Avos on
  weekdays.

- [x] **T4. Binding smoke tests in 9 of 11 languages can't catch a wrong time-zone offset, and the
  Python example checks nothing.** Medium. Reproduced (Python) and confirmed (the rest).

  **What is wrong.** The C, C++, Go, .NET, Kotlin, Swift, Dart, Ruby and wasm smoke tests in
  `kosherz/tests/` only check `Sunrise`, with a fixed -4 hour zone. Sunrise doesn't depend on the
  offset. In a Dart probe, offsets of -4 h, +4 h, 0 and +13 h all gave the same instant,
  1713866795540. A binding that passed the offset with the wrong sign or units would still pass.
  Only the Node test ([kosherz/tests/node/smoke.test.mjs:51](kosherz/tests/node/smoke.test.mjs#L51))
  checks a time that depends on the offset. It checks `TchilasZmanKidushLevana3Days` in Lakewood
  (40.09, -74.22), America/New_York, on 2024-11-04, which must be 08:44:50, and it also tests a
  callback that throws. `kosherz/examples/smoke.py`, which is the Python CI job, only prints, so
  wrong values pass.

  **Fix.** Port the Node Kiddush Levana case and the throwing-callback case to every language.
  Turn `smoke.py` into assertions.

- [x] **T5. Unit tests use different float math from the shipped crate.** Low. From reading.

  **What is wrong.** `#![cfg_attr(not(test), no_std)]` in [src/lib.rs:11](src/lib.rs#L11) means
  unit tests are built with std and call std's `f64::sin` and friends. Every real build, even
  on a desktop, goes through `core_maths`, which uses libm. The last bits can differ, so the unit
  tests don't exercise the float path users get. Tests that compare exact nanoseconds could then
  give different results on different platforms. Integration tests and doctests in `tests/`
  don't have this problem, because they link the normal library build.

  **Fix.** Make the crate `no_std` in tests as well (`#![no_std]` with `extern crate std` in test
  modules), or move numeric checks into `tests/`.

## 4. Bindings (`kosherz`)

- [x] **F1. Dart crashes the process when a `Zmanim` or `MyZmanim` with a time zone is
  garbage-collected instead of disposed.** High for Dart users. Confirmed with Dart 3.13.4.

  **Resolved.** The bindings no longer take a time zone callback. Callers pass a `LocalDay` record instead, so no Dart code runs when a `Zmanim` or `MyZmanim` is freed. The underlying WeaveFFI bug still affects any other callback interface.

  **What happens.** The Dart wrapper registers a `NativeFinalizer` that calls
  `kosherz_times_Zmanim_destroy`
  ([kosherz/generated/dart/lib/kosherz.dart:4482-4483](kosherz/generated/dart/lib/kosherz.dart#L4482)).
  Destroying the last reference drops the Rust `Arc<dyn TimeZoneOffset>`, which calls the
  callback's `free` entry. In Dart that entry is a `NativeCallable.isolateGroupBound` that runs
  Dart code ([kosherz.dart:926-929](kosherz/generated/dart/lib/kosherz.dart#L926)). Dart doesn't
  allow Dart code to run inside a GC finalizer, so the VM aborts.

  **How to see it.** This program exits with code 127 and
  `exceptions.cc: 765: expected: thread->long_jump_base() == nullptr` inside
  `_FfiCallback_forwardCallbackFree`. The same loop with `z.dispose()`, or with `null` as the time
  zone, runs cleanly.

  ```dart
  import 'package:kosherz/kosherz.dart';

  class Fixed implements TimeZoneOffset {
    @override
    int offsetSeconds(int unixSeconds) => -14400;
    @override
    int localToUnixSeconds(int y, int mo, int d, int h, int mi, int s) =>
        DateTime.utc(y, mo, d, h, mi, s).millisecondsSinceEpoch ~/ 1000 + 14400;
  }

  void main() async {
    final date = GregorianDate(year: 2024, month: 4, day: 23);
    final loc = seaLevelLocation(40.09, -74.22);
    for (var i = 0; i < 20000; i++) {
      Zmanim(loc, date, defaultConfig(), Fixed()).get(Zman.sunrise); // never disposed
    }
    await Future.delayed(Duration(milliseconds: 500));
  }
  ```

  **Why it matters.** The generated Dart README tells users the finalizer is a valid way to clean
  up ("call `dispose()` when you're done, or let the garbage collector's finalizer release it",
  [kosherz/generated/dart/README.md:24-25](kosherz/generated/dart/README.md#L24)). CI misses it
  because `kosherz/tests/dart/bin/smoke.dart` always disposes.

  **Fix.** This is a WeaveFFI generator bug, so report it there. The `free` path has to stay
  native, for example by posting a message with `Dart_PostCObject` instead of entering Dart.
  Until then, say "always call `dispose()`" in [kosherz/README.md](kosherz/README.md) (the Dart
  README is generated), and add a test like the program above.

- [x] **F2. The generated packages can't be published as they are.** Medium. Confirmed.

  **Resolved.** `kosherz/Cargo.toml` now has version 0.3.0, the author, the repository and the
  homepage, and every generated manifest picks them up. `kosherz/weaveffi.toml` names the packages.
  Node is `kosherz`, WebAssembly is `kosherz-wasm`, the Go module is
  `github.com/dickermoshe/kosher-rust/kosherz/generated/go`, and the Kotlin package and group are
  `io.github.dickermoshe.kosherz`. Every package directory has a copy of `LICENSE`, which the
  codegen test keeps in sync. `dart pub publish --dry-run` now only asks for a `CHANGELOG.md`.

  **What is wrong.**

  - Every manifest says version `0.0.0`, because [kosherz/Cargo.toml:3](kosherz/Cargo.toml#L3)
    does. The core crate is 0.3.0.
  - `kosherz/generated/node/package.json` and `kosherz/generated/wasm/package.json` are both
    named `kosherz`, so only one of them can be published to npm.
  - The Go module path is `kosherz` ([kosherz/generated/go/go.mod](kosherz/generated/go/go.mod)),
    which `go get` can't fetch.
  - The Kotlin group and Android namespace are `kosherz`, which Maven Central won't accept.
  - The gemspec's authors are `['kosherz']`.
  - No manifest has a repository or homepage URL.
  - `dart pub publish --dry-run` fails with "You must have a LICENSE file in the root directory."

  The license is LGPL-2.1-or-later everywhere, which is correct.

  **Fix.** Add the version, authors and repository to `kosherz/Cargo.toml`, or to a `[package]`
  table in `kosherz/weaveffi.toml` (WeaveFFI reads both). Set per-language names with the
  generator options (`[generators.go] module_path`, and the `name` options for node, wasm and
  kotlin). Copy `LICENSE` into each package when packaging.

- [x] **F3. The docs say .NET's `TimeZoneInfo` can answer the time-zone callback the way Java
  does.** Low. From reading (no .NET SDK was available).

  **Resolved.** The callback is gone. The kosherz README now says how to fill in a `LocalDay` in each language, including .NET's `IsInvalidTime` check and Go's open choice for a skipped midnight.

  **What is wrong.** The callback's docs at
  [kosherz/src/api.rs:1046-1048](kosherz/src/api.rs#L1046), copied into every binding, say
  "Java's `ZoneId`, .NET's `TimeZoneInfo`, Python's `zoneinfo` and JavaScript's `Temporal` can
  answer both". The second question is the instant of a local time, resolved the way Java does it.
  .NET's `TimeZoneInfo.ConvertTimeToUtc` throws for a skipped local time and picks the later
  instant for a repeated one, where Java picks the earlier. Every sun-based zman asks for local
  midnight. So in a zone whose midnight is skipped, such as America/Havana on its
  daylight-saving start, a .NET callback written the obvious way fails every zman that day.
  Go's `time.Date` doesn't define its result for skipped times either.

  **Fix.** Document how to handle `IsInvalidTime` and `IsAmbiguousTime` in .NET, and the Go
  caveat, or give a correct example for each.

- [x] **F4. The drift-test message gives a command that fails.** Low. Reproduced.

  **Resolved.** Both places in `codegen.rs` now say `cargo test -p kosherz codegen`.

  [kosherz/src/codegen.rs:7](kosherz/src/codegen.rs#L7) and
  [line 188](kosherz/src/codegen.rs#L188) say to run
  `UPDATE_GENERATED=1 cargo test -p kosher-rust-bindings codegen`. The package is called
  `kosherz`. The README has the right command.

- [x] **F5. The `kosherz` crate docs are out of date.** Low. Reproduced.

  **Resolved.** The crate docs list WebAssembly and describe the `LocalDay` record, which replaced
  the callback.

  [kosherz/src/lib.rs:1-3](kosherz/src/lib.rs#L1) lists the languages but leaves out
  WebAssembly. [Lines 10-12](kosherz/src/lib.rs#L10) describe the time-zone callback as only
  giving the UTC offset. Since db9e53b it also resolves local times.

- [x] **F6. A bad location with no time zone reports the wrong error first.** Low. Reproduced.

  **Resolved.** The coordinates are validated first, so longitude 200 with no local day reports
  `InvalidLongitude`. `parity.rs` checks it.

  [kosherz/src/convert/mod.rs:91](kosherz/src/convert/mod.rs#L91) checks "more than 150° and no
  time zone" before validating the coordinates. Longitude 200 with no time zone reports
  `TimeZoneRequired`. Add a time zone and you then get `InvalidLongitude`. Validate first.

- [x] **F7. The Tehillim and Pirkei Avos display helpers accept impossible ranges.** Low.
  Confirmed.

  **Resolved.** A range that ends before it starts is `InvalidUnit`, and a combined Pirkei Avos
  portion needs its second chapter after its first. Verses are still checked against 176, not
  against each psalm's length, and the docs say so.

  They only range-check each number. `tehillim_display(Psalms { 5, 2 })` gives "Tehillim 5-2",
  `PsalmVerses { 1, 170, 2 }` gives "Tehillim 1:170-2", and
  `pirkei_avos_display(Combined { 6, 1 })` gives "Pirkei Avos 6-1". Require start ≤ end and check
  verse counts per psalm, or document that the input is trusted.

- [x] **F8. A `Zman`'s number in the bindings changes when a zman is added.** Low. From reading.

  **Resolved.** The `Zman` docs in every binding now say to store `zman_java_name` rather than the
  number.

  A `Zman`'s integer value is its position in `Zman::ALL`, which is grouped by category. Adding a
  zman in the core (B4) renumbers every zman after it. An app that stored the integer would read
  the wrong zman after upgrading. The contract hash already catches a library and bindings that
  don't match each other, but not stored data. Document "store `java_name`, not the number", or
  number the variants append-only.

- [x] **F9. Python edge cases in WeaveFFI's generated code.** Low. Confirmed.

  **Won't fix.**

  An argument outside the `i32` range raises `struct.error` instead of a typed error. A callback
  that returns `2**32 + 3600` is silently truncated to 3600. Both come from the generator, so
  report them upstream.

- [ ] **F10. The Node addon doesn't build on Windows.** Medium. From reading.

  **What is wrong.** The generated
  [kosherz/generated/node/binding.gyp](kosherz/generated/node/binding.gyp) links `-lkosherz`,
  which on MSVC resolves to `kosherz.lib`. Because `kosherz` is built as both `cdylib` and
  `staticlib`, `kosherz.lib` is the static library, and the DLL's import library is
  `kosherz.dll.lib`. The file also has no `OS=='win'` branch to put `kosherz.dll` next to the
  `.node` file, since Windows has no rpath. The note at
  [kosherz/README.md:58](kosherz/README.md#L58) already says this.

  **Why the tests miss it.** CI builds the Node binding on Linux only (I3).

  **Fix.** This is in WeaveFFI's template, and an issue has been filed upstream (link it here).
  When a fixed release is out, update `weaveffi-cli`, regenerate, remove the README note, and
  add a Windows Node job to CI.

## 5. Docs and API

- [x] **D1. The limudim docs are wrong about when schedules return `None`.** Low. Confirmed.

  [src/limudim/mod.rs:4-6](src/limudim/mod.rs#L4) says `on()` returns `None` "on Shabbos for a
  weekly program". That isn't true.

  - `DafHashavuaBavli` on Shabbos 2024-05-04 returns Kesubos 86. Its own doc says the week runs
    "Sunday to Shabbos" ([src/limudim/daf_hashavua_bavli.rs:86](src/limudim/daf_hashavua_bavli.rs#L86)).
  - `PirkeiAvos` on a weekday (Tuesday 2024-05-07) returns the coming Shabbos's chapter,
    `Single(2)`.
  - `PirkeiAvos` returns `None` on the weekdays of a week whose Shabbos is skipped for Tisha B'Av,
    and between the last Shabbos and Rosh Hashana.

  The `PirkeiAvos` doc ([src/limudim/pirkei_avos.rs:44](src/limudim/pirkei_avos.rs#L44)) says "a
  chapter each Shabbos afternoon" and doesn't mention any of the weekday behaviour. Rewrite both
  to say what each schedule returns on a weekday and on Shabbos.

- [x] **D2. The `Parsha` numbering doc is off by one.** Low. Confirmed by reading both sides.

  [src/calendar/parsha.rs:10-11](src/calendar/parsha.rs#L10) says "The numbering matches
  KosherJava's `JewishCalendar.Parsha`." Java's enum is named `Parshah`, and it starts with
  `NONE = 0`, `BERESHIS = 1`. Rust starts with `Bereshis = 0`. The interop harness subtracts 1
  from the Java ordinal to make them line up. The `u8` conversions are public, so anyone porting
  Java ordinals gets the wrong parsha. Fix the doc, or renumber from 1.

- [x] **D3. "Reproduced to the second" overstates MyZmanim.** Low. Confirmed.

  [README.md:114-115](README.md#L114), [src/lib.rs:10](src/lib.rs#L10) and
  [src/myzmanim/mod.rs:1](src/myzmanim/mod.rs#L1) say the times are reproduced "to the second".
  Measured against the site data, 92% are within 1 second, 99.8% within 2 seconds, and the worst
  is 13.6 seconds. The module body already says this
  ([src/myzmanim/mod.rs:9-11](src/myzmanim/mod.rs#L9)). Make the summary lines match it.

- [x] **D4. The "more than 150° needs a time zone" rule has an unstated assumption.** Low.
  Confirmed.

  The rule assumes the time zone is less than about 10 hours from local solar time on the far
  side of the date line. At longitude -149 with a +14 hours zone, sunrise is
  2024-05-31T16:19Z with the zone and 2024-06-01T16:19Z without it, with no error. No real zone
  does this today. A sentence in the docs on the time-zone section would cover it.

- [x] **D5. `Zmanim` and `MyZmanim` take candle lighting differently.** Low. From reading.

  `Zmanim` takes it as `Config::candle_lighting_offset`, a `SignedDuration`. `MyZmanim` takes it
  as `f64` minutes through the `candle_lighting` builder
  ([src/myzmanim/myzman.rs:286](src/myzmanim/myzman.rs#L286)), which also lets NaN through (C6).
  `MyZmanim` derives `PartialEq` and `Zmanim` doesn't. Pick one shape for both.

- [x] **D6. Custom time zones are second-class.** Low. Confirmed.

  - `get_zoned` exists only on `Zmanim<TimeZone>` and `MyZmanim<TimeZone>`
    ([src/zmanim/calculator.rs:76](src/zmanim/calculator.rs#L76)). With a custom `TimeZoneLike`,
    or with `&TimeZone`, there is no way to get a local time. A generic version could build the
    `Zoned` from `TimeZone::fixed(tz.offset_at(timestamp))`.
  - `Zmanim::with_custom_timezone(location, date, None, config)` doesn't compile (E0283), because
    nothing says what `Tz` is. Users have to write `None::<MyZone>`.
  - `TimeZoneLike::local_to_timestamp` has no default, though one could be derived from
    `offset_at`. Every implementor has to get Java's gap and overlap rules right on their own. A
    custom zone that returns `None` makes every zman return `Internal` ("please report"), when the
    cause is the caller's time zone.

- [x] **D7. The closed sets don't all have the same shape.** Low. From reading.

  `Zman` and `MyZman` have `ALL` and `name()`. `SpecialDay`, `Parsha` and `Tractate` have neither,
  so an app can't list them for a settings screen. `Tractate` already has a crate-private
  `TRACTATES` array in Mishnah order, which Mishna Yomis uses
  ([src/limudim/units.rs:276-278](src/limudim/units.rs#L276)). It could become `Tractate::ALL`,
  and its `#[allow(dead_code)]` looks unnecessary.
  None of the enums implement `FromStr`. `SpecialDay`'s methods take `&self`
  ([src/calendar/special_day.rs:475](src/calendar/special_day.rs#L475) and lines 490, 499 and 513), while
  `HebrewMonth::hebrew`, `Parsha::hebrew` and `Tractate::hebrew` take `self`.

- [x] **D8. `Molad` only gives a `Timestamp`.** Low. From reading.

  [src/calendar/molad.rs:16-25](src/calendar/molad.rs#L16) exposes only `timestamp()`. The
  traditional announcement ("Friday, 5:49 and 3 *chalakim*") needs the weekday, hours, minutes
  and *chalakim*, which Java's `getMolad` gives. Right now the *chalakim* pass through an `f64`
  and can't be recovered. Consider accessors for them.

- [x] **D9. Small inconsistencies in the limudim and holiday types.** Low. From reading.

  - `Tractate` sorts in Mishnah order (Tamid, Midos, Kinnim), so the derived ordering of `Daf`
    and `Amud` doesn't follow the Bavli Daf Yomi order (Kinnim, Tamid, Midos).
  - `Mishna` and `MishnaPair` don't implement `Ord`, while `Daf` and `Amud` do.
  - `Mishna::chapter` is a `usize` ([src/limudim/units.rs:532](src/limudim/units.rs#L532)),
    while `Mishna::mishna` and `Daf::daf` are `u16`.
  - The holiday variant names mix styles. There is `YomHaShoah`, but also `YomHazikaron` and
    `YomHaatzmaut`. `MacharHachodesh` displays as "Machar Chodesh".
  - Modern Israeli holidays are returned for any year, for example Yom HaAtzmaut in 5000. This
    matches Java, but the docs could say so.

- [x] **D10. Many public docs break your own docs rule against mid-sentence colons.** Low.
  Reproduced.

  About 50 public doc lines outside `src/astronomy` use a colon mid-sentence, which your docs
  rule asks to avoid. [CONTRIBUTING.md](CONTRIBUTING.md) doesn't mention the rule yet, so
  consider adding it to "Writing docs" too. Examples are
  `ZmanimError::NotApplicable` ([src/zmanim/error.rs:13](src/zmanim/error.rs#L13)), `Config`
  ([src/zmanim/config.rs:60](src/zmanim/config.rs#L60)), `shaah_zmanis`
  ([src/zmanim/calculator.rs:153](src/zmanim/calculator.rs#L153)), `Location`
  ([src/location.rs:72](src/location.rs#L72)) and `HebrewDate::new`
  ([src/calendar/date.rs:59](src/calendar/date.rs#L59)). This command lists them.

  ```bash
  grep -rnE "^\s*(///|//!).*[a-z*\`)]: [a-z\`*\[]" src --include=*.rs | grep -v "src/astronomy\|tests.rs"
  ```

## 6. CI and tooling

- [x] **I1. `rust-toolchain.toml` downloads 12 cross-compilation targets for everyone.** Low.
  From reading.

  **Resolved.** The toolchain file keeps the channel and components and no longer lists targets. CI has no
  release jobs that cross-compile, and the kosherz README says to add a target with `rustup target
  add` before building for another platform.

  [rust-toolchain.toml](rust-toolchain.toml) pins 1.99.1 and lists Android, iOS, Windows, Linux
  and macOS targets. rustup installs all of them on the first build, for every contributor and
  every CI job that reads the file. Only release builds of the bindings need them. Keep the
  channel pin, and add targets in the jobs that cross-compile, for example with the setup action's
  `target` input.

- [x] **I2. CI never lints or documents with all features.** Low. Reproduced locally that both
  pass today.

  **Resolved.** CI runs clippy and `cargo doc` a second time with `--all-features`. Both pass locally.

  The clippy job ([.github/workflows/tests.yaml:37](.github/workflows/tests.yaml#L37)) runs
  without `--all-features`, and the docs job ([line 47](.github/workflows/tests.yaml#L47)) uses
  `--no-default-features`. docs.rs builds with all features, including `defmt`, so a broken doc
  link that only shows up with `defmt` wouldn't fail CI. `cargo hack check --feature-powerset`
  does compile every combination. Add `--all-features` runs of both.

- [x] **I3. The bindings only run on Linux.** Low. From reading.

  **Resolved.** CI has a Swift job on macOS and a .NET job on Windows, and each builds the library on its own
  runner. Neither could be run locally.

  Every job in [.github/workflows/bindings.yaml](.github/workflows/bindings.yaml) runs on
  `ubuntu-latest`. Library loading, paths and calling conventions differ on Windows and macOS,
  which are where the Swift, .NET and Kotlin bindings are most used. Add at least a macOS Swift
  job and a Windows .NET job.

- [x] **I4. The MyZmanim site test takes about 100 seconds in a debug build.** Low. Reproduced.

  **Resolved.** `[profile.test]` sets `opt-level = 2`, and the site test now takes about 27 seconds.

  `tests/myzmanim_site.rs` compares 944,603 cells. It is the slowest part of
  `cargo test -p kosher-rust`, which [CONTRIBUTING.md](CONTRIBUTING.md) calls "what you run while
  working". Adding this to `Cargo.toml` should cut it a lot (not measured).

  ```toml
  [profile.test]
  opt-level = 2
  ```

- [x] **I5. `--all-features` builds print a future-incompatibility warning.** Low. Reproduced.

  **Resolved.** `defmt` 1.2.0 no longer depends on `proc-macro-error2`, and the lockfile now has it.
  `Cargo.toml` keeps `version = "1"`, as the defmt FAQ for library authors recommends, and
  `--future-incompat-report` finds nothing. The `no_std` CI job also builds with `defmt` for
  Cortex-M.

  `defmt-macros` depends on `proc-macro-error2` 2.0.1, which Rust will reject in a future
  version. It comes from `defmt` and only appears with the `defmt` feature. Update `defmt` when a
  fixed release is out.

## 7. Housekeeping

- [x] **H1. `PLAN.md` is deleted in the working tree but not in git.** It was readable at the start
  of the review and showed as deleted (`D PLAN.md`) partway through. None of the reviewers
  touched repo files. If that wasn't on purpose, `git checkout -- PLAN.md` restores it. If it was,
  commit the deletion and remove `/PLAN.md` from `exclude` in `Cargo.toml`.

  **Resolved.** The deletion is committed and `/PLAN.md` is out of `exclude`.

- [x] **H2. Remove or exclude this file before publishing.** `REVIEW.md` sits in the crate root and
  isn't in `exclude` in `Cargo.toml`, so `cargo package` would ship it.

  **Resolved.** This file will be deleted before release, so it isn't added to `exclude`.
