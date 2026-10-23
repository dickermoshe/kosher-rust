// Calls the generated WASM bindings on Node.js. Build the module, then from kosherz:
//
//     RUSTFLAGS="-C link-arg=--export-table -C link-arg=--growable-table" \
//         cargo build -p kosherz --target wasm32-unknown-unknown
//     KOSHERZ_LIBRARY=../target/wasm32-unknown-unknown/debug/kosherz.wasm node --test tests/wasm/*.test.mjs
import { before, test } from 'node:test';
import assert from 'node:assert/strict';

import { calendar, init, times } from '../../generated/wasm/index.js';

/** `date` at UTC plus `hours` all year: its two local midnights and the offsets at them. */
function fixedDay(hours, { year, month, day }) {
  const start = Date.UTC(year, month - 1, day) - hours * 3_600_000;
  return {
    start_ms: BigInt(start),
    start_offset_seconds: hours * 3600,
    end_ms: BigInt(start + 86_400_000),
    end_offset_seconds: hours * 3600,
  };
}

const date = { year: 2024, month: 4, day: 23 };
const lakewood = () => times.seaLevelLocation(40.09, -74.22);

before(() => init());

test('sunrise in Lakewood on 2024-04-23 is 06:06:35 EDT', () => {
  const day = new times.Zmanim(lakewood(), date, times.defaultConfig(), fixedDay(-4, date));
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
    () => new times.Zmanim(times.seaLevelLocation(91, 0), date, times.defaultConfig(), fixedDay(-4, date)),
    times.InvalidLatitudeError,
  );
});

// Kiddush Levana is the one time whose day depends on the offset: with UTC-4 it falls on
// 3 December 2024, and with UTC+4 it does not.
test('kiddush levana in Lakewood on 2024-12-03 needs the right offset', () => {
  const december = { year: 2024, month: 12, day: 3 };
  const day = new times.Zmanim(lakewood(), december, times.defaultConfig(), fixedDay(-4, december));
  const wrongSign = new times.Zmanim(lakewood(), december, times.defaultConfig(), fixedDay(4, december));
  const noDay = new times.Zmanim(lakewood(), december, times.defaultConfig(), null);
  try {
    assert.equal(day.get(times.Zman.TchilasZmanKidushLevana3Days), 1733279333504n);
    assert.throws(() => wrongSign.get(times.Zman.TchilasZmanKidushLevana3Days), times.NotApplicableError);
    assert.throws(() => noDay.get(times.Zman.TchilasZmanKidushLevana3Days), times.TimeZoneRequiredError);
  } finally {
    day.close();
    wrongSign.close();
    noDay.close();
  }
});

test('a local day that ends before it starts is a typed error', () => {
  const backwards = { start_ms: 1n, start_offset_seconds: 0, end_ms: 0n, end_offset_seconds: 0 };
  assert.throws(
    () => new times.Zmanim(lakewood(), date, times.defaultConfig(), backwards),
    times.InvalidLocalDayError,
  );
});
