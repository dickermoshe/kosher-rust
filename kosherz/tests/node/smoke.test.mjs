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

/** `date` in the IANA time zone `zone`, through Temporal: its two local midnights and the offsets
 * at them. */
function localDay(zone, date) {
  const day = Temporal.PlainDate.from(date);
  const start = day.toZonedDateTime(zone);
  const end = day.add({ days: 1 }).toZonedDateTime(zone);
  return {
    start_ms: BigInt(start.epochMilliseconds),
    start_offset_seconds: start.offsetNanoseconds / 1e9,
    end_ms: BigInt(end.epochMilliseconds),
    end_offset_seconds: end.offsetNanoseconds / 1e9,
  };
}

/** `ms` as HH:MM:SS in New York. */
function newYorkTime(ms) {
  return new Date(Number(ms)).toLocaleTimeString('en-GB', { timeZone: 'America/New_York' });
}

const lakewood = times.seaLevelLocation(40.09, -74.22);
const newYork = (date) => localDay('America/New_York', date);

test('zmanim', () => {
  const april = { year: 2024, month: 4, day: 23 };
  const day = new times.Zmanim(lakewood, april, times.defaultConfig(), newYork(april));
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
  const november = { year: 2024, month: 11, day: 4 };
  const day = new times.Zmanim(lakewood, november, times.defaultConfig(), newYork(november));
  const time = day.get(times.Zman.TchilasZmanKidushLevana3Days);
  assert.equal(newYorkTime(time), '08:44:50');

  const noZone = new times.Zmanim(lakewood, { year: 2024, month: 11, day: 4 }, times.defaultConfig(), null);
  assert.throws(() => noZone.get(times.Zman.TchilasZmanKidushLevana3Days), times.TimeZoneRequiredError);
});

test('a local day that ends before it starts is a typed error', () => {
  const backwards = { start_ms: 1n, start_offset_seconds: 0, end_ms: 0n, end_offset_seconds: 0 };
  assert.throws(
    () => new times.Zmanim(lakewood, { year: 2024, month: 4, day: 23 }, times.defaultConfig(), backwards),
    times.InvalidLocalDayError,
  );
});

test('invalid input is a typed error', () => {
  const config = times.defaultConfig();
  const january = newYork({ year: 2024, month: 1, day: 1 });
  assert.throws(
    () => new times.Zmanim(times.seaLevelLocation(91, 0), { year: 2024, month: 1, day: 1 }, config, january),
    times.InvalidLatitudeError,
  );
  assert.throws(
    () => new times.Zmanim(lakewood, { year: 2024, month: 2, day: 30 }, config, january),
    times.InvalidGregorianDateError,
  );
  assert.throws(
    () => new times.Zmanim(lakewood, { year: 2024, month: 1, day: 1 }, { ...config, candle_lighting_minutes: 1441 }, january),
    times.InvalidMinutesError,
  );
  assert.throws(
    () => new myzmanim.MyZmanim(lakewood, { year: 2024, month: 1, day: 1 }, NaN, january),
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
  const november = { year: 2026, month: 11, day: 6 };
  const day = new myzmanim.MyZmanim(jerusalem, november, 40, localDay('Asia/Jerusalem', november));
  assert.ok(day.get(myzmanim.MyZman.CandleLighting) < day.get(myzmanim.MyZman.SunsetElevated));
  assert.equal(day.all().length, myzmanim.allMyZmanim().length);
});
