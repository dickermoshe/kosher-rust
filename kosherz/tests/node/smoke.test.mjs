// Calls the generated Node bindings. Run after `weaveffi generate --target node` and
// `npm install` in generated/node (see .github/workflows/bindings.yaml):
//
//     node --test tests/node/*.test.mjs
import { test } from 'node:test';
import assert from 'node:assert/strict';

import { calendar, times } from '../../generated/node/index.js';

const { limudim } = calendar;
const { myzmanim } = times;

/**
 * A time zone backed by Node's ICU data. It gives the UTC offset of `name` at an instant, and the
 * instant of a local date and time resolved the way Java's `ZonedDateTime.of` resolves it.
 */
function ianaZone(name) {
  const format = new Intl.DateTimeFormat('en-US', {
    timeZone: name,
    hourCycle: 'h23',
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
  return {
    offsetSeconds(unixSeconds) {
      const instant = new Date(Number(unixSeconds) * 1000);
      const parts = Object.fromEntries(format.formatToParts(instant).map((part) => [part.type, part.value]));
      const local = Date.UTC(parts.year, parts.month - 1, parts.day, parts.hour, parts.minute, parts.second);
      return Math.round((local - instant.getTime()) / 1000);
    },
    localToUnixSeconds(year, month, day, hour, minute, second) {
      const asUtc = Date.UTC(year, month - 1, day, hour, minute, second) / 1000;
      // The offsets a day before and a day after are the only ones the local time can have.
      const before = this.offsetSeconds(BigInt(asUtc - 86400));
      const after = this.offsetSeconds(BigInt(asUtc + 86400));
      // The earlier instant comes from the larger offset. Use it if the clocks show the time then.
      for (const offset of [Math.max(before, after), Math.min(before, after)]) {
        if (this.offsetSeconds(BigInt(asUtc - offset)) === offset) return BigInt(asUtc - offset);
      }
      // The clocks skipped the time. Java moves it later by the gap, using the offset before it.
      return BigInt(asUtc - before);
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
  assert.equal(times.zmanFromJavaName('getSofZmanShmaGRA'), times.Zman.SofZmanShmaGra);
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
  assert.deepEqual(date.holidays(calendar.Region.Diaspora), [{ tag: 'Pesach' }]);
  assert.deepEqual(date.toGregorian(), { year: 2024, month: 4, day: 23 });

  const chanukah = calendar.HebrewDate.fromGregorian({ year: 2024, month: 12, day: 27 });
  const holidays = chanukah.holidays(calendar.Region.Diaspora);
  assert.ok(holidays.some((holiday) => holiday.tag === 'Chanukah' && holiday.day === 2), JSON.stringify(holidays));

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
