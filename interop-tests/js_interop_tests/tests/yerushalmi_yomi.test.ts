// Test file for limudim-wasm - Daf Yomi Yerushalmi only
// Run with: bun test

import { daf_yomi_yerushalmi } from "../pkg/limudim_wasm.js";
import { HDate } from "@hebcal/core";
import { yerushalmiYomi, vilna } from "@hebcal/learning";
import { faker } from "@faker-js/faker";
import { expect, test, describe } from "bun:test";

const ITERATIONS = 1000;

// Mapping from our tractate names to hebcal's tractate names (Yerushalmi)
const TRACTATE_MAP: Record<string, string> = {
  // Yerushalmi-specific spellings
  Berachos: "Berakhot",
  Berakhot: "Berakhot",
  Peah: "Peah",
  Demai: "Demai",
  Kilayim: "Kilayim",
  Sheviis: "Sheviit",
  Sheviit: "Sheviit",
  Terumos: "Terumot",
  Terumot: "Terumot",
  Maasros: "Maasrot",
  Maasrot: "Maasrot",
  MaaserSheni: "Maaser Sheni",
  "Maaser Sheni": "Maaser Sheni",
  Chalah: "Challah",
  Challah: "Challah",
  Orlah: "Orlah",
  Bikurim: "Bikkurim",
  Bikkurim: "Bikkurim",
  Shabbos: "Shabbat",
  Shabbat: "Shabbat",
  Eruvin: "Eruvin",
  Pesachim: "Pesachim",
  Beitzah: "Beitzah",
  RoshHashanah: "Rosh Hashanah",
  "Rosh Hashanah": "Rosh Hashanah",
  Yoma: "Yoma",
  Sukkah: "Sukkah",
  Taanis: "Taanit",
  Taanit: "Taanit",
  Shekalim: "Shekalim",
  Megillah: "Megillah",
  Chagigah: "Chagigah",
  MoedKatan: "Moed Katan",
  "Moed Katan": "Moed Katan",
  Yevamos: "Yevamot",
  Yevamot: "Yevamot",
  Kesubos: "Ketubot",
  Ketubot: "Ketubot",
  Sotah: "Sotah",
  Nedarim: "Nedarim",
  Nazir: "Nazir",
  Gitin: "Gittin",
  Gittin: "Gittin",
  Kiddushin: "Kiddushin",
  BavaKamma: "Bava Kamma",
  "Bava Kamma": "Bava Kamma",
  BavaMetzia: "Bava Metzia",
  "Bava Metzia": "Bava Metzia",
  BavaBasra: "Bava Batra",
  "Bava Batra": "Bava Batra",
  Sanhedrin: "Sanhedrin",
  Makkos: "Makkot",
  Makkot: "Makkot",
  Shevuos: "Shevuot",
  Shevuot: "Shevuot",
  AvodahZarah: "Avodah Zarah",
  "Avodah Zarah": "Avodah Zarah",
  Horiyos: "Horayot",
  Horayot: "Horayot",
  Niddah: "Niddah",
};

function mapTractate(ourTractate: string): string {
  return TRACTATE_MAP[ourTractate] || ourTractate;
}

interface DafResult {
  tractate: string;
  page: number;
}

// Helper to generate random dates within a range
function randomDate(from: string, to: string): Date {
  return faker.date.between({ from, to });
}

/** The daf on a Gregorian date from this crate and from hebcal, as "Tractate:page", or null. */
function dafimOn(y: number, m: number, d: number): { ours: string | null; hebcal: string | null } {
  let ours: DafResult | null = null;
  let hebcal: any = null;
  try {
    ours = daf_yomi_yerushalmi(y, m, d);
  } catch (e) {
    ours = null;
  }
  try {
    hebcal = yerushalmiYomi(new HDate(new Date(Date.UTC(y, m - 1, d))), vilna);
  } catch (e) {
    hebcal = null;
  }
  return {
    ours: ours ? `${mapTractate(ours.tractate)}:${ours.page}` : null,
    hebcal: hebcal ? `${mapTractate(hebcal.name)}:${hebcal.blatt}` : null,
  };
}

describe("Daf Yomi Yerushalmi", () => {
  // Every day from the first cycle through 7 August 2172. This crosses every cycle boundary in
  // that span, including the cycle that ends on 7 August 2172 with Niddah 13. From 8 August 2172
  // hebcal's own cycle runs one day too long, so it is no reference after that.
  test("matches @hebcal/learning on every day from 1980 through 7 August 2172", () => {
    const end = Date.UTC(2172, 7, 7);
    const day = 24 * 60 * 60 * 1000;
    for (let time = Date.UTC(1980, 1, 2); time <= end; time += day) {
      const date = new Date(time);
      const y = date.getUTCFullYear();
      const m = date.getUTCMonth() + 1;
      const d = date.getUTCDate();
      const { ours, hebcal } = dafimOn(y, m, d);
      if (ours !== hebcal) {
        expect({ date: date.toISOString().slice(0, 10), daf: ours }).toEqual({
          date: date.toISOString().slice(0, 10),
          daf: hebcal,
        });
      }
    }
  });

  test("matches @hebcal/learning for random dates 1980-2060", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      const date = randomDate("1900-01-01", "2060-12-31");

      // Extract year, month, day to avoid timezone issues
      const y = date.getUTCFullYear();
      const m = date.getUTCMonth() + 1;
      const d = date.getUTCDate();

      // Create a clean UTC date for hebcal
      const utcDate = new Date(Date.UTC(y, m - 1, d));
      const hd = new HDate(utcDate);

      let wasmResult: DafResult | null = null;
      let hebcalResult: any = null;

      try {
        wasmResult = daf_yomi_yerushalmi(y, m, d);
      } catch (e) {
        wasmResult = null;
      }

      try {
        hebcalResult = yerushalmiYomi(new HDate(utcDate), vilna);
      } catch (e) {
        hebcalResult = null;
      }

      // Both should agree on whether result exists
      if (wasmResult === null && hebcalResult === null) continue;

      if (!wasmResult || !hebcalResult) {
        const dateStr = `${y}-${m.toString().padStart(2, "0")}-${d.toString().padStart(2, "0")}`;
        const hebrewMonth = hd.getMonth();
        const hebrewDay = hd.getDate();
        console.log(
          `\nNull mismatch on ${dateStr} (Hebrew: ${hebrewMonth}/${hebrewDay}):`,
        );
        console.log(
          `  WASM: ${wasmResult ? `${wasmResult.tractate}:${wasmResult.page}` : "null"}`,
        );
        console.log(
          `  Hebcal: ${hebcalResult ? `${hebcalResult.name}:${hebcalResult.blatt}` : "null"}`,
        );
      }
      expect(wasmResult).not.toBeNull();
      expect(hebcalResult).not.toBeNull();

      const wasmMapped = `${mapTractate(wasmResult!.tractate)}:${wasmResult!.page}`;
      const hebcalMapped = `${mapTractate(hebcalResult.name)}:${hebcalResult.blatt}`;

      if (wasmMapped !== hebcalMapped) {
        const dateStr = `${y}-${m.toString().padStart(2, "0")}-${d.toString().padStart(2, "0")}`;
        const hebrewMonth = hd.getMonth();
        const hebrewDay = hd.getDate();
        console.log(
          `\nPage mismatch on ${dateStr} (Hebrew: ${hebrewMonth}/${hebrewDay}):`,
        );
        console.log(`  WASM: ${wasmMapped}`);
        console.log(`  Hebcal: ${hebcalMapped}`);
      }
      expect(wasmMapped).toBe(hebcalMapped);
    }
  });
});
