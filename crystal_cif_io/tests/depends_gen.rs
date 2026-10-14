//! Generator check for `codegen/depends.json` (roadmap D10a).
//!
//! The committed `codegen/depends.json` is the output of
//! `codegen/extract_depends.py` over the two pinned dictionaries.
//! This test reruns the script into a temporary file and fails when
//! the output differs from the committed file. It also checks that
//! the committed file covers every item of both recorded
//! inventories: the DDL1 inventory names must all be present, and
//! every shell inventory name must be present or listed in
//! `naming_exceptions` (shell frames that record a DDL1 item under
//! a different shell spelling).

use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The workspace root. The `codegen/` artifacts and the pinned
/// dictionaries live there, one level above the crate.
fn repo_root() -> PathBuf {
    manifest().parent().expect("workspace root").to_path_buf()
}

fn load(path: &std::path::Path) -> Value {
    let raw =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

fn inventory_names(which: &str) -> HashSet<String> {
    let path = manifest().join(if which == "shell" {
        "tests/dict_inventory_shell.json"
    } else {
        "tests/dict_inventory.json"
    });
    load(&path)["items"]
        .as_array()
        .cloned()
        .expect("items array")
        .into_iter()
        .map(|i| i["name"].as_str().expect("item name").to_ascii_lowercase())
        .collect()
}

/// First byte index where the two texts differ (length of the
/// common prefix when one is a prefix of the other).
fn first_divergence(a: &str, b: &str) -> usize {
    a.bytes()
        .zip(b.bytes())
        .position(|(x, y)| x != y)
        .unwrap_or_else(|| a.len().max(b.len()))
}

/// The line holding byte `at`, padded so the failure text is readable.
fn context_span(text: &str, at: usize) -> String {
    let at = at.min(text.len().saturating_sub(1).max(1));
    let lo = text[..at]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0)
        .min(at.saturating_sub(60));
    let hi = text[at..]
        .find('\n')
        .map(|i| at + i + 1)
        .unwrap_or(text.len())
        .max(at + 60)
        .min(text.len());
    text[lo..hi].to_string()
}

fn depends_names() -> (HashSet<String>, HashSet<String>) {
    let path = repo_root().join("codegen/depends.json");
    let doc = load(&path);
    let mut names = HashSet::new();
    let mut exceptions = HashSet::new();
    for cat in doc["categories"].as_array().expect("categories array") {
        for item in cat["items"].as_array().expect("items array") {
            names.insert(
                item["name"]
                    .as_str()
                    .expect("item name")
                    .to_ascii_lowercase(),
            );
        }
    }
    for exc in doc["naming_exceptions"]
        .as_array()
        .expect("exceptions array")
    {
        exceptions.insert(
            exc["inventory_name"]
                .as_str()
                .expect("inventory_name")
                .to_ascii_lowercase(),
        );
    }
    (names, exceptions)
}

#[test]
fn rerun_produces_the_committed_file() {
    let script = repo_root().join("codegen/extract_depends.py");
    let tmp = repo_root().join("target/depends_rerun.json");
    let status = Command::new("python3")
        .args(["codegen/extract_depends.py", "--out"])
        .arg(&tmp)
        .current_dir(&repo_root())
        .status()
        .unwrap_or_else(|e| panic!("run python3 {script:?}: {e}"));
    assert!(status.success(), "extract_depends.py failed: {status}");
    let fresh = std::fs::read_to_string(&tmp).expect("read rerun output");
    let committed =
        std::fs::read_to_string(repo_root().join("codegen/depends.json")).expect("read committed");
    if fresh != committed {
        let at = first_divergence(&committed, &fresh);
        panic!(
            "codegen/depends.json is stale. First difference at byte {at}.\n\
             committed: {}\nregenerated: {}\n\
             Regenerate with: python3 codegen/extract_depends.py",
            context_span(&committed, at),
            context_span(&fresh, at)
        );
    }
    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn the_committed_file_covers_both_inventories() {
    let (names, exceptions) = depends_names();
    let ddl1 = inventory_names("ddl1");
    let missing: Vec<String> = ddl1
        .iter()
        .filter(|n| !names.contains(*n))
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "DDL1 inventory items missing from depends.json: {missing:?}"
    );

    let shell = inventory_names("shell");
    let covered = |n: &str| names.contains(n) || exceptions.contains(n);
    let missing: Vec<String> = shell.iter().filter(|n| !covered(*n)).cloned().collect();
    assert!(
        missing.is_empty(),
        "shell inventory items in neither depends.json nor \
         naming_exceptions: {missing:?}"
    );
    // Every exception must point at an item that is in the file.
    let doc = load(&repo_root().join("codegen/depends.json"));
    for exc in doc["naming_exceptions"]
        .as_array()
        .expect("exceptions array")
    {
        let item = exc["item"].as_str().expect("item").to_ascii_lowercase();
        assert!(
            names.contains(&item),
            "naming exception points at missing item {item:?}"
        );
    }
}

#[test]
fn the_counts_are_consistent() {
    let doc = load(&repo_root().join("codegen/depends.json"));
    let counts = &doc["counts"];
    let item_total = doc["categories"]
        .as_array()
        .expect("categories array")
        .iter()
        .map(|c| c["items"].as_array().expect("items array").len())
        .sum::<usize>();
    assert_eq!(
        counts["items"].as_u64().expect("items"),
        item_total as u64,
        "the recorded item total moved"
    );
    assert_eq!(
        counts["categories"].as_u64().expect("categories"),
        doc["categories"]
            .as_array()
            .expect("categories array")
            .len() as u64,
        "the recorded category count moved"
    );
}
