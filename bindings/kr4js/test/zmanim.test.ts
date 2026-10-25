import { describe, expect, test } from "vitest";

import {
  allZmanim,
  MyZmanim,
  shaahZmanis,
  Zmanim,
  zmanimIn,
  zmanInfo,
  type Zman,
  type ZmanimError,
} from "@dickermoshe/kr4js";

const lakewood = { latitude: 40.09, longitude: -74.22, elevation: 30 };
const options = { location: lakewood, timeZone: "America/New_York", date: { year: 2024, month: 4, day: 23 } } as const;

/** The ZmanimError `run` throws. */
function zmanimError(run: () => unknown): ZmanimError {
  try {
    run();
  } catch (error) {
    expect(error).toBeInstanceOf(Error);
    expect((error as ZmanimError).name).toBe("ZmanimError");
    return error as ZmanimError;
  }
  throw new Error("expected a ZmanimError");
}

function clock(time: Date | null, zone = "America/New_York"): string {
  if (!time) throw new Error("expected a time");
  return time.toLocaleTimeString("en-GB", { timeZone: zone, hour: "2-digit", minute: "2-digit" });
}

describe("Zmanim", () => {
  test("gives the times in the README", () => {
    const zmanim = new Zmanim(options);
    expect(clock(zmanim.get("sofZmanShmaGra"))).toBe("09:31");
  });

  test("keeps what it was built with", () => {
    const zmanim = new Zmanim({ ...options, candleLightingOffset: 40 });
    expect(zmanim.location).toEqual(lakewood);
    expect(zmanim.date).toEqual({ year: 2024, month: 4, day: 23 });
    expect(zmanim.timeZone).toBe("America/New_York");
    expect(zmanim.config).toEqual({ candleLightingOffset: 40 });
  });

  test("returns null on a day without the time", () => {
    const zmanim = new Zmanim(options);
    // The latest time for chametz is only on Erev Pesach, and this is the first day.
    expect(zmanim.get("sofZmanAchilasChametzGra")).toBeNull();
    // Erev Pesach has one.
    expect(zmanim.on({ year: 2024, month: 4, day: 22 }).get("sofZmanAchilasChametzGra")).toBeInstanceOf(Date);
  });

  test("returns null where the sun never gets low or high enough", () => {
    const tromso = { latitude: 69.6492, longitude: 18.9553 };
    const summer = new Zmanim({ location: tromso, timeZone: "Europe/Oslo", date: { year: 2024, month: 6, day: 21 } });
    expect(summer.get("sunrise")).toBeNull();
    expect(summer.all().find((t) => t.zman.id === "sunrise")?.unavailable).toBe("allDay");
    const winter = summer.on({ year: 2024, month: 12, day: 21 });
    expect(winter.all().find((t) => t.zman.id === "sunrise")?.unavailable).toBe("allNight");
  });

  test("throws when a time needs a time zone", () => {
    const zmanim = new Zmanim({ location: lakewood, date: { year: 2024, month: 4, day: 23 } });
    expect(zmanim.get("sunrise")).toBeInstanceOf(Date);
    expect(zmanimError(() => zmanim.get("sofZmanKidushLevana15Days")).code).toBe("timeZoneRequired");
    // Far from Greenwich every time needs one.
    const apia = new Zmanim({ location: { latitude: -13.86, longitude: -171.75 }, date: { year: 2024, month: 4, day: 23 } });
    expect(() => apia.get("sunrise")).toThrow(/time zone/);
    expect(apia.all().filter((t) => t.time)).toEqual([]);
    expect(apia.all().filter((t) => t.unavailable === "timeZoneRequired").length).toBeGreaterThan(100);
  });

  test("all() lists every zman in order", () => {
    const all = new Zmanim(options).all();
    expect(all.map((t) => t.zman)).toEqual([...allZmanim()]);
    for (const { time, unavailable } of all) expect(time === null).toBe(unavailable !== undefined);
  });

  test("on() keeps the place, time zone and settings", () => {
    const zmanim = new Zmanim({ ...options, candleLightingOffset: 40 });
    const tomorrow = zmanim.on({ year: 2024, month: 4, day: 24 });
    expect(tomorrow.date).toEqual({ year: 2024, month: 4, day: 24 });
    expect(tomorrow.config).toEqual(zmanim.config);
    expect(tomorrow.get("sunrise")!.getTime() - zmanim.get("sunrise")!.getTime()).toBeLessThan(24 * 3600_000);
  });

  test("candle lighting follows the offset", () => {
    const sunset = new Zmanim(options).get("seaLevelSunset")!;
    const candles = new Zmanim({ ...options, candleLightingOffset: 40 }).get("candleLighting")!;
    expect(sunset.getTime() - candles.getTime()).toBe(40 * 60_000);
  });

  test("elevation moves sunrise earlier only when used", () => {
    const high = { ...lakewood, elevation: 3000 };
    const plain = new Zmanim({ ...options, location: high });
    const elevated = new Zmanim({ ...options, location: high, useElevation: true });
    expect(plain.get("alos72Minutes")!.getTime()).toBeGreaterThan(elevated.get("alos72Minutes")!.getTime());
    expect(plain.get("sunrise")).toEqual(elevated.get("sunrise"));
  });

  test("a raised horizon makes sunrise later and sunset earlier", () => {
    const high = { ...lakewood, elevation: 800 };
    const open = new Zmanim({ ...options, location: high });
    const hills = new Zmanim({ ...options, location: { ...high, easternHorizon: 800, westernHorizon: 400 } });
    expect(hills.get("sunrise")!.getTime()).toBeGreaterThan(open.get("sunrise")!.getTime());
    expect(hills.get("sunset")!.getTime()).toBeLessThan(open.get("sunset")!.getTime());
    expect(hills.get("sunrise")).toEqual(new Zmanim({ ...options, location: { ...lakewood, elevation: 0 } }).get("sunrise"));
  });

  test("the calculators agree to within a minute", () => {
    const noaa = new Zmanim(options).get("sunrise")!.getTime();
    for (const calculator of ["meeus", "spa"] as const) {
      const other = new Zmanim({ ...options, calculator }).get("sunrise")!.getTime();
      expect(Math.abs(other - noaa)).toBeLessThan(60_000);
      expect(other).not.toBe(noaa);
    }
  });

  test("custom angles", () => {
    const zmanim = new Zmanim(options);
    expect(zmanim.sunriseOffsetByDegrees(16.1)).toEqual(zmanim.get("alos16Point1Degrees"));
    expect(zmanim.sunsetOffsetByDegrees(8.5)).toEqual(zmanim.get("tzaisGeonim8Point5Degrees"));
    expect(zmanimError(() => zmanim.sunriseOffsetByDegrees(91)).code).toBe("invalidDegrees");
  });

  test("rejects bad input", () => {
    expect(() => new Zmanim({ ...options, location: { latitude: 91, longitude: 0 } })).toThrow(RangeError);
    expect(() => new Zmanim({ ...options, location: { latitude: 0, longitude: 181 } })).toThrow(RangeError);
    expect(() => new Zmanim({ ...options, location: { ...lakewood, elevation: -1 } })).toThrow(RangeError);
    expect(() => new Zmanim({ ...options, date: { year: 2024, month: 2, day: 30 } })).toThrow(RangeError);
    expect(() => new Zmanim({ ...options, date: "2024-04-23" as unknown as Date })).toThrow(TypeError);
    expect(() => new Zmanim({ ...options, candleLightingOffset: -5 })).toThrow(RangeError);
    expect(() => new Zmanim({ ...options, candleLightingOffset: Number.NaN })).toThrow(RangeError);
    expect(() => new Zmanim({ ...options, calculator: "sundial" as "noaa" })).toThrow(TypeError);
    expect(() => new Zmanim({ ...options, timeZone: "Mars/Olympus_Mons" })).toThrow(RangeError);
    expect(() => new Zmanim(options).get("sunrize" as Zman)).toThrow(TypeError);
  });

  test("accepts dates in every form", () => {
    const expected = new Zmanim(options).get("sunrise");
    for (const date of [{ year: 2024, month: 4, day: 23 }, { year: 2024, month: 4, day: 23 }, new Date(2024, 3, 23, 15)]) {
      expect(new Zmanim({ ...options, date }).get("sunrise")).toEqual(expected);
    }
    const ancient = new Date(2024, 3, 23, 12);
    ancient.setFullYear(-100);
    expect(new Zmanim({ ...options, date: ancient }).date).toEqual({ year: -100, month: 4, day: 23 });
  });

  test("Kiddush Levana times are found on their dates", () => {
    const zmanim = new Zmanim(options);
    const found: number[] = [];
    for (let day = 1; day <= 30; day++) {
      const time = zmanim.on({ year: 2024, month: 4, day }).get("tchilasZmanKidushLevana7Days");
      if (time) found.push(day);
    }
    expect(found).toHaveLength(1);
  });
});

