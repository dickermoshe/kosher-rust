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

## Updating the vendored KosherJava

`third-party/kosher-java` is a git subtree of upstream KosherJava. It is the reference the
Java conformance suite compares against. Currently tracked: commit
[cb2b183](https://github.com/KosherJava/zmanim/commit/cb2b1835ae875c5fb2fbd3b35ae18f523ab93bfe)
(post-3.0.0 master).

```bash
git subtree pull --prefix=third-party/kosher-java https://github.com/KosherJava/zmanim <tag-or-branch> --squash
```

Then port any new or changed zmanim (see below), update the tracked commit above, and run the
Java conformance suite.

## Adding a zman

Every zman is one entry in [`src/zmanim/catalog.rs`](src/zmanim/catalog.rs), under its
category: a doc comment for developers, the KosherJava getter, a display name, the formula, and
an `about` text for end users. Then:

1. `cargo test -p kosher-rust catalog_snapshot` fails, showing the new entry.
2. Regenerate the snapshot with `UPDATE_SNAPSHOT=1` and check the diff contains only your entry.
3. Run the Java conformance suite; it picks the new zman up from `Zman::ALL`.
