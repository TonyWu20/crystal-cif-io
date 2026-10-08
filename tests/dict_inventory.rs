//! Tag-to-inventory test (roadmap section 7).
//!
//! Every implemented `CifTerm` tag in the central registry
//! (`src/data_dict/tags.rs`) must exist in the recorded dictionary
//! inventory (`tests/dict_inventory.json`). An unknown tag fails the
//! test. The test prints the per-category coverage table.
//!
//! Tag names compare case-insensitively. The dictionary writes
//! conventional case (for example `cell_formula_units_Z`), while the
//! registry stores the crate tag spelling (`cell_formula_units_z`).
//! A typo still fails: the tag must match a recorded item name.
//!
//! Dictionary drift: the recorded inventory is DDL1
//! version 2.5.0-dev.1. Two tags the crate implements were removed
//! from the dictionary in that version. They stay registered for
//! legacy output and are listed in `KNOWN_DICTIONARY_DRIFT`. If an
//! item reappears in the inventory, drop it from that list.

use std::collections::HashMap;
use std::path::PathBuf;

use crystal_cif_io::data_dict::tags;
use serde_json::Value;

/// Implemented tags whose items the recorded dictionary no longer
/// defines. The DDL1 archive changelog records the removal of
/// `_space_group_id` and `_space_group_symop_sg_id`.
const KNOWN_DICTIONARY_DRIFT: &[&str] = &["space_group_id", "space_group_symop_sg_id"];

#[test]
fn registry_tags_exist_in_inventory() {
    let inventory = load_inventory();
    let items = inventory["items"].as_array().expect("items array");
    let categories = inventory["categories"].as_array().expect("categories array");
    assert!(!items.is_empty(), "empty inventory");
    assert!(!categories.is_empty(), "empty category list");

    let by_name: HashMap<String, &Value> = items
        .iter()
        .map(|i| {
            let name = i["name"].as_str().expect("item name").to_ascii_lowercase();
            (name, i)
        })
        .collect();

    let mut missing: Vec<String> = Vec::new();
    for tag in tags::ALL {
        let key = tag.to_ascii_lowercase();
        if !by_name.contains_key(&key) {
            missing.push(tag.to_string());
        }
    }

    // Every missing tag must be documented dictionary drift.
    let unexpected: Vec<&String> = missing
        .iter()
        .filter(|m| !KNOWN_DICTIONARY_DRIFT.contains(&m.as_str()))
        .collect();
    assert!(
        unexpected.is_empty(),
        "registry tags absent from the inventory: {unexpected:?}"
    );

    // Each drift entry must still be absent. If one reappears, drop
    // it from KNOWN_DICTIONARY_DRIFT.
    for drift in KNOWN_DICTIONARY_DRIFT {
        let key = drift.to_ascii_lowercase();
        assert!(
            !by_name.contains_key(&key),
            "{drift} is back in the inventory; remove it from KNOWN_DICTIONARY_DRIFT"
        );
    }

    print_coverage(categories, &by_name);
}

/// Print the per-category coverage table: recorded inventory items
/// versus implemented tags, plus the totals.
fn print_coverage(categories: &[Value], by_name: &HashMap<String, &Value>) {
    let cat_names: Vec<&str> =
        categories.iter().map(|c| c["name"].as_str().expect("category name")).collect();
    let mut implemented_in: HashMap<String, usize> = HashMap::new();
    let mut implemented = 0usize;
    for tag in tags::ALL {
        implemented += 1;
        let cat = longest_category(&cat_names, tag);
        if let Some(cat) = cat {
            *implemented_in.entry(cat.to_string()).or_insert(0) += 1;
        }
    }
    println!("category coverage (implemented tags vs recorded inventory):");
    println!("{:<32} {:>9} {:>11} {:>8}", "category", "inventory", "implemented", "missing");
    for c in categories {
        let name = c["name"].as_str().unwrap();
        let total = c["item_count"].as_u64().expect("item_count");
        let done = implemented_in.get(name).copied().unwrap_or(0);
        let missing = total.saturating_sub(done as u64);
        println!("{:<32} {:>9} {:>11} {:>8}", name, total, done, missing);
    }
    let total: u64 = categories
        .iter()
        .map(|c| c["item_count"].as_u64().expect("item_count"))
        .sum();
    let matched = tags::ALL
        .iter()
        .filter(|t| by_name.contains_key(&t.to_ascii_lowercase()))
        .count();
    println!(
        "{:<32} {:>9} {:>11} {:>8}",
        "total",
        total,
        implemented,
        total.saturating_sub(matched as u64)
    );
    println!(
        "registry implemented tags: {implemented}; matched in inventory: {matched}; \
         dictionary drift: {}",
        KNOWN_DICTIONARY_DRIFT.len()
    );
}

/// Assign one tag to its inventory category. The category is the
/// longest recorded category name that is a whole-underscore prefix
/// of the tag (`cell_measurement_refln` beats `cell`,
/// `atom_sites` beats `atom_site`).
fn longest_category<'a>(categories: &'a [&'a str], tag: &str) -> Option<&'a str> {
    let key = tag.to_ascii_lowercase();
    let mut best: Option<&str> = None;
    for cat in categories {
        let c = cat.to_ascii_lowercase();
        let is_prefix = key == c || key.starts_with(&format!("{c}_"));
        if is_prefix {
            match best {
                Some(b) if b.len() >= c.len() => {}
                _ => best = Some(cat),
            }
        }
    }
    best
}

fn load_inventory() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/dict_inventory.json");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("inventory JSON must parse")
}
