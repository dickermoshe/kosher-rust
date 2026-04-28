import 'package:test_with_java/src/harness/comparison.dart';
import 'package:test_with_java/src/harness/init.dart';
import 'package:test_with_java/src/jewish_date/cases.dart';
import 'package:test_with_java/src/jewish_date/java_adapter.dart';
import 'package:test_with_java/src/rust/api.dart';

void runJewishDateTests({
  required int iterations,
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  _runJewishDateRegressionTests();

  for (var iteration = 0; iteration < iterations; iteration++) {
    _testGregorianDateToJewishDate(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
    _testJewishDateToGregorianDate(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
    _testAddDaysToJewishDate(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
    _testAddMonthsToJewishDate(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
    _testAddYearsToJewishDate(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
    _testMinusDaysToJewishDate(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
    _testRandomJewishCalendar(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
    );
  }
}

void _runJewishDateRegressionTests() {
  for (final (year, month, day) in gregorianRegressionDates) {
    final rustResult =
        gregorianDateToJewishDate(year: year, month: month, day: day);
    final javaResult = javaGregorianDateToJewishDate(year, month, day);
    assertDateResultsMatch(
      inputDate: (year, month, day),
      targetDateType: "Jewish",
      javaDate: javaResult,
      rustDate: rustResult,
    );
  }

  for (final date in [
    ...jewishRegressionDates,
    ...invalidJewishRegressionDates
  ]) {
    _assertJewishDateOperations(date);
  }
}

void _assertJewishDateOperations((int, int, int) date) {
  final (year, month, day) = date;
  final rustGregorian =
      jewishDateToGregorianDate(year: year, month: month, day: day);
  final javaGregorian = javaJewishDateToGregorianDate(year, month, day);
  assertDateResultsMatch(
    inputDate: date,
    targetDateType: "Gregorian",
    javaDate: javaGregorian,
    rustDate: rustGregorian,
  );
  if (rustGregorian == null && javaGregorian == null) {
    return;
  }

  for (final dayOffset in jewishDayOffsets) {
    final rustResult = addDaysToJewishDate(
      year: year,
      month: month,
      day: day,
      daysToAdd: dayOffset,
    );
    final javaResult = dayOffset >= 0
        ? javaAddDaysToJewishDate(year, month, day, dayOffset)
        : javaMinusDaysToJewishDate(year, month, day, -dayOffset);
    assertDateResultsMatch(
      inputDate: date,
      targetDateType: "Jewish after adding $dayOffset days",
      javaDate: javaResult,
      rustDate: rustResult,
    );
  }

  for (final monthOffset in jewishMonthOffsets) {
    final rustResult = addMonthsToJewishDate(
      year: year,
      month: month,
      day: day,
      monthsToAdd: monthOffset,
    );
    final javaResult = javaAddMonthsToJewishDate(year, month, day, monthOffset);
    assertDateResultsMatch(
      inputDate: date,
      targetDateType: "Jewish after adding $monthOffset months",
      javaDate: javaResult,
      rustDate: rustResult,
    );
  }

  for (final yearOffset in jewishYearOffsets) {
    final rustResult = addYearsToJewishDate(
      year: year,
      month: month,
      day: day,
      yearsToAdd: yearOffset,
    );
    final javaResult = javaAddYearsToJewishDate(year, month, day, yearOffset);
    assertDateResultsMatch(
      inputDate: date,
      targetDateType: "Jewish after adding $yearOffset years",
      javaDate: javaResult,
      rustDate: rustResult,
    );
  }

  for (final inIsrael in [false, true]) {
    for (final useModernHolidays in [false, true]) {
      final javaSnapshot = javaJewishCalendarSnapshot(
        year: year,
        month: month,
        day: day,
        inIsrael: inIsrael,
        useModernHolidays: useModernHolidays,
      );
      if (javaSnapshot == null) {
        throw Exception(
          "Could not produce Java JewishCalendar snapshot for ${(
            year,
            month,
            day
          )}",
        );
      }
      testJewishCalendar(
        year: year,
        month: month,
        day: day,
        inIsrael: inIsrael,
        useModernHolidays: useModernHolidays,
        java: javaSnapshot,
      );
    }
  }
}

void _testGregorianDateToJewishDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomGregorianDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  final rustResult =
      gregorianDateToJewishDate(year: year, month: month, day: day);
  final javaResult = javaGregorianDateToJewishDate(year, month, day);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Jewish",
    javaDate: javaResult,
    rustDate: rustResult,
  );
}

void _testJewishDateToGregorianDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomJewishDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  final rustResult =
      jewishDateToGregorianDate(year: year, month: month, day: day);
  final javaResult = javaJewishDateToGregorianDate(year, month, day);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Gregorian",
    javaDate: javaResult,
    rustDate: rustResult,
  );
}

void _testAddDaysToJewishDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomJewishDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  final daysToAdd = random.nextInt(600) + 1;
  final rustResult = addDaysToJewishDate(
      year: year, month: month, day: day, daysToAdd: daysToAdd);
  final javaResult = javaAddDaysToJewishDate(year, month, day, daysToAdd);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Jewish after adding $daysToAdd days",
    javaDate: javaResult,
    rustDate: rustResult,
  );
}

