// Test file for limudim-wasm - Amud Yomi Bavli Dirshu only
// Run with: bun test

import { amud_yomi_bavli_dirshu } from "../pkg/limudim_wasm.js";
import { HDate } from "@hebcal/core";
import { calculateDirshuAmud } from "@hebcal/learning";
import { faker } from "@faker-js/faker";
import { expect, test, describe } from "bun:test";

const ITERATIONS = 1000;

// Mapping from our tractate names to hebcal's tractate names (Bavli)
const TRACTATE_MAP: Record<string, string> = {
  Berachos: "Berachot",
  Shabbos: "Shabbat",
  Eruvin: "Eruvin",
  Pesachim: "Pesachim",
  Shekalim: "Shekalim",
  Yoma: "Yoma",
  Sukkah: "Sukkah",
  Beitzah: "Beitzah",
  RoshHashanah: "Rosh Hashana",
  Taanis: "Taanit",
  Megillah: "Megillah",
  MoedKatan: "Moed Katan",
  Chagigah: "Chagigah",
  Yevamos: "Yevamot",
  Kesubos: "Ketubot",
  Nedarim: "Nedarim",
  Nazir: "Nazir",
  Sotah: "Sotah",
  Gitin: "Gitin",
  Gittin: "Gitin",
  Kiddushin: "Kiddushin",
  BavaKamma: "Baba Kamma",
  BavaMetzia: "Baba Metzia",
  BavaBasra: "Baba Batra",
  Sanhedrin: "Sanhedrin",
  Makkos: "Makkot",
  Shevuos: "Shevuot",
  AvodahZarah: "Avodah Zarah",
  Horiyos: "Horayot",
  Zevachim: "Zevachim",
  Menachos: "Menachot",
  Chullin: "Chullin",
  Bechoros: "Bechorot",
  Bechorot: "Bechorot",
  Bekhorot: "Bechorot",
  Arachin: "Arachin",
  Arakhin: "Arachin",
  Temurah: "Temurah",
  Kerisos: "Keritot",
  Meilah: "Meilah",
  Kinnim: "Kinnim",
  Tamid: "Tamid",
  Midos: "Midot",
  Midot: "Midot",
  Middot: "Midot",
  Niddah: "Niddah",
};

function mapTractate(ourTractate: string): string {
  return TRACTATE_MAP[ourTractate] || ourTractate;
}

interface AmudResult {
  tractate: string;
  page: number;
  side: string;
}

const BAVLI_STARTS: Record<string, { page: number; side: "a" | "b" }> = {
  Kinnim: { page: 22, side: "b" },
  Tamid: { page: 25, side: "b" },
  Midos: { page: 34, side: "a" },
};

function pageSideIndex(page: number, side: "a" | "b"): number {
  return (page - 2) * 2 + (side === "b" ? 1 : 0);
}

function wasmLinearIndex(wasm: AmudResult): number {
  const start = BAVLI_STARTS[wasm.tractate] ?? { page: 2, side: "a" };
  return (
    pageSideIndex(wasm.page, wasm.side as "a" | "b") -
    pageSideIndex(start.page, start.side)
  );
}

function hebcalLinearIndex(amud: { amud: number; side: "a" | "b" }): number {
  return (amud.amud - 2) * 2 + (amud.side === "b" ? 1 : 0);
}

function randomDate(from: string, to: string): Date {
  return faker.date.between({ from, to });
}

describe("Amud Yomi Bavli Dirshu", () => {
  test("matches @hebcal/learning implementation for random dates 2023-2099", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      const date = randomDate("2023-10-16", "2099-12-31");
      const y = date.getUTCFullYear();
      const m = date.getUTCMonth() + 1;
      const d = date.getUTCDate();
      const utcDate = new Date(Date.UTC(y, m - 1, d));

      let wasmResult: AmudResult | null = null;
      let hebcalResult: ReturnType<typeof calculateDirshuAmud> | null = null;

      try {
        wasmResult = amud_yomi_bavli_dirshu(y, m, d);
      } catch {
        wasmResult = null;
      }

      try {
        hebcalResult = calculateDirshuAmud(new HDate(utcDate));
      } catch {
        hebcalResult = null;
      }

      if (wasmResult === null && hebcalResult === null) continue;

      expect(wasmResult).not.toBeNull();
      expect(hebcalResult).not.toBeNull();

      expect(mapTractate(wasmResult!.tractate)).toBe(hebcalResult!.name);
      // Hebcal reports synthetic daf numbers (always from 2a) per tractate; we use
      // Vilna page numbers (with Dirshu starts for Kinnim/Tamid/Midos). Compare position
      // within the tractate via linear index, not raw page/side.
      expect(wasmLinearIndex(wasmResult!)).toBe(
        hebcalLinearIndex(hebcalResult!),
      );
    }
  }, 60_000);
});
