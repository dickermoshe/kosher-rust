// The entry for browsers and bundlers. The WebAssembly is inlined, so it needs no bundler plugin,
// no server route and no `await`.
import { start } from "./runtime.js";
import { WASM_BASE64 } from "./wasm/inline.js";

start(() => {
  const fromBase64 = (Uint8Array as { fromBase64?: (text: string) => Uint8Array }).fromBase64;
  if (fromBase64) return fromBase64(WASM_BASE64);
  const binary = atob(WASM_BASE64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) bytes[i] = binary.charCodeAt(i);
  return bytes;
});

export * from "./index.js";
