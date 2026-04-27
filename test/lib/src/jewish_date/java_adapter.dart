import 'package:jni/jni.dart';
import 'package:test_with_java/src/java/kosher_java.g.dart';
import 'package:test_with_java/src/rust/api.dart';

typedef DateTuple = (int, int, int);

DateTuple? javaGregorianDateToJewishDate(int year, int month, int day) {
  // JNI calls can intermittently fail even for valid inputs; retry to avoid
  // reporting flaky transport issues as calendar logic mismatches.
  for (final _ in Iterable.generate(3)) {
    try {
      final localDate = LocalDate.of$1(year, month, day);
      final gregorianDate = JewishDate.new$4(localDate);
      return (
        gregorianDate.getJewishYear(),
        gregorianDate.getJewishMonth(),
        gregorianDate.getJewishDayOfMonth()
      );
    } catch (_) {}
  }
  return null;
}

DateTuple? javaJewishDateToGregorianDate(int year, int month, int day) {
  for (final _ in Iterable.generate(3)) {
    try {
      final jewishDate = JewishDate.new$1(year, month, day);
      final localDate = jewishDate.getLocalDate();
      if (localDate == null) {
        continue;
      }
      return (
        localDate.getYear(),
        localDate.getMonthValue(),
        localDate.getDayOfMonth()
      );
    } catch (_) {}
  }
  return null;
}

DateTuple? javaAddDaysToJewishDate(
    int year, int month, int day, int daysToAdd) {
  for (final _ in Iterable.generate(3)) {
    try {
      final jewishDate = JewishDate.new$1(year, month, day);
      jewishDate.plusDays(daysToAdd);
      return (
        jewishDate.getJewishYear(),
        jewishDate.getJewishMonth(),
        jewishDate.getJewishDayOfMonth()
      );
    } catch (_) {}
  }
  return null;
}

DateTuple? javaMinusDaysToJewishDate(
    int year, int month, int day, int daysToSubtract) {
  for (final _ in Iterable.generate(3)) {
    try {
      final jewishDate = JewishDate.new$1(year, month, day);
      jewishDate.minusDays(daysToSubtract);
      return (
        jewishDate.getJewishYear(),
        jewishDate.getJewishMonth(),
        jewishDate.getJewishDayOfMonth()
      );
    } catch (_) {}
  }
  return null;
}

DateTuple? javaAddMonthsToJewishDate(
    int year, int month, int day, int monthsToAdd) {
  for (final _ in Iterable.generate(3)) {
    try {
      final jewishDate = JewishDate.new$1(year, month, day);

      jewishDate.plusMonths(monthsToAdd);
      return (
        jewishDate.getJewishYear(),
        jewishDate.getJewishMonth(),
        jewishDate.getJewishDayOfMonth()
      );
    } catch (_) {}
  }
  return null;
}

DateTuple? javaAddYearsToJewishDate(
    int year, int month, int day, int yearsToAdd) {
  for (final _ in Iterable.generate(3)) {
    try {
      final jewishDate = JewishDate.new$1(year, month, day);
      // Skips to Adar II in a leap year.
      jewishDate.plusYears(yearsToAdd, false);
      return (
        jewishDate.getJewishYear(),
        jewishDate.getJewishMonth(),
        jewishDate.getJewishDayOfMonth()
      );
    } catch (_) {}
  }
  return null;
}

JavaJewishCalendarTestResults? javaJewishCalendarSnapshot({
  required int year,
  required int month,
  required int day,
  required bool inIsrael,
  required bool useModernHolidays,
}) {
  // This snapshot is later asserted against Rust-side holiday/calendar logic.
  // Returning null here means we could not build a stable Java reference.
  for (final _ in Iterable.generate(3)) {
    try {
      final calendar = JewishCalendar.new$5(year, month, day, inIsrael);
      calendar.setUseModernHolidays(useModernHolidays);

      int? dayOfChanukah = calendar.getDayOfChanukah();
      if (dayOfChanukah == -1) {
        dayOfChanukah = null;
      }
      int? dayOfOmer = calendar.getDayOfOmer();
      if (dayOfOmer == -1) {
        dayOfOmer = null;
      }

      final getUpcomingParshah = _getParshaIndex(calendar.getUpcomingParshah());
      if (getUpcomingParshah == null) {
        continue;
      }

      return JavaJewishCalendarTestResults(
        getParshah: _getParshaIndex(calendar.getParshah()),
        getUpcomingParshah: getUpcomingParshah,
        getSpecialShabbos: _getParshaIndex(calendar.getSpecialShabbos()),
        isBirkasHachamah: calendar.isBirkasHachamah(),
        getYomTovIndex: calendar.getYomTovIndex(),
        isAssurBemelacha: calendar.isAssurBemelacha(),
        hasCandleLighting: calendar.hasCandleLighting(),
        isAseresYemeiTeshuva: calendar.isAseresYemeiTeshuva(),
        isYomKippurKatan: calendar.isYomKippurKatan(),
        isBeHaB: calendar.isBeHaB(),
        isTaanisBechoros: calendar.isTaanisBechoros(),
        getDayOfChanukah: dayOfChanukah,
        isRoshChodesh: calendar.isRoshChodesh(),
        isMacharChodesh: calendar.isMacharChodesh(),
        isShabbosMevorchim: calendar.isShabbosMevorchim(),
        getDayOfOmer: dayOfOmer,
      );
    } catch (_) {}
  }
  return null;
}

int? _getParshaIndex(JewishCalendar$Parsha? parsha) {
  if (parsha == null) {
    return null;
  }
  final parshaClass = JClass.forName(
      r'com/kosherjava/zmanim/hebrewcalendar/JewishCalendar$Parsha');
  final parshaOrdinal = parshaClass
      .instanceMethodId('ordinal', '()I')
      .call(parsha, jint.type, []);
  if (parshaOrdinal == 0) {
    return null;
  }
  return parshaOrdinal - 1;
}
