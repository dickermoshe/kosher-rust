> This crate is part of the [Rust Zmanim Project](https://github.com/dickermoshe/rust-zmanim-project).

# Zmanim Calculator

A Rust library for calculating halachic zmanim (times), following KosherJava naming and behavior. Supports `no_std` environments.

[![Crates.io](https://img.shields.io/crates/v/zmanim-calculator.svg)](https://crates.io/crates/zmanim-calculator)
[![Documentation](https://docs.rs/zmanim-calculator/badge.svg)](https://docs.rs/zmanim-calculator)

## Installation

```bash
cargo add zmanim-calculator jiff
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
    let mut calc =
        ZmanimCalculator::new(location, date, CalculatorConfig::default()).expect("calculator");

    let sunrise = calc.calculate(SEA_LEVEL_SUNRISE).expect("sunrise");
    let tzais = calc.calculate(TZAIS_72_MINUTES).expect("tzais");

    println!("Sunrise (UTC): {sunrise}");
    println!("Tzais 72 (UTC): {tzais}");
}
```

If you omit a timezone, calculations near the anti-meridian (`|longitude| > 150`) will fail. Kiddush Levana and Molad calculations require a timezone as well.

## Feature Flags

- **`defmt`** — Enables `defmt` formatting/logging for embedded targets

## Compatibility

The API aims to follow KosherJava naming and behavior where possible. For background and broader algorithm documentation, see the [KosherJava documentation](https://kosherjava.com/zmanim-project/how-to-use-the-zmanim-api/).

## Preset generation

Most public zman constants (`ALOS_72_MINUTES`, `SOF_ZMAN_SHMA_GRA`, and ~168 others) are **not** written by hand. They are produced from vendored KosherJava sources:

See **[tools/README.md](tools/README.md)** for the full pipeline, regeneration commands, and notes on porting the same approach to another language.

## Testing

This crate uses randomized parity tests against the bundled Java implementation. A small number of tolerance and policy exceptions are allowed where the Rust SPA calculations intentionally differ from NOAA while remaining accurate enough for supported use cases.

To run the full test suite, first build the Java jar:

```bash
cd java
mvn package
```

Then run the Rust tests. Cargo regenerates the JNI bindings via `build.rs`:

```bash
cargo test
```


## Updating from KosherJava

To pull in the latest changes from KosherJava, run:
```bash
git subtree pull --prefix=crates/zmanim-calculator/java https://github.com/KosherJava/zmanim master  --squash
```

Then follow the workflow in the [tools/README.md](tools/README.md) to regenerate the zman presets.

## Differences from KosherJava

- The `Sof Zman Achilas Chametz` and `Sof Zman Biur Chametz` zmanim are returned for any date, not just Erev Pesach.
- Chatzos always uses astronomical chatzos, not the midpoint of sunrise to sunset.

## License

Licensed under LGPL-2.1. See [LICENSE](LICENSE) for details.
