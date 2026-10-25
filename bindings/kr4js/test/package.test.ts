// The published package: what's in it, and that each entry point loads the WebAssembly its way.
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";

import { afterAll, beforeAll, describe, expect, test } from "vitest";

const root = new URL("../", import.meta.url);
const read = (path: string) => readFileSync(new URL(path, root), "utf8");
const pkg = JSON.parse(read("package.json"));
const npm = process.platform === "win32" ? "npm.cmd" : "npm";
const shell = process.platform === "win32";

describe("package.json", () => {
  test("reads the file in Node and inlines it everywhere else, with no runtime-specific entries", () => {
    expect(pkg.exports["."]).toEqual({ types: "./dist/index.d.ts", node: "./dist/node.js", default: "./dist/browser.js" });
  });

  test("marks the entries as having side effects, so bundlers keep the loader", () => {
    expect(pkg.sideEffects).toEqual(["./dist/browser.js", "./dist/node.js"]);
  });

  test("publishes dist and nothing from the sources", () => {
    const output = execFileSync(npm, ["pack", "--dry-run", "--json"], { cwd: root, encoding: "utf8", shell });
    const files: string[] = JSON.parse(output)[0].files.map((f: { path: string }) => f.path);
    for (const file of ["dist/index.d.ts", "dist/browser.js", "dist/node.js", "dist/slim.js", "dist/slim.d.ts", "dist/wasm/kosher_rust_bg.wasm", "dist/wasm/inline.js", "dist/wasm/module.d.ts", "README.md", "LICENSE"]) {
      expect(files).toContain(file);
    }
    expect(files.filter((f) => f.startsWith("js/") || f.startsWith("src/") || f.startsWith("test/"))).toEqual([]);
  });
});

describe("entries", () => {
  test("the browser entry needs no URL, fetch or top-level await", () => {
    const browser = read("dist/browser.js");
    const glue = read("dist/wasm/kosher_rust.js");
    for (const source of [browser, glue, read("dist/index.js"), read("dist/runtime.js")]) {
      expect(source).not.toMatch(/new URL\(/);
      expect(source).not.toMatch(/^await /m);
    }
    expect(browser).not.toMatch(/node:/);
  });

  test("the slim entry leaves out the inlined WebAssembly and points bundlers at the file", () => {
    expect(pkg.exports["./slim"]).toEqual({ types: "./dist/slim.d.ts", default: "./dist/slim.js" });
    expect(pkg.exports["./kosher_rust.wasm"]).toEqual({ types: "./dist/wasm/module.d.ts", default: "./dist/wasm/kosher_rust_bg.wasm" });
    const slim = read("dist/slim.js");
    expect(slim).not.toMatch(/inline\.js/);
    // Vite, webpack and Parcel copy a file referred to this way into the build.
    expect(slim).toMatch(/new URL\("\.\/wasm\/kosher_rust_bg\.wasm", import\.meta\.url\)/);
  });

  test("the WebAssembly stays small", () => {
    const wasm = readFileSync(new URL("dist/wasm/kosher_rust_bg.wasm", root));
    // No time zone database and no panic messages.
    expect(wasm.includes("America/New_York")).toBe(false);
    expect(wasm.includes("TZif")).toBe(false);
    expect(wasm.includes("called `Result::unwrap()`")).toBe(false);
    expect(gzipSync(wasm, { level: 9 }).length).toBeLessThan(130 * 1024);
  });
});

describe("types", () => {
  const tsc = fileURLToPath(new URL("node_modules/typescript/bin/tsc", root));
  let project = "";

  // A TypeScript project with the packed package installed, as a user gets it from npm.
  beforeAll(() => {
    project = mkdtempSync(join(tmpdir(), "kosher-rust-types-"));
    const packed = execFileSync(npm, ["pack", "--json", "--pack-destination", project], { cwd: root, encoding: "utf8", shell });
    const tarball = join(project, JSON.parse(packed)[0].filename);
    writeFileSync(join(project, "package.json"), '{ "private": true, "type": "module" }\n');
    execFileSync(npm, ["install", "--offline", "--no-audit", "--no-fund", tarball], { cwd: project, shell });
    copyFileSync(fileURLToPath(new URL("test/fixtures/consumer/use.ts", root)), join(project, "use.ts"));
  }, 120_000);

  afterAll(() => {
    if (project) rmSync(project, { recursive: true, force: true });
  });

  test.each([
    ["node16", "node16"],
    ["nodenext", "nodenext"],
    ["esnext", "bundler"],
    ["commonjs", "node10"],
  ])("check without skipLibCheck under module %s and moduleResolution %s", (module, moduleResolution) => {
    const args = ["--noEmit", "--strict", "--target", "es2022", "--lib", "es2022,dom", "--module", module, "--moduleResolution", moduleResolution, "use.ts"];
    let errors = "";
    try {
      execFileSync(process.execPath, [tsc, ...args], { cwd: project, encoding: "utf8" });
    } catch (error) {
      errors = String((error as { stdout?: string }).stdout);
    }
    expect(errors).toBe("");
  }, 60_000);
});
