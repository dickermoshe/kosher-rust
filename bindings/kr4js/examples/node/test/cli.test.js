// @ts-check
// Runs with `node --test` and with `bun test`. The command line tests start the app under both runtimes.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { describe, test } from "node:test";
import { fileURLToPath } from "node:url";

import { CITIES, format, report } from "../src/report.js";

const cli = fileURLToPath(new URL("../src/cli.js", import.meta.url));

describe("report", () => {
  test("Lakewood on the first day of Pesach", () => {
    const data = report(CITIES.lakewood, { year: 2024, month: 4, day: 23 });
    assert.equal(data.hebrew, "15 Nisan 5784");
    assert.deepEqual(data.specialDays, ["Pesach"]);
    assert.equal(data.dafYomi, "Bava Metzia 55");
    assert.equal(data.mishnaYomis, "Nazir 5:5-6");
    const shma = data.zmanim.find((z) => z.name === "Sof Zman Shma (GR'A)");
    assert.equal(shma?.time.slice(0, 5), "09:31");
  });

  test("candle lighting shows only before Shabbos and Yom Tov", () => {
    const names = (/** @type {number} */ day) => report(CITIES.jerusalem, { year: 2024, month: 5, day }).zmanim.map((z) => z.name);
    assert.ok(names(3).includes("Candle Lighting"));
    assert.ok(!names(5).includes("Candle Lighting"));
  });

  test("the parsha on Shabbos", () => {
    assert.equal(report(CITIES.jerusalem, { year: 2024, month: 5, day: 4 }).parsha, "Achrei Mos");
  });

  test("every city works today", () => {
    for (const city of Object.values(CITIES)) {
      const data = report(city);
      assert.ok(data.zmanim.length >= 12, city.name);
      assert.ok(format(data).startsWith(city.name));
    }
  });
});

/** The runtimes to start the app with: this one, and the other one when it is installed. */
function runtimes() {
  const bun = "Bun" in globalThis;
  const found = [[bun ? "bun" : "node", process.execPath]];
  const other = bun ? "node" : "bun";
  try {
    execFileSync(other, ["--version"]);
    found.push([other, other]);
  } catch {
    // Not installed, so only this runtime is checked.
  }
  return found;
}

describe("command line", () => {
  for (const [name, executable] of runtimes()) {
    test(`runs on ${name}`, () => {
      const text = execFileSync(executable, [cli, "--city", "lakewood", "--date", "2024-04-23"], { encoding: "utf8" });
      assert.match(text, /^Lakewood — 2024-04-23 — 15 Nisan 5784/);
      assert.match(text, /Sof Zman Shma \(GR'A\)\s+09:31/);

      const json = JSON.parse(
        execFileSync(executable, [cli, "--city", "jerusalem", "--date", "2024-05-04", "--json"], { encoding: "utf8" }),
      );
      assert.equal(json.parsha, "Achrei Mos");
    });
  }

  test("rejects an unknown city", () => {
    assert.throws(() => execFileSync(process.execPath, [cli, "--city", "atlantis"], { stdio: "pipe" }));
  });
});
