// Calls the generated WASM bindings on Node.js. Build the module, then from kosherz:
//
//     RUSTFLAGS="-C link-arg=--export-table -C link-arg=--growable-table" \
//         cargo build -p kosherz --target wasm32-unknown-unknown
//     KOSHERZ_LIBRARY=../target/wasm32-unknown-unknown/debug/kosherz.wasm node --test tests/wasm/*.test.mjs
import { before, test } from 'node:test';
import assert from 'node:assert/strict';

import { calendar, init, times } from '../../generated/wasm/index.js';

/** UTC plus `hours`, every day of the year. */
const fixed = (hours) => ({
  offsetSeconds: () => hours * 3600,
  localToUnixSeconds: (year, month, day, hour, minute, second) =>
    BigInt(Date.UTC(year, month - 1, day, hour, minute, second) / 1000 - hours * 3600),
});
/** Eastern daylight time, every day of the year. */
const edt = fixed(-4);
const date = { year: 2024, month: 4, day: 23 };

before(() => init());

test('sunrise in Lakewood on 2024-04-23 is 06:06:35 EDT', () => {
  const day = new times.Zmanim(times.seaLevelLocation(40.09, -74.22), date, times.defaultConfig(), edt);
  try {
    assert.equal(day.get(times.Zman.Sunrise) / 1000n, 1713866795n);
  } finally {
    day.close();
  }
});

test('daf yomi on 2024-04-23 is Bava Metzia 55', () => {
  const hebrew = calendar.HebrewDate.fromGregorian(date);
  try {
    assert.equal(calendar.limudim.dafYomiBavli(hebrew)?.display, 'Bava Metzia 55');
  } finally {
    hebrew.close();
  }
});

test('latitude 91 throws InvalidLatitudeError', () => {
  assert.throws(
    () => new times.Zmanim(times.seaLevelLocation(91, 0), date, times.defaultConfig(), edt),
    times.InvalidLatitudeError,
  );
});

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
test('kiddush levana in Lakewood on 2024-12-03 needs the right offset', () => {
  const december = { year: 2024, month: 12, day: 3 };
  const lakewood = times.seaLevelLocation(40.09, -74.22);
  const day = new times.Zmanim(lakewood, december, times.defaultConfig(), edt);
  const wrongSign = new times.Zmanim(lakewood, december, times.defaultConfig(), fixed(4));
  const noZone = new times.Zmanim(lakewood, december, times.defaultConfig(), null);
  try {
    assert.equal(day.get(times.Zman.TchilasZmanKidushLevana3Days), 1733279333504n);
    assert.throws(() => wrongSign.get(times.Zman.TchilasZmanKidushLevana3Days), times.NotApplicableError);
    assert.throws(() => noZone.get(times.Zman.TchilasZmanKidushLevana3Days), times.TimeZoneRequiredError);
  } finally {
    day.close();
    wrongSign.close();
    noZone.close();
  }
});

test('a throwing time zone callback is a typed error', () => {
  const broken = {
    offsetSeconds() {
      throw new Error('no time zone data');
    },
    localToUnixSeconds() {
      throw new Error('no time zone data');
    },
  };
  const day = new times.Zmanim(times.seaLevelLocation(40.09, -74.22), date, times.defaultConfig(), broken);
  try {
    assert.throws(() => day.get(times.Zman.Sunrise), times.TimeZoneCallbackFailedError);
  } finally {
    day.close();
  }
});
