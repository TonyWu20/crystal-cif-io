//! CIF 1.1 groundtruth harness.
//!
//! Layout:
//! - `tests/fixtures/*.cif`            reference inputs
//! - `tests/fixtures/expected/*.json`  expected canonical IR
//!
//! The expected JSON for each fixture is either:
//! - positive: `{"schema_version": 1, "fixture": "...", "blocks": [...]}`
//! - negative: `{"schema_version": 1, "fixture": "...",
//!    "expect_error": {"stage": "parse"|"validate",
//!                     "violation": "...", "description": "..."}}`
//!
//! A positive fixture may carry `"known_gap": true`: the expected JSON
//! is spec-correct, but the parser is known to disagree with it. The
//! verify test reports such fixtures as gaps instead of failing; flip
//! the flag off once the parser catches up.
//!
//! Commands:
//! - `cargo test ground_truth_verify`                 check parser against groundtruth
//! - `cargo run --example gt_generate`                 write missing expected files
//! - `cargo run --example gt_generate -- --force`      overwrite existing expected
//!                                                      files (guarded: files with
//!                                                      "known_gap": true are kept;
//!                                                      review git diff before commit)

use crystal_cif_io::canonical;

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value as Json};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn expected_path(fixture: &std::path::Path) -> PathBuf {
    let stem = fixture
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    fixtures_dir().join("expected").join(format!("{stem}.json"))
}

fn all_fixtures() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = fs::read_dir(fixtures_dir())
        .expect("read fixture directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.is_file() && p.extension().is_some_and(|ext| ext == "cif"))
        .collect();
    out.sort();
    out
}

/// Parse with the chumsky entry point. `Ok` means the whole input was
/// consumed (the top level is strict).
fn parse_cif(content: &str) -> Result<crystal_cif_io::CifDocument, String> {
    match crystal_cif_io::parse_cif(content) {
        Ok(doc) => Ok(doc),
        Err(errors) => Err(format!("{errors:?}")),
    }
}

#[test]
fn ground_truth_verify() {
    let mut failures: Vec<String> = Vec::new();
    let mut gaps: Vec<String> = Vec::new();
    for fixture in all_fixtures() {
        let name = fixture.file_name().unwrap().to_string_lossy().to_string();
        let content = fs::read_to_string(&fixture).expect("read fixture");
        let expected_file = expected_path(&fixture);
        let raw = fs::read_to_string(&expected_file)
            .unwrap_or_else(|e| panic!("missing expected file {expected_file:?}: {e}"));
        let expected: Json =
            serde_json::from_str(&raw).expect("expected JSON must be valid: {expected_file:?}");

        if expected.get("expect_error").is_some() {
            check_negative(&name, &content, &expected, &mut failures);
        } else {
            check_positive(&name, &content, &expected, &mut failures, &mut gaps);
        }
    }
    for gap in &gaps {
        println!("known gap (expected): {gap}");
    }
    assert!(
        failures.is_empty(),
        "groundtruth failures:\n{}",
        failures.join("\n")
    );
}

fn check_positive(
    name: &str,
    content: &str,
    expected: &Json,
    failures: &mut Vec<String>,
    gaps: &mut Vec<String>,
) {
    let known_gap = expected.get("known_gap") == Some(&json!(true));
    let doc = match parse_cif(content) {
        Err(err) => {
            if known_gap {
                gaps.push(format!("{name}: parser rejected the fixture ({err})"));
            } else {
                failures.push(format!("{name}: parser error: {err}"));
            }
            return;
        }
        Ok(doc) => doc,
    };
    let actual = canonical::document_json(&doc, name);
    // Compare the `blocks` tree only: `known_gap` / `known_gap_note` are
    // bookkeeping fields, not part of the canonical IR.
    let blocks_equal = canonical::trees_equal(&expected["blocks"], &actual["blocks"]);
    if blocks_equal {
        if known_gap {
            gaps.push(format!(
                "{name}: parser output now matches the spec-correct JSON; \
                 remove the stale known_gap flag"
            ));
        }
        return;
    }
    if known_gap {
        let stem = name.trim_end_matches(".cif");
        gaps.push(format!(
            "{name}: parser output diverges from the spec-correct expected JSON \
             (see tests/fixtures/expected/{stem}.json)"
        ));
    } else {
        failures.push(format!(
            "{name}: canonical mismatch.\nexpected blocks: {}\nactual blocks:   {}",
            serde_json::to_string_pretty(&expected["blocks"]).unwrap_or_default(),
            serde_json::to_string_pretty(&actual["blocks"]).unwrap_or_default()
        ));
    }
}

fn check_negative(name: &str, content: &str, expected: &Json, failures: &mut Vec<String>) {
    let expect = &expected["expect_error"];
    let stage = expect["stage"].as_str().expect("expect_error.stage");
    let violation = expect["violation"]
        .as_str()
        .expect("expect_error.violation");
    match parse_cif(content) {
        Err(msg) => {
            // A hard parser rejection satisfies both stages.
            if stage == "validate" {
                failures.push(format!(
                    "{name}: expected a parse success with a {violation} validation error, \
                     but the parser hard-failed: {msg}"
                ));
            }
        }
        Ok(doc) => {
            if stage == "parse" {
                failures.push(format!(
                    "{name}: expected the parser to reject the file, \
                     but it parsed"
                ));
                return;
            }
            let violations = canonical::validate(&doc);
            let codes: Vec<&str> = violations.iter().map(|v| v.code()).collect();
            if !codes.iter().any(|c| *c == violation) {
                let detail: Vec<String> = violations.iter().map(|v| v.to_string()).collect();
                failures.push(format!(
                    "{name}: expected violation {violation}, found {codes:?}\n{detail:#?}"
                ));
            }
        }
    }
}
