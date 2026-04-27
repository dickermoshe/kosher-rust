import 'package:test_with_java/src/harness/constants.dart';

typedef DateTuple = (int, int, int);

/// Compare the Java and Rust date conversion results.
/// A null/null outcome is considered a match for invalid inputs.
void assertDateResultsMatch({
  required DateTuple inputDate,
  required String targetDateType,
  DateTuple? javaDate,
  DateTuple? rustDate,
}) {
  final (year, month, day) = inputDate;
  switch ((javaDate, rustDate)) {
    case (null, null):
      return;
    case (null, (int _, int _, int _)):
      throw Exception("Converting ${(
        year,
        month,
        day
      )} to $targetDateType date failed. Rust result: $rustDate, Java result: $javaDate");
    case ((int _, int _, int _), null):
      throw Exception("Converting ${(
        year,
        month,
        day
      )} to $targetDateType date failed. Rust result: $rustDate, Java result: $javaDate");
    case ((int _, int _, int _), (int _, int _, int _)):
      if (rustDate != javaDate) {
        throw Exception("Converting ${(
          year,
          month,
          day
        )} to $targetDateType date failed. Rust result: $rustDate, Java result: $javaDate");
      }
      return;
  }
}

/// Format a difference in milliseconds as a human readable string.
String formatDifference(int differenceMs) {
  if (differenceMs > HOURS_MS) {
    return '${differenceMs / HOURS_MS} hours';
  }
  if (differenceMs > MINUTES_MS) {
    return '${differenceMs / MINUTES_MS} minutes';
  }
  if (differenceMs > SECONDS_MS) {
    return '${differenceMs / SECONDS_MS} seconds';
  }
  return '${differenceMs} milliseconds';
}
