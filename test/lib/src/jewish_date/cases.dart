import 'package:test_with_java/src/harness/date.dart';
import 'package:test_with_java/src/harness/init.dart';

typedef DateTuple = (int, int, int);

final List<DateTuple> gregorianRegressionDates = [
  (1900, 1, 1),
  (1999, 12, 31),
  (2000, 2, 29),
  (2024, 3, 24),
  (2024, 4, 22),
  (2024, 10, 3),
  (2100, 12, 31),
];

final List<DateTuple> jewishRegressionDates = [
  (5660, 10, 1),
  (5760, 7, 1),
  (5782, 13, 14),
  (5784, 12, 30),
  (5784, 13, 1),
  (5785, 1, 15),
  (5861, 9, 29),
];

final List<DateTuple> invalidJewishRegressionDates = [
  (5784, 0, 10),
  (5784, 14, 10),
  (5784, 13, 30),
  (5785, 13, 1),
  (5785, 2, 30),
  (5784, 7, 0),
  (5784, 7, 31),
];

final List<int> jewishDayOffsets = [0, 1, -1, 29, -29, 365, -365];
final List<int> jewishMonthOffsets = [0, 1, -1, 6, -6, 12];
final List<int> jewishYearOffsets = [0, 1, -1, 5, -5];

/// Return a random Jewish date within configured year bounds.
/// The returned date is not guaranteed to be valid.
(int, int, int) randomJewishDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final minJewishYear = minGregorianYear + 3760;
  final maxJewishYear = maxGregorianYear + 3760;
  final year =
      random.nextInt(maxJewishYear - minJewishYear + 1) + minJewishYear;
  final month = random.nextInt(13) + 1;
  final day = random.nextInt(30) + 1;
  return (year, month, day);
}

/// Return a random valid Gregorian date within configured year bounds.
(int, int, int) randomGregorianDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final randomDateTime = randomGregorianDateTimeUtc(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
    random: random,
  );
  return (randomDateTime.year, randomDateTime.month, randomDateTime.day);
}
