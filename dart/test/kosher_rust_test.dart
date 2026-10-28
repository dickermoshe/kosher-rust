import 'package:kosher_rust/kosher_rust.dart';
import 'package:test/test.dart';

/// Israel Standard Time, two hours ahead of UTC, and three in summer.
class Jerusalem extends TimeZone {
  @override
  Duration offset(int instant) {
    // Daylight saving time in 2024, from 29 March to 27 October.
    final summer = instant >= 1711670400 && instant < 1729987200;
    return Duration(hours: summer ? 3 : 2);
  }
}

void main() {
  setUpAll(initKosherRust);

  test('converts a Gregorian date to the Hebrew calendar', () {
    final day = HebrewDate.fromGregorian(
      date: (year: 2024, month: 4, day: 23),
    )!;
    expect(day.toString(), '15 Nisan 5784');
    expect(day.month, HebrewMonth.nisan);
    expect(day.day, 15);
    expect(day.year.value, 5784);
    expect(day.weekday, 2);
    expect(
      day.specialDays(region: Region.diaspora),
      contains(const SpecialDay.pesach()),
    );
    expect(day.toGregorian(), (year: 2024, month: 4, day: 23));
    expect(HebrewMonth.nisan.hebrewName, 'ניסן');
  });

  test('rejects a day that does not exist', () {
    expect(HebrewDate.of(year: 5785, month: HebrewMonth.adarI, day: 1), isNull);
  });

  test('gives the daily learning', () {
    final day = HebrewDate.fromGregorian(
      date: (year: 2024, month: 4, day: 23),
    )!;
    expect(dafYomiBavli(date: day).toString(), 'Bava Metzia 55');
    expect(mishnaYomis(date: day).toString(), 'Nazir 5:5-6');
    expect(tehillimMonthly(date: day).toString(), startsWith('Tehillim '));
  });

  test('calculates zmanim in a Dart time zone', () async {
    final location = Location(latitude: 31.778, longitude: 35.235);
    final zmanim = ZmanimCalculator(
      location: location,
      date: (year: 2024, month: 4, day: 23),
      offset: Jerusalem().offset,
    );
    final sunset = await zmanim.time(zman: Zman.sunset);
    // Sunset in Jerusalem on 23 April 2024 is about 19:13 local time, 16:13 UTC.
    const expected = 1713888780;
    expect((sunset - expected).abs(), lessThan(180));
  });

  test('needs a time zone for Kiddush Levana', () async {
    final location = Location(latitude: 31.778, longitude: 35.235);
    final zmanim = ZmanimCalculator.withoutZone(
      location: location,
      date: (year: 2024, month: 4, day: 23),
    );
    await expectLater(
      zmanim.time(zman: Zman.sofZmanKidushLevana15Days),
      throwsA(ZmanimException.timeZoneRequired),
    );
  });

  test('validates the location', () {
    expect(
      () => Location(latitude: 91, longitude: 0),
      throwsA(LocationError.invalidLatitude),
    );
  });

  test('changes the config', () {
    final config = ZmanimConfig.defaults().copyWith(
      astronomicalCalculator: const AstronomicalCalculator.meeus(
        applyDeltaT: true,
      ),
    );
    expect(config.candleLightingOffset, const Duration(minutes: 18));
    expect(
      config.astronomicalCalculator,
      const AstronomicalCalculator.meeus(applyDeltaT: true),
    );
    expect(
      const AstronomicalCalculator.noaa().displayName,
      contains('Oceanic'),
    );
  });
}
