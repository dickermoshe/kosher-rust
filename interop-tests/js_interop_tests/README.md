# JS Interop Tests

JavaScript/TypeScript tests that compare the Rust WASM bindings (`limudim_wasm`) against [@hebcal/learning](https://github.com/hebcal/hebcal-learning) reference implementations.
You must have [Bun](https://bun.sh/) installed to run the tests.

## Run tests

Run the following commands to build and run the tests.

```bash
bun install
bun run build
bun test
# To run a single test file:
bun test tests/daf_yomi.test.ts
```

