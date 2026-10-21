# Contributing

## Running the tests

| Suite | Needs | Command |
|---|---|---|
| Library tests, doctests, MyZmanim site fixture, preset catalog snapshot | Rust | `cargo test -p kosher-rust` |
| KosherJava conformance | Java 21, Maven | see below |
| hebcal conformance (limudim) | Bun, `wasm32-unknown-unknown` target | see below |

The library suite is what you run while working. The two conformance suites compare against
the reference implementations and run in CI on every push.

### KosherJava

```bash
cd third-party/kosher-java && mvn --batch-mode -Dmaven.javadoc.skip=true package && cd -
cd interop-tests/java_interop_tests && cargo test
```

Each test compares random dates and locations against KosherJava. Raise the counts for a
thorough run (CI uses 10000):

- `ZMANIM_JAVA_PARITY_ITERATIONS`
- `ZMANIM_HEBREW_DATE_JAVA_PARITY_ITERATIONS`
- `ZMANIM_LIMUDIM_JAVA_PARITY_ITERATIONS`

### hebcal

```bash
rustup target add wasm32-unknown-unknown
cd interop-tests/js_interop_tests && cargo test --test js_interop
```

## The preset catalog snapshot

`tests/fixtures/zman_catalog.snap` records the definition of every zman preset: its
KosherJava name, display name, category, formula and description. If you change a preset on
purpose, regenerate the snapshot and review the diff:

```bash
UPDATE_SNAPSHOT=1 cargo test -p kosher-rust catalog_snapshot
```
