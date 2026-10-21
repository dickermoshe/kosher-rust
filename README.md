# kosher-rust

Rust port of [KosherJava](https://github.com/KosherJava/zmanim): Jewish holidays, *zmanim* for your location, and daily learning schedules like Daf Yomi and Pirkei Avos.

## Documentation

Examples and API reference: **[docs.rs/kosher-rust](https://docs.rs/kosher-rust)**.

## Installation

```bash
cargo add kosher-rust jiff
```

## Solar algorithms

Every astronomical zman is computed by one of the three solar position algorithms that
KosherJava ships, selected through `CalculatorConfig::astronomical_calculator`:

- `AstronomicalCalculator::Noaa` (default) — the NOAA algorithm, as in `NOAACalculator`
- `AstronomicalCalculator::Meeus { apply_delta_t }` — Jean Meeus's higher-accuracy VSOP87 algorithm, as in `MeeusCalculator`
- `AstronomicalCalculator::Spa { apply_delta_t }` — the NREL Solar Position Algorithm, as in `SPACalculator`

```rust
use kosher_rust::zmanim::prelude::*;

let config = CalculatorConfig {
    astronomical_calculator: AstronomicalCalculator::Meeus { apply_delta_t: true },
    ..Default::default()
};
```

## Features

- **`alloc`** (default) — Zman preset descriptions; without it the crate stays `no_std` and calculation APIs are unchanged
- **`defmt`** — `defmt::Format` on calculator, config, location, and error types

## License

Licensed under the GNU Lesser General Public License v2.1 — see [LICENSE](LICENSE).

## Acknowledgments

This project is based on the [KosherJava](https://github.com/KosherJava/zmanim) project by Eliyahu Hershfeld.

## Maintainence

To update the project to the latest version of KosherJava, run the following command:
```bash
git subtree pull  --prefix=third-party/kosher-java  https://github.com/KosherJava/zmanim <tag-or-branch>  --squash
```

Then add the zmanim definitions to `/tools/dsl.py` and run `tools/generate-rust.py` to update the Rust code.
In some instances, you may need to add features to the DSL to support the new zmanim.
Updates to preset documentation should be made in `/tools/dsl.py` as well.

We are currently tracking commit [cb2b183](https://github.com/KosherJava/zmanim/commit/cb2b1835ae875c5fb2fbd3b35ae18f523ab93bfe) of KosherJava (post-3.0.0 master).

# TODOs

There are some parts of KosherJava which have not been ported to Rust yet.  
These may happen someday or never. If you need them, please open an issue and we'll see what we can do.

- `JewishCalendar.getTekufa`
- `JewishCalendar.getTekufaAsInstant`
- `ZmanimFormatter`
- `TefilaRules`
- `HebrewDateFormatter`
