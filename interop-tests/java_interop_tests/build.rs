#![allow(missing_docs)]
use jbindgen::Builder;
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let java_source_dir = manifest_dir
        .join("..")
        .join("..")
        .join("third-party")
        .join("kosher-java")
        .join("src")
        .join("main");
    emit_java_rerun_directives(&java_source_dir)?;

    let bindings_path = PathBuf::from(env::var("OUT_DIR")?).join("java_bindings.rs");
    let bindings = Builder::new()
        .root_path("crate::java_bindings")
        .input_sources(vec![java_source_dir], Vec::new(), vec!["**".to_owned()])
        .generate()?;

    fs::write(bindings_path, bindings.to_string())?;

    Ok(())
}

fn emit_java_rerun_directives(path: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            emit_java_rerun_directives(&path)?;
        } else if path.extension().is_some_and(|extension| extension == "java") {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    Ok(())
}
