import 'dart:math';

/// Returns a random Gregorian date in the inclusive year range.
DateTime randomGregorianDateTimeUtc({
  required int minGregorianYear,
  required int maxGregorianYear,
  required Random random,
}) {
  final start = DateTime.utc(minGregorianYear, 1, 1);
  final endExclusive = DateTime.utc(maxGregorianYear, 1, 1);
  final spanDays = endExclusive.difference(start).inDays;
  final randomDayOffset = random.nextInt(spanDays);
  return start.add(Duration(days: randomDayOffset));
}
