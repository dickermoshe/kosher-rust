/// Jewish calendar, halachic times (*zmanim*) and daily learning schedules, from the kosher-rust
/// crate.
///
/// Call [initKosherRust] once before anything else.
///
/// ```dart
/// await initKosherRust();
/// final location = Location(latitude: 31.778, longitude: 35.235);
/// final zmanim = ZmanimCalculator(
///   location: location,
///   date: (year: 2024, month: 4, day: 23),
///   offset: TimeZone.local.offset,
/// );
/// print(await zmanim.time(zman: Zman.sunset));
/// ```
library;

import 'src/loader.dart';
import 'src/rust/frb_generated.dart';

export 'src/gregorian_date.dart';
export 'src/rust/api/calendar.dart';
export 'src/rust/api/limudim.dart';
export 'src/rust/api/zmanim.dart';
export 'src/rust/third_party/kosher_rust.dart' hide Zone;
export 'src/rust/third_party/kosher_rust/calendar.dart';
export 'src/rust/third_party/kosher_rust/limudim.dart';
export 'src/rust/third_party/kosher_rust/myzmanim.dart';
export 'src/rust/third_party/kosher_rust/zmanim.dart';
export 'src/time_zone.dart';

/// Loads the native library. Call it once, and wait for it, before using anything else.
Future<void> initKosherRust() =>
    RustLib.init(externalLibrary: bundledLibrary());

/// Releases the native library. A command-line program calls it when it is done, since the open
/// library keeps the program running.
void disposeKosherRust() => RustLib.dispose();
