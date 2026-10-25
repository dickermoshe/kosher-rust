// The lists of everything the package knows, and the names they carry.
import { describe, expect, test } from "vitest";

import {
  allHebrewMonths,
  allMyZmanim,
  allParshiyos,
  allSpecialDays,
  allTractates,
  allZmanim,
  zmanInfo,
} from "kosher-rust";

describe("lists", () => {
  test.each([
    ["zmanim", allZmanim],
    ["MyZmanim rows", allMyZmanim],
    ["parshiyos", allParshiyos],
    ["months", allHebrewMonths],
    ["tractates", allTractates],
  ] as const)("%s each have a unique id and a name", (_, all) => {
    const list = all();
    const ids = list.map((entry) => entry.id);
    expect(list.length).toBeGreaterThan(0);
    expect(new Set(ids).size).toBe(ids.length);
    for (const entry of list) expect(entry.name).not.toBe("");
  });

  test("special days list each kind once, Chanukah and the Omer as their first day", () => {
    const list = allSpecialDays();
    const ids = list.map((entry) => entry.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(list.filter((entry) => entry.day !== undefined).map((entry) => [entry.id, entry.day])).toEqual([
      ["chanukah", 1],
      ["countOfTheOmer", 1],
    ]);
  });

  test("entries carry the names from Rust", () => {
    expect(zmanInfo("sofZmanShmaGra")).toEqual({
      id: "sofZmanShmaGra",
      name: "Sof Zman Shma (GR'A)",
      javaName: "getSofZmanShmaGRA",
      category: "sofZmanShma",
      deprecated: false,
    });
    expect(allMyZmanim().find((row) => row.id === "candleLighting")?.name).toBe("Candle lighting");
    expect(allSpecialDays().find((day) => day.id === "yomKippur")).toEqual({
      id: "yomKippur",
      name: "Yom Kippur",
      hebrew: "יום כיפור",
      isAssurBemelacha: true,
      isFastDay: true,
      isModern: false,
    });
    expect(allParshiyos()[0]).toEqual({ id: "bereshis", name: "Bereshis", hebrew: "בראשית" });
    expect(allHebrewMonths().find((month) => month.id === "nisan")).toEqual({ id: "nisan", name: "Nisan", hebrew: "ניסן" });
    expect(allZmanim().filter((zman) => zman.deprecated)).toHaveLength(24);
  });
});
