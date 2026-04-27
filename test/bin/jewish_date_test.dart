import 'dart:io';

import 'package:config/config.dart';
import 'package:test_with_java/src/harness/init.dart';
import 'package:test_with_java/src/harness/options.dart';
import 'package:test_with_java/src/jewish_date/runner.dart';

Future<void> main(List<String> args) async {
  final configuration = Configuration.resolve(
      options: CommonTestOption.values, args: args, env: Platform.environment);
  final seed = configuration.value(CommonTestOption.seed);
  final iterations = configuration.value(CommonTestOption.iterations);
  final minYear = configuration.value(CommonTestOption.minYear);
  final maxYear = configuration.value(CommonTestOption.maxYear);
  if (minYear > maxYear) {
    throw ArgumentError(
        'min-year ($minYear) cannot be greater than max-year ($maxYear)');
  }
  print("Seed: $seed");
  print("Iterations: $iterations");
  print("Min year: $minYear");
  print("Max year: $maxYear");

  await init(seed: seed);
  runJewishDateTests(
    iterations: iterations,
    minGregorianYear: minYear,
    maxGregorianYear: maxYear,
  );
  print("All tests passed");
  exit(0);
}
