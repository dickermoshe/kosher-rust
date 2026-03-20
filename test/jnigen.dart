import 'dart:io';

import 'package:jnigen/jnigen.dart';

void main(List<String> args) {
  final packageRoot = Platform.script;
  final localKosherJavaSources =
      packageRoot.resolve('java/src/main/java').toFilePath();
  final siblingKosherJavaSources =
      packageRoot.resolve('../../zmanim-modern/src/main/java').toFilePath();

  // Prefer sources vendored in this repository for CI stability.
  final sourcePath = Directory(localKosherJavaSources).existsSync()
      ? localKosherJavaSources
      : siblingKosherJavaSources;
  if (!Directory(sourcePath).existsSync()) {
    throw FileSystemException(
      'Could not find Java sources for jnigen. Checked: '
      '$localKosherJavaSources and $siblingKosherJavaSources',
    );
  }

  generateJniBindings(
    Config(
      outputConfig: OutputConfig(
        dartConfig: DartCodeOutputConfig(
          // Required. Output path for generated bindings.
          path: packageRoot.resolve('lib/src/java/kosher_java.g.dart'),
          // Optional. Write bindings into a single file (instead of one file per class).
          structure: OutputStructure.singleFile,
        ),
      ),
      // Optional. List of directories that contain the source files for which to generate bindings.
      sourcePath: [Uri.directory(sourcePath)],
      // Required. List of classes or packages for which bindings should be generated.
      classes: [
        // KosherJava
        'com.kosherjava.zmanim.ComprehensiveZmanimCalendar',
        'com.kosherjava.zmanim.ZmanimCalendar',
        'com.kosherjava.zmanim.AstronomicalCalendar',
        'com.kosherjava.zmanim.util.AstronomicalCalculator',
        'com.kosherjava.zmanim.util.GeoLocation',
        'com.kosherjava.zmanim.hebrewcalendar.JewishDate',
        'com.kosherjava.zmanim.hebrewcalendar.JewishCalendar',
        'com.kosherjava.zmanim.hebrewcalendar.Daf',
        // Java Time
        'java.time.ZonedDateTime',
        'java.time.LocalDateTime',
        'java.time.LocalDate',
        'java.time.Instant',
        'java.time.ZoneId',
      ],
    ),
  );
}
