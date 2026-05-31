# kosher-rust

Rust port of [KosherJava](https://github.com/KosherJava/zmanim): Jewish holidays, *zmanim* for your location, and daily learning schedules like Daf Yomi and Pirkei Avos.

## Documentation

Examples and API reference: **[docs.rs/kosher-rust](https://docs.rs/kosher-rust)**.

## Installation

```bash
cargo add kosher-rust jiff
```

## Features

- **`alloc`** (default) — Zman preset descriptions; without it the crate stays `no_std` and calculation APIs are unchanged
- **`defmt`** — `defmt::Format` on calculator, config, location, and error types

## License

Licensed under the GNU Lesser General Public License v2.1 — see [LICENSE](LICENSE).
