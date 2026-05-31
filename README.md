# kosher-rust

Rust port of [KosherJava](https://github.com/KosherJava/zmanim): Jewish holidays, *zmanim* for your location, and daily learning schedules like Daf Yomi and Pirkei Avos.

## Modules

| Module | Purpose |
|--------|---------|
| `calendar` | Hebrew dates, holidays, *parshiyot*, month constants, and calendar traits |
| `zmanim` | Sunrise, candle lighting, *alos*, *tzeis*, and other halachic times |
| `limudim` | Daf Yomi, Mishna Yomis, Tehillim, Pirkei Avos, and related daily units |

Each module has its own prelude (`calendar::prelude`, `zmanim::prelude`, `limudim::prelude`). Use `kosher_rust::prelude` when an application needs more than one area.

## Installation

```bash
cargo add kosher-rust jiff
```
## Documentation

Examples and API reference: **[docs.rs/kosher-rust](https://docs.rs/kosher-rust)** (published with the crates.io release).

## Features

| Feature | Default | Effect |
|---------|---------|--------|
| `alloc` | yes | Zman preset descriptions; without it the crate stays `no_std` and calculation APIs are unchanged |
| `defmt` | no | `defmt::Format` on calculator, config, location, and error types |

## License

Licensed under the GNU Lesser General Public License v2.1 — see [LICENSE](LICENSE).

This project ports behavior from [KosherJava](https://github.com/KosherJava/zmanim); consult that project for the original Java API and halachic documentation of individual zmanim.
