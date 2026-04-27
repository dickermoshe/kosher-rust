import 'package:jni/jni.dart';
import 'package:test_with_java/src/harness/zman_test_case.dart';
import 'package:test_with_java/src/java/kosher_java.g.dart';
import 'package:test_with_java/src/zmanim/models.dart';

/// Calculate the zman using the Java library. Returns null when unavailable.
ZmanResult? calculateJavaZman(ZmanTestCase testCase) {
  // JNI can intermittently throw while invoking otherwise valid calls. Retry a
  // few times before failing so transient JNI errors do not create false test
  // failures.
  for (final i in Iterable.generate(3)) {
    try {
      final javaZoneId = ZoneId.of$1(testCase.timezone.toJString())!;
      final localDate =
          LocalDate.of$1(testCase.year, testCase.month, testCase.day);
      final location = GeoLocation.new$1("".toJString(), testCase.latitude,
          testCase.longitude, testCase.elevation, javaZoneId);
      final calendar = ComprehensiveZmanimCalendar.new1(location);
      calendar.setUseElevation(testCase.useElevation);
      calendar.setCandleLightingOffset(
          testCase.candleLightingOffsetMinutes.toDouble());

      // Compare CHATZOS_ASTRONOMICAL with getChatzos by forcing astronomical chatzos.
      calendar.setUseAstronomicalChatzos(true);
      calendar.setUseAstronomicalChatzosForOtherZmanim(
        testCase.useAstronomicalChatzosForOtherZmanim,
      );
      calendar.setAteretTorahSunsetOffset(
          testCase.ateretTorahSunsetOffsetMinutes.toDouble());
      calendar.setLocalDate(localDate);

      final methodId = calendar.jClass.instanceMethodId(
        testCase.zman.name(),
        r'()Ljava/time/Instant;',
      );
      final result = methodId.call(calendar, $Instant$NullableType$(), []);
      if (result == null) {
        return null;
      }

      final milliseconds = result.toEpochMilli();
      final instant = Instant.ofEpochMilli(milliseconds);
      final ztd = ZonedDateTime.ofInstant(instant, javaZoneId);
      return ZmanResult(ztd!.toString$1()!.toDartString(), milliseconds);
    } on JniException catch (_) {
      if (i == 2) {
        rethrow;
      }
    }
  }
  return null;
}