describe("zman metadata", () => {
  test("zmanInfo and zmanimIn", () => {
    expect(zmanInfo("sofZmanShmaGra")).toMatchObject({ name: "Sof Zman Shma (GR'A)", javaName: "getSofZmanShmaGRA", category: "sofZmanShma" });
    expect(zmanimIn("sofZmanShma").map((z) => z.id)).toContain("sofZmanShmaMga72Minutes");
    expect(() => zmanInfo("nope" as Zman)).toThrow(TypeError);
  });

  test("ids are unique", () => {
    expect(new Set(allZmanim().map((z) => z.id)).size).toBe(allZmanim().length);
  });

  test("shaahZmanis is a twelfth of the day", () => {
    const zmanim = new Zmanim(options);
    const [sunrise, sunset] = [zmanim.get("seaLevelSunrise")!, zmanim.get("seaLevelSunset")!];
    expect(sunrise.getTime() + 3 * shaahZmanis(sunrise, sunset)).toBeCloseTo(zmanim.get("sofZmanShmaGra")!.getTime(), -1);
  });
});

describe("MyZmanim", () => {
  const jerusalem = { latitude: 31.778, longitude: 35.235, elevation: 754, easternHorizon: 754, westernHorizon: 0 };

  test("candle lighting in Jerusalem", () => {
    const times = new MyZmanim({ location: jerusalem, timeZone: "Asia/Jerusalem", date: { year: 2026, month: 11, day: 6 }, candleLightingOffset: 40 });
    const sunset = times.get("sunsetElevated")!;
    expect(sunset.getTime() - times.get("candleLighting")!.getTime()).toBe(40 * 60_000);
    expect(times.on({ year: 2026, month: 11, day: 7 }).candleLightingOffset).toBe(40);
  });

  test("works in another time zone", () => {
    const anvers = new MyZmanim({ location: { latitude: 51.216667, longitude: 4.416667 }, timeZone: "Europe/Brussels", date: { year: 2027, month: 3, day: 1 } });
    expect(clock(anvers.get("shemaGra"), "Europe/Brussels")).toBe("10:11");
    expect(anvers.proportionalHourGra()).toBeGreaterThan(50 * 60_000);
  });
});
