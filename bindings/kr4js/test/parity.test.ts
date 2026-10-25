// The package must give exactly what the Rust crate gives with the real time zone database.
// test/fixtures/parity.json is written by `UPDATE_GENERATED=1 cargo test -p kr4js`.
import { readFileSync } from "node:fs";

import { describe, expect, test } from "vitest";

import {
  allMyZmanim,
  allParshiyos,
  allSpecialDays,
  allZmanim,
  amudYomiDirshu,
  dafHashavuaBavli,
  dafYomiBavli,
  dafYomiYerushalmi,
  HebrewDate,
  MyZmanim,
  mishnaYomis,
  pirkeiAvos,
  tehillimMonthly,
  Zmanim,
  type GregorianDate,
  type LocationOptions,
  type ParshaInfo,
  type SpecialDay,
  type ZmanimConfig,
  type ZmanimError,
} from "@dickermoshe/kr4js";

type Time = number | string;
type ParshaId = ParshaInfo["id"] | null;

interface ZmanimCase {
  name: string;
  location: LocationOptions;
  timeZone: string | null;
  date: string;
  config: ZmanimConfig;
  times: Time[];
  sunriseOffsetBy11Degrees: [Time];
  sunsetOffsetBy8Point5Degrees: [Time];
}

interface MyZmanimCase {
  name: string;
  location: LocationOptions;
  timeZone: string;
  date: string;
  candleLightingOffset: number;
  times: Time[];
  proportionalHourGra: number | null;
}

interface CalendarCase {
  date: string;
  hebrew: string;
  dayOfYear: number;
  weekday: number;
  specialDays: { israel: SpecialDay[]; diaspora: SpecialDay[] };
  assur: [boolean, boolean];
  candles: [boolean, boolean];
  aseres: boolean;
  parsha: [ParshaId, ParshaId];
  specialParsha: [ParshaId, ParshaId];
  nextParsha: [ParshaId, ParshaId];
  molad: [number, number, number, number, number];
  dafYomiBavli: string | null;
  dafHashavuaBavli: string | null;
  dafYomiYerushalmi: string | null;
  amudYomiDirshu: string | null;
  mishnaYomis: string | null;
  tehillimMonthly: string | null;
  pirkeiAvos: [string | null, string | null];
}

const fixture: {
  zmanim: ZmanimCase[];
  myZmanim: MyZmanimCase[];
  calendar: CalendarCase[];
} = JSON.parse(
  readFileSync(new URL("./fixtures/parity.json", import.meta.url), "utf8"),
);

/** The `YYYY-MM-DD` day `text` as an object. */
function gregorian(text: string): GregorianDate {
  const [year, month, day] = text.split("-").map(Number) as [number, number, number];
  return { year, month, day };
}

/** What `all()` reports, in the fixture's form. */
function asFixture(entry: { time: Date | null; unavailable?: string }): Time {
  return entry.time ? entry.time.getTime() : entry.unavailable!;
}

/** What `get()` does: a time, `null`, or a thrown ZmanimError with a code. */
function viaGet(get: () => Date | null): Time {
  try {
    const time = get();
    return time ? time.getTime() : "none";
  } catch (error) {
    expect((error as ZmanimError).name).toBe("ZmanimError");
    return (error as ZmanimError).code;
  }
}

function expectedFromGet(time: Time): Time {
  return typeof time === "number" || !["allDay", "allNight", "notApplicable"].includes(time) ? time : "none";
}

describe("zmanim match the crate", () => {
  test.each(fixture.zmanim.map((c) => [`${c.name} ${c.date} ${c.timeZone ?? "no time zone"}`, c] as const))(
    "%s",
    (_, c) => {
      const zmanim = new Zmanim({ ...c.config, location: c.location, date: gregorian(c.date), ...(c.timeZone ? { timeZone: c.timeZone } : {}) });
      expect(zmanim.all().map(asFixture)).toEqual(c.times);
      expect(allZmanim().map((zman) => viaGet(() => zmanim.get(zman.id)))).toEqual(c.times.map(expectedFromGet));
      expect(viaGet(() => zmanim.sunriseOffsetByDegrees(11))).toEqual(expectedFromGet(c.sunriseOffsetBy11Degrees[0]));
      expect(viaGet(() => zmanim.sunsetOffsetByDegrees(8.5))).toEqual(expectedFromGet(c.sunsetOffsetBy8Point5Degrees[0]));
    },
  );
});

