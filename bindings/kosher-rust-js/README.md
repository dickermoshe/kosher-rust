# kosher-rust for JavaScript

Jewish calendar, *zmanim* and daily learning schedules for JavaScript and TypeScript. The
calculations are the [kosher-rust](https://github.com/dickermoshe/kosher-rust) crate, compiled to
WebAssembly.

- **Calendar.** Hebrew dates, holidays, fasts, the weekly *parsha* and the molad.
- **Zmanim.** Sunrise, sof zman shma, candle lighting and over 160 other halachic times for any
  place on earth.
- **Limudim.** Daf Yomi, Mishna Yomis, Yerushalmi, Tehillim, Pirkei Avos and more.

It runs in browsers, Node, Bun and Deno with the same import. There is nothing to configure, no
plugin to add to your bundler, and no `await` before the first call. Cloudflare Workers and other
runtimes that don't compile WebAssembly while running need two more lines, shown in
[Where it runs](#where-it-runs).

```bash
npm install kosher-rust
```

## Zmanim

Give `Zmanim` a location, a date and a time zone, then ask for any zman. Times come back as
JavaScript `Date`s.

```ts
import { Zmanim } from "kosher-rust";

const zmanim = new Zmanim({
  location: { latitude: 40.09, longitude: -74.22, elevation: 30 },
  timeZone: "America/New_York",
  date: { year: 2024, month: 4, day: 23 },
});

const shma = zmanim.get("sofZmanShmaGra");
shma?.toLocaleTimeString("en-US", { timeZone: "America/New_York" }); // "9:31:00 AM"

for (const { zman, time } of zmanim.all()) {
  console.log(zman.name, time?.toISOString() ?? "none today");
}
```

`get` returns `null` when the day has no such time. In the far north in summer, the sun never sinks
16.1° below the horizon, so there is no *alos* at 16.1°. The latest time for chametz only exists on
Erev Pesach. `all()` lists every zman with its time, and gives the reason in `unavailable` when
there is none.

Use `zmanim.on({ year: 2024, month: 4, day: 24 })` for the same place on another day. `allZmanim()` lists every zman with its
id, name and category, and `zmanimIn("sofZmanShma")` gives one category.

### Settings

Every setting is optional.

```ts
new Zmanim({
  location: { latitude: 31.778, longitude: 35.235, elevation: 754 },
  timeZone: "Asia/Jerusalem",
  date: { year: 2024, month: 4, day: 23 },
  candleLightingOffset: 40,   // minutes before sunset, 18 by default
  useElevation: true,         // elevation adjusts the zmanim built on sunrise and sunset
  calculator: "spa",          // "noaa" (the default), "meeus" or "spa"
});
```

For an opinion the list doesn't have, `sunriseOffsetByDegrees(11.5)` and `sunsetOffsetByDegrees(8.5)`
give the time the sun is that far below the horizon.

## Time zones

The package has no time zone database of its own. Every zman works with a time zone. Without one,
the four Kiddush Levana times throw an error, and so does every time more than 150° east or west of
Greenwich. Its `name` is `"ZmanimError"` and its `code` is `"timeZoneRequired"`.

A time zone can be an IANA name. Names are looked up in the time zone data the browser or runtime
already has, the same data `toLocaleString` uses.

```ts
new Zmanim({ location, date, timeZone: "Europe/London" });
```

It can also come from any date library you already use. Pass an object whose `offsetAt` takes Unix
milliseconds and returns the UTC offset in minutes east of UTC, or pass that function by itself.

```ts
import { DateTime } from "luxon";
import { Temporal } from "@js-temporal/polyfill";
import { getTimezoneOffset } from "date-fns-tz";

// Luxon
const zone = { offsetAt: (ms) => DateTime.fromMillis(ms, { zone: "America/New_York" }).offset };

// Temporal
const zone = {
  offsetAt: (ms) =>
    Temporal.Instant.fromEpochMilliseconds(ms).toZonedDateTimeISO("America/New_York").offsetNanoseconds / 6e10,
};

// date-fns-tz
const zone = (ms) => getTimezoneOffset("America/New_York", ms) / 60_000;

new Zmanim({ location, date, timeZone: zone });
```

`fixedOffset(-300)` is a zone that is always five hours behind UTC. `localTime(date, zone)` shows a
time on the clock of any zone, and `today(zone)` gives today's date there.

## The Hebrew calendar

A Gregorian date converts to a `HebrewDate`, which answers questions about that day. Pass
`"israel"` or `"diaspora"` wherever they differ.

```ts
import { HebrewDate } from "kosher-rust";

const day = HebrewDate.from({ year: 2024, month: 4, day: 23 });
day.toString();                        // "15 Nisan 5784"
day.specialDays("diaspora");           // [{ id: "pesach", name: "Pesach", hebrew: "פסח", … }]
day.isAssurBemelacha("diaspora");      // true
day.hebrewYear.isLeap;                 // true

HebrewDate.from({ year: 2024, month: 5, day: 4 }).parsha("israel")?.name; // "Achrei Mos"
HebrewDate.of(5785, "nisan", 15).gregorian; // { year: 2025, month: 4, day: 13 }
```

A `HebrewDate` is the civil day it mostly overlaps, so 23 April 2024 is 15 Nisan even though the
Hebrew day began the evening before. It covers 1 January of year 1 through 29 Elul 9999.

Dates can be given as `{ year, month, day }` objects such as a `Temporal.PlainDate`, as a
`HebrewDate`, or as a `Date`. A `Date` is read in the runtime's local time zone. `new Date("2024-04-23")`
is midnight UTC, which is still the 22nd in America, so prefer the object.

## Learning schedules

Each schedule gives that day's portion, or `undefined` when nothing is scheduled.

```ts
import { dafYomiBavli, mishnaYomis, pirkeiAvos } from "kosher-rust";

const date = { year: 2024, month: 4, day: 23 };
String(dafYomiBavli(date)); // "Bava Metzia 55"
String(mishnaYomis(date));  // "Nazir 5:5-6"
pirkeiAvos({ year: 2024, month: 5, day: 4 }, "israel")?.chapters; // [1]
```

| Function | Gives | |
|---|---|---|
| `dafYomiBavli` | `Daf` | one daf a day, since 1923 |
| `dafHashavuaBavli` | `Daf` | one daf a week, since 2005 |
| `amudYomiDirshu` | `Amud` | one amud a day, since 2023 |
| `dafYomiYerushalmi` | `Daf` | Vilna edition, since 1980 |
| `mishnaYomis` | `MishnaPair` | two mishnayos a day, since 1947 |
| `tehillimMonthly` | `TehillimPortion` | the book of Tehillim each Hebrew month |
| `pirkeiAvos` | `PirkeiAvosPortion` | summer Shabbos afternoons |

## MyZmanim

`MyZmanim` calculates the times the way [MyZmanim](https://www.myzmanim.com/) does. Use it when
your users compare your times with that site, with the site's own coordinates, elevation and
horizons for the area.

```ts
import { MyZmanim } from "kosher-rust";

const times = new MyZmanim({
  location: { latitude: 31.778, longitude: 35.235, elevation: 754, easternHorizon: 754, westernHorizon: 0 },
  timeZone: "Asia/Jerusalem",
  date: { year: 2026, month: 11, day: 6 },
  candleLightingOffset: 40,
});
times.get("candleLighting");
```

## Where it runs

| Runtime | |
|---|---|
| Browsers, with Vite, webpack, Rollup, esbuild, Parcel or no bundler | The WebAssembly is inlined in the module. |
| Node 22.12 and later, Bun and Deno | The WebAssembly file is read from the package. |

The WebAssembly is compiled when the package is imported, which takes a few milliseconds.
### Cloudflare Workers and other edge runtimes

Some runtimes, such as Cloudflare Workers, don't compile WebAssembly while running. Their build tool
compiles it when the code is uploaded instead. Import the WebAssembly file and the slim entry, and
pass the file to `init` before the first call.

```ts
import wasm from "kosher-rust/kosher_rust.wasm";
import { init, Zmanim } from "kosher-rust/slim";

export default {
  async fetch(request) {
    await init(wasm);
    // ...
  },
};
```

Where the runtime refuses to compile it, importing the main entry throws an error that says to do
this.

### Smaller browser bundles

In browsers the WebAssembly is inlined as base64, which costs about a third more than the file
itself, roughly 160 KB gzipped instead of 125 KB. To serve it as a separate file instead, import
`kosher-rust/slim` and call `init` once before anything else.

```ts
import { HebrewDate, init } from "kosher-rust/slim";

await init();
HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString(); // "15 Nisan 5784"
```

`init` fetches the file from next to the package's own module. Vite, webpack 5 and Parcel copy the
file into your build without any setup, and the browser compiles it while it downloads. To load it
from somewhere else, pass `init` a URL, a `fetch` response, the file's bytes or a compiled
`WebAssembly.Module`. The file is exported as `kosher-rust/kosher_rust.wasm`.

Calling anything in `kosher-rust/slim` before `init` has finished throws.

## Building from source

You need [rustup](https://rustup.rs/), [wasm-pack](https://github.com/drager/wasm-pack) and Node.
The repository's `rust-toolchain.toml` installs the Rust version, the `wasm32-unknown-unknown`
target and the standard library source that the build uses.

```bash
npm install
npm run build
npm test
```

`npm run test:bun` runs the same tests under Bun, and `npm run test:rust` runs the Rust tests. The [examples](examples) are a Vite and React app,
a Node and Bun command line app, and a Cloudflare Worker, each with its own tests.

## License

LGPL-2.1-or-later.
