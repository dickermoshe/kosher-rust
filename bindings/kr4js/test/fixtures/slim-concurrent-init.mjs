// Prints "ok" when kosher-rust/slim loads the WebAssembly once however `init` is called. Runs in a
// fresh process, where nothing else has loaded the WebAssembly yet.
import { readFileSync } from "node:fs";

import { HebrewDate, init } from "../../dist/slim.js";

const bytes = readFileSync(new URL("../../dist/wasm/kosher_rust_bg.wasm", import.meta.url));
const wasm = () => new Response(bytes, { headers: { "content-type": "application/wasm" } });
const broken = () => new Response(new Uint8Array([0, 1, 2, 3]), { headers: { "content-type": "application/wasm" } });

// A failed load can be retried.
const failed = await init(broken()).then(
  () => false,
  () => true,
);

// A call made while the first is loading waits for it and never reads its own source.
await Promise.all([init(wasm()), init(broken())]);

console.log(failed && HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString() === "15 Nisan 5784" ? "ok" : "failed");
