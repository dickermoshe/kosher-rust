import { describe, expect, test } from "vitest";

import { allHebrewMonths, HebrewDate, HebrewYear, type Region } from "@dickermoshe/kr4js";

describe("HebrewDate", () => {
  test("converts from the Gregorian calendar", () => {
    const day = HebrewDate.from({ year: 2024, month: 4, day: 23 });
    expect(day.toString()).toBe("15 Nisan 5784");
    expect(day.year).toBe(5784);
    expect(day.month).toEqual({ id: "nisan", name: "Nisan", hebrew: "ניסן" });
    expect(day.day).toBe(15);
    expect(day.weekday).toBe(2);
    expect(day.gregorian).toEqual({ year: 2024, month: 4, day: 23 });
    expect(JSON.stringify({ day })).toBe('{"day":"2024-04-23"}');
  });

  test("accepts dates in every form", () => {
    const expected = HebrewDate.from({ year: 2024, month: 4, day: 23 });
    const temporalLike = { year: 2024, month: 4, day: 23, dayOfWeek: 2, calendarId: "iso8601" };
    for (const input of [{ year: 2024, month: 4, day: 23 }, temporalLike, new Date(2024, 3, 23, 22), expected]) {
      expect(HebrewDate.from(input).equals(expected)).toBe(true);
    }
  });

  test("builds from a Hebrew date", () => {
    expect(HebrewDate.of(5784, "nisan", 15).gregorian).toEqual({ year: 2024, month: 4, day: 23 });
    expect(HebrewDate.of(5784, allHebrewMonths()[0]!.id, 1).toString()).toBe("1 Tishrei 5784");
    expect(HebrewDate.of(5784, "adarI", 14).toString()).toBe("14 Adar I 5784");
    expect(HebrewDate.of(5784, "adar", 14).toString()).toBe("14 Adar II 5784");
    expect(HebrewDate.of(5785, "adar", 14).toString()).toBe("14 Adar 5785");
  });

  test("rejects days that don't exist", () => {
    expect(() => HebrewDate.of(5785, "adarI", 1)).toThrow(RangeError);
    expect(() => HebrewDate.of(5784, "nisan", 31)).toThrow(RangeError);
    expect(() => HebrewDate.of(5784, "smarch" as "nisan", 1)).toThrow(TypeError);
    expect(() => HebrewDate.from({ year: 0, month: 12, day: 31 })).toThrow(RangeError);
    expect(() => HebrewDate.from({ year: 2024, month: 13, day: 1 })).toThrow(RangeError);
    expect(() => HebrewDate.from({ year: 2024, month: 2, day: 30 })).toThrow(RangeError);
    expect(() => HebrewDate.from({ year: 2024, month: 4, day: 23.5 })).toThrow(RangeError);
    expect(() => HebrewDate.from(new Date(Number.NaN))).toThrow(RangeError);
    // Years a Date holds that are larger than 16 bits.
    expect(() => HebrewDate.from(new Date(Date.UTC(67560, 0, 1, 12)))).toThrow(RangeError);
    expect(() => HebrewDate.from(new Date(Date.UTC(70000, 0, 1, 12)))).toThrow(RangeError);
  });

  test("takes no strings", () => {
    for (const input of ["2024-04-23", 42, undefined]) {
      expect(() => HebrewDate.from(input as unknown as Date), String(input)).toThrow(TypeError);
    }
  });

  test("special days", () => {
    const pesach = HebrewDate.from({ year: 2024, month: 4, day: 24 });
    expect(pesach.is("pesach", "diaspora")).toBe(true);
    expect(pesach.is("cholHamoedPesach", "israel")).toBe(true);
    const chanukah = HebrewDate.from({ year: 2024, month: 12, day: 27 }).specialDays("diaspora");
    expect(chanukah).toContainEqual(expect.objectContaining({ id: "chanukah", day: 2 }));
    const omer = HebrewDate.from({ year: 2024, month: 5, day: 26 }).specialDays("israel");
    expect(omer).toContainEqual(expect.objectContaining({ id: "lagBomer" }));
    expect(omer).toContainEqual(expect.objectContaining({ id: "countOfTheOmer", day: 33 }));
    expect(HebrewDate.from({ year: 2024, month: 10, day: 12 }).specialDays("israel")[0]).toMatchObject({ id: "yomKippur", isFastDay: true, isAssurBemelacha: true });
  });

  test("Shabbos, Yom Tov and candles", () => {
    const friday = HebrewDate.from({ year: 2024, month: 5, day: 3 });
    expect(friday.hasCandleLighting("diaspora")).toBe(true);
    expect(friday.isAssurBemelacha("diaspora")).toBe(false);
    expect(friday.addDays(1).isAssurBemelacha("diaspora")).toBe(true);
    expect(HebrewDate.from({ year: 2024, month: 10, day: 13 }).isAseresYemeiTeshuva()).toBe(false);
    expect(HebrewDate.from({ year: 2024, month: 10, day: 4 }).isAseresYemeiTeshuva()).toBe(true);
  });

  test("parshiyos", () => {
    const shabbos = HebrewDate.from({ year: 2024, month: 5, day: 4 });
    expect(shabbos.parsha("israel")?.name).toBe("Achrei Mos");
    expect(shabbos.parsha("israel")?.hebrew).toBe("אחרי מות");
    expect(HebrewDate.from({ year: 2024, month: 5, day: 3 }).parsha("israel")).toBeUndefined();
    expect(HebrewDate.from({ year: 2024, month: 5, day: 3 }).nextParsha("israel")?.id).toBe("achreiMos");
    expect(HebrewDate.from({ year: 2024, month: 4, day: 20 }).specialParsha("diaspora")?.id).toBe("hagadol");
  });

  test("the molad", () => {
    const molad = HebrewDate.from({ year: 2024, month: 5, day: 9 }).molad();
    expect([molad.weekday, molad.hours, molad.minutes, molad.chalakim]).toEqual([3, 11, 41, 8]);
    expect(molad.time).toBeInstanceOf(Date);
  });

  test("arithmetic and comparison", () => {
    const day = HebrewDate.of(5784, "elul", 29);
    expect(day.addDays(1).toString()).toBe("1 Tishrei 5785");
    expect(day.addDays(-29).toString()).toBe("30 Av 5784");
    expect(day.compare(day.addDays(1))).toBeLessThan(0);
    expect(day.addDays(1).compare(day)).toBeGreaterThan(0);
    expect(day.compare(HebrewDate.of(5784, "elul", 29))).toBe(0);
    expect(() => day.addDays(1.5)).toThrow(RangeError);
    expect(() => HebrewDate.of(9999, "elul", 29).addDays(1)).toThrow(RangeError);
  });

  test("rejects unknown regions", () => {
    expect(() => HebrewDate.from({ year: 2024, month: 4, day: 23 }).specialDays("mars" as Region)).toThrow(TypeError);
  });
});

describe("HebrewYear", () => {
  test("lengths and months", () => {
    const leap = HebrewYear.of(5784);
    expect(leap).toMatchObject({ year: 5784, isLeap: true, days: 383, length: "chaserim", isCheshvanLong: false, isKislevShort: true });
    expect(leap.months).toHaveLength(13);
    expect(leap.daysIn("adarI")).toBe(30);
    const common = HebrewYear.of(5785);
    expect(common.isLeap).toBe(false);
    expect(common.months.map((m) => m.id)).not.toContain("adarI");
    expect(common.daysIn("adarI")).toBeUndefined();
    expect(common.roshHashana.gregorian).toEqual({ year: 2024, month: 10, day: 3 });
    expect(HebrewDate.from({ year: 2024, month: 4, day: 23 }).hebrewYear.year).toBe(5784);
  });

  test("rejects years out of range", () => {
    expect(() => HebrewYear.of(3760)).toThrow(RangeError);
    expect(() => HebrewYear.of(10000)).toThrow(RangeError);
    expect(() => HebrewYear.of(5784.5)).toThrow(RangeError);
  });
});
