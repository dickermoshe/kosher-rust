// The entry for Node, Bun and Deno, which read the WebAssembly file next to this one.
import { readFileSync } from "node:fs";

import { start } from "./runtime.js";

start(() => readFileSync(new URL("./wasm/kr4js_bg.wasm", import.meta.url)));

export * from "./index.js";
