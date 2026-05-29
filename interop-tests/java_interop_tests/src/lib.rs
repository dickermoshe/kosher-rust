#![allow(
    warnings,
    missing_docs,
    clippy::panic,
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]

#[allow(warnings, missing_docs, clippy::all, clippy::pedantic, clippy::nursery)]
pub(crate) mod java_bindings {
    include!(concat!(env!("OUT_DIR"), "/java_bindings.rs"));
}

pub(crate) mod calendar;
pub(crate) mod limudim;
pub(crate) mod zmanim;

use std::{error::Error, sync::OnceLock};

use jni::{InitArgsBuilder, JNIVersion, JavaVM, objects::JClassLoader, objects::LoaderContext};

static JVM: OnceLock<JavaVM> = OnceLock::new();
static BINDINGS_INITIALIZED: OnceLock<()> = OnceLock::new();

/// Returns the process-wide JVM used by calendar, zmanim, and limudim parity tests.
///
/// JNI allows only one JVM per process; every module must share this instance.
pub(crate) fn java_vm() -> &'static JavaVM {
    JVM.get_or_init(|| {
        let classpath = format!(
            "-Djava.class.path={}",
            std::env::var("KOSHER_JAVA_CLASSPATH").unwrap_or_else(|_| {
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("..")
                    .join("third-party")
                    .join("kosher-java")
                    .join("target")
                    .join("classes")
                    .to_string_lossy()
                    .into_owned()
            })
        );
        let args = InitArgsBuilder::new()
            .version(JNIVersion::V1_8)
            .option(&classpath)
            .build()
            .expect("failed to build JVM init args");
        JavaVM::new(args).expect("failed to create JVM for java parity tests")
    })
}

/// Initializes generated Java bindings once for the current JNI environment.
pub(crate) fn init_bindings(env: &mut jni::Env<'_>) -> Result<(), Box<dyn Error>> {
    if BINDINGS_INITIALIZED.get().is_some() {
        return Ok(());
    }

    let loader = JClassLoader::get_system_class_loader(env)?;
    let loader_context = LoaderContext::Loader(&loader);
    jni::__test_bindings_init(env, &loader_context);
    crate::java_bindings::jni_init(env, &loader_context)?;
    let _ = BINDINGS_INITIALIZED.set(());
    Ok(())
}
