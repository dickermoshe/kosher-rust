//! Runs the Bun/TypeScript WASM interop tests via Cargo.
//!
//! Requires [Bun](https://bun.sh/) and the `wasm32-unknown-unknown` target.
//! Run with: `cargo test --test js_interop` (from this crate directory).
#![allow(clippy::expect_used, clippy::panic)]
use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn package_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn run(working_dir: &Path, command: &str, args: &[&str]) {
    eprintln!("Running: {command} {}", args.join(" "));

    let output = Command::new(command)
        .args(args)
        .current_dir(working_dir)
        .output()
        .unwrap_or_else(|error| panic!("failed to execute {command}: {error}"));

    if !output.status.success() {
        eprintln!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
        panic!("{command} {} failed with status {}", args.join(" "), output.status);
    }
}

fn maybe_bun_install(working_dir: &Path) {
    if !working_dir.join("node_modules").exists() {
        run(working_dir, "bun", &["install"]);
    }
}

#[test]
fn bun_interop_tests() {
    let working_dir = package_dir();

    maybe_bun_install(&working_dir);
    run(&working_dir, "bun", &["run", "build"]);
    run(&working_dir, "bun", &["test", "tests", "--timeout", "60000"]);
}
