import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// kosher-rust needs nothing here. `server.fs.allow` only lets the dev server read the package from
// ../.., where this example links it from instead of from npm. The WebAssembly is inlined in the
// package's JavaScript, which makes the chunk larger than Vite's default warning size.
//
// The app is index.html. slim.html uses `kosher-rust/slim`, which loads the WebAssembly as a file.
export default defineConfig({
  plugins: [react()],
  server: { fs: { allow: ["../.."] } },
  build: {
    chunkSizeWarningLimit: 1000,
    rollupOptions: { input: { app: "index.html", slim: "slim.html" } },
  },
});
