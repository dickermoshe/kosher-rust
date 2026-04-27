import 'dart:io';

import 'package:config/config.dart';
import 'package:test_with_java/src/harness/init.dart';
import 'package:test_with_java/src/harness/options.dart';
import 'package:test_with_java/src/zmanim/runner.dart';

Future<void> main(List<String> args) async {
  final options = <OptionDefinition<dynamic>>[
    ...CommonTestOption.values,
    ...ZmanimOnlyOption.values,
  ];
  final configuration = Configuration.resolve(
      options: options, args: args, env: Platform.environment);

  final seed = configuration.value(CommonTestOption.seed);
  final iterations = configuration.value(CommonTestOption.iterations);
  final minYear = configuration.value(CommonTestOption.minYear);
  final maxYear = configuration.value(CommonTestOption.maxYear);
  final methodFilter =
      configuration.optionalValue(ZmanimOnlyOption.methodFilter);
  final allowNullMismatch =
      configuration.value(ZmanimOnlyOption.allowNullMismatch);
  if (minYear > maxYear) {
    throw ArgumentError(
        'min-year ($minYear) cannot be greater than max-year ($maxYear)');
  }

  print("Seed: $seed");
  print("Iterations: $iterations");
  print("Min year: $minYear");
  print("Max year: $maxYear");
  print("Method filter: $methodFilter");
  print("Allow null mismatch: $allowNullMismatch");

  await init(seed: seed);
  runZmanimTests(
    iterations: iterations,
    minGregorianYear: minYear,
    maxGregorianYear: maxYear,
    methodFilter: methodFilter,
    allowNullMismatch: allowNullMismatch,
  );
  print("All tests passed");
  exit(0);
}
