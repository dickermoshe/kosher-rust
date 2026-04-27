import 'package:test_with_java/src/harness/comparison.dart';
import 'package:test_with_java/src/harness/zman_test_case.dart';

class ZmanResult {
  final String formattedDate;
  final int timestampMs;
  ZmanResult(this.formattedDate, this.timestampMs);

  String toDebugString() {
    return "Zman: $formattedDate ($timestampMs)";
  }
}

class FailedZmanTest implements Exception {
  final ZmanTestCase testCase;
  final String message;
  FailedZmanTest(this.testCase, this.message);

  static FailedZmanTest nullMismatch(
      ZmanTestCase testCase, ZmanResult? javaZman, ZmanResult? rustZman) {
    final message = [
      "Java: ${javaZman?.toDebugString()}",
      "Rust: ${rustZman?.toDebugString()}",
      "Test Case: ${testCase.toJson()}",
    ].join("\n");
    return FailedZmanTest(testCase, message);
  }

  static FailedZmanTest differenceTooLarge(ZmanTestCase testCase,
      int difference, int maxDiffMs, ZmanResult javaZman, ZmanResult rustZman) {
    final message = [
      "Difference too large: $difference ms. Max allowed: $maxDiffMs ms.",
      "Difference: ${formatDifference(difference)}",
      "Java: ${javaZman.toDebugString()}",
      "Rust: ${rustZman.toDebugString()}",
      "Test Case: ${testCase.toJson()}",
    ].join("\n");
    return FailedZmanTest(testCase, message);
  }

  @override
  String toString() => message;
}
