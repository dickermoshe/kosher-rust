# Contributing

## Running the tests

| Suite | Needs | Command |
|---|---|---|
| Library tests, doctests (including the README), MyZmanim site fixture, zman catalog snapshot | Rust | `cargo test -p kosher-rust` |
| Benchmarks | Rust | `cargo bench -p kosher-rust` |
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

`tests/fixtures/zman_catalog.snap` records the definition of every zman: its
KosherJava name, display name, category, formula and description. If you change a zman on
purpose, regenerate the snapshot and review the diff:

```bash
UPDATE_SNAPSHOT=1 cargo test -p kosher-rust catalog_snapshot
```

## Updating the vendored KosherJava

`third-party/kosher-java` is a git subtree of upstream KosherJava. It is the reference the
Java conformance suite compares against. The tracked commit is listed under "Port status" in the
[README](README.md).

```bash
git subtree pull --prefix=third-party/kosher-java https://github.com/KosherJava/zmanim <tag-or-branch> --squash
```

Then port any new or changed zmanim (see below), update the tracked commit in the README, and run the
Java conformance suite.

## Adding a zman

Every zman is one entry in [`src/zmanim/catalog.rs`](src/zmanim/catalog.rs), under its
category: a doc comment for developers, the KosherJava getter, a display name, the formula, and
an `about` text for end users. Then:

1. `cargo test -p kosher-rust catalog_snapshot` fails, showing the new entry.
2. Regenerate the snapshot with `UPDATE_SNAPSHOT=1` and check the diff contains only your entry.
3. Run the Java conformance suite; it picks the new zman up from `Zman::ALL`.

## Writing docs

Docs are for people using the crate. Notes for maintainers go in `//` comments next to the code
they explain.

- **README**: it is also the crate's front page on docs.rs, and its examples run as doctests.
  Keep it to install, one example per area, features, and what is not ported.
- **Modules**: two or three sentences on what the module is for, one runnable example, and the
  types to reach for. No architecture.
- **Types and their main methods**: what it is, when to use it, and when it fails. Start
  `Option` docs with "Returns `None` when..."; give `Result` an `# Errors` section.
- **Enum variants**: one line. Name the opinion for a `Zman`; give the transliteration and the
  Hebrew for a `SpecialDay`, `Parsha` or `Tractate`.
- **Constants**: the value, its unit, and where it comes from.
- **Never in `///`**: how something is computed internally, which private helper it calls, or the
  history of the port.

`cargo clippy` and `cargo doc` enforce the basics: every public item documented, no broken
links, and an `# Errors` section on every function that returns `Result`.

## Releasing

1. Move the "Unreleased" section of `CHANGELOG.md` under the new version.
2. Bump `version` in `Cargo.toml`.
3. Wait for CI, including both conformance suites, to pass on `main`.
4. `cargo publish -p kosher-rust`, then tag the commit `vX.Y.Z`.
