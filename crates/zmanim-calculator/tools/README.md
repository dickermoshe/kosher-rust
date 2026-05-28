# Zmanim Presets Generation

This directory contains the code and scripts for generating Rust zman presets from the KosherJava source, producing the `ZmanPreset` statics in `../src/presets_gen.rs`.

## Quick commands

Install the Python dependencies:

```bash
uv sync
```

Regenerate only the Rust preset file:

```bash
uv run python generate-rust.py
```

Regenerate documentation from KosherJava Javadocs, then regenerate Rust:

```bash
uv run python generate-docs.py
uv run python generate-rust.py
```

Documentation generation uses the OpenAI API. Put the key in `tools/.env`:

```env
OPENAI_API_KEY=sk-...
```

## Important files

| File | Hand edit? | Purpose |
| ---- | ---------- | ------- |
| `dsl.py` | Yes | Maps each KosherJava `get*` method to a `ZmanPrimitive` formula. |
| `generate-docs.py` | Yes | Parses KosherJava Javadocs and writes user-facing docs. |
| `generate-rust.py` | Yes | Joins `dsl.py`, `docs.py`, and `deprecated.py`, then emits Rust presets. |
| `docs.py` | No | Generated documentation strings keyed by KosherJava method name. |
| `deprecated.py` | No | Generated list of deprecated KosherJava method names. |
| `../src/presets_gen.rs` | No | Generated Rust preset constants. |
| `../src/presets.rs` | Yes | Public wrapper module for generated presets. |
| `../src/primitive_zman.rs` | Yes | Runtime implementations for primitive zman calculations. |
| `../java/` | Vendored | KosherJava source used for docs and parity tests. |

`docs.py`, `deprecated.py`, and `presets_gen.rs` are generated, but they are
still checked in.

## How the pipeline works

1. `generate-docs.py` scans `../java/src/main/java` with tree-sitter.
2. It keeps public no-argument methods returning `java.time.Instant` from:
   `AstronomicalCalendar`, `ZmanimCalendar`, and
   `ComprehensiveZmanimCalendar`.
3. It turns the attached Javadocs into concise user documentation and writes
   `docs.py`; it also writes `deprecated.py` from Java `@Deprecated`
   annotations.
4. `dsl.py` provides the formula for each supported KosherJava getter.
5. `generate-rust.py` validates that `docs.py` and `dsl.py` have matching
   method keys, then writes `../src/presets_gen.rs`. Deprecated presets are
   emitted behind `#[cfg(test)]` so parity tests can cover them without exposing
   them in the normal public preset surface.

The Java method bodies are not parsed into formulas. If a zman changes upstream,
read the Java implementation or parity tests, update `dsl.py`, and regenerate.

## Common workflows

### Change a formula

1. Update the relevant entry in `ZMAN_NAMES` inside `dsl.py`.
2. Run `uv run python generate-rust.py`.
3. Run `cargo test` from `crates/zmanim-calculator`.

### Change generated wording

1. Update `generate-docs.py` prompts or parsing logic.
2. Run `uv run python generate-docs.py`.
3. Run `uv run python generate-rust.py`.
4. Review both generated files before committing.

For faster prompt iteration, generate a sample:

```bash
uv run python generate-docs.py --dev
uv run python generate-docs.py --count 5 --seed 42
```

Sample runs replace `docs.py` with only the sampled methods. `deprecated.py`
still uses all parsed KosherJava methods. Run the full documentation generation
before regenerating Rust for release-quality output.

### Add a new zman

1. Confirm the KosherJava getter is included by `generate-docs.py`.
2. Add the method name to the `ZmanimMethods` literal in `dsl.py` if needed.
3. Add a matching `ZMAN_NAMES` entry with the correct primitive formula.
4. Regenerate docs if the method needs documentation.
5. Run `uv run python generate-rust.py`.
6. Run `cargo test`.

If no existing `ZmanPrimitive` can express the calculation, add the runtime
primitive in `../src/primitive_zman.rs` and teach `generate-rust.py` how to emit
it.

## Porting note

The reusable part is `dsl.py`: it is a compact semantic model for the KosherJava
zman surface. A port to another language can keep the Java parsing and DSL, then
replace `generate-rust.py` with a language-specific emitter and runtime
primitive evaluator.
