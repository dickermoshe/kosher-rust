// The slim entry, which loads the WebAssembly from wherever `init` is told to.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { describe, expect, test } from "vitest";

import { dafYomiBavli, HebrewDate, init, Zmanim } from "../dist/slim.js";

const bytes = readFileSync(new URL("../dist/wasm/kr4js_bg.wasm", import.meta.url));

describe("kr4js/slim", () => {
  test("throws before init is called", () => {
    const script = fileURLToPath(new URL("./fixtures/slim-before-init.mjs", import.meta.url));
    expect(execFileSync(process.execPath, [script], { encoding: "utf8" }).trim()).toBe("threw");
  });

  test("loads the WebAssembly once when init is called again while loading", () => {
    const script = fileURLToPath(new URL("./fixtures/slim-concurrent-init.mjs", import.meta.url));
    expect(execFileSync(process.execPath, [script], { encoding: "utf8" }).trim()).toBe("ok");
  });

  test("works once init has loaded the WebAssembly", async () => {
    // A Response is what `fetch` gives in a browser, so this takes the same path.
    await init(new Response(bytes, { headers: { "content-type": "application/wasm" } }));
    expect(HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString()).toBe("15 Nisan 5784");
    expect(String(dafYomiBavli({ year: 2024, month: 4, day: 23 }))).toBe("Bava Metzia 55");
    const zmanim = new Zmanim({ location: { latitude: 40.09, longitude: -74.22 }, timeZone: "America/New_York", date: { year: 2024, month: 4, day: 23 } });
    expect(zmanim.get("sofZmanShmaGra")?.toISOString()).toMatch(/^2024-04-23T13:31/);
  });

  test("ignores later calls", async () => {
    await init(new WebAssembly.Module(bytes));
    // Never fetched.
    await init("https://example.invalid/kr4js.wasm");
    expect(HebrewDate.from({ year: 2024, month: 4, day: 24 }).toString()).toBe("16 Nisan 5784");
  });
});
