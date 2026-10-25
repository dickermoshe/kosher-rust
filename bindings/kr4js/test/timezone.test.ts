import { describe, expect, test } from "vitest";

import { Zmanim } from "@dickermoshe/kr4js";

import { offsetAt } from "../js/src/timezone.js";

const at = (iso: string) => Date.parse(iso);
const offset = (iso: string, zone: string) => offsetAt(zone)(at(iso));

describe("time zone names", () => {
  test("read offsets from the runtime", () => {
    expect(offset("2024-01-15T12:00:00Z", "America/New_York")).toBe(-300);
    expect(offset("2024-07-15T12:00:00Z", "America/New_York")).toBe(-240);
    expect(offset("2024-07-15T12:00:00Z", "Asia/Kolkata")).toBe(330);
    expect(offset("2024-07-15T12:00:00Z", "Asia/Kathmandu")).toBe(345);
    expect(offset("2024-01-15T12:00:00Z", "Pacific/Chatham")).toBe(825);
    expect(offsetAt("UTC")(0)).toBe(0);
  });

  test("change on the second of a transition", () => {
    // Clocks went forward at 2:00 EST on 10 March 2024, which is 07:00 UTC.
    expect(offset("2024-03-10T06:59:59.999Z", "America/New_York")).toBe(-300);
    expect(offset("2024-03-10T07:00:00Z", "America/New_York")).toBe(-240);
  });

  test("keep historical offsets with seconds", () => {
    // Jerusalem kept local mean time, 2:20:54 ahead of UTC, before 1880.
    expect(offset("1870-01-01T00:00:00Z", "Asia/Jerusalem") * 60).toBe(2 * 3600 + 20 * 60 + 54);
  });

  test("work before year 100 and before year 1", () => {
    expect(offsetAt("UTC")(Date.UTC(50, 0, 1))).toBe(0);
    const ancient = new Date(0);
    ancient.setUTCFullYear(-100, 0, 1);
    expect(offsetAt("UTC")(ancient.getTime())).toBe(0);
  });

  test("work in Zmanim", () => {
    const options = { location: { latitude: 40.09, longitude: -74.22 }, date: { year: 2024, month: 3, day: 10 } };
    const zmanim = new Zmanim({ ...options, timeZone: "America/New_York" });
    expect(zmanim.timeZone).toBe("America/New_York");
    expect(zmanim.all().filter((t) => t.time).length).toBeGreaterThan(150);
  });

  test("reject unknown names", () => {
    expect(() => offsetAt("Nowhere/Special")).toThrow(RangeError);
    expect(() => new Zmanim({ location: { latitude: 0, longitude: 0 }, date: new Date(), timeZone: "Nowhere/Special" })).toThrow(RangeError);
  });

  test("are the only kind of time zone", () => {
    const options = { location: { latitude: 40.09, longitude: -74.22 }, date: { year: 2024, month: 3, day: 10 } };
    const callback = (() => -300) as unknown as string;
    expect(() => new Zmanim({ ...options, timeZone: callback })).toThrow(TypeError);
    expect(() => new Zmanim({ ...options, timeZone: -300 as unknown as string })).toThrow(TypeError);
  });
});