void _testMinusDaysToJewishDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomJewishDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  final daysToAdd = random.nextInt(600) + 1;
  final rustResult = addDaysToJewishDate(
    year: year,
    month: month,
    day: day,
    daysToAdd: -daysToAdd,
  );
  final javaResult = javaMinusDaysToJewishDate(year, month, day, daysToAdd);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Jewish after subtracting $daysToAdd days",
    javaDate: javaResult,
    rustDate: rustResult,
  );
}

void _testAddMonthsToJewishDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomJewishDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  final monthsToAdd = random.nextInt(120) + 1;
  final rustResult = addMonthsToJewishDate(
      year: year, month: month, day: day, monthsToAdd: monthsToAdd);
  final javaResult = javaAddMonthsToJewishDate(year, month, day, monthsToAdd);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Jewish after adding $monthsToAdd months",
    javaDate: javaResult,
    rustDate: rustResult,
  );
}

void _testAddYearsToJewishDate({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomJewishDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  final yearsToAdd = random.nextInt(60) + 1;
  final rustResult = addYearsToJewishDate(
      year: year, month: month, day: day, yearsToAdd: yearsToAdd);
  final javaResult = javaAddYearsToJewishDate(year, month, day, yearsToAdd);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Jewish after adding $yearsToAdd years",
    javaDate: javaResult,
    rustDate: rustResult,
  );
}

void _testRandomJewishCalendar({
  required int minGregorianYear,
  required int maxGregorianYear,
}) {
  final (year, month, day) = randomJewishDate(
    minGregorianYear: minGregorianYear,
    maxGregorianYear: maxGregorianYear,
  );
  // We intentionally allow invalid randomly generated Jewish dates. For those
  // inputs, parity means both runtimes reject the conversion (null/null).
  final rustDate =
      jewishDateToGregorianDate(year: year, month: month, day: day);
  final javaDate = javaJewishDateToGregorianDate(year, month, day);
  assertDateResultsMatch(
    inputDate: (year, month, day),
    targetDateType: "Gregorian",
    javaDate: javaDate,
    rustDate: rustDate,
  );
  if (rustDate == null && javaDate == null) {
    return;
  }

  final inIsrael = random.nextBool();
  final useModernHolidays = random.nextBool();
  final javaSnapshot = javaJewishCalendarSnapshot(
    year: year,
    month: month,
    day: day,
    inIsrael: inIsrael,
    useModernHolidays: useModernHolidays,
  );
  if (javaSnapshot == null) {
    throw Exception("Could not produce Java JewishCalendar snapshot for ${(
      year,
      month,
      day
    )}");
  }

  testJewishCalendar(
      year: year,
      month: month,
      day: day,
      inIsrael: inIsrael,
      useModernHolidays: useModernHolidays,
      java: javaSnapshot);
}
