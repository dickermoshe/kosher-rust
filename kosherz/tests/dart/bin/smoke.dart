// Calls the generated Dart bindings. Build the library with `cargo build -p kosherz`, then
// from tests/dart:
//
//     dart pub get
//     KOSHERZ_LIBRARY=$PWD/../../../target/debug/libkosherz.so dart run bin/smoke.dart
import 'dart:io';

import 'package:kosherz/kosherz.dart';

/// UTC plus [hours], every day of the year.
class Fixed implements TimeZoneOffset {
  Fixed(this.hours);

  final int hours;

  @override
  int offsetSeconds(int unixSeconds) => hours * 3600;

  @override
  int localToUnixSeconds(int year, int month, int day, int hour, int minute, int second) =>
      DateTime.utc(year, month, day, hour, minute, second).millisecondsSinceEpoch ~/ 1000 - hours * 3600;
}

/// Eastern daylight time, every day of the year.
final edt = Fixed(-4);

/// A time zone whose callbacks fail.
class Broken implements TimeZoneOffset {
  @override
  int offsetSeconds(int unixSeconds) => throw StateError('no time zone data');

  @override
  int localToUnixSeconds(int year, int month, int day, int hour, int minute, int second) =>
      throw StateError('no time zone data');
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
    final day = Zmanim(seaLevelLocation(40.09, -74.22), date, defaultConfig(), edt);
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
      Zmanim(seaLevelLocation(91, 0), date, defaultConfig(), edt).dispose();
    } on InvalidLatitudeException {
      return;
    }
    throw 'no exception';
  });

  // Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
  // 3 December 2024, and with UTC+4 it does not.
  final december = GregorianDate(year: 2024, month: 12, day: 3);
  check('kiddush levana in Lakewood on 2024-12-03 needs the right offset', () {
    final day = Zmanim(seaLevelLocation(40.09, -74.22), december, defaultConfig(), edt);
    final wrongSign = Zmanim(seaLevelLocation(40.09, -74.22), december, defaultConfig(), Fixed(4));
    try {
      final ms = day.get(Zman.tchilasZmanKidushLevana3Days);
      if (ms != 1733279333504) throw 'got $ms, expected 1733279333504';
      try {
        wrongSign.get(Zman.tchilasZmanKidushLevana3Days);
        throw 'UTC+4 gave a time';
      } on NotApplicableException {
        // expected
      }
    } finally {
      day.dispose();
      wrongSign.dispose();
    }
  });

  check('kiddush levana without a time zone throws TimeZoneRequiredException', () {
    final day = Zmanim(seaLevelLocation(40.09, -74.22), december, defaultConfig(), null);
    try {
      day.get(Zman.tchilasZmanKidushLevana3Days);
    } on TimeZoneRequiredException {
      return;
    } finally {
      day.dispose();
    }
    throw 'no exception';
  });

  check('a throwing time zone callback throws TimeZoneCallbackFailedException', () {
    final day = Zmanim(seaLevelLocation(40.09, -74.22), date, defaultConfig(), Broken());
    try {
      day.get(Zman.sunrise);
    } on TimeZoneCallbackFailedException {
      return;
    } finally {
      day.dispose();
    }
    throw 'no exception';
  });

  if (failures > 0) {
    print('$failures check(s) failed');
    exit(1);
  }
}
