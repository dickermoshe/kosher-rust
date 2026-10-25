# kr4js

kr4js is a JavaScript and TypeScript library for the Jewish calendar. It can do three things.

- **Zmanim.** It calculates halachic times, such as sunrise, the latest time for Shema and candle
  lighting, for any place on earth. There are over 160 of them, covering the common opinions.
- **The Hebrew calendar.** It converts dates between the Gregorian and Hebrew calendars, and tells
  you the holidays, fasts, weekly *parsha* and molad for any day.
- **Learning schedules.** It tells you the day's Daf Yomi, Mishna Yomis, Yerushalmi, Tehillim and
  Pirkei Avos.

The calculations are done by the [kosher-rust](https://github.com/dickermoshe/kosher-rust) crate,
which is written in Rust and compiled to WebAssembly. You don't need to know anything about Rust or
WebAssembly to use this package.

## Contents

- [Installation](#installation)
- [A first example](#a-first-example)
- [Zmanim](#zmanim)
- [Time zones](#time-zones)
- [The Hebrew calendar](#the-hebrew-calendar)
- [Learning schedules](#learning-schedules)
- [Matching the MyZmanim website](#matching-the-myzmanim-website)
- [Where it runs](#where-it-runs)
- [Building from source](#building-from-source)
- [License](#license)

## Installation

```bash
npm install @dickermoshe/kr4js
```

The package works in browsers, Node, Bun and Deno with the same import. You don't need to configure
your bundler, and you can call it as soon as it is imported. Cloudflare Workers need a small extra
step, which is explained in [Cloudflare Workers](#cloudflare-workers).

## A first example

This prints the Hebrew date, the latest time for Shema and the Daf Yomi for 23 April 2024 in
Lakewood, New Jersey.

```ts
import { HebrewDate, Zmanim, dafYomiBavli } from "@dickermoshe/kr4js";

const date = new Date(2024, 3, 23); // 23 April 2024

const zmanim = new Zmanim({
  location: { latitude: 40.09, longitude: -74.22, elevation: 30 },
  timeZone: "America/New_York",
  date,
});

const shma = zmanim.get("sofZmanShmaGra");

console.log(HebrewDate.from(date).toString());
// 15 Nisan 5784

console.log(shma?.toLocaleTimeString("en-US", { timeZone: "America/New_York" }));
// 9:31:00 AM

console.log(String(dafYomiBavli(date)));
// Bava Metzia 55
```

## Zmanim

### Setting up a location

To calculate zmanim, create a `Zmanim` object for one place on one day.

```ts
import { Zmanim } from "@dickermoshe/kr4js";

const zmanim = new Zmanim({
  location: { latitude: 40.09, longitude: -74.22, elevation: 30 },
  timeZone: "America/New_York",
  date: new Date(2024, 3, 23),
});
```

These are the three things it needs.

| Option | What it is |
|---|---|
| `location` | The place. See [The location](#the-location) below. |
| `timeZone` | The place's time zone, such as `"America/New_York"`. See [Time zones](#time-zones). |
| `date` | The day, as a JavaScript `Date`. Use `new Date()` for today. See [Ways to give a date](#ways-to-give-a-date). |

### The location

| Field | What it is |
|---|---|
| `latitude` | Degrees north of the equator. Places south of the equator are negative. |
| `longitude` | Degrees east of Greenwich. Places west of Greenwich, such as America, are negative. |
| `elevation` | How high the observer is above sea level, in meters. It is optional and 0 by default. |
| `easternHorizon` | How high the land the sun rises over is above sea level, in meters. It is optional and 0 by default. |
| `westernHorizon` | How high the land the sun sets behind is above sea level, in meters. It is optional and 0 by default. |

The horizons matter in hilly places. An observer at 800 meters looking east over a sea-level plain
sees the sun rise earlier than one looking east at a mountain that is also 800 meters high. Sunrise
is adjusted for the observer's height above the eastern horizon, and sunset for the height above the
western horizon.

```ts
new Zmanim({
  location: {
    latitude: 31.778,
    longitude: 35.235,
    elevation: 754,
    easternHorizon: 754,
    westernHorizon: 0,
  },
  timeZone: "Asia/Jerusalem",
  date: new Date(2024, 3, 23),
});
```

Sunrise and sunset themselves always use the elevation and horizons. The other zmanim built on
sunrise and sunset use them only when the `useElevation` setting is `true`.

### Getting a time

Call `get` with the name of a zman. It returns a normal JavaScript `Date`.

```ts
const shma = zmanim.get("sofZmanShmaGra");
const sunset = zmanim.get("sunset");
```

A `Date` is a moment in time and has no time zone of its own. To show it to a user, format it in the
location's time zone.

```ts
shma?.toLocaleTimeString("en-US", { timeZone: "America/New_York" }); // "9:31:00 AM"
sunset?.toLocaleTimeString("en-US", { timeZone: "America/New_York" }); // "7:45:11 PM"
```

### When there is no time

Some zmanim don't happen every day, so `get` returns `null` for them. There are two reasons this can
happen.

1. The time only exists on certain days. For example, the latest time to eat chametz only exists on
   Erev Pesach.
2. The sun never reaches the position the time is based on. For example, in northern Norway in the
   summer the sun never goes 16.1° below the horizon, so there is no dawn at 16.1°.

```ts
zmanim.get("sofZmanAchilasChametzGra"); // null, because 23 April 2024 is not Erev Pesach
```

Your code should always handle `null`, which is why the examples above use `?.`.

### Getting every time at once

`all()` returns every zman for the day. Each entry has the zman's details, its time, and the reason
when it has no time.

```ts
for (const { zman, time, unavailable } of zmanim.all()) {
  if (time) {
    console.log(zman.name, time.toISOString());
  } else {
    console.log(zman.name, "has no time today because", unavailable);
  }
}
```

The reason in `unavailable` is one of these.

| Value | Meaning |
|---|---|
| `"notApplicable"` | The time doesn't apply to this day, like the chametz times on any day but Erev Pesach. |
| `"allDay"` | The sun stays above the angle the time needs for the whole day. |
| `"allNight"` | The sun stays below the angle the time needs for the whole day. |

### Another day in the same place

`on` gives you the same location, time zone and settings for a different day.

```ts
const tomorrow = zmanim.on(new Date(2024, 3, 24));
tomorrow.get("sunrise");
```

### Finding the name of a zman

Every zman has an id, which is what you pass to `get`. Your editor will suggest the ids as you type,
but you can also list them in code.

```ts
import { allZmanim, zmanimIn, zmanInfo } from "@dickermoshe/kr4js";

allZmanim();                  // every zman, 167 in all
zmanimIn("sofZmanShma");      // only the zmanim for the latest time for Shema
zmanInfo("sofZmanShmaGra");   // { id: "sofZmanShmaGra", name: "Sof Zman Shma (GR'A)", category: "sofZmanShma", … }
```

The categories are `twilight`, `alos`, `misheyakir`, `netz`, `sofZmanShma`, `sofZmanTfila`,
`sofZmanAchilasChametz`, `sofZmanBiurChametz`, `chatzosHayom`, `minchaGedola`, `plagHamincha`,
`samuchLeMinchaKetana`, `minchaKetana`, `bainHashmashos`, `candleLighting`, `shkiya`, `tzais`,
`kidushLevana` and `chatzosHalayla`.

### A time that isn't in the list

If you follow an opinion based on the sun's angle that the list doesn't include, you can calculate
it yourself. `sunriseOffsetByDegrees` gives the time the sun is a number of degrees below the horizon
in the morning, and `sunsetOffsetByDegrees` does the same in the evening.

```ts
zmanim.sunriseOffsetByDegrees(11.5); // when the sun is 11.5° below the horizon before sunrise
zmanim.sunsetOffsetByDegrees(8.5);   // when the sun is 8.5° below the horizon after sunset
```

Like `get`, these return `null` when the sun doesn't get that low.

### Settings

You can change how the times are calculated by adding settings next to `location`, `timeZone` and
`date`. Every setting is optional.

```ts
new Zmanim({
  location: { latitude: 31.778, longitude: 35.235, elevation: 754 },
  timeZone: "Asia/Jerusalem",
  date: new Date(2024, 3, 23),
  candleLightingOffset: 40,
  useElevation: true,
});
```

| Setting | Default | What it does |
|---|---|---|
| `candleLightingOffset` | `18` | How many minutes before sunset candles are lit. Jerusalem uses 40. |
| `useElevation` | `false` | Whether the location's elevation and horizons move sunrise and sunset in the zmanim that are based on them. |
| `ateretTorahSunsetOffset` | `40` | How many minutes after sunset Tzais Ateret Torah is. |
| `useAstronomicalChatzos` | `true` | Whether chatzos is the moment the sun is highest. When `false`, it is halfway between sunrise and sunset. |
| `useAstronomicalChatzosForOtherZmanim` | `false` | Whether the zmanim measured in proportional hours count the morning and afternoon separately from chatzos. |
| `calculator` | `"noaa"` | The formula for the sun's position. It can be `"noaa"`, `"meeus"` or `"spa"`. They agree to within seconds for sunrise and sunset. |
| `applyDeltaT` | `true` | Whether `"meeus"` and `"spa"` correct for the slowing of the earth's rotation, which is about a minute. |

## Time zones

A time zone is an IANA name, such as `"America/New_York"`, `"Europe/London"` or
`"Asia/Jerusalem"`.

```ts
new Zmanim({ location, date, timeZone: "Europe/London" });
```

kr4js doesn't include its own time zone database. It looks the name up in the data that the browser
or runtime already has, which is the same data that `toLocaleString` uses. Every modern browser,
Node, Bun, Deno and Cloudflare Workers have it. A name the runtime doesn't know throws a
`RangeError`.

### When you need a time zone

Every zman works when you give a time zone, so you should always pass one.

If you leave it out, most zmanim still work, but some don't. The four Kiddush Levana times throw an
error, and so does every time at a location more than 150° east or west of Greenwich. The error's
`name` is `"ZmanimError"` and its `code` is `"timeZoneRequired"`.

## The Hebrew calendar

### Converting a Gregorian date

`HebrewDate.from` gives you the Hebrew date of a Gregorian day.

```ts
import { HebrewDate } from "@dickermoshe/kr4js";

const day = HebrewDate.from(new Date(2024, 3, 23));

day.toString();     // "15 Nisan 5784"
day.day;            // 15
day.month.name;     // "Nisan"
day.month.hebrew;   // "ניסן"
day.year;           // 5784
day.weekday;        // 2, which is Tuesday (0 is Sunday and 6 is Shabbos)
```

A Hebrew day starts at nightfall, but a `HebrewDate` stands for the whole civil day it overlaps the
most. So 23 April 2024 is 15 Nisan, even though 15 Nisan began the evening before.

`HebrewDate.from(new Date())` gives today's Hebrew date.

### Converting a Hebrew date

`HebrewDate.of` makes a date from a Hebrew year, month and day. Use `gregorian` to get the
Gregorian day back.

```ts
HebrewDate.of(5785, "nisan", 15).gregorian; // { year: 2025, month: 4, day: 13 }
```

The months are `tishrei`, `cheshvan`, `kislev`, `teves`, `shevat`, `adarI`, `adar`, `nisan`, `iyar`,
`sivan`, `tammuz`, `av` and `elul`. In a leap year, `adarI` is Adar I and `adar` is Adar II.

Dates from 1 January of year 1 through 29 Elul 9999 are supported.

### Israel and the diaspora

Some answers depend on whether you are in Israel. Outside Israel, Yom Tov has a second day, and that
sometimes changes the weekly *parsha*. Methods that depend on this take `"israel"` or `"diaspora"`.

```ts
const day = HebrewDate.from(new Date(2022, 3, 23)); // 22 Nisan 5782, a Shabbos

day.parsha("israel")?.name;   // "Achrei Mos"
day.parsha("diaspora")?.name; // undefined, because it is the last day of Pesach
```

### Holidays and special days

`specialDays` lists the holidays, fasts and other notable days that fall on a date.

```ts
const day = HebrewDate.from(new Date(2024, 3, 23));

day.specialDays("diaspora");
// [{ id: "pesach", name: "Pesach", hebrew: "פסח", isAssurBemelacha: true, isFastDay: false, isModern: false }]

day.is("pesach", "diaspora");         // true
day.isAssurBemelacha("diaspora");     // true, because it is Yom Tov
day.hasCandleLighting("diaspora");    // true, because tomorrow is also Yom Tov
```

Chanukah and the Omer also say which day it is.

```ts
HebrewDate.from(new Date(2024, 11, 27)).specialDays("diaspora");
// [{ id: "chanukah", day: 2, name: "Chanukah", … }]
```

The list includes the modern Israeli days, such as Yom Ha'atzmaut. To leave them out, filter on
`isModern`. `allSpecialDays()` lists every special day kr4js knows about.

### The weekly parsha

| Method | What it gives |
|---|---|
| `parsha(region)` | The *parsha* read on this Shabbos. It is `undefined` on a weekday, and when Yom Tov falls on Shabbos. |
| `nextParsha(region)` | The *parsha* of the next Shabbos that has one. On Shabbos, this is next week's. |
| `specialParsha(region)` | The special Shabbos this is, such as Shekalim or Hagadol, or `undefined`. |

```ts
HebrewDate.from(new Date(2024, 4, 4)).parsha("israel")?.name; // "Achrei Mos"
```

`allParshiyos()` lists every *parsha*, including the double ones and the special Shabbosos.

### The Hebrew year

`hebrewYear` gives details about the year a date is in. You can also get one with
`HebrewYear.of(5784)`.

```ts
const year = HebrewDate.from(new Date(2024, 3, 23)).hebrewYear;

year.isLeap;              // true
year.days;                // 383
year.roshHashana.gregorian; // { year: 2023, month: 9, day: 16 }
year.daysIn("adarI");     // 30
year.months;              // every month of the year, in order
```

### The molad

`molad()` gives the molad of a date's month. The weekday, hours, minutes and *chalakim* are in
Jerusalem mean time, the way it is announced in shul. `time` is the same moment as a `Date`.

```ts
HebrewDate.from(new Date(2024, 3, 23)).molad();
// { weekday: 1, hours: 22, minutes: 57, chalakim: 7, time: Date }
```

### Moving between dates

```ts
day.addDays(1).toString();  // "16 Nisan 5784"
day.addDays(-1);            // the day before
day.equals(other);          // whether two dates are the same day
day.compare(other);         // negative if day is earlier, positive if later, 0 if the same
```

### Ways to give a date

Anywhere kr4js asks for a date, you can give it a JavaScript `Date`. kr4js uses the day that the
`Date`'s own `getFullYear`, `getMonth` and `getDate` give, which is the day on the clock of the
computer or browser the code runs on. The time of day doesn't matter.

```ts
new Date();             // today
new Date(2024, 3, 23);  // 23 April 2024, since a Date counts months from 0
```

Be careful with `new Date("2024-04-23")`. JavaScript reads a date written that way as midnight in
UTC, which is still 22 April in America. Write `new Date(2024, 3, 23)` or `new Date("2024-04-23T00:00")`
instead.

You can also give any of these.

- A `HebrewDate`.
- An object with `year`, `month` and `day`, such as `{ year: 2024, month: 4, day: 23 }`. Here the
  month is counted from 1. A `Temporal.PlainDate` works too.

## Learning schedules

Each schedule is a function that takes a date and returns that day's portion. It returns `undefined`
when nothing is scheduled, such as a day before the schedule began. Every portion can be turned into
readable text with `String()`.

```ts
import { dafYomiBavli, mishnaYomis, pirkeiAvos } from "@dickermoshe/kr4js";

const date = new Date(2024, 3, 23);

String(dafYomiBavli(date)); // "Bava Metzia 55"
String(mishnaYomis(date));  // "Nazir 5:5-6"
```

The portions also have their parts as separate fields, so you can format them yourself.

```ts
const daf = dafYomiBavli(date);

daf?.tractate.name;   // "Bava Metzia"
daf?.tractate.hebrew; // "בבא מציעא"
daf?.daf;             // 55
```

These are the schedules.

| Function | What it gives | Example for 23 April 2024 |
|---|---|---|
| `dafYomiBavli` | One daf of the Bavli each day, since 1923 | Bava Metzia 55 |
| `dafHashavuaBavli` | One daf of the Bavli each week, since 2005 | Kesubos 85 |
| `amudYomiDirshu` | One amud of the Bavli each day, since 2023 | Shabbos 34b |
| `dafYomiYerushalmi` | One daf of the Yerushalmi (Vilna edition) each day, since 1980 | Eruvin 40 |
| `mishnaYomis` | Two mishnayos each day, since 1947 | Nazir 5:5-6 |
| `tehillimMonthly` | All of Tehillim each Hebrew month | Tehillim 77-78 |
| `pirkeiAvos` | A chapter of Pirkei Avos on summer Shabbos afternoons | |

`pirkeiAvos` also needs to know whether you are in Israel, because the second day of Yom Tov can
shift the order.

```ts
pirkeiAvos(new Date(2024, 4, 4), "israel")?.chapters; // [1]
```

## Matching the MyZmanim website

`MyZmanim` calculates times the same way the [MyZmanim](https://www.myzmanim.com/) website does. Use
it when your users compare your times with that site and expect them to match.

To match the site, use the coordinates, elevation and horizons that MyZmanim shows for the area.
The fields are the same as in [The location](#the-location).

```ts
import { MyZmanim } from "@dickermoshe/kr4js";

const times = new MyZmanim({
  location: {
    latitude: 31.778,
    longitude: 35.235,
    elevation: 754,
    easternHorizon: 754,
    westernHorizon: 0,
  },
  timeZone: "Asia/Jerusalem",
  date: new Date(2026, 10, 6),
  candleLightingOffset: 40,
});

times.get("candleLighting")?.toLocaleTimeString("en-US", { timeZone: "Asia/Jerusalem" });
// "4:10:20 PM"
```

`MyZmanim` has the same `get`, `all` and `on` methods as `Zmanim`, but it uses the names from the
site's "All zmanim" table, such as `shemaGra` and `nightShabbos`. `allMyZmanim()` lists them.

## Where it runs

| Runtime | Setup |
|---|---|
| Browsers, with Vite, webpack, Rollup, esbuild, Parcel or no bundler | None |
| Node 22.12 and later | None |
| Bun and Deno | None |
| Cloudflare Workers | Two extra lines, shown below |

The package needs a few milliseconds to start up when it is first imported.

### Cloudflare Workers

Cloudflare Workers don't allow WebAssembly to be loaded while the code is running. Wrangler loads it
ahead of time instead, when the Worker is uploaded.

To use kr4js in a Worker, import the WebAssembly file and the `slim` version of the package. Then
pass the file to `init` before you use anything else. Wrangler needs no extra configuration for
this.

```ts
import wasm from "@dickermoshe/kr4js/kr4js.wasm";
import { init, Zmanim } from "@dickermoshe/kr4js/slim";

export default {
  async fetch(request) {
    await init(wasm);
    // You can use Zmanim and everything else from here on.
  },
};
```

If you forget this step, importing the package throws an error that tells you to do it.

### Smaller browser bundles

By default the WebAssembly is stored inside the JavaScript as text. That makes it about a third
larger than it needs to be, around 160 KB gzipped instead of 125 KB. If download size matters for
your site, you can load the WebAssembly as a separate file instead.

To do this, import from `@dickermoshe/kr4js/slim` and call `init` once before anything else.

```ts
import { HebrewDate, init } from "@dickermoshe/kr4js/slim";

await init();

HebrewDate.from(new Date(2024, 3, 23)).toString(); // "15 Nisan 5784"
```

Vite, webpack 5 and Parcel copy the WebAssembly file into your build for you, with no extra setup.
The browser starts compiling it while it is still downloading.

If you host the file somewhere else, pass its location to `init`. It accepts a URL, a `fetch`
response, the file's bytes or a compiled `WebAssembly.Module`. The file is available from the
package as `@dickermoshe/kr4js/kr4js.wasm`.

Anything you call from `@dickermoshe/kr4js/slim` before `init` has finished throws an error.

## Building from source

You need [rustup](https://rustup.rs/), [wasm-pack](https://github.com/drager/wasm-pack) and Node.
The repository's `rust-toolchain.toml` installs the right Rust version and everything else the build
needs.

```bash
npm install
npm run build
npm test
```

`npm run test:bun` runs the same tests under Bun, and `npm run test:rust` runs the Rust tests.

The [examples](examples) folder has three complete projects, each with its own tests. There is a
Vite and React app, a command line app for Node and Bun, and a Cloudflare Worker.

## License

LGPL-2.1-or-later.
