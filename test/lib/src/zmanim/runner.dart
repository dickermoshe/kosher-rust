import 'package:test_with_java/src/rust/api.dart';
import 'package:test_with_java/src/harness/zman_test_case.dart';
import 'package:test_with_java/src/zmanim/cases.dart';
import 'package:test_with_java/src/zmanim/java_adapter.dart';
import 'package:test_with_java/src/zmanim/models.dart';

/// Default max allowed difference in milliseconds.
const defaultMaxDiffMs = 30000;

void runZmanimTests({
  required int iterations,
  required int minGregorianYear,
  required int maxGregorianYear,
  required String? methodFilter,
  required bool allowNullMismatch,
}) {
  final validTimezones = resolveValidTimezones();
  final allZmanimPresets = presets();
  final filteredPresets = allZmanimPresets
      .where((e) =>
          methodFilter == null ||
          e.name().toLowerCase().contains(methodFilter.toLowerCase()))
      .toList();

  _runRegressionTests(
    regressionMaps: regressionTests,
    allZmanimPresets: allZmanimPresets,
    validTimezones: validTimezones,
    allowNullMismatch: allowNullMismatch,
  );

  if (filteredPresets.isEmpty) {
    throw Exception("No zmanim presets found");
  }

  for (var iteration = 0; iteration < iterations; iteration++) {
    for (final zman in filteredPresets) {
      final testCase = createRandomTestCase(
        zman: zman,
        iteration: iteration,
        minGregorianYear: minGregorianYear,
        maxGregorianYear: maxGregorianYear,
        validTimezones: validTimezones,
      );
      runSingleZmanimParityTest(
        testCase,
        allowNullMismatch: allowNullMismatch,
      );
    }
  }
}

void _runRegressionTests({
  required List<Map<String, dynamic>> regressionMaps,
  required List<ZmanimPreset> allZmanimPresets,
  required List<String> validTimezones,
  required bool allowNullMismatch,
}) {
  if (regressionMaps.isEmpty) {
    return;
  }

  for (final regressionMap in regressionMaps) {
    final testCase = ZmanTestCase.fromMap(regressionMap, allZmanimPresets);
    if (!validTimezones.contains(testCase.timezone)) {
      throw Exception(
          "Regression test has unsupported timezone: ${testCase.timezone}");
    }
    runSingleZmanimParityTest(
      testCase,
      allowNullMismatch: allowNullMismatch,
    );
  }
}

bool runSingleZmanimParityTest(
  ZmanTestCase testCase, {
  required bool allowNullMismatch,
}) {
  final javaZman = calculateJavaZman(testCase);
  final rustZman = _calculateRustZman(testCase);
  switch ((javaZman, rustZman)) {
    case (null, null):
      return false;
    case (null, ZmanResult()) || (ZmanResult(), null):
      if (allowNullMismatch) {
        return false;
      }
      // Zmanim related to Chametz are only returned by Java if it is Erev Pesach,
      // while Rust can return them on any date.
      if (testCase.zmanName.contains("Chametz")) {
        return false;
      }
      throw FailedZmanTest.nullMismatch(testCase, javaZman, rustZman);
    case (ZmanResult javaZman, ZmanResult rustZman):
      final difference = (javaZman.timestampMs - rustZman.timestampMs).abs();
      if (difference > defaultMaxDiffMs) {
        throw FailedZmanTest.differenceTooLarge(
          testCase,
          difference,
          defaultMaxDiffMs,
          javaZman,
          rustZman,
        );
      }
      return true;
  }
  throw StateError('Unreachable');
}

ZmanResult? _calculateRustZman(ZmanTestCase testCase) {
  final result = calculateZman(
    useElevation: testCase.useElevation,
    ateretTorahSunsetOffsetMinutes: testCase.ateretTorahSunsetOffsetMinutes,
    candleLightingOffsetMinutes: testCase.candleLightingOffsetMinutes,
    useAstronomicalChatzosForOtherZmanim:
        testCase.useAstronomicalChatzosForOtherZmanim,
    latitude: testCase.latitude,
    longitude: testCase.longitude,
    elevation: testCase.elevation,
    timezone: testCase.timezone,
    randomYear: testCase.year,
    randomMonth: testCase.month,
    randomDay: testCase.day,
    zman: testCase.zman,
  );
  if (result == null) {
    return null;
  }
  final (formattedDate, timestampMs) = result;
  return ZmanResult(formattedDate, timestampMs);
}
