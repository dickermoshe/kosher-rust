// Calls the generated Dart bindings. Build the library with `cargo build -p kosherz`, then
// from tests/dart:
//
//     dart pub get
//     KOSHERZ_LIBRARY=$PWD/../../../target/debug/libkosherz.so dart run bin/smoke.dart
import 'dart:io';

import 'package:kosherz/kosherz.dart';

/// Eastern daylight time, every day of the year.
class Edt implements TimeZoneOffset {
  @override
  int offsetSeconds(int unixSeconds) => -4 * 3600;

  @override
  int localToUnixSeconds(int year, int month, int day, int hour, int minute, int second) =>
      DateTime.utc(year, month, day, hour, minute, second).millisecondsSinceEpoch ~/ 1000 + 4 * 3600;
}

var failures = 0;

/// Runs [body], which throws a message on failure, and reports the result.
void check(String name, void Function() body) {
  try {
    body();
    print('ok   $name');
  } catch (e) {
    print('FAIL $name: $e');
    failures++;
  }
}

void main() {
  final date = GregorianDate(year: 2024, month: 4, day: 23);

  check('sunrise in Lakewood on 2024-04-23 is 06:06:35 EDT', () {
    final day = Zmanim(seaLevelLocation(40.09, -74.22), date, defaultConfig(), Edt());
    try {
      final seconds = day.get(Zman.sunrise) ~/ 1000;
      if (seconds != 1713866795) throw 'got $seconds, expected 1713866795';
    } finally {
      day.dispose();
    }
  });

  check('daf yomi on 2024-04-23 is Bava Metzia 55', () {
    final hebrew = HebrewDate.fromGregorian(date);
    try {
      final display = dafYomiBavli(hebrew)?.display;
      if (display != 'Bava Metzia 55') throw 'got $display';
    } finally {
      hebrew.dispose();
    }
  });

  check('latitude 91 throws InvalidLatitudeException', () {
    try {
      Zmanim(seaLevelLocation(91, 0), date, defaultConfig(), Edt()).dispose();
    } on InvalidLatitudeException {
      return;
    }
    throw 'no exception';
  });

  if (failures > 0) {
    print('$failures check(s) failed');
    exit(1);
  }
}
