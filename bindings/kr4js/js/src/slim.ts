// The entry without the inlined WebAssembly. Call `init` once before anything else.
import { load } from "./runtime.js";

/**
 * Where to load the WebAssembly from: a URL, a `fetch` response, the file's bytes, or a compiled
 * `WebAssembly.Module`, or a promise of one.
 */
export type WasmSource = URL | string | Request | Response | BufferSource | WebAssembly.Module;

/**
 * Loads the WebAssembly. Call it once, before anything else in the package. Later calls do nothing.
 * Using the package before it has finished throws.
 *
 * With no argument, it fetches the WebAssembly file from next to this module. Vite, webpack and
 * Parcel copy that file into your build on their own. Pass a source to load it from somewhere else.
 *
 * ```ts
 * import { HebrewDate, init } from "@dickermoshe/kr4js/slim";
 *
 * await init();
 * HebrewDate.from({ year: 2024, month: 4, day: 23 }).toString(); // "15 Nisan 5784"
 * ```
 *
 * In Node, pass the file's bytes, since `fetch` doesn't read files. In Cloudflare Workers, import
 * `@dickermoshe/kr4js/kr4js.wasm` and pass the module it gives.
 */
export function init(source?: WasmSource | Promise<WasmSource>): Promise<void> {
  return load(source ?? new URL("./wasm/kr4js_bg.wasm", import.meta.url));
}

export * from "./index.js";
