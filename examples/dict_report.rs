//! Per-category coverage report (roadmap section 7 gate).
//!
//! Prints the per-category table of implemented `CifTerm` tags
//! (the central registry in `src/data_dict/tags.rs`) against the
//! recorded dictionary inventory (`tests/dict_inventory.json`),
//! plus the totals. The same matching logic lives in
//! `tests/dict_inventory.rs`.
//!
//! The `--shell` flag adds the T6 view: the coverage table of the
//! recorded shell inventory (`tests/dict_inventory_shell.json`),
//! split per category into the items the standard inventory lacks
//! (the T6 items), the `_su` items skipped per decision D2, and
//! the implemented T6 items.
//!
//! Usage: `cargo run --example dict_report [--shell]`.

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::Value;

use crystal_cif_io::data_dict::tags;

fn main() {
    let shell = std::env::args().any(|a| a == "--shell");
    let inventory = load_inventory();
    let items = inventory["items"].as_array().expect("items array");
    let categories = inventory["categories"]
        .as_array()
        .expect("categories array");
    let cat_names: Vec<&str> = categories
        .iter()
        .map(|c| c["name"].as_str().expect("category name"))
        .collect();

    // The recorded category of each item (lowercased name to category).
    let by_name: HashMap<String, &str> = items
        .iter()
        .map(|i| {
            (
                i["name"].as_str().expect("item name").to_ascii_lowercase(),
                i["category"].as_str().expect("item category"),
            )
        })
        .collect();

    let mut implemented_in: HashMap<String, usize> = HashMap::new();
    for tag in tags::ALL {
        // The recorded category is authoritative. Some items carry a
        // different category name prefix in their name (for example the
        // `diffrn_radiation_detector` item of the `diffrn_detector`
        // category), so the name-prefix heuristic would misassign them.
        // Prefer the recorded category, fall back to the prefix rule.
        let key = tag.to_ascii_lowercase();
        let cat = by_name
            .get(&key)
            .map(|c| c.to_string())
            .or_else(|| longest_category(&cat_names, tag).map(|c| c.to_string()));
        if let Some(cat) = cat {
            *implemented_in.entry(cat).or_insert(0) += 1;
        }
    }

    // The shell inventory names the T6 items. A tag recorded there is
    // not drift for the standard view.
    let shell_names = load_shell_inventory_if_present()
        .map(|shell| {
            shell["items"]
                .as_array()
                .expect("shell items array")
                .iter()
                .map(|i| i["name"].as_str().expect("item name").to_ascii_lowercase())
                .collect::<std::collections::HashSet<_>>()
        })
        .unwrap_or_default();

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
        .filter(|t| {
            !by_name.contains_key(&t.to_ascii_lowercase())
                && !shell_names.contains(&t.to_ascii_lowercase())
        })
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
    if shell {
        shell_view();
    }
}

/// The T6 view: the per-category table of the shell inventory
/// against the registry. The table holds only the categories that
/// carry T6 items (shell items the standard inventory lacks).
fn shell_view() {
    let shell = load_shell_inventory();
    let items = shell["items"].as_array().expect("shell items array");
    let categories = shell["categories"]
        .as_array()
        .expect("shell categories array");
    let standard = load_inventory();
    let std_names: std::collections::HashSet<String> = standard["items"]
        .as_array()
        .expect("items array")
        .iter()
        .map(|i| i["name"].as_str().expect("item name").to_ascii_lowercase())
        .collect();
    let registry: std::collections::HashSet<String> =
        tags::ALL.iter().map(|t| t.to_ascii_lowercase()).collect();

    // Per shell category: the T6 split.
    #[derive(Default)]
    struct Split {
        total: usize,
        su: usize,
        done: usize,
        missing: Vec<String>,
    }
    let mut splits: HashMap<String, Split> = HashMap::new();
    for item in items {
        let name = item["name"]
            .as_str()
            .expect("item name")
            .to_ascii_lowercase();
        if std_names.contains(&name) {
            continue; // not a T6 item
        }
        let cat = item["category"]
            .as_str()
            .expect("item category")
            .to_string();
        let split = splits.entry(cat).or_default();
        split.total += 1;
        if name.ends_with("_su") {
            split.su += 1; // decision D2: the _su items are skipped
            continue;
        }
        if registry.contains(&name) {
            split.done += 1;
        } else {
            split.missing.push(name);
        }
    }

    let version = shell["source"]["dictionary_version"]
        .as_str()
        .unwrap_or("unknown");
    println!("shell inventory: tests/dict_inventory_shell.json ({version})");
    println!(
        "{:<32} {:>9} {:>10} {:>11} {:>8}",
        "category", "t6 items", "su skipped", "implemented", "missing"
    );
    let mut cat_lines: Vec<&Value> = Vec::new();
    for c in categories {
        let name = c["name"].as_str().unwrap().to_string();
        if splits.get(&name).is_some_and(|s| s.total > 0) {
            cat_lines.push(c);
        }
    }
    let mut total_all = 0usize;
    let mut total_su = 0usize;
    let mut total_done = 0usize;
    let mut total_missing = 0usize;
    for c in &cat_lines {
        let name = c["name"].as_str().unwrap();
        let split = &splits[name];
        total_all += split.total;
        total_su += split.su;
        total_done += split.done;
        total_missing += split.missing.len();
        println!(
            "{:<32} {:>9} {:>10} {:>11} {:>8}",
            name,
            split.total,
            split.su,
            split.done,
            split.missing.len()
        );
    }
    println!(
        "{:<32} {:>9} {:>10} {:>11} {:>8}",
        "total", total_all, total_su, total_done, total_missing
    );
    if total_missing == 0 {
        println!(
            "T6 gate: pass (all {} T6 items implemented; {} _su items skipped per D2)",
            total_done, total_su
        );
    } else {
        println!("T6 gate: fail ({} T6 items not implemented)", total_missing);
        for c in &cat_lines {
            let name = c["name"].as_str().unwrap();
            for tag in &splits[name].missing {
                println!("  missing {name}: {tag}");
            }
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

fn load_shell_inventory() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/dict_inventory_shell.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("shell inventory JSON must parse")
}

/// The shell inventory, when the file exists. The standard view is
/// the DDL1 inventory alone; the shell file is optional input.
fn load_shell_inventory_if_present() -> Option<Value> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/dict_inventory_shell.json");
    let raw = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&raw).ok()
}
