> This crate is part of the [Rust Zmanim Project](https://github.com/dickermoshe/rust-zmanim-project).

# Zmanim Calculator

A Rust library for calculating zmanim (Jewish halachic times) with
KosherJava-style naming and behavior. The crate supports `no_std`; the default
feature set enables `alloc` for preset descriptions.

[![Crates.io](https://img.shields.io/crates/v/zmanim-calculator.svg)](https://crates.io/crates/zmanim-calculator)
[![Documentation](https://docs.rs/zmanim-calculator/badge.svg)](https://docs.rs/zmanim-calculator)

## Installation

```bash
cargo add zmanim-calculator jiff
```

For a smaller `no_std` build without generated preset descriptions:

```toml
zmanim-calculator = { version = "0.6", default-features = false }
```

## Usage

```rust
use jiff::{civil::Date, tz::TimeZone};
use zmanim_calculator::{
    prelude::{CalculatorConfig, Location, ZmanimCalculator},
    presets::{SEA_LEVEL_SUNRISE, TZAIS_72_MINUTES},
};

fn main() {
    let location = Location::new(
        40.7128,
        -74.0060,
        10.0,
        Some(TimeZone::get("America/New_York").expect("valid timezone")),
    )
    .expect("valid location");

    let date = Date::new(2026, 3, 1).expect("valid date");
    let mut calculator =
        ZmanimCalculator::new(location, date, CalculatorConfig::default()).expect("calculator");

    let sunrise = calculator.calculate(SEA_LEVEL_SUNRISE).expect("sunrise");
    let tzais = calculator.calculate(TZAIS_72_MINUTES).expect("tzais");

    println!("Sunrise (UTC): {sunrise}");
    println!("Tzais 72 (UTC): {tzais}");
}
```

`calculate` returns a UTC `jiff::Timestamp`. It takes `&mut self` so repeated
calculations can reuse internal state. If that does not fit your borrow pattern,
clone the calculator and use each clone independently.

If you omit a timezone, calculations near the anti-meridian (`|longitude| > 150`)
will fail. Kiddush Levana and Molad calculations require a timezone as well.

## Presets

Most users should calculate one of the ready-made constants in
`zmanim_calculator::presets`, such as `ALOS_72_MINUTES`, `SOF_ZMAN_SHMA_GRA`,
or `TZAIS_72_MINUTES`.

The preset module also exposes:

- `ALL`: every non-deprecated generated preset.
- `ZmanPreset::name()`: a short display name.
- `ZmanPreset::description(&calculator)`: a user-facing description when the
  `alloc` feature is enabled.

Descriptions can depend on calculator configuration, such as elevation mode,
candle-lighting offset, and Ateret Torah offset.

## Feature Flags

- `alloc` (default): enables generated preset descriptions and allocation-backed
  strings.
- `defmt`: enables `defmt` formatting for embedded targets.

Disable default features if you need the smallest `no_std` surface and do not
need preset descriptions.

## Generated Code

The public preset constants are generated from the DSL in `tools/dsl.py` and
written to `src/presets_gen.rs`. Do not edit `presets_gen.rs` by hand.

To regenerate presets:

```bash
cd crates/zmanim-calculator/tools
uv run python generate-rust.py
```

See [tools/README.md](tools/README.md) for the current generator workflow and
DSL details.

## Testing

This crate has randomized parity tests against the bundled KosherJava source.
First build the Java jar:

```bash
cd crates/zmanim-calculator/java
mvn package
```

Then run the Rust tests from `crates/zmanim-calculator`:

```bash
cargo test
```

## Updating KosherJava

From the repository root:

```bash
git subtree pull --prefix=crates/zmanim-calculator/java https://github.com/KosherJava/zmanim master --squash
```

After pulling upstream changes, review the Java implementation, update
`tools/dsl.py` where needed, regenerate presets, and run parity tests.

## Differences From KosherJava

- `Sof Zman Achilas Chametz` and `Sof Zman Biur Chametz` are returned for any
  date, not only Erev Pesach.
- Chatzos uses astronomical chatzos, not the midpoint of sunrise to sunset.


## License

Licensed under LGPL-2.1. See [LICENSE](LICENSE) for details.
