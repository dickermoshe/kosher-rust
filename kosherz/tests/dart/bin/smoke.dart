// Calls the generated Dart bindings. Build the library with `cargo build -p kosherz`, then
// from tests/dart:
//
//     dart pub get
//     KOSHERZ_LIBRARY=$PWD/../../../target/debug/libkosherz.so dart run bin/smoke.dart
import 'dart:io';

import 'package:kosherz/kosherz.dart';

/// [date] at UTC plus [hours] all year: its two local midnights and the offsets at them. Dart has
/// no time zone database of its own, so the tests use fixed offsets.
LocalDay fixedDay(int hours, GregorianDate date) {
  final start = DateTime.utc(date.year, date.month, date.day).millisecondsSinceEpoch - hours * 3600 * 1000;
  return LocalDay(
    startMs: start,
    startOffsetSeconds: hours * 3600,
    endMs: start + 24 * 3600 * 1000,
    endOffsetSeconds: hours * 3600,
  );
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
  final lakewood = seaLevelLocation(40.09, -74.22);

  check('sunrise in Lakewood on 2024-04-23 is 06:06:35 EDT', () {
    final day = Zmanim(lakewood, date, defaultConfig(), fixedDay(-4, date));
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
      Zmanim(seaLevelLocation(91, 0), date, defaultConfig(), fixedDay(-4, date)).dispose();
    } on InvalidLatitudeException {
      return;
    }
    throw 'no exception';
  });

  // Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
  // 3 December 2024, and with UTC+4 it does not.
  final december = GregorianDate(year: 2024, month: 12, day: 3);
  check('kiddush levana in Lakewood on 2024-12-03 needs the right offset', () {
    final day = Zmanim(lakewood, december, defaultConfig(), fixedDay(-4, december));
    final wrongSign = Zmanim(lakewood, december, defaultConfig(), fixedDay(4, december));
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

  check('kiddush levana without a local day throws TimeZoneRequiredException', () {
    final day = Zmanim(lakewood, december, defaultConfig(), null);
    try {
      day.get(Zman.tchilasZmanKidushLevana3Days);
    } on TimeZoneRequiredException {
      return;
    } finally {
      day.dispose();
    }
    throw 'no exception';
  });

  check('a local day that ends before it starts throws InvalidLocalDayException', () {
    final backwards = LocalDay(startMs: 1, startOffsetSeconds: 0, endMs: 0, endOffsetSeconds: 0);
    try {
      Zmanim(lakewood, date, defaultConfig(), backwards).dispose();
    } on InvalidLocalDayException {
      return;
    }
    throw 'no exception';
  });

  if (failures > 0) {
    print('$failures check(s) failed');
    exit(1);
  }
}
