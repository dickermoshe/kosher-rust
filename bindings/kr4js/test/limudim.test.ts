import { describe, expect, test } from "vitest";

import {
  allTractates,
  Amud,
  amudYomiDirshu,
  Daf,
  dafHashavuaBavli,
  dafYomiBavli,
  dafYomiYerushalmi,
  HebrewDate,
  MishnaPair,
  mishnaYomis,
  pirkeiAvos,
  TehillimPortion,
  tehillimMonthly,
  type Region,
} from "kr4js";

describe("schedules", () => {
  test("give the README's portions", () => {
    const daf = dafYomiBavli({ year: 2024, month: 4, day: 23 })!;
    expect(daf).toBeInstanceOf(Daf);
    expect(daf.toString()).toBe("Bava Metzia 55");
    expect(daf.tractate).toMatchObject({ id: "bavaMetzia", name: "Bava Metzia" });
    expect(daf.daf).toBe(55);

    const mishnayos = mishnaYomis({ year: 2024, month: 4, day: 23 })!;
    expect(mishnayos).toBeInstanceOf(MishnaPair);
    expect(String(mishnayos)).toBe("Nazir 5:5-6");
    expect(mishnayos.first).toMatchObject({ chapter: 5, mishna: 5 });
    expect(mishnayos.second).toMatchObject({ chapter: 5, mishna: 6 });
  });

  test("give undefined before a program began", () => {
    expect(dafYomiBavli({ year: 1900, month: 1, day: 1 })).toBeUndefined();
    expect(amudYomiDirshu({ year: 2020, month: 1, day: 1 })).toBeUndefined();
  });

  test("take a HebrewDate", () => {
    expect(String(dafYomiBavli(HebrewDate.of(5784, "nisan", 15)))).toBe("Bava Metzia 55");
  });

  test("other units", () => {
    const amud = amudYomiDirshu({ year: 2024, month: 4, day: 23 })!;
    expect(amud).toBeInstanceOf(Amud);
    expect(["a", "b"]).toContain(amud.side);
    expect(String(amud)).toBe(`${amud.tractate.name} ${amud.daf}${amud.side}`);
    expect(dafYomiYerushalmi({ year: 2024, month: 4, day: 23 })).toBeInstanceOf(Daf);
    expect(dafHashavuaBavli({ year: 2024, month: 4, day: 23 })).toBeInstanceOf(Daf);

    const tehillim = tehillimMonthly(HebrewDate.of(5784, "nisan", 1))!;
    expect(tehillim).toBeInstanceOf(TehillimPortion);
    expect(String(tehillim)).toBe("Tehillim 1-9");
    const verses = tehillimMonthly(HebrewDate.of(5784, "nisan", 25))!;
    expect(verses.verses).toEqual({ start: 1, end: 96 });
    expect(String(verses)).toBe("Tehillim 119:1-96");

    const avos = pirkeiAvos({ year: 2024, month: 5, day: 4 }, "israel")!;
    expect(avos.chapters).toHaveLength(1);
    expect(pirkeiAvos({ year: 2024, month: 12, day: 1 }, "israel")).toBeUndefined();
    expect(() => pirkeiAvos({ year: 2024, month: 5, day: 4 }, "mars" as Region)).toThrow(TypeError);
  });

  test("tractates have Hebrew names", () => {
    expect(allTractates().find((t) => t.id === "berachos")?.hebrew).toBe("ברכות");
  });
});
