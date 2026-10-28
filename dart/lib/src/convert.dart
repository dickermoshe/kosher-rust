// Conversions between Dart's types and the ones the Rust side encodes them as.

import 'gregorian_date.dart';

export 'gregorian_date.dart';

/// A learning program. Get a day's portion with the functions such as `dafYomiBavli`.
abstract interface class Schedule {}

/// A civil date from its year, month and day.
GregorianDate dateToDart((int, int, int) date) =>
    (year: date.$1, month: date.$2, day: date.$3);

/// The year, month and day of `date`.
(int, int, int) dateFromDart(GregorianDate date) =>
    (date.year, date.month, date.day);
