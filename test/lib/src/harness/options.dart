import 'package:config/config.dart';
import 'package:test_with_java/src/harness/init.dart';

enum CommonTestOption<V> implements OptionDefinition<V> {
  seed(IntOption(
    argName: 'seed',
    argAbbrev: 's',
    helpText: 'The seed to use for the random number generator',
    min: 0,
    fromDefault: defaultSeed,
  )),
  iterations(IntOption(
    argName: 'iterations',
    argAbbrev: 'i',
    helpText: 'The number of iterations to test',
    min: 1,
    max: 100000,
    defaultsTo: 1000,
    envName: 'TEST_ITERATIONS',
  )),
  minYear(IntOption(
    argName: 'min-year',
    helpText: 'Minimum Gregorian year to test',
    defaultsTo: 1900,
  )),
  maxYear(IntOption(
    argName: 'max-year',
    helpText: 'Maximum Gregorian year to test',
    defaultsTo: 2100,
  ));

  const CommonTestOption(this.option);

  @override
  final ConfigOptionBase<V> option;
}

enum ZmanimOnlyOption<V> implements OptionDefinition<V> {
  methodFilter(StringOption(
    argName: 'filter',
    argAbbrev: 'f',
    helpText: 'Filter the methods to test',
  )),
  allowNullMismatch(FlagOption(
    argName: 'allow-null-mismatch',
    argAbbrev: 'n',
    helpText: 'Do not fail the test if one of the zmanim is null',
    defaultsTo: false,
  ));

  const ZmanimOnlyOption(this.option);

  @override
  final ConfigOptionBase<V> option;
}
