import 'package:test_with_java/src/harness/date.dart';
import 'package:test_with_java/src/harness/init.dart';
import 'package:test_with_java/src/harness/rnd.dart';

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
  final timestamp = random.getDouble(
      yearToTimestamp(minGregorianYear), yearToTimestamp(maxGregorianYear));
  final randomDateTime =
      DateTime.fromMillisecondsSinceEpoch((timestamp * 1000).toInt());
  return (randomDateTime.year, randomDateTime.month, randomDateTime.day);
}
