//! Per-category coverage report (roadmap section 7 gate).
//!
//! Prints the per-category table of implemented `CifTerm` tags
//! (the central registry in `src/data_dict/tags.rs`) against the
//! recorded dictionary inventory (`tests/dict_inventory.json`),
//! plus the totals. The same matching logic lives in
//! `tests/dict_inventory.rs`.
//!
//! Usage: `cargo run --example dict_report`.

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::Value;

use crystal_cif_io::data_dict::tags;

fn main() {
    let inventory = load_inventory();
    let items = inventory["items"].as_array().expect("items array");
    let categories = inventory["categories"]
        .as_array()
        .expect("categories array");
    let cat_names: Vec<&str> = categories
        .iter()
        .map(|c| c["name"].as_str().expect("category name"))
        .collect();

    let by_name: HashMap<String, ()> = items
        .iter()
        .map(|i| {
            (
                i["name"].as_str().expect("item name").to_ascii_lowercase(),
                (),
            )
        })
        .collect();

    let mut implemented_in: HashMap<String, usize> = HashMap::new();
    for tag in tags::ALL {
        if let Some(cat) = longest_category(&cat_names, tag) {
            *implemented_in.entry(cat.to_string()).or_insert(0) += 1;
        }
    }

    let version = inventory["source"]["dictionary_version"]
        .as_str()
        .unwrap_or("unknown");
    println!("dictionary inventory: tests/dict_inventory.json (DDL1 {version})");
    println!("category coverage (implemented tags vs recorded inventory):");
    println!(
        "{:<32} {:>9} {:>11} {:>8}",
        "category", "inventory", "implemented", "missing"
    );
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
    let implemented = tags::ALL.len();
    let matched = tags::ALL
        .iter()
        .filter(|t| by_name.contains_key(&t.to_ascii_lowercase()))
        .count();
    let drifted: Vec<String> = tags::ALL
        .iter()
        .filter(|t| !by_name.contains_key(&t.to_ascii_lowercase()))
        .map(|t| t.to_string())
        .collect();
    println!(
        "{:<32} {:>9} {:>11} {:>8}",
        "total",
        total,
        implemented,
        total.saturating_sub(matched as u64)
    );
    println!(
        "coverage: {implemented} of {total} inventory items ({:.1}%)",
        100.0 * matched as f64 / total as f64
    );
    if !drifted.is_empty() {
        println!("registry tags absent from the recorded inventory (dictionary drift):");
        for tag in &drifted {
            println!("  {tag}");
        }
    }
}

/// Assign one tag to its inventory category. The category is the
/// longest recorded category name that is a whole-underscore prefix
/// of the tag (`cell_measurement_refln` beats `cell`,
/// `atom_sites` beats `atom_site`).
fn longest_category<'a>(categories: &'a [&'a str], tag: &str) -> Option<&'a str> {
    let key = tag.to_ascii_lowercase();
    let mut best: Option<&'a str> = None;
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
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("inventory JSON must parse")
}
