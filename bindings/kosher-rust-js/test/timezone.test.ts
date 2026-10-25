import { describe, expect, test } from "vitest";

import { fixedOffset, localTime, timeZone, today, Zmanim, type TimeZone } from "kosher-rust";

const at = (iso: string) => Date.parse(iso);

describe("timeZone", () => {
  test("reads offsets from the runtime", () => {
    const newYork = timeZone("America/New_York");
    expect(newYork.name).toBe("America/New_York");
    expect(newYork.offsetAt(at("2024-01-15T12:00:00Z"))).toBe(-300);
    expect(newYork.offsetAt(at("2024-07-15T12:00:00Z"))).toBe(-240);
    expect(timeZone("Asia/Kolkata").offsetAt(at("2024-07-15T12:00:00Z"))).toBe(330);
    expect(timeZone("Asia/Kathmandu").offsetAt(at("2024-07-15T12:00:00Z"))).toBe(345);
    expect(timeZone("Pacific/Chatham").offsetAt(at("2024-01-15T12:00:00Z"))).toBe(825);
    expect(timeZone("UTC").offsetAt(0)).toBe(0);
  });

  test("changes on the second of a transition", () => {
    const newYork = timeZone("America/New_York");
    // Clocks went forward at 2:00 EST on 10 March 2024, which is 07:00 UTC.
    expect(newYork.offsetAt(at("2024-03-10T06:59:59.999Z"))).toBe(-300);
    expect(newYork.offsetAt(at("2024-03-10T07:00:00Z"))).toBe(-240);
  });

  test("keeps historical offsets with seconds", () => {
    // Jerusalem kept local mean time, 2:20:54 ahead of UTC, before 1880.
    expect(timeZone("Asia/Jerusalem").offsetAt(at("1870-01-01T00:00:00Z")) * 60).toBe(2 * 3600 + 20 * 60 + 54);
  });

  test("works before year 100 and before year 1", () => {
    expect(timeZone("UTC").offsetAt(Date.UTC(50, 0, 1) - 1900 * 0)).toBe(0);
    const ancient = new Date(0);
    ancient.setUTCFullYear(-100, 0, 1);
    expect(timeZone("UTC").offsetAt(ancient.getTime())).toBe(0);
  });

  test("rejects unknown names", () => {
    expect(() => timeZone("Nowhere/Special")).toThrow(RangeError);
  });
});

describe("fixedOffset", () => {
  test("is the same at every instant", () => {
    const zone = fixedOffset(-300);
    expect(zone.name).toBe("-05:00");
    expect(zone.offsetAt(0)).toBe(-300);
    expect(zone.offsetAt(at("2024-07-01T00:00:00Z"))).toBe(-300);
    expect(fixedOffset(330).name).toBe("+05:30");
    expect(() => fixedOffset(Number.NaN)).toThrow(RangeError);
  });
});

describe("custom time zones", () => {
  const options = { location: { latitude: 40.09, longitude: -74.22 }, date: { year: 2024, month: 3, day: 10 } };
  const intl = timeZone("America/New_York");
  const expected = new Zmanim({ ...options, timeZone: "America/New_York" }).all();

  test("an object with offsetAt", () => {
    const zone: TimeZone = { offsetAt: (ms) => intl.offsetAt(ms) };
    expect(new Zmanim({ ...options, timeZone: zone }).all()).toEqual(expected);
  });

  test("a plain function", () => {
    expect(new Zmanim({ ...options, timeZone: (ms: number) => intl.offsetAt(ms) }).all()).toEqual(expected);
  });

  test("is asked about the days around the date, as the calculations need it", () => {
    const asked: number[] = [];
    const zmanim = new Zmanim({ ...options, timeZone: (ms: number) => (asked.push(ms), intl.offsetAt(ms)) });
    // Building the zmanim checks the time zone once, at the start of the day.
    expect(asked.length).toBeLessThan(10);
    zmanim.all();
    const days = (Math.max(...asked) - Math.min(...asked)) / 86_400_000;
    expect(days).toBeLessThanOrEqual(4);
  });

  test("errors thrown while calculating reach the caller", () => {
    let calls = 0;
    const flaky = (ms: number) => {
      if (++calls > 5) throw new Error("lost the connection");
      return intl.offsetAt(ms);
    };
    const zmanim = new Zmanim({ ...options, timeZone: flaky });
    expect(() => zmanim.all()).toThrow("lost the connection");
  });

  test("errors thrown by the time zone reach the caller", () => {
    const failing = () => {
      throw new Error("no network");
    };
    expect(() => new Zmanim({ ...options, timeZone: failing })).toThrow("no network");
  });

  test("an offset that isn't a number is a TypeError", () => {
    expect(() => new Zmanim({ ...options, timeZone: () => "five" as unknown as number })).toThrow(TypeError);
    expect(() => new Zmanim({ ...options, timeZone: {} as TimeZone })).toThrow(TypeError);
  });

  test("an offset beyond 25:59:59 is a RangeError", () => {
    expect(() => new Zmanim({ ...options, timeZone: fixedOffset(27 * 60) })).toThrow(RangeError);
  });
});

describe("localTime and today", () => {
  test("localTime shows an instant on another clock", () => {
    expect(localTime(at("2024-04-23T13:31:00.138Z"), "America/New_York")).toEqual({
      year: 2024,
      month: 4,
      day: 23,
      hour: 9,
      minute: 31,
      second: 0,
      millisecond: 138,
      offset: -240,
    });
    expect(localTime(new Date(at("2024-04-23T23:30:00Z")), fixedOffset(180)).day).toBe(24);
  });

  test("today in a time zone", () => {
    const now = new Date();
    const expected = localTime(now, "Pacific/Kiritimati");
    const date = today("Pacific/Kiritimati");
    // Unless the day turns over between the two calls.
    expect([expected.day, (expected.day % 31) + 1]).toContain(date.day);
    expect(today()).toEqual({ year: now.getFullYear(), month: now.getMonth() + 1, day: now.getDate() });
  });
});
