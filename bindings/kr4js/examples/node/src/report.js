// @ts-check
// Builds the day's report with the Hebrew date, what the calendar marks, the learning and the zmanim.
import {
  HebrewDate,
  Zmanim,
  dafYomiBavli,
  localTime,
  mishnaYomis,
  tehillimMonthly,
  today,
} from "kr4js";

/** @typedef {{ name: string, location: import("kr4js").LocationOptions, timeZone: string, region: import("kr4js").Region }} City */

/** Places the app knows by name. @type {Record<string, City>} */
export const CITIES = {
  lakewood: { name: "Lakewood", location: { latitude: 40.0961, longitude: -74.2224, elevation: 30 }, timeZone: "America/New_York", region: "diaspora" },
  jerusalem: { name: "Jerusalem", location: { latitude: 31.778, longitude: 35.235, elevation: 754 }, timeZone: "Asia/Jerusalem", region: "israel" },
  london: { name: "London", location: { latitude: 51.5074, longitude: -0.1278, elevation: 20 }, timeZone: "Europe/London", region: "diaspora" },
  "los-angeles": { name: "Los Angeles", location: { latitude: 34.0522, longitude: -118.2437, elevation: 90 }, timeZone: "America/Los_Angeles", region: "diaspora" },
  melbourne: { name: "Melbourne", location: { latitude: -37.8136, longitude: 144.9631, elevation: 30 }, timeZone: "Australia/Melbourne", region: "diaspora" },
};

/** The zmanim the report shows, in order. @type {import("kr4js").Zman[]} */
export const SHOWN = [
  "alos72Minutes",
  "misheyakir10Point2Degrees",
  "sunrise",
  "sofZmanShmaMga72Minutes",
  "sofZmanShmaGra",
  "sofZmanTfilaGra",
  "chatzosHayom",
  "minchaGedolaGra",
  "minchaKetanaGra",
  "plagHaminchaGra",
  "candleLighting",
  "sunset",
  "tzaisGeonim8Point5Degrees",
  "tzais72Minutes",
];

/** `time` as HH:MM:SS in `timeZone`. @param {Date | null} time @param {string} timeZone */
function clock(time, timeZone) {
  if (!time) return "—";
  const { hour, minute, second } = localTime(time, timeZone);
  return [hour, minute, second].map((n) => String(n).padStart(2, "0")).join(":");
}

/**
 * The report for `city` on `date`, today in the city when left out.
 * @param {City} city
 * @param {import("kr4js").DateInput} [date]
 */
export function report(city, date = today(city.timeZone)) {
  const hebrew = HebrewDate.from(date);
  const zmanim = new Zmanim({ location: city.location, timeZone: city.timeZone, date });
  const shown = new Set(SHOWN);
  const times = zmanim
    .all()
    .filter(({ zman }) => shown.has(zman.id))
    .sort((a, b) => SHOWN.indexOf(a.zman.id) - SHOWN.indexOf(b.zman.id))
    .filter(({ zman }) => zman.id !== "candleLighting" || hebrew.hasCandleLighting(city.region))
    .map(({ zman, time }) => ({ name: zman.name, time: clock(time, city.timeZone) }));

  return {
    city: city.name,
    gregorian: hebrew.toJSON(),
    hebrew: hebrew.toString(),
    specialDays: hebrew.specialDays(city.region).map((day) => (day.day ? `${day.name} ${day.day}` : day.name)),
    parsha: hebrew.parsha(city.region)?.name ?? null,
    dafYomi: String(dafYomiBavli(hebrew) ?? "—"),
    mishnaYomis: String(mishnaYomis(hebrew) ?? "—"),
    tehillim: String(tehillimMonthly(hebrew) ?? "—"),
    zmanim: times,
  };
}

/** `data` as the text the command line prints. @param {ReturnType<typeof report>} data */
export function format(data) {
  const lines = [
    `${data.city} — ${data.gregorian} — ${data.hebrew}`,
    ...(data.specialDays.length ? [data.specialDays.join(", ")] : []),
    ...(data.parsha ? [`Parshas ${data.parsha}`] : []),
    "",
    `Daf Yomi     ${data.dafYomi}`,
    `Mishna Yomis ${data.mishnaYomis}`,
    `Tehillim     ${data.tehillim}`,
    "",
  ];
  const width = Math.max(...data.zmanim.map((z) => z.name.length));
  for (const { name, time } of data.zmanim) lines.push(`${name.padEnd(width)}  ${time}`);
  return lines.join("\n");
}
