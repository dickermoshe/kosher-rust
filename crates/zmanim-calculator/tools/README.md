# Zmanim Preset Tools

## The DSL

`dsl.py` is the source of truth for the generated presets. It contains everything 
needed to generate the zmanim presets.

## Rust Generation


Run it from this directory:

```bash
uv run python generate-rust.py
```

The generated Rust file should not be edited by hand. Update `dsl.py`/`generate-rust.py`, then
regenerate.

## Other Languages

The DSL is not Rust-specific. Other developers are welcome to use `dsl.py` to
generate presets for any language or runtime they want. The intended split is:

- keep `dsl.py` as the shared semantic model of the zman presets;
- write a language-specific generator like `generate-rust.py`;
- implement the target language's equivalent of the primitive calculations.

## KosherJava Differences

This project follows KosherJava naming and behavior closely, but it differs in a
few places. Check the [README](../README.md) in the parent directory for the current list of
known differences before using this DSL.
