This Dart package is the Java/Rust parity harness for the Rust Zmanim project.
It drives both runtimes with the same randomized and regression inputs and checks
that results match (or fail in the same way for invalid inputs).

## Layout

```
test/
├── bin/
│   ├── jewish_date_test.dart   # Jewish date/calendar parity entry point
│   └── zmanim_test.dart        # Zmanim parity entry point
├── lib/src/
│   ├── harness/                # Shared setup, options, ranges, comparison helpers
│   ├── jewish_date/            # Jewish date case generation, Java adapter, runner
│   ├── zmanim/                 # Zmanim case generation, Java adapter, runner
│   ├── java/                   # Generated Java bindings (jnigen)
│   └── rust/                   # Generated Rust bindings (flutter_rust_bridge) + thin api wrapper
├── java/                       # KosherJava source subtree and Maven build
└── rust/                       # FRB bridge crate used by this harness
```

## Prerequisites

```
# Cargo
cargo install cargo-expand
cargo install flutter_rust_bridge_codegen

# Dart
dart pub get
dart run jni:setup

# Java
# Ensure Java and Maven are installed and JAVA_HOME is set.
# On Windows, ensure JAVA_HOME\bin\server is on PATH (for jvm.dll).
```

## Build and Code Generation

Generate Dart bindings when Java or Rust bridge APIs change:

```
flutter_rust_bridge_codegen generate
dart run jnigen.dart
```

Build the Rust bridge crate and Java jar used by the harness:

```
dart run build.dart
```

## Run the Harness

These are the same entry points used in CI:

```
dart run bin/zmanim_test.dart --iterations 10000
dart run bin/jewish_date_test.dart --iterations 10000
```

Useful flags:

```
# Shared
--seed <int>
--iterations <int>   # or TEST_ITERATIONS env var
--min-year <int>
--max-year <int>

# Zmanim-only
--filter <substring>
--allow-null-mismatch
```

## Behavior Notes

- `zmanim_test.dart` compares Java and Rust zman timestamps in Dart with a max
  allowed millisecond delta for model differences near edge astronomical cases.
- `jewish_date_test.dart` allows random invalid Jewish dates and expects Java/Rust
  to agree on rejection (`null`/`null`).
- Jewish calendar parity is asserted by passing a Java snapshot into the Rust
  bridge (`test_jewish_calendar`) and checking Rust-side assertions.
- Zmanim regression cases live in `lib/src/zmanim/cases.dart` and always run,
  even when random methods are filtered.

## Updating KosherJava Subtree

Replace `<branch-name>` with the desired KosherJava branch:

```
git subtree pull --prefix=test/java https://github.com/KosherJava/zmanim <branch-name> --squash
```
