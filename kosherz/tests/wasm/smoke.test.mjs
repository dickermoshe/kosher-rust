// Calls the generated WASM bindings on Node.js. Build the module, then from kosherz:
//
//     RUSTFLAGS="-C link-arg=--export-table -C link-arg=--growable-table" \
//         cargo build -p kosherz --target wasm32-unknown-unknown
//     KOSHERZ_LIBRARY=../target/wasm32-unknown-unknown/debug/kosherz.wasm node --test tests/wasm/*.test.mjs
import { before, test } from 'node:test';
import assert from 'node:assert/strict';

import { calendar, init, times } from '../../generated/wasm/index.js';

/** Eastern daylight time, every day of the year. */
const edt = { offsetSeconds: () => -4 * 3600 };
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
