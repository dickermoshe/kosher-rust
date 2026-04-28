import 'package:test_with_java/src/harness/date.dart';
import 'package:test_with_java/src/harness/init.dart';
import 'package:test_with_java/src/harness/rnd.dart';
import 'package:test_with_java/src/harness/zman_test_case.dart';
import 'package:test_with_java/src/java/kosher_java.g.dart';
import 'package:test_with_java/src/rust/api.dart';

/// Maximum attempts to find a random point with a shared Java/Rust timezone.
const maxTimezoneAttempts = 1000;

/// Regression tests that must run every execution.
final List<Map<String, dynamic>> regressionTests = [
  {
    'iteration': -1,
    'year': 2024,
    'month': 4,
    'day': 22,
    'latitude': 31.778,
    'longitude': 35.235,
    'elevation': 754.0,
    'timezone': 'Asia/Jerusalem',
    'zman': 'getSofZmanAchilasChametzGRA',
    'ateretTorahSunsetOffsetMinutes': 0,
    'candleLightingOffsetMinutes': 18,
    'useAstronomicalChatzosForOtherZmanim': false,
    'useElevation': false,
  },
  {
    'iteration': -1,
    'year': 2024,
    'month': 4,
    'day': 22,
    'latitude': 31.778,
    'longitude': 35.235,
    'elevation': 754.0,
    'timezone': 'Asia/Jerusalem',
    'zman': 'getSofZmanBiurChametzGRA',
    'ateretTorahSunsetOffsetMinutes': 0,
    'candleLightingOffsetMinutes': 18,
    'useAstronomicalChatzosForOtherZmanim': false,
    'useElevation': false,
  },
  {
    'iteration': -1,
    'year': 2026,
    'month': 1,
    'day': 3,
    'latitude': 39.36463,
    'longitude': -76.70222,
    'elevation': 0.0,
    'timezone': 'America/New_York',
    'zman': 'getSofZmanKidushLevanaBetweenMoldos',
    'ateretTorahSunsetOffsetMinutes': 0,
    'candleLightingOffsetMinutes': 18,
    'useAstronomicalChatzosForOtherZmanim': false,
    'useElevation': false,
  },
  {
    "iteration": 741,
    "year": 2058,
    "month": 7,
    "day": 31,
    "latitude": -18.88480386694347,
    "longitude": -174.522379072958,
    "elevation": 2671.332842032057,
    "timezone": "Pacific/Tongatapu",
    "zman": "getFixedLocalChatzos",
    "ateretTorahSunsetOffsetMinutes": 19,
    "candleLightingOffsetMinutes": 6,
    "useAstronomicalChatzosForOtherZmanim": true,
    "useElevation": false
  },
  {
    "iteration": -1,
    "year": 2037,
    "month": 12,
    "day": 29,
    "latitude": -32.93056753553307,
    "longitude": -125.36776050346323,
    "elevation": 940.4531699881416,
    "timezone": "Etc/GMT+8",
    "zman": "getSofZmanShma3HoursBeforeChatzos",
    "ateretTorahSunsetOffsetMinutes": 21,
    "candleLightingOffsetMinutes": 9,
    "useAstronomicalChatzosForOtherZmanim": true,
    "useElevation": false
  }
];

List<String> resolveValidTimezones() {
  final javaTimezones =
      ZoneId.getAvailableZoneIds()!.map((e) => e!.toDartString()).toSet();
  final rustTimezones = timezones().toSet();
  // We only sample from zones that both runtimes support so mismatches reflect
  // calculation differences, not unsupported timezone IDs.
  return javaTimezones.intersection(rustTimezones).toList();
}

ZmanTestCase createRandomTestCase(
    {required ZmanimPreset zman,
    required int iteration,
    required int minGregorianYear,
    required int maxGregorianYear,
    required List<String> validTimezones}) {
  final maxLatitude = maxLatitudeForZman(zman);

  for (var attempt = 0; attempt < maxTimezoneAttempts; attempt++) {
    final randomDateTime = randomGregorianDateTimeUtc(
      minGregorianYear: minGregorianYear,
      maxGregorianYear: maxGregorianYear,
      random: random,
    );
    final randomLatitude = random.getDouble(-maxLatitude, maxLatitude);
    final randomLongitude = random.getDouble(-180.0, 180.0);
    final tz =
        findTimezone(longitude: randomLongitude, latitude: randomLatitude);

    if (!validTimezones.contains(tz)) {
      continue;
    }

    final randomElevation = random.getDouble(0.0, 4000.0);
    final randomUseElevation = random.nextBool();
    final randomAteretTorahSunsetOffsetMinutes = random.nextInt(60);
    final randomCandleLightingOffsetMinutes = random.nextInt(60);
    final randomUseAstronomicalChatzosForOtherZmanim = random.nextBool();
    return ZmanTestCase(
      iteration: iteration,
      year: randomDateTime.year,
      month: randomDateTime.month,
      day: randomDateTime.day,
      latitude: randomLatitude,
      longitude: randomLongitude,
      elevation: randomElevation,
      timezone: tz,
      zman: zman,
      ateretTorahSunsetOffsetMinutes: randomAteretTorahSunsetOffsetMinutes,
      candleLightingOffsetMinutes: randomCandleLightingOffsetMinutes,
      useAstronomicalChatzosForOtherZmanim:
          randomUseAstronomicalChatzosForOtherZmanim,
      useElevation: randomUseElevation,
    );
  }

  throw Exception(
      "Failed to find a supported timezone after $maxTimezoneAttempts attempts");
}

double maxLatitudeForZman(ZmanimPreset zman) {
  // The Java and Rust implementations use slightly different astronomical
  // models. While usually negligible, differences become more pronounced near
  // solar transition boundaries (sunrise/sunset and related dawn/dusk zmanim),
  // especially at high latitudes.
  //
  // At extreme latitudes there are periods where sunrise/sunset do not exist,
  // which correctly yields null in both runtimes. The noisier case is near the
  // edge of those periods: small model differences can produce large deltas.
  //
  // To keep random comparisons focused on meaningful parity checks, we use a
  // narrower latitude window for edge-sensitive zmanim and a wider one for
  // stable calculations such as chatzos.
  switch (zman.name()) {
    case "getChatzos":
      return 85.0;
    case "getSunriseWithElevation" ||
          "getSeaLevelSunrise" ||
          "getSunsetWithElevation" ||
          "getSeaLevelSunset" ||
          "getChatzos" ||
          "getChatzosAsHalfDay" ||
          "getFixedLocalChatzos":
      return 60.0;
    default:
      return 40.0;
  }
}
