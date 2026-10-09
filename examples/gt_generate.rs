//! Regenerate `tests/fixtures/expected/*.json` from the current parser.
//!
//! Each expected file carries the `schema_version` of its fixture:
//! 2 for a `##CIF_2.0` file (the CIF 2.0 dialect), 1 otherwise.
//!
//! Usage:
//! - `cargo run --example gt_generate`
//!   writes only **missing** expected files; existing files are left alone.
//! - `cargo run --example gt_generate -- --force`
//!   overwrites existing files, with two protections:
//!   - a file containing `"known_gap": true` is never overwritten
//!     (it holds spec-corrected expectations; fix the parser instead,
//!     then remove the flag by hand);
//!   - a human-written `expect_error.description` in an existing file is
//!     preserved when the regenerated body still has the same shape.
//!
//! After running, review `git diff -- tests/fixtures/expected` before
//! committing. The verification half lives in `tests/ground_truth.rs`
//! (`cargo test ground_truth_verify`), which is read-only.

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value as Json};

use crystal_cif_io::{canonical, CifDialect};

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn expected_path(stem: &str) -> PathBuf {
    fixture_dir().join("expected").join(format!("{stem}.json"))
}

/// Parse with the chumsky entry point. `Ok` means the whole input was
/// consumed (the top level is strict).
fn parse_cif(content: &str) -> Result<crystal_cif_io::CifDocument, String> {
    match crystal_cif_io::parse_cif(content) {
        Ok(doc) => Ok(doc),
        Err(errors) => Err(format!("{errors:?}")),
    }
}

fn expect_error(
    fixture: &str,
    stage: &str,
    violation: &str,
    description: &str,
    schema_version: u32,
) -> Json {
    json!({
        "schema_version": schema_version,
        "fixture": fixture,
        "expect_error": {
            "stage": stage,
            "violation": violation,
            "description": description,
        }
    })
}

/// Build the expected-body for one fixture from its current parse.
///
/// The `schema_version` follows the dialect of the fixture: a
/// `##CIF_2.0` magic code at the file start (after any byte order
/// mark) gives version 2; anything else stays version 1.
fn regenerate(content: &str, fixture: &str) -> Json {
    let schema_version = CifDialect::detect(content).schema_version();
    let parse = parse_cif(content);
    match parse {
        Err(msg) => expect_error(
            fixture,
            "parse",
            "unparseable",
            &format!(
                "parser rejects this file: {msg}. Spec: the file violates the \
                 CIF syntax rules recorded in docs/ground-truth.md."
            ),
            schema_version,
        ),
        Ok(doc) => {
            let violations = canonical::validate(&doc);
            if violations.is_empty() {
                canonical::document_json(&doc, fixture, schema_version)
            } else {
                let code = violations[0].code().to_string();
                let desc = violations
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("; ");
                expect_error(fixture, "validate", &code, &desc, schema_version)
            }
        }
    }
}

fn main() {
    let force = std::env::args().any(|a| a == "--force");
    let dir = fixture_dir();
    let mut fixtures: Vec<(String, String)> = Vec::new();
    let mut guarded: Vec<String> = Vec::new();
    let mut skipped: u32 = 0;
    let mut written: u32 = 0;

    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".cif") {
            continue;
        }
        fixtures.push((name, fs::read_to_string(entry.path()).unwrap()));
    }
    fixtures.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, content) in &fixtures {
        let stem = name.trim_end_matches(".cif").to_string();
        let expected = expected_path(&stem);
        let fresh = regenerate(content, name);

        let has_known_gap = expected
            .is_file()
            .then(|| fs::read_to_string(&expected).unwrap())
            .map(|old| {
                old.parse::<Json>()
                    .map(|j| j.get("known_gap") == Some(&Json::Bool(true)))
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        if has_known_gap {
            guarded.push(name.clone());
            eprintln!("  guard: kept {name} (\"known_gap\": true file, never overwritten)");
            continue;
        }

        if expected.is_file() && !force {
            skipped += 1;
            continue;
        }

        // Preserve a human-written description when the old file had one
        // and the regenerated body keeps the expect_error shape.
        let body = if let Ok(old_text) = fs::read_to_string(&expected) {
            let old: Json = old_text.parse().unwrap();
            let old_desc = old
                .get("expect_error")
                .and_then(|e| e.get("description"))
                .and_then(|d| d.as_str())
                .filter(|_| fresh.get("expect_error").is_some());
            if let Some(desc) = old_desc {
                let mut b = fresh.clone();
                b["expect_error"]["description"] = Json::String(desc.to_string());
                b
            } else {
                fresh
            }
        } else {
            fresh
        };

        fs::write(
            &expected,
            serde_json::to_string_pretty(&body).unwrap() + "\n",
        )
        .unwrap();
        written += 1;
        println!("  wrote {name}");
    }

    println!("regenerated {written} file(s), skipped {skipped} existing, guarded {guarded:?}");
    if !guarded.is_empty() {
        eprintln!("note: guarded files keep their spec-corrected expectations.");
    }
}
