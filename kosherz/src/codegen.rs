//! Writes the binding enums that mirror kosher-rust's large, frequently extended enums, `Zman` and
//! `MyZman`, together with their conversions.
//!
//! WeaveFFI reads the exported module's source tokens, so these enums have to be written out in
//! `src/api.rs` rather than built by a macro. They live between `// BEGIN GENERATED: <name>` and
//! `// END GENERATED: <name>` markers. The test fails when a region is out of date; run
//! `UPDATE_GENERATED=1 cargo test -p kosherz codegen` to rewrite them.
//!
//! The same test checks that every package under `generated/` has a copy of the repository's
//! `LICENSE`, and the same command copies it.
//!
//! kosher-rust's enums are `#[non_exhaustive]`, so the conversion from kosher-rust is a `TryFrom`
//! that fails for a variant the bindings don't have yet. This test fails until they are
//! regenerated.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{fmt::Write as _, fs, path::Path};

use kosher_rust::{Zman, myzmanim::MyZman, zmanim::Category};

/// One generated region: the file it is in, its marker name and its contents.
pub struct Region {
    /// Path relative to the crate root.
    pub file: &'static str,
    /// The name after `BEGIN GENERATED:`.
    pub name: &'static str,
    /// The lines between the markers.
    pub body: String,
}

/// Every generated region.
#[allow(deprecated)]
pub fn regions() -> Vec<Region> {
    let zmanim: Vec<Variant> = Zman::ALL
        .into_iter()
        .map(|zman| Variant {
            ident: format!("{zman:?}"),
            doc: format!("{} (KosherJava `{}`).", zman.name(), zman.java_name()),
            deprecated: zman.is_deprecated(),
            needs_time_zone: zman.category() == Category::KidushLevana,
        })
        .collect();
    let my_zmanim: Vec<Variant> = MyZman::ALL
        .into_iter()
        .map(|zman| Variant {
            ident: format!("{zman:?}"),
            doc: format!("{}.", zman.name()),
            deprecated: false,
            needs_time_zone: false,
        })
        .collect();

    vec![
        Region {
            file: "src/api.rs",
            name: "Zman",
            body: render_enum(
                "Zman",
                "    ",
                &[
                    "A halachic time this library can calculate, named after the KosherJava getter that",
                    "calculates it, without `get`. Its value is its position in `all_zmanim`, which changes",
                    "when a zman is added, so store `zman_java_name` rather than the number.",
                ],
                &zmanim,
            ),
        },
        Region {
            file: "src/convert/zmanim.rs",
            name: "Zman",
            body: render_conversions("times::Zman", "kosher_rust::Zman", &zmanim),
        },
        Region {
            file: "src/api.rs",
            name: "MyZman",
            body: render_enum(
                "MyZman",
                "        ",
                &["One row of the MyZmanim website's \"All zmanim\" table, in the table's order."],
                &my_zmanim,
            ),
        },
        Region {
            file: "src/convert/myzmanim.rs",
            name: "MyZman",
            body: render_conversions("times::myzmanim::MyZman", "kosher_rust::myzmanim::MyZman", &my_zmanim),
        },
    ]
}

struct Variant {
    ident: String,
    doc: String,
    deprecated: bool,
    needs_time_zone: bool,
}

