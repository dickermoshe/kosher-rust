import { fileURLToPath } from "node:url";

import { defineConfig } from "vitest/config";

const entry = (file: string) => fileURLToPath(new URL(`./dist/${file}`, import.meta.url));

// The suite runs twice: against the Node entry, which reads the .wasm file, and against the
// browser entry, which inlines it.
export default defineConfig({
  test: {
    projects: [
      { resolve: { alias: { "kosher-rust": entry("node.js") } }, test: { name: "node entry", include: ["test/**/*.test.ts"] } },
      { resolve: { alias: { "kosher-rust": entry("browser.js") } }, test: { name: "browser entry", include: ["test/**/*.test.ts"] } },
    ],
  },
});
