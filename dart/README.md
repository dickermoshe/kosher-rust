# kosher_rust

Dart bindings for the [kosher-rust](https://github.com/dickermoshe/kosher-rust) crate. They cover the
Jewish calendar, halachic times (*zmanim*) and daily learning schedules, and they work in plain Dart
and in Flutter.

```dart
import 'package:kosher_rust/kosher_rust.dart';

Future<void> main() async {
  await initKosherRust();

  const date = (year: 2024, month: 4, day: 23);
  final today = HebrewDate.fromGregorian(date: date)!;
  print(today); // 15 Nisan 5784
  print(dafYomiBavli(date: today)); // Bava Metzia 55

  final jerusalem = Location(latitude: 31.778, longitude: 35.235);
  final zmanim = ZmanimCalculator(
    location: jerusalem,
    date: date,
    offset: TimeZone.local.offset,
  );
  print(await zmanim.time(zman: Zman.sunset)); // seconds since the Unix epoch

  disposeKosherRust();
}
```

Call `initKosherRust` once, and wait for it, before using anything else. A command-line program
calls `disposeKosherRust` when it is done, because the open library keeps the program running. A
Flutter app doesn't need to.

## Dates, times and time zones

Dates are `GregorianDate` records, such as `(year: 2024, month: 4, day: 23)`. Times come back as
whole seconds since the Unix epoch, and instants passed in are given the same way. Weekdays are
numbers, 1 for Monday through 7 for Sunday. Lengths of time are Dart `Duration`s.

The calculations read the time zone through `TimeZone`, which you implement with whatever time zone
library you use. `TimeZone.local` is the device's zone, `TimeZone.utc` is UTC, and `FixedTimeZone`
never changes. Pass the zone's `offset` method when you build `ZmanimCalculator` or
`MyZmanimCalculator`, or use their `withoutZone` constructors. Getting a time is asynchronous,
because it reads the time zone by calling back into Dart. Everything else is synchronous.

## How the API differs from Rust

Most types and methods have the same names as in kosher-rust. These differ.

| Rust | Dart |
|---|---|
| `Zmanim`, `MyZmanim` | `ZmanimCalculator`, `MyZmanimCalculator`, with `time` in place of `get` |
| `zmanim::Config` | `ZmanimConfig`, starting from `ZmanimConfig.defaults()` and changed with `copyWith` |
| `ZmanimError` | `ZmanimException`, thrown by `time` |
| `HebrewDate::new`, `HebrewYear::new` | `HebrewDate.of`, `HebrewYear.of`, which return null where Rust returns `None` |
| `HebrewDate::try_from(date)` | `HebrewDate.fromGregorian` |
| `HebrewYear::get` | `HebrewYear.value` |
| `name()`, `hebrew()` | `displayName`, `hebrewName` |
| `Zman::description` | `Zman.describe` |
| `DafYomiBavli.on(date)` and the other schedules | `dafYomiBavli(date: ...)` and the other functions |
| `Daf<BavliTractate>`, `Daf<YerushalmiTractate>` | `BavliDaf`, `YerushalmiDaf` |

Enums whose variants hold data, such as `SpecialDay`, `AstronomicalCalculator` and `TehillimUnit`,
are sealed classes, so they compare with `==` and work in a `switch`. The portions of learning
print as they do in Rust, for example "Nazir 5:5-6".

`Location` throws a `LocationError` for an invalid coordinate or elevation. A `GregorianDate` that
does not exist, such as 30 February, throws an exception.

## Running

Building needs [rustup](https://rustup.rs). The toolchain and its targets are in
`rust/rust-toolchain.toml`. The first build compiles the Rust crate in release mode and takes a few
minutes. Later builds reuse it.

```bash
dart test
```

```bash
dart run example/kosher_rust_example.dart
```

The example prints today's Hebrew date, learning and *zmanim* for Jerusalem, or for the latitude
and longitude you pass, in this computer's time zone.

## Project structure

* `rust` is the Rust crate the bindings are generated from. It depends on kosher-rust in the
  repository root, with the `__dart_enabled` feature, which makes `SpecialDay` and
  `AstronomicalCalculator` exhaustive so they can cross to Dart as sealed classes.
* `hook` is flutter_rust_bridge's build hook. It compiles the crate and bundles it with the package.
* `lib` has the Dart API and the generated bridge code. `lib/src/loader.dart` finds the bundled
  library, which flutter_rust_bridge's own loader does not do outside Flutter.
* `example` is the command-line program above.
* `test` has the tests.

## Changing the bindings

flutter_rust_bridge scans kosher-rust itself, so its enums and plain structs, with their
documentation, cross to Dart as they are. Two folders in `rust/src` adjust that.

* `third_party/kosher_rust` holds attributes for kosher-rust's own items, such as one to leave a
  method out or to make a type opaque. Each module mirrors the kosher-rust module the items are in.
* `api` has what Dart can't call as it is. That covers methods that take `self`, names returned as
  `&'static str`, the calculators that are generic over their time zone, the config and error
  types, and the conversions for jiff's types.

After changing either, or kosher-rust's public API, regenerate the Dart code from this directory.

```bash
flutter_rust_bridge_codegen generate
```

The code generator's version must match the `flutter_rust_bridge` versions in `pubspec.yaml` and
`rust/Cargo.toml`, which is 2.13.0.