/// The enum, indented by `indent` to sit in its module.
fn render_enum(name: &str, indent: &str, doc: &[&str], variants: &[Variant]) -> String {
    let mut out = String::new();
    for line in doc {
        writeln!(out, "{indent}/// {line}").unwrap();
    }
    writeln!(out, "{indent}#[weaveffi::enumeration]").unwrap();
    writeln!(out, "{indent}#[repr(i32)]").unwrap();
    writeln!(out, "{indent}#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
    writeln!(out, "{indent}pub enum {name} {{").unwrap();
    for (value, variant) in variants.iter().enumerate() {
        writeln!(out, "{indent}    /// {}", variant.doc).unwrap();
        if variant.deprecated {
            writeln!(out, "{indent}    ///").unwrap();
            writeln!(out, "{indent}    /// Deprecated in KosherJava.").unwrap();
        }
        if variant.needs_time_zone {
            writeln!(out, "{indent}    ///").unwrap();
            writeln!(
                out,
                "{indent}    /// This time needs a time zone. Without one it fails with `ZmanimError::TimeZoneRequired`."
            )
            .unwrap();
        }
        writeln!(out, "{indent}    {} = {value},", variant.ident).unwrap();
    }
    writeln!(out, "{indent}}}").unwrap();
    out
}

/// `From` the binding enum (`ours`, a path under `crate::api`) to kosher-rust's (`theirs`), and
/// `TryFrom` back, which fails for a variant the bindings don't have.
fn render_conversions(ours: &str, theirs: &str, variants: &[Variant]) -> String {
    let mut out = String::new();
    let ours = format!("crate::api::{ours}");
    writeln!(out, "#[allow(deprecated)]").unwrap();
    writeln!(out, "#[rustfmt::skip]").unwrap();
    writeln!(out, "impl From<{ours}> for {theirs} {{").unwrap();
    writeln!(out, "    fn from(value: {ours}) -> Self {{").unwrap();
    writeln!(out, "        use {ours} as F;").unwrap();
    writeln!(out, "        match value {{").unwrap();
    for variant in variants {
        writeln!(out, "            F::{0} => Self::{0},", variant.ident).unwrap();
    }
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out, "#[allow(deprecated)]").unwrap();
    writeln!(out, "#[rustfmt::skip]").unwrap();
    writeln!(out, "impl TryFrom<{theirs}> for {ours} {{").unwrap();
    writeln!(out, "    type Error = ();").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "    fn try_from(value: {theirs}) -> Result<Self, ()> {{").unwrap();
    writeln!(out, "        use {theirs} as F;").unwrap();
    writeln!(out, "        Ok(match value {{").unwrap();
    for variant in variants {
        writeln!(out, "            F::{0} => Self::{0},", variant.ident).unwrap();
    }
    writeln!(out, "            _ => return Err(()),").unwrap();
    writeln!(out, "        }})").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();
    out
}

/// `source` with the region named `name` replaced by `body`.
pub fn splice(source: &str, name: &str, body: &str) -> String {
    let begin = format!("// BEGIN GENERATED: {name}\n");
    let end = format!("// END GENERATED: {name}");
    let start = source.find(&begin).unwrap_or_else(|| panic!("no `{begin}` marker")) + begin.len();
    let stop = start
        + source[start..]
            .find(&end)
            .unwrap_or_else(|| panic!("no `{end}` marker"));
    // Keep the end marker's indentation.
    let stop = source[..stop].rfind('\n').map_or(stop, |newline| newline + 1);
    format!("{}{body}{}", &source[..start], &source[stop..])
}

/// Rewrites every region under `crate_root`, returning the files that changed.
pub fn update(crate_root: &Path) -> Vec<&'static str> {
    let mut changed = Vec::new();
    for region in regions() {
        let path = crate_root.join(region.file);
        let source = fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
        let updated = splice(&source, region.name, &region.body);
        if updated != source {
            fs::write(&path, updated).unwrap();
            changed.push(region.file);
        }
    }
    changed
}

#[test]
fn generated_regions_are_current() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    if std::env::var_os("UPDATE_GENERATED").is_some() {
        update(root);
        return;
    }
    for region in regions() {
        let source = fs::read_to_string(root.join(region.file))
            .unwrap()
            .replace("\r\n", "\n");
        assert!(
            splice(&source, region.name, &region.body) == source,
            "{} region `{}` is out of date: run `UPDATE_GENERATED=1 cargo test -p kosherz codegen`",
            region.file,
            region.name,
        );
    }
}

#[test]
fn packages_have_the_license() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let license = fs::read_to_string(root.join("../LICENSE"))
        .unwrap()
        .replace("\r\n", "\n");
    for package in fs::read_dir(root.join("generated")).unwrap() {
        let package = package.unwrap();
        if !package.file_type().unwrap().is_dir() || package.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let path = package.path().join("LICENSE");
        if std::env::var_os("UPDATE_GENERATED").is_some() {
            fs::write(&path, &license).unwrap();
            continue;
        }
        let copy = fs::read_to_string(&path).unwrap_or_default().replace("\r\n", "\n");
        assert!(
            copy == license,
            "{} is missing or out of date: run `UPDATE_GENERATED=1 cargo test -p kosherz codegen`",
            path.display(),
        );
    }
}
