# JS Interop Tests

JavaScript/TypeScript tests that compare the Rust WASM bindings (`limudim_wasm`) against [@hebcal/learning](https://github.com/hebcal/hebcal-learning) reference implementations.
You must have [Bun](https://bun.sh/) installed to run the tests.

## Run tests

From this directory, with [Bun](https://bun.sh/) and the `wasm32-unknown-unknown` target installed:

```bash
cargo test --test js_interop
```

Or run the Bun steps directly:

```bash
bun install
bun run build
bun test
# To run a single test file:
bun test tests/daf_yomi.test.ts
```

