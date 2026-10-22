// Calls the generated Node bindings. Run after `weaveffi generate --target node` and
// `npm install` in generated/node (see .github/workflows/bindings.yaml):
//
//     node --test tests/node/*.test.mjs
import { test } from 'node:test';
import assert from 'node:assert/strict';

import { Temporal } from '@js-temporal/polyfill';

import { calendar, times } from '../../generated/node/index.js';

const { limudim } = calendar;
const { myzmanim } = times;

/** The IANA time zone `name`, through Temporal. */
function ianaZone(name) {
  return {
    offsetSeconds(unixSeconds) {
      const instant = Temporal.Instant.fromEpochMilliseconds(Number(unixSeconds) * 1000);
      return instant.toZonedDateTimeISO(name).offsetNanoseconds / 1e9;
    },
    localToUnixSeconds(year, month, day, hour, minute, second) {
      const local = Temporal.PlainDateTime.from({ year, month, day, hour, minute, second });
      return BigInt(local.toZonedDateTime(name).epochMilliseconds / 1000);
    },
  };
}

/** `ms` as HH:MM:SS in New York. */
function newYorkTime(ms) {
  return new Date(Number(ms)).toLocaleTimeString('en-GB', { timeZone: 'America/New_York' });
}

const lakewood = times.seaLevelLocation(40.09, -74.22);
const newYork = ianaZone('America/New_York');

test('zmanim', () => {
  const day = new times.Zmanim(lakewood, { year: 2024, month: 4, day: 23 }, times.defaultConfig(), newYork);
  assert.equal(newYorkTime(day.get(times.Zman.Sunrise)), '06:06:35');
  assert.equal(newYorkTime(day.get(times.Zman.SofZmanShmaGra)), '09:31:00');

  const all = day.all();
  assert.equal(all.length, times.allZmanim().length);
  assert.ok(all.some((row) => row.failure === times.ZmanFailure.NotApplicable));
  assert.equal(times.zmanJavaName(times.Zman.SofZmanShmaGra), 'getSofZmanShmaGRA');
  assert.equal(day.config().candle_lighting_minutes, 18);
  day.close();
});

test('Kiddush Levana through a JS time zone, the day after the clocks went back', () => {
  const day = new times.Zmanim(lakewood, { year: 2024, month: 11, day: 4 }, times.defaultConfig(), newYork);
  const time = day.get(times.Zman.TchilasZmanKidushLevana3Days);
  assert.equal(newYorkTime(time), '08:44:50');

  const noZone = new times.Zmanim(lakewood, { year: 2024, month: 11, day: 4 }, times.defaultConfig(), null);
  assert.throws(() => noZone.get(times.Zman.TchilasZmanKidushLevana3Days), times.TimeZoneRequiredError);
});

test('a throwing time zone callback is a typed error', () => {
  const broken = {
    offsetSeconds() {
      throw new Error('no time zone data');
    },
  };
  const day = new times.Zmanim(lakewood, { year: 2024, month: 4, day: 23 }, times.defaultConfig(), broken);
  assert.throws(() => day.get(times.Zman.Sunrise), times.TimeZoneCallbackFailedError);
});

test('invalid input is a typed error', () => {
  const config = times.defaultConfig();
  assert.throws(
    () => new times.Zmanim(times.seaLevelLocation(91, 0), { year: 2024, month: 1, day: 1 }, config, newYork),
    times.InvalidLatitudeError,
  );
  assert.throws(
    () => new times.Zmanim(lakewood, { year: 2024, month: 2, day: 30 }, config, newYork),
    times.InvalidGregorianDateError,
  );
  assert.throws(
    () => new times.Zmanim(lakewood, { year: 2024, month: 1, day: 1 }, { ...config, candle_lighting_minutes: 1441 }, newYork),
    times.InvalidMinutesError,
  );
  assert.throws(
    () => new myzmanim.MyZmanim(lakewood, { year: 2024, month: 1, day: 1 }, NaN, newYork),
    times.InvalidMinutesError,
  );
});

test('calendar', () => {
  const date = calendar.HebrewDate.fromGregorian({ year: 2024, month: 4, day: 23 });
  assert.equal(date.display(), '15 Nisan 5784');
  assert.equal(date.month(), calendar.HebrewMonth.Nisan);
  assert.deepEqual(date.specialDays(calendar.Region.Diaspora), [{ tag: 'Pesach' }]);
  assert.deepEqual(date.toGregorian(), { year: 2024, month: 4, day: 23 });

  const chanukah = calendar.HebrewDate.fromGregorian({ year: 2024, month: 12, day: 27 });
  const specialDays = chanukah.specialDays(calendar.Region.Diaspora);
  assert.ok(specialDays.some((day) => day.tag === 'Chanukah' && day.day === 2), JSON.stringify(specialDays));

  const shabbos = calendar.HebrewDate.fromGregorian({ year: 2024, month: 10, day: 26 });
  assert.equal(shabbos.parsha(calendar.Region.Diaspora), calendar.Parsha.Bereshis);
  assert.equal(calendar.parshaName(calendar.Parsha.LechLecha), 'Lech Lecha');
  assert.equal(calendar.yearInfo(5784).is_leap, true);
});

test('limudim', () => {
  const date = calendar.HebrewDate.fromGregorian({ year: 2024, month: 4, day: 23 });
  const daf = limudim.dafYomiBavli(date);
  assert.equal(daf.display, 'Bava Metzia 55');
  assert.equal(daf.tractate, limudim.Tractate.BavaMetzia);
  assert.equal(limudim.mishnaYomis(date).display, 'Nazir 5:5-6');
  assert.equal(limudim.tehillimDisplay(limudim.tehillimMonthly(date)), 'Tehillim 77-78');
});

test('MyZmanim', () => {
  const jerusalem = {
    latitude: 31.778,
    longitude: 35.235,
    elevation: 754,
    eastern_horizon_elevation: 754,
    western_horizon_elevation: 0,
  };
  const day = new myzmanim.MyZmanim(jerusalem, { year: 2026, month: 11, day: 6 }, 40, ianaZone('Asia/Jerusalem'));
  assert.ok(day.get(myzmanim.MyZman.CandleLighting) < day.get(myzmanim.MyZman.SunsetElevated));
  assert.equal(day.all().length, myzmanim.allMyZmanim().length);
});
