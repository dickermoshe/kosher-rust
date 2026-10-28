// Today on the Hebrew calendar, today's learning, and today's zmanim for a place, in this
// computer's time zone.
//
//   dart run example/kosher_rust_example.dart                      # Jerusalem
//   dart run example/kosher_rust_example.dart 40.09 -74.22         # Lakewood, in the diaspora

import 'package:kosher_rust/kosher_rust.dart';

Future<void> main(List<String> args) async {
  await initKosherRust();

  final (latitude, longitude, region) = switch (args) {
    [final lat, final lon] => (
      double.parse(lat),
      double.parse(lon),
      Region.diaspora,
    ),
    _ => (31.778, 35.235, Region.israel),
  };

  final now = DateTime.now();
  final date = (year: now.year, month: now.month, day: now.day);
  final hebrew = HebrewDate.fromGregorian(date: date)!;
  print('${now.year}-${_two(now.month)}-${_two(now.day)}  $hebrew');

  for (final day in hebrew.specialDays(region: region)) {
    print('  ${day.displayName} (${day.hebrewName})');
  }
  final parsha = hebrew.nextParsha(region: region);
  if (parsha != null) print("  This week's parsha: ${parsha.displayName}");

  print('');
  print('Learning');
  _line('Daf Yomi', dafYomiBavli(date: hebrew));
  _line('Mishna Yomis', mishnaYomis(date: hebrew));
  _line('Tehillim', tehillimMonthly(date: hebrew));
  _line('Pirkei Avos', pirkeiAvos(date: hebrew, region: region));

  print('');
  print('Zmanim at $latitude, $longitude');
  final zmanim = ZmanimCalculator(
    location: Location(latitude: latitude, longitude: longitude),
    date: date,
    offset: TimeZone.local.offset,
  );
  final headline = [
    Zman.alos72Minutes,
    Zman.sunrise,
    Zman.sofZmanShmaGra,
    Zman.sofZmanTfilaGra,
    Zman.chatzosHayom,
    Zman.minchaGedolaGra,
    Zman.plagHaminchaGra,
    Zman.sunset,
    Zman.tzaisGeonim8Point5Degrees,
    Zman.tzais72Minutes,
    if (hebrew.hasCandleLighting(region: region)) Zman.candleLighting,
  ];
  for (final zman in headline) {
    final time = await zmanim
        .time(zman: zman)
        .then(_clock)
        .catchError((Object e) => '(${(e as ZmanimException).name})');
    print('  ${zman.displayName.padRight(32)} $time');
  }

  disposeKosherRust();
}

void _line(String label, Object? portion) {
  if (portion != null) print('  ${'$label:'.padRight(14)}$portion');
}

/// The local time of an instant given in seconds since the Unix epoch.
String _clock(int seconds) {
  final time = DateTime.fromMillisecondsSinceEpoch(seconds * 1000);
  return '${_two(time.hour)}:${_two(time.minute)}:${_two(time.second)}';
}

String _two(int value) => value.toString().padLeft(2, '0');
