// Builds the npm package into dist/.
//
// 1. Regenerates the parity fixture from the Rust crate.
// 2. Compiles the crate to WebAssembly with wasm-pack and wasm-bindgen, into js/src/wasm/.
// 3. Inlines the WebAssembly as base64 for the browser entry.
// 4. Compiles the TypeScript and copies the WebAssembly glue next to it.
// 5. Copies the repository's LICENSE into the package.
import { execFileSync } from "node:child_process";
import { cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const wasmDir = join(root, "js", "src", "wasm");
const dist = join(root, "dist");

function run(command, args, env = {}) {
  console.log(`> ${command} ${args.join(" ")}`);
  execFileSync(command, args, { cwd: root, stdio: "inherit", env: { ...process.env, ...env } });
}

run("cargo", ["test", "--quiet", "-p", "kr4js", "--test", "parity"], { UPDATE_GENERATED: "1" });

// Built for size. A panic aborts without a message, which drops the formatting code and the text
// of every message the libraries might panic with. `Debug` formatting and source locations are
// left out too, and the standard library is built with its smaller implementations. These need
// the standard library rebuilt, which stable Rust allows with RUSTC_BOOTSTRAP.
run(
  "wasm-pack",
  [
    "build",
    "--release",
    "--target",
    "web",
    "--out-dir",
    "js/src/wasm",
    "--out-name",
    "kr4js",
    "--no-pack",
    "--",
    "--config",
    'unstable.build-std=["std", "panic_abort"]',
    "--config",
    'unstable.build-std-features=["optimize_for_size"]',
    "--config",
    "profile.release.lto=true",
    "--config",
    "profile.release.codegen-units=1",
    "--config",
    'profile.release.opt-level="z"',
  ],
  {
    RUSTC_BOOTSTRAP: "1",
    RUSTFLAGS: "-Zunstable-options -Cpanic=immediate-abort -Zfmt-debug=none -Zlocation-detail=none",
    CARGO_TARGET_DIR: join(root, "..", "..", "target", "npm"),
  },
);

// The glue's async initializer would load the .wasm file by URL. Each entry point loads it its own
// way, and a `new URL(..., import.meta.url)` left in makes bundlers copy the file into every build.
const gluePath = join(wasmDir, "kr4js.js");
const glue = readFileSync(gluePath, "utf8");
const urlLine = "module_or_path = new URL('kr4js_bg.wasm', import.meta.url);";
if (!glue.includes(urlLine)) throw new Error("the wasm-bindgen glue changed; update scripts/build.mjs");
writeFileSync(
  gluePath,
  glue.replace(urlLine, 'throw new Error("kr4js loads its WebAssembly itself");'),
);

// The classes declare `[Symbol.dispose]`, which TypeScript only knows with this library.
const typesPath = join(wasmDir, "kr4js.d.ts");
writeFileSync(typesPath, `/// <reference lib="esnext.disposable" />\n${readFileSync(typesPath, "utf8")}`);

const wasm = readFileSync(join(wasmDir, "kr4js_bg.wasm"));
writeFileSync(join(wasmDir, "inline.js"), `export const WASM_BASE64 = "${wasm.toString("base64")}";\n`);
writeFileSync(join(wasmDir, "inline.d.ts"), "export declare const WASM_BASE64: string;\n");
// The type of `kr4js/kr4js.wasm`, which runtimes such as Cloudflare Workers import as a
// compiled module.
writeFileSync(
  join(wasmDir, "module.d.ts"),
  "declare const module: WebAssembly.Module;\nexport default module;\n",
);

rmSync(dist, { recursive: true, force: true });
run(process.execPath, [join(root, "node_modules", "typescript", "bin", "tsc"), "-p", "tsconfig.json"]);

mkdirSync(join(dist, "wasm"), { recursive: true });
for (const file of ["kr4js.js", "kr4js.d.ts", "kr4js_bg.wasm", "inline.js", "inline.d.ts", "module.d.ts"]) {
  cpSync(join(wasmDir, file), join(dist, "wasm", file));
}
cpSync(join(root, "..", "..", "LICENSE"), join(root, "LICENSE"));

console.log(`dist/ is ready: ${(wasm.length / 1024).toFixed(0)} KiB of WebAssembly`);