describe("MyZmanim match the crate", () => {
  test.each(fixture.myZmanim.map((c) => [`${c.name} ${c.date}`, c] as const))("%s", (_, c) => {
    const zmanim = new MyZmanim({ ...c, date: gregorian(c.date) });
    expect(zmanim.all().map(asFixture)).toEqual(c.times);
    expect(allMyZmanim().map((zman) => viaGet(() => zmanim.get(zman.id)))).toEqual(c.times.map(expectedFromGet));
    expect(zmanim.proportionalHourGra()).toEqual(c.proportionalHourGra);
  });
});

describe("the calendar and schedules match the crate", () => {
  const kinds = new Map(allSpecialDays().map(({ id, day: _, ...info }) => [id, info]));
  const special = (days: SpecialDay[]) =>
    days.map((day) => ({ id: day.id, ...("day" in day ? { day: day.day } : {}), ...kinds.get(day.id) }));
  const parshiyos = new Map(allParshiyos().map((parsha) => [parsha.id, parsha]));
  const parsha = (id: ParshaId) => (id === null ? undefined : parshiyos.get(id));
  const text = (value: { toString(): string } | undefined) => (value === undefined ? null : value.toString());

  test.each(fixture.calendar.map((c) => [c.date, c] as const))("%s", (_, c) => {
    const date = gregorian(c.date);
    const day = HebrewDate.from(date);
    expect(day.toString()).toBe(c.hebrew);
    expect(day.toJSON()).toBe(c.date.length === 10 ? c.date : day.toJSON());
    expect(HebrewDate.of(day.year, day.month.id, day.day).equals(day)).toBe(true);
    expect(day.dayOfYear).toBe(c.dayOfYear);
    expect(day.weekday).toBe(c.weekday);
    expect(day.specialDays("israel")).toEqual(special(c.specialDays.israel));
    expect(day.specialDays("diaspora")).toEqual(special(c.specialDays.diaspora));
    expect([day.isAssurBemelacha("israel"), day.isAssurBemelacha("diaspora")]).toEqual(c.assur);
    expect([day.hasCandleLighting("israel"), day.hasCandleLighting("diaspora")]).toEqual(c.candles);
    expect(day.isAseresYemeiTeshuva()).toBe(c.aseres);
    expect([day.parsha("israel"), day.parsha("diaspora")]).toEqual(c.parsha.map(parsha));
    expect([day.specialParsha("israel"), day.specialParsha("diaspora")]).toEqual(c.specialParsha.map(parsha));
    expect([day.nextParsha("israel"), day.nextParsha("diaspora")]).toEqual(c.nextParsha.map(parsha));
    const molad = day.molad();
    expect([molad.time.getTime(), molad.weekday, molad.hours, molad.minutes, molad.chalakim]).toEqual(c.molad);
    expect(text(dafYomiBavli(date))).toBe(c.dafYomiBavli);
    expect(text(dafHashavuaBavli(date))).toBe(c.dafHashavuaBavli);
    expect(text(dafYomiYerushalmi(date))).toBe(c.dafYomiYerushalmi);
    expect(text(amudYomiDirshu(date))).toBe(c.amudYomiDirshu);
    expect(text(mishnaYomis(date))).toBe(c.mishnaYomis);
    expect(text(tehillimMonthly(date))).toBe(c.tehillimMonthly);
    expect([text(pirkeiAvos(date, "israel")), text(pirkeiAvos(date, "diaspora"))]).toEqual(c.pirkeiAvos);
  });
});
