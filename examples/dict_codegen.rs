//! The dictionary codegen step (roadmap decision D4, `docs/codegen.md`).
//!
//! Reads:
//! - `tests/dict_inventory.json` (the recorded item list, per category)
//! - `codegen/annotations.json` (the per-item annotations)
//!
//! Writes (overwritten on every run):
//! - `src/data_dict/core_cif/<category>/mod.rs`
//! - `src/data_dict/core_cif/<category>/<type>.rs`
//!   (one file per closed code set and per range check)
//!
//! Updates (append-only: existing entries are never removed or
//! reordered):
//! - `src/data_dict/core_cif/mod.rs` (the `pub mod` lines)
//! - `src/data_dict/tags.rs` (the TAGS sections, `ALL`, the tests)
//!
//! The output is deterministic: categories and items sort
//! alphabetically, so a rerun over unchanged inputs produces a clean
//! diff. The emitted code follows the conventions of
//! `src/data_dict/core_cif/cell/mod.rs`.
//!
//! Usage: `cargo run --example dict_codegen`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, ExitCode};

use serde_json::Value;

/// Rust's line width, matching `cargo fmt` (`max_width = 100`).
const MAX_WIDTH: usize = 100;

/// Rust keywords that may not be used as a generated variant name.
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

/// Names a generated type must not shadow in a category module.
const RESERVED_TYPE_NAMES: &[&str] = &[
    "CharString",
    "CifTerm",
    "DataItems",
    "Display",
    "Float",
    "Integer",
    "LoopColumn",
    "LoopValueTerm",
    "Matrix",
    "Numeric",
    "Number",
    "SingleLineData",
    "SingleValueTerm",
    "Tag",
    "TextField",
    "UnsignedInteger",
    "Value",
];

fn main() -> ExitCode {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let inventory = load_json(&manifest.join("tests/dict_inventory.json"));
    // The T6 dual-inventory step: when the shell inventory is
    // recorded, the shell items join the per-category item set.
    let shell_path = manifest.join("tests/dict_inventory_shell.json");
    let shell = shell_path.is_file().then(|| load_json(&shell_path));
    let annotations = load_json(&manifest.join("codegen/annotations.json"));
    // The legacy drift tags: registry strings for items the
    // dictionary no longer defines. Kept per category.
    let drift = annotations["drift"].clone();

    let errors = validate(&inventory, shell.as_ref(), &annotations);
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("dict_codegen error: {e}");
        }
        eprintln!("fix the annotations (codegen/annotations.json) and rerun");
        return ExitCode::FAILURE;
    }

    let warnings = warn(&inventory, shell.as_ref(), &annotations);
    for w in &warnings {
        println!("dict_codegen warning: {w}");
    }

    let report = emit(&manifest, &inventory, shell.as_ref(), &annotations, &drift);
    for line in &report {
        println!("{line}");
    }
    ExitCode::SUCCESS
}

fn load_json(path: &std::path::Path) -> Value {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

/// One inventory item, with the crate-spelling tag derived.
#[derive(Clone)]
struct Item {
    /// The raw inventory record (the `category`, `type`, `is_loop`,
    /// and `enumeration` fields).
    item: Value,
    /// The dictionary spelling, without the leading underscore.
    name: String,
    /// The crate spelling: the name, lowercased.
    tag: String,
    /// The name minus the category prefix.
    suffix: String,
    /// True when the name carries the `<category>_` prefix. When
    /// false, the item name equals the category name, so its tag is
    /// the name itself rather than `<category>_<suffix>`.
    prefixed: bool,
    is_loop: bool,
}

/// One annotation entry, resolved and validated.
#[derive(Clone)]
struct Plan {
    item: Item,
    /// The annotation value type: `string`, `numeric`, `closed`,
    /// or `range`.
    kind: String,
    /// The closed codes, in annotation order.
    values: Vec<String>,
    min: Option<f64>,
    max: Option<f64>,
    variant: String,
    /// The generated type name, for `closed` and `range`.
    type_name: Option<String>,
    /// The annotation note, emitted as a doc comment.
    note: Option<String>,
}

/// One generated type file.
struct TypeFile {
    file: String,
    name: String,
    kind: String,
}

/// The per-category plan: items split by single/loop form.
struct CatPlan {
    cat: String,
    single: Vec<Plan>,
    loop_items: Vec<Plan>,
    types: Vec<TypeFile>,
}

/// Validate the annotations against the inventory. Every problem is an
/// error: the annotations must drive the output, and a mismatch with
/// the recorded inventory is drift.
fn validate(inventory: &Value, shell: Option<&Value>, annotations: &Value) -> Vec<String> {
    let mut errors = Vec::new();
    let cats = ann_categories(annotations, &mut errors);
    let standard = inv_items(inventory, &mut errors);
    let items: Vec<Item> = if shell.is_some() {
        union_items(shell, &standard, &mut errors)
    } else {
        standard
    };

    for (cat, entries) in &cats {
        if !cats_has_inventory_category(inventory, cat, shell, &mut errors) {
            continue;
        }
        let in_cat: BTreeSet<&str> = items
            .iter()
            .filter(|i| i.item["category"].as_str() == Some(cat))
            .map(|i| i.suffix.as_str())
            .collect();
        let have: BTreeSet<&str> = entries.keys().map(|s| s.as_str()).collect();
        for missing in in_cat.difference(&have) {
            errors.push(format!(
                "category `{cat}`: inventory item `{cat}_{missing}` has no annotation"
            ));
        }
        for extra in have.difference(&in_cat) {
            errors.push(format!(
                "category `{cat}`: annotation `{cat}_{extra}` matches no inventory item"
            ));
        }

        for (suffix, entry) in entries {
            let found = items
                .iter()
                .find(|i| i.suffix == *suffix && i.item["category"].as_str() == Some(cat));
            let Some(item) = found else {
                continue; // already reported above
            };
            let Some(kind) = entry.get("type").and_then(|v| v.as_str()) else {
                errors.push(format!("annotation `{cat}_{suffix}`: missing `type`"));
                continue;
            };
            match kind {
                "string" | "numeric" | "closed" | "range" | "matrix" => {}
                other => {
                    errors.push(format!(
                        "annotation `{cat}_{suffix}`: unknown value type `{other}`"
                    ));
                    continue;
                }
            }
            match (kind, item.item["type"].as_str().unwrap()) {
                ("string", "char") | ("closed", "char") => {}
                ("numeric", "numb") | ("range", "numb") => {}
                // The matrix payload binds the matrix container
                // items, whose contents the dictionary records as
                // numbers.
                ("matrix", "numb") => {}
                (k, t) => {
                    errors.push(format!(
                        "annotation `{cat}_{suffix}`: value type `{k}` does not match \
                         the inventory item type `{t}`"
                    ));
                }
            }
            if kind == "closed" {
                let codes: Vec<&str> = entry["values"]
                    .as_array()
                    .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();
                let set: BTreeSet<&str> = codes.iter().copied().collect();
                if codes.is_empty() {
                    errors.push(format!(
                        "annotation `{cat}_{suffix}`: a closed set needs a `values` list"
                    ));
                }
                if set.len() != codes.len() {
                    errors.push(format!(
                        "annotation `{cat}_{suffix}`: duplicate codes in `values`"
                    ));
                }
                if let Some(inv_enum) = item.item["enumeration"].as_object() {
                    if inv_enum["kind"].as_str() == Some("closed") {
                        let inv_codes: BTreeSet<&str> = inv_enum["values"]
                            .as_array()
                            .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
                            .unwrap_or_default();
                        if inv_codes != set {
                            errors.push(format!(
                                "annotation `{cat}_{suffix}`: `values` {set:?} differs from \
                                 the inventory enumeration {inv_codes:?}"
                            ));
                        }
                    }
                }
            }
            if kind == "range" {
                let min = entry["min"].as_f64();
                let max = entry["max"].as_f64();
                match (min, max) {
                    (Some(lo), Some(hi)) if lo < hi => {}
                    (Some(_), None) | (None, Some(_)) => {}
                    _ => {
                        errors.push(format!(
                            "annotation `{cat}_{suffix}`: a range needs a `min` and/or `max` bound (with `min` < `max` when both are set)"
                        ));
                    }
                }
            }
        }
    }
    errors
}

fn cats_has_inventory_category(
    inventory: &Value,
    cat: &str,
    shell: Option<&Value>,
    errors: &mut Vec<String>,
) -> bool {
    let mut ok = inventory["categories"]
        .as_array()
        .map(|a| a.iter().any(|c| c["name"].as_str() == Some(cat)))
        .unwrap_or(false);
    if !ok {
        // The T6 step: a shell-only category is spelled lowercase
        // in the annotations, matching the registry tag case.
        ok = shell
            .map(|v| {
                v["categories"]
                    .as_array()
                    .map(|a| {
                        a.iter().any(|c| {
                            c["name"]
                                .as_str()
                                .map(|n| n.to_ascii_lowercase() == cat)
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(false);
    }
    if !ok {
        errors.push(format!(
            "annotation category `{cat}`: not in the inventory category list"
        ));
    }
    ok
}

/// Drift warnings: the annotations diverge from what the dictionary
/// records. They never block the run.
fn warn(inventory: &Value, shell: Option<&Value>, annotations: &Value) -> Vec<String> {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();
    let cats = ann_categories(annotations, &mut errors);
    let standard = inv_items(inventory, &mut errors);
    let items: Vec<Item> = if shell.is_some() {
        union_items(shell, &standard, &mut errors)
    } else {
        standard
    };
    if !errors.is_empty() {
        return warnings;
    }

    for (cat, entries) in &cats {
        for (suffix, entry) in entries {
            let found = items
                .iter()
                .find(|i| i.suffix == *suffix && i.item["category"].as_str() == Some(cat));
            let Some(item) = found else { continue };
            let kind = entry["type"].as_str().unwrap_or("");
            if let Some(inv_enum) = item.item["enumeration"].as_object() {
                if inv_enum["kind"].as_str() == Some("closed")
                    && (kind == "string" || kind == "numeric")
                {
                    warnings.push(format!(
                        "item `{cat}_{suffix}`: the dictionary records a closed code set, \
                         but the annotation uses a plain `{kind}` value. Record the opt-out \
                         in the annotation `note`."
                    ));
                }
                if inv_enum["kind"].as_str() == Some("range") && kind != "range" {
                    let range = inv_enum["range"].as_str().unwrap_or("?");
                    warnings.push(format!(
                        "item `{cat}_{suffix}`: the dictionary records a range constraint \
                         `{range}`, but the annotation uses a plain `{kind}` value"
                    ));
                }
            }
            if kind == "closed" && item.item["enumeration"].as_object().is_none() {
                warnings.push(format!(
                    "item `{cat}_{suffix}`: the annotation adds a closed code set the \
                     inventory does not record (dictionary drift)"
                ));
            }
            // The T6 matrix payload: a plain annotation on an item
            // the shell records with a matrix shape is a drift note.
            if !item.item["matrix_shape"].is_null() && (kind == "string" || kind == "numeric") {
                let shape = item.item["matrix_shape"].as_str().unwrap_or("?");
                warnings.push(format!(
                    "item `{cat}_{suffix}`: the shell dictionary records the matrix \
                     shape `{shape}`, but the annotation uses a plain `{kind}` value"
                ));
            }
        }
    }
    warnings
}

/// Plan every category, then emit the modules and the registry
/// updates. Nothing is written when a plan has an error.
fn emit(
    manifest: &Path,
    inventory: &Value,
    shell: Option<&Value>,
    annotations: &Value,
    drift: &Value,
) -> Vec<String> {
    let mut errors = Vec::new();
    let cats = ann_categories(annotations, &mut errors);
    let standard = inv_items(inventory, &mut errors);
    let items: Vec<Item> = if shell.is_some() {
        union_items(shell, &standard, &mut errors)
    } else {
        standard
    };
    if !errors.is_empty() {
        fail(&errors);
    }

    let mut plans: Vec<CatPlan> = Vec::new();
    for (cat, entries) in &cats {
        let plan = plan_category(cat, entries, &items, &mut errors);
        plans.push(plan);
    }
    if !errors.is_empty() {
        fail(&errors);
    }

    let mut report = Vec::new();
    for plan in &plans {
        write_category(manifest, plan, &mut report);
    }

    let core_cif_mod = manifest.join("src/data_dict/core_cif/mod.rs");
    let ann_cats: Vec<String> = plans.iter().map(|p| p.cat.clone()).collect();
    update_core_cif_mod(&core_cif_mod, &ann_cats, &mut report);
    let tags = manifest.join("src/data_dict/tags.rs");
    update_tags_registry(&tags, &cats, &items, &drift, &mut report);
    report
}

fn fail(errors: &[String]) -> ! {
    for e in errors {
        eprintln!("dict_codegen error: {e}");
    }
    process::abort();
}

/// Resolve one category's plans (items, variants, type names).
fn plan_category(
    cat: &str,
    entries: &BTreeMap<String, Value>,
    items: &[Item],
    errors: &mut Vec<String>,
) -> CatPlan {
    let mut plans: Vec<Plan> = Vec::new();
    let mut type_names: BTreeSet<String> = BTreeSet::new();
    let mut types: Vec<TypeFile> = Vec::new();

    for (suffix, entry) in entries {
        let found = items
            .iter()
            .find(|i| i.suffix == *suffix && i.item["category"].as_str() == Some(cat));
        let item = match found.cloned() {
            Some(i) => i,
            None => {
                errors.push(format!(
                    "category `{cat}`: no inventory item `{cat}_{suffix}`"
                ));
                continue;
            }
        };
        let kind = entry["type"].as_str().unwrap().to_string();
        let variant = entry
            .get("variant")
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or_else(|| default_variant(&item.suffix));
        if RUST_KEYWORDS.contains(&variant.as_str()) {
            errors.push(format!(
                "item `{cat}_{suffix}`: variant name `{variant}` is a Rust keyword"
            ));
            continue;
        }
        let mut type_name = None;
        if kind == "closed" || kind == "range" {
            let name = entry
                .get("rust")
                .and_then(|v| v.as_str())
                .map(|n| n.to_string())
                .unwrap_or_else(|| default_type_name(&item.suffix));
            if RESERVED_TYPE_NAMES.contains(&name.as_str())
                || RUST_KEYWORDS.contains(&name.as_str())
            {
                errors.push(format!(
                    "item `{cat}_{suffix}`: type name `{name}` is reserved or a keyword"
                ));
                continue;
            }
            if !type_names.insert(name.clone()) {
                errors.push(format!(
                    "item `{cat}_{suffix}`: type name `{name}` collides with another item \
                     in category `{cat}`; set a distinct `rust` name"
                ));
                continue;
            }
            types.push(TypeFile {
                file: snake_case(&name),
                name: name.clone(),
                kind: kind.clone(),
            });
            type_name = Some(name);
        }
        plans.push(Plan {
            item,
            kind,
            values: entry["values"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .map(|v| v.as_str().unwrap_or_default().to_string())
                        .collect()
                })
                .unwrap_or_default(),
            min: entry["min"].as_f64(),
            max: entry["max"].as_f64(),
            variant,
            type_name,
            note: entry
                .get("note")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        });
    }

    plans.sort_by(|a, b| a.item.suffix.cmp(&b.item.suffix));
    types.sort_by(|a, b| a.file.cmp(&b.file));
    let single = plans.iter().filter(|p| !p.item.is_loop).cloned().collect();
    let loop_items = plans.iter().filter(|p| p.item.is_loop).cloned().collect();
    CatPlan {
        cat: cat.to_string(),
        single,
        loop_items,
        types,
    }
}

/// Write one category's module and type files.
fn write_category(manifest: &Path, plan: &CatPlan, report: &mut Vec<String>) {
    let dir = manifest.join("src/data_dict/core_cif").join(&plan.cat);
    fs::create_dir_all(&dir).expect("create category directory");

    for t in &plan.types {
        let path = dir.join(format!("{}.rs", t.file));
        let body = if t.kind == "closed" {
            emit_closed(plan, t)
        } else {
            emit_range(plan, t)
        };
        write_if_changed(&path, &body, report);
    }

    let mod_path = dir.join("mod.rs");
    let body = emit_mod(plan);
    write_if_changed(&mod_path, &body, report);
}

fn write_if_changed(path: &Path, body: &str, report: &mut Vec<String>) {
    let same = fs::read_to_string(path)
        .map(|old| old == body)
        .unwrap_or(false);
    if same {
        report.push(format!("unchanged {}", path.display()));
    } else {
        fs::write(path, body).expect("write generated file");
        report.push(format!("wrote {}", path.display()));
    }
}

/// Emit a closed code set type file.
fn emit_closed(plan: &CatPlan, t: &TypeFile) -> String {
    let plan_item = plan
        .single
        .iter()
        .chain(plan.loop_items.iter())
        .find(|p| p.type_name.as_deref() == Some(t.name.as_str()))
        .expect("type file matches an item");
    let codes = &plan_item.values;
    let item = &plan_item.item.name;

    let mut out = String::new();
    out.push_str("use std::fmt::Display;\n\n");
    out.push_str("use crate::grammar::{CharString, UnquotedString, Value};\n\n");
    let doc = format!(
        "/// The closed code set of `_{item}`. The codes are recorded in\n\
         /// the dictionary `_enumeration` (see `tests/dict_inventory.json`).\n"
    );
    out.push_str(&doc);
    if let Some(note) = &plan_item.note {
        out.push_str("///\n");
        for line in wrap(note, 72) {
            let l = format!("/// {line}");
            out.push_str(&l);
            out.push('\n');
        }
    }
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    out.push_str("#[allow(non_camel_case_types, clippy::upper_case_acronyms)]\n");
    let head = format!("pub enum {} {{\n", t.name);
    out.push_str(&head);
    for code in codes {
        let variant = code_variant(code);
        let line = format!("    /// `{code}`\n    {variant},\n");
        out.push_str(&line);
    }
    out.push_str("}\n\n");
    let head = format!("impl Display for {} {{\n", t.name);
    out.push_str(&head);
    out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    out.push_str("        let code = match self {\n");
    for code in codes {
        let variant = code_variant(code);
        let line = format!("            Self::{variant} => \"{code}\",\n");
        out.push_str(&line);
    }
    out.push_str("        };\n        f.write_str(code)\n    }\n}\n\n");
    let head = format!("impl From<{}> for UnquotedString {{\n", t.name);
    out.push_str(&head);
    let body = format!(
        "    fn from(value: {}) -> Self {{\n        UnquotedString::new(format!(\"{{value}}\"))\n    }}\n}}\n\n",
        t.name
    );
    out.push_str(&body);
    let head = format!("impl From<{}> for CharString {{\n", t.name);
    out.push_str(&head);
    let body = format!(
        "    fn from(value: {}) -> Self {{\n        CharString::Unquoted(UnquotedString::from(value))\n    }}\n}}\n\n",
        t.name
    );
    out.push_str(&body);
    let head = format!("impl From<{}> for Value {{\n", t.name);
    out.push_str(&head);
    let body = format!(
        "    fn from(value: {}) -> Self {{\n        Value::CharString(CharString::from(value))\n    }}\n}}\n",
        t.name
    );
    out.push_str(&body);
    out
}

/// Emit a range-check type file.
fn emit_range(plan: &CatPlan, t: &TypeFile) -> String {
    let plan_item = plan
        .single
        .iter()
        .chain(plan.loop_items.iter())
        .find(|p| p.type_name.as_deref() == Some(t.name.as_str()))
        .expect("type file matches an item");
    let min = plan_item.min;
    let max = plan_item.max;
    let item = &plan_item.item.name;
    let interval = match (min, max) {
        (Some(lo), Some(hi)) => format!("[{}, {}]", f64_bound(lo), f64_bound(hi)),
        (Some(lo), None) => format!("[{}, +infinity)", f64_bound(lo)),
        (None, Some(hi)) => format!("(-infinity, {}]", f64_bound(hi)),
        (None, None) => "[unbounded]".to_string(),
    };

    let mut out = String::new();
    out.push_str("use std::fmt::Display;\n\n");
    out.push_str("use crate::grammar::{Float, Number, Numeric, Value};\n\n");
    let doc = format!(
        "/// The write-time range check for `_{item}`. `new` clamps the value\n\
         /// to {interval}, matching the dictionary `_enumeration`.\n"
    );
    out.push_str(&doc);
    if let Some(note) = &plan_item.note {
        out.push_str("///\n");
        for line in wrap(note, 72) {
            let l = format!("/// {line}");
            out.push_str(&l);
            out.push('\n');
        }
    }
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]\n");
    let head = format!("pub struct {}(Float);\n\n", t.name);
    out.push_str(&head);
    let head = format!("impl {} {{\n", t.name);
    out.push_str(&head);
    out.push_str("    pub fn new(float: Float) -> Self {\n");
    let clamp = match (min, max) {
        (Some(lo), Some(hi)) => {
            format!("float.clamp({}, {})", f64_bound(lo), f64_bound(hi))
        }
        (Some(lo), None) => format!("float.max({})", f64_bound(lo)),
        (None, Some(hi)) => format!("float.min({})", f64_bound(hi)),
        (None, None) => "float".to_string(),
    };
    let line = format!("        Self(Float({clamp}))\n");
    out.push_str(&line);
    out.push_str("    }\n\n");
    out.push_str("    pub fn value(&self) -> Float {\n        self.0\n    }\n");
    out.push_str("}\n\n");
    let head = format!("impl Display for {} {{\n", t.name);
    out.push_str(&head);
    out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    out.push_str("        write!(f, \"{}\", self.0)\n    }\n}\n\n");
    let head = format!("impl From<{}> for Value {{\n", t.name);
    out.push_str(&head);
    let body = format!(
        "    fn from(value: {}) -> Self {{\n        Value::Numeric(Numeric::new(Number::Float(value.value()), None))\n    }}\n}}\n\n",
        t.name
    );
    out.push_str(&body);
    let head = format!("impl From<f64> for {} {{\n", t.name);
    out.push_str(&head);
    let body = "    fn from(value: f64) -> Self {\n        Self::new(Float(value))\n    }\n}\n\n";
    out.push_str(body);
    let head = format!("impl From<f32> for {} {{\n", t.name);
    out.push_str(&head);
    let body =
        "    fn from(value: f32) -> Self {\n        Self::new(Float(value as f64))\n    }\n}\n";
    out.push_str(body);
    out
}

/// The rustfmt import-list order key: names that start lowercase
/// sort before names that start uppercase; within each group the
/// case-folded name decides.
fn import_order(name: &str) -> (u8, String) {
    let lower = name.chars().next().is_some_and(|c| c.is_ascii_lowercase());
    (u8::from(!lower), name.to_lowercase())
}

/// Emit the category module (`mod.rs`).
fn emit_mod(plan: &CatPlan) -> String {
    let cat = &plan.cat;
    let single_name = format!("{}Terms", default_type_name(cat));
    let loop_name = format!("{}LoopItem", default_type_name(cat));
    let has_single = !plan.single.is_empty();
    let has_loop = !plan.loop_items.is_empty();

    let mut out = String::new();
    for t in &plan.types {
        let line = format!("mod {};", t.file);
        out.push_str(&line);
        out.push('\n');
    }
    if !plan.types.is_empty() {
        out.push('\n');
    }
    if has_single {
        out.push_str("use std::fmt::Display;\n\n");
    }

    let mut traits = vec!["CifTerm".to_string()];
    if has_loop {
        traits.push("LoopValueTerm".to_string());
    }
    if has_single {
        traits.push("SingleValueTerm".to_string());
    }
    let any_matrix = plan
        .single
        .iter()
        .chain(plan.loop_items.iter())
        .any(|p| p.kind == "matrix");
    if any_matrix {
        // The matrix payload helper module (data_dict::matrix).
        traits.push("matrix".to_string());
    }
    // rustfmt sorts import lists with lowercase names first.
    traits.sort_by(|a, b| import_order(a).cmp(&import_order(b)));
    let mut gram: Vec<&str> = vec!["Tag", "Value"];
    let any_string = plan
        .single
        .iter()
        .chain(plan.loop_items.iter())
        .any(|p| p.kind == "string");
    let any_numeric = plan
        .single
        .iter()
        .chain(plan.loop_items.iter())
        .any(|p| p.kind == "numeric");
    if any_string {
        gram.insert(0, "CharString");
    }
    if any_numeric {
        gram.insert(1, "Numeric");
    }
    if any_matrix {
        gram.push("Matrix");
    }
    // rustfmt sorts import lists with lowercase names first.
    gram.sort_by(|a, b| import_order(a).cmp(&import_order(b)));
    out.push_str("use crate::{\n");
    let line = format!("    data_dict::{{{}}},\n", traits.join(", "));
    out.push_str(&line);
    let line = format!("    grammar::{{{}}},\n", gram.join(", "));
    out.push_str(&line);
    out.push_str("};\n\n");
    for t in &plan.types {
        let line = format!("pub use self::{}::{};\n", t.file, t.name);
        out.push_str(&line);
    }
    if !plan.types.is_empty() {
        out.push('\n');
    }

    if has_single {
        out.push_str(&emit_enum(&single_name, &plan.single, false));
        out.push('\n');
        out.push_str(&emit_cif_term(cat, &single_name, &plan.single));
        out.push('\n');
        out.push_str(&emit_value(&single_name, &plan.single));
        out.push('\n');
        out.push_str(&format!("impl Display for {single_name} {{\n"));
        out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
        out.push_str("        write!(f, \"{}\", self.to_single_value_data())\n");
        out.push_str("    }\n}\n");
    }
    if has_loop {
        if has_single {
            out.push('\n');
        }
        out.push_str(&emit_enum(&loop_name, &plan.loop_items, true));
        out.push('\n');
        out.push_str(&emit_cif_term(cat, &loop_name, &plan.loop_items));
        out.push('\n');
        out.push_str(&emit_loop_values(&loop_name, &plan.loop_items));
    }
    out
}

fn emit_enum(enum_name: &str, plans: &[Plan], loop_form: bool) -> String {
    let mut out = String::new();
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("#[allow(non_camel_case_types, clippy::upper_case_acronyms)]\n");
    let head = format!("pub enum {enum_name} {{\n");
    out.push_str(&head);
    for p in plans {
        if let Some(note) = &p.note {
            for line in wrap(note, 68) {
                let l = format!("    /// {line}");
                out.push_str(&l);
                out.push('\n');
            }
        }
        let ty = field_type(p);
        let field = if loop_form { format!("Vec<{ty}>") } else { ty };
        let line = format!("    {}({field}),\n", p.variant);
        out.push_str(&line);
    }
    out.push_str("}\n");
    out
}

fn emit_cif_term(cat: &str, enum_name: &str, plans: &[Plan]) -> String {
    let mut out = String::new();
    let head = format!("impl CifTerm for {enum_name} {{\n");
    out.push_str(&head);
    out.push_str("    fn tag(&self) -> Tag {\n");
    let full_tag = plans.iter().any(|p| !p.item.prefixed);
    out.push_str(if full_tag {
        "        let tag = match self {\n"
    } else {
        "        let suffix = match self {\n"
    });
    for p in plans {
        let pattern = format!("{enum_name}::{}(_)", p.variant);
        let body = if full_tag {
            format!("\"{}\"", p.item.name.to_ascii_lowercase())
        } else {
            format!("\"{}\"", p.item.suffix.to_ascii_lowercase())
        };
        for line in fmt_arm("            ", &[pattern], &body) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out.push_str("        };\n");
    if full_tag {
        out.push_str("        Tag::new(tag.to_string())\n");
    } else {
        let line = format!("        Tag::new(format!(\"{cat}_{{suffix}}\"))\n");
        out.push_str(&line);
    }
    out.push_str("    }\n}\n");
    out
}

fn emit_value(enum_name: &str, plans: &[Plan]) -> String {
    let mut out = String::new();
    let head = format!("impl SingleValueTerm for {enum_name} {{\n");
    out.push_str(&head);
    out.push_str("    fn value(&self) -> Value {\n");
    out.push_str("        match self {\n");
    for g in value_groups(plans, false) {
        let pats: Vec<String> = g
            .variants
            .iter()
            .map(|v| format!("{enum_name}::{v}({})", g.var))
            .collect();
        for line in fmt_arm("            ", &pats, &g.body) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out.push_str("        }\n    }\n}\n");
    out
}

fn emit_loop_values(enum_name: &str, plans: &[Plan]) -> String {
    let mut out = String::new();
    let head = format!("impl LoopValueTerm for {enum_name} {{\n");
    out.push_str(&head);
    out.push_str("    fn values(&self) -> Vec<Value> {\n");
    out.push_str("        match self {\n");
    for g in value_groups(plans, true) {
        let pats: Vec<String> = g
            .variants
            .iter()
            .map(|v| format!("{enum_name}::{v}({})", g.var))
            .collect();
        for line in fmt_arm("            ", &pats, &g.body) {
            out.push_str(&line);
            out.push('\n');
        }
    }
    out.push_str("        }\n    }\n}\n");
    out
}

/// One match-arm group: the variant names, the binding variable,
/// and the arm body.
struct ValueGroup {
    variants: Vec<String>,
    var: String,
    body: String,
}

/// Group the plans by value type, in first-appearance order.
///
/// Items group by `(kind, type name)`: two closed items with
/// distinct types get distinct arms, and each group gets its own
/// binding variable so the arm bodies stay textually distinct.
/// The single-value form converts one value; the loop form drains
/// the whole column.
fn value_groups(plans: &[Plan], loop_form: bool) -> Vec<ValueGroup> {
    let mut keys: Vec<(String, Option<String>)> = Vec::new();
    for p in plans {
        let key = (p.kind.clone(), p.type_name.clone());
        if !keys.contains(&key) {
            keys.push(key);
        }
    }
    let mut typed_idx = 0usize;
    let mut groups: Vec<ValueGroup> = Vec::new();
    for (kind, type_name) in keys {
        let var: String = match (kind.as_str(), loop_form) {
            ("string", _) => "s".to_string(),
            ("numeric", _) => "n".to_string(),
            ("matrix", true) => "v".to_string(),
            ("matrix", false) => "m".to_string(),
            _ => {
                const POOL: &[&str] = &[
                    "v", "m", "c", "b", "t", "u", "w", "x", "y", "z", "q", "r", "f", "g", "h", "j",
                    "k", "l", "p", "d",
                ];
                let picked = if typed_idx < POOL.len() {
                    POOL[typed_idx].to_string()
                } else {
                    format!("v{typed_idx}")
                };
                typed_idx += 1;
                picked
            }
        };
        let variants: Vec<String> = plans
            .iter()
            .filter(|p| p.kind == kind && p.type_name == type_name)
            .map(|p| p.variant.clone())
            .collect();
        let body = if loop_form {
            match kind.as_str() {
                "string" => format!("{var}.iter().cloned().map(Value::from).collect()"),
                "numeric" => format!("{var}.iter().cloned().map(Value::from).collect()"),
                "matrix" => {
                    format!("{var}.iter().cloned().map(|m| matrix::to_value(&m)).collect()")
                }
                _ => format!("{var}.iter().cloned().map(Value::from).collect()"),
            }
        } else {
            match kind.as_str() {
                "string" => "Value::CharString(s.clone())".to_string(),
                "numeric" => "Value::Numeric(*n)".to_string(),
                "matrix" => "matrix::to_value(m)".to_string(),
                _ => format!("Value::from(*{var})"),
            }
        };
        groups.push(ValueGroup {
            variants,
            var: var.to_string(),
            body,
        });
    }
    groups
}

fn field_type(p: &Plan) -> String {
    match p.kind.as_str() {
        "string" => "CharString".to_string(),
        "numeric" => "Numeric".to_string(),
        "matrix" => "Matrix".to_string(),
        other => p
            .type_name
            .clone()
            .unwrap_or_else(|| panic!("item kind `{other}` has no type name")),
    }
}

/// Format a range bound as a Rust float literal, guaranteeing a
/// decimal point so the emitted code reads as a float.
fn f64_bound(v: f64) -> String {
    let s = v.to_string();
    if s.contains('.') || s.contains('e') || s.contains('f') || s.contains('N') {
        s
    } else {
        format!("{s}.0")
    }
}

/// Code variant name for a closed code. Alphanumeric characters are
/// kept, every other character maps to `_`, underscore runs collapse,
/// and the leading/trailing `_` are trimmed. The first character is
/// uppercased; a leading digit gets a `C` prefix. The result is a
/// valid Rust identifier, stable for the alphanumeric codes of the
/// existing tiers.
fn code_variant(code: &str) -> String {
    let mut out: Vec<char> = Vec::new();
    for c in code.chars() {
        let mapped = if c.is_alphanumeric() { c } else { '_' };
        if mapped == '_' && out.last() == Some(&'_') {
            continue;
        }
        out.push(mapped);
    }
    while out.first() == Some(&'_') {
        out.remove(0);
    }
    while out.last() == Some(&'_') {
        out.pop();
    }
    let mut id: String = out.iter().collect();
    if id.is_empty() {
        id = "Code".to_string();
    } else {
        let mut chars = id.chars();
        let first = chars.next().map(|c| c.to_ascii_uppercase()).unwrap();
        id = first.to_string() + &chars.collect::<String>();
    }
    if id.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        id = format!("C{id}");
    }
    id
}

/// Default variant name: the item suffix with the first character
/// uppercased, the rest as written (`measurement_radiation` becomes
/// `Measurement_radiation`, matching the cell module).
fn default_variant(suffix: &str) -> String {
    // The shell dictionary records dotted item names; the dot is
    // a name separator, so it joins the underscore in the
    // generated identifier.
    let suffix = suffix.replace('.', "_");
    let mut chars = suffix.chars();
    let first = chars
        .next()
        .map(|c| c.to_ascii_uppercase().to_string())
        .unwrap_or_default();
    format!("{first}{}", chars.collect::<String>())
}

/// Default type name: the item suffix camel-cased
/// (`coordinate_linkage` becomes `CoordinateLinkage`).
fn default_type_name(suffix: &str) -> String {
    suffix
        .replace('.', "_")
        .split('_')
        .map(|w| {
            let mut chars = w.chars();
            let first = chars
                .next()
                .map(|c| c.to_ascii_uppercase().to_string())
                .unwrap_or_default();
            format!("{first}{}", chars.collect::<String>())
        })
        .collect()
}

/// Camel-case to snake_case (`ExtraDefn` becomes `extra_defn`).
fn snake_case(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            if !out.is_empty() {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Format one match arm with or-patterns, following rustfmt's line
/// breaking at `MAX_WIDTH`:
/// - one line when the whole arm fits;
/// - the patterns on one line with a block body, when that pattern
///   line fits;
/// - one pattern per line, the expression body on the last line,
///   when that last line fits;
/// - one pattern per line with a block body otherwise.
fn fmt_arm(indent: &str, pats: &[String], body: &str) -> Vec<String> {
    let joined = pats.join(" | ");
    let full = format!("{indent}{joined} => {body},");
    if full.len() <= MAX_WIDTH {
        return vec![full];
    }
    let pat_line = format!("{indent}{joined} => {{");
    if pat_line.len() <= MAX_WIDTH {
        return vec![
            pat_line,
            format!("{indent}    {body}"),
            format!("{indent}}}"),
        ]
        .into_iter()
        .collect();
    }
    let mut out: Vec<String> = pats
        .iter()
        .enumerate()
        .map(|(i, p)| {
            if i == 0 {
                format!("{indent}{p}")
            } else {
                format!("{indent}| {p}")
            }
        })
        .collect();
    let last_expr = format!("{indent}| {} => {body},", pats.last().unwrap());
    if pats.len() > 1 && last_expr.len() <= MAX_WIDTH {
        out.last_mut().unwrap().push_str(&format!(" => {body},"));
        return out;
    }
    out.last_mut().unwrap().push_str(" => {");
    out.push(format!("{indent}    {body}"));
    out.push(format!("{indent}}}"));
    out
}

/// Word-wrap for doc comments, keeping lines under `width`.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if !cur.is_empty() && cur.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// The annotation categories, as an ordered map.
fn ann_categories(
    annotations: &Value,
    errors: &mut Vec<String>,
) -> BTreeMap<String, BTreeMap<String, Value>> {
    let Some(cats) = annotations["categories"].as_object() else {
        errors.push("annotations: missing the top-level `categories` object".to_string());
        return BTreeMap::new();
    };
    let mut out = BTreeMap::new();
    for (cat, entries) in cats {
        let Some(entries) = entries.as_object() else {
            errors.push(format!("annotation category `{cat}`: not an object"));
            continue;
        };
        let map: BTreeMap<String, Value> = entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        out.insert(cat.clone(), map);
    }
    out
}

/// The inventory items, with the derived crate-spelling tag.
fn inv_items(inventory: &Value, errors: &mut Vec<String>) -> Vec<Item> {
    let Some(items) = inventory["items"].as_array() else {
        errors.push("inventory: missing the `items` array".to_string());
        return Vec::new();
    };
    items
        .iter()
        .map(|i| {
            let name = i["name"].as_str().unwrap_or_default().to_string();
            let cat = i["category"].as_str().unwrap_or_default();
            let prefixed = name.starts_with(&format!("{cat}_"));
            let suffix = name
                .strip_prefix(&format!("{cat}_"))
                .unwrap_or(&name)
                .to_string();
            let tag = name.to_ascii_lowercase();
            Item {
                item: i.clone(),
                name,
                tag,
                suffix,
                prefixed,
                is_loop: i["is_loop"].as_bool().unwrap_or(false),
            }
        })
        .collect()
}

/// The T6 item union: the standard inventory items plus the shell
/// items whose name the standard inventory lacks. The shell items
/// are normalized to the crate spelling: lowercase item names and
/// lowercase category names, matching the registry tag case.
///
/// The shell `_su` uncertainty items stay out of the union per
/// decision D2: the uncertainty stays inside `Numeric`, and the
/// base item already carries the value.
fn union_items(shell: Option<&Value>, standard: &[Item], errors: &mut Vec<String>) -> Vec<Item> {
    let mut out = standard.to_vec();
    let shell = match shell {
        Some(v) => v,
        None => return out,
    };
    let std_names: std::collections::HashSet<String> =
        standard.iter().map(|i| i.tag.clone()).collect();
    let empty: Vec<Value> = Vec::new();
    for record in shell["items"].as_array().unwrap_or(&empty) {
        let name = record["name"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if name.is_empty() {
            errors.push(format!("shell inventory: an item record has no `name`"));
            continue;
        }
        if std_names.contains(&name) {
            continue; // the standard inventory already records the item
        }
        if name.ends_with("_su") {
            continue; // decision D2: the _su items are skipped
        }
        let cat = record["category"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let prefixed = name.starts_with(&format!("{cat}_"));
        let suffix = if prefixed {
            name[cat.len() + 1..].to_string()
        } else {
            name.clone()
        };
        let mut item = record.clone();
        // The registry keying uses the crate-spelling
        // lowercase category name.
        item["category"] = Value::String(cat);
        out.push(Item {
            item,
            name: name.clone(),
            tag: name,
            suffix,
            prefixed,
            is_loop: record["is_loop"].as_bool().unwrap_or(false),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Insert the missing `pub mod` lines, in sorted position. The
/// existing lines are kept verbatim; the file must hold only `pub
/// mod` lines or the run aborts.
fn update_core_cif_mod(path: &Path, cats: &[String], report: &mut Vec<String>) {
    let raw = fs::read_to_string(path).expect("read core_cif/mod.rs");
    let mut known: BTreeSet<String> = BTreeSet::new();
    for line in raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let Some(name) = line
            .strip_prefix("pub mod ")
            .and_then(|rest| rest.strip_suffix(';'))
        else {
            eprintln!(
                "dict_codegen: unexpected line in {}: `{line}`",
                path.display()
            );
            process::abort();
        };
        known.insert(name.to_string());
    }
    let existing = known.len();
    for c in cats {
        known.insert(c.clone());
    }
    let new_raw: String = known.iter().map(|c| format!("pub mod {c};\n")).collect();
    if new_raw != raw {
        fs::write(path, &new_raw).expect("write core_cif/mod.rs");
        let added = known.len() - existing;
        report.push(format!(
            "updated {} (+{added} pub mod lines)",
            path.display()
        ));
    } else {
        report.push(format!("unchanged {}", path.display()));
    }
}

/// Regenerate the tag registry: the existing sections plus the
/// annotated categories, in sorted order. The tag lists grow only;
/// nothing registered before is removed or reordered.
fn update_tags_registry(
    path: &Path,
    cats: &BTreeMap<String, BTreeMap<String, Value>>,
    items: &[Item],
    drift: &Value,
    report: &mut Vec<String>,
) {
    let raw = fs::read_to_string(path).expect("read tags.rs");
    let lines: Vec<&str> = raw.lines().collect();

    // The header runs up to the first section doc comment.
    let mut start = 0;
    while start + 1 < lines.len()
        && !(lines[start].starts_with("/// `") && lines[start + 1].starts_with("pub mod "))
    {
        start += 1;
    }
    let header: Vec<String> = lines[..start].iter().map(|l| l.to_string()).collect();

    // The sections: `/// `name` category tags (n).` + the block.
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut i = start;
    while i + 1 < lines.len()
        && lines[i].starts_with("/// `")
        && lines[i + 1].starts_with("pub mod ")
    {
        let name = lines[i + 1]
            .trim_start_matches("pub mod ")
            .trim_end_matches(" {")
            .to_string();
        let mut tags: Vec<String> = Vec::new();
        let mut closed = false;
        i += 2;
        for line in &lines[i..] {
            let trimmed = line.trim();
            // One line may hold the whole array (`= &["a", "b"];`).
            // The quoted tokens are the odd fields of a `"` split.
            let fields: Vec<&str> = trimmed.split('"').collect();
            for (idx, field) in fields.iter().enumerate() {
                if idx % 2 == 1 {
                    tags.push(field.to_string());
                }
            }
            if trimmed.starts_with(']') || trimmed.ends_with("];") {
                closed = true;
                break;
            }
            if !trimmed.is_empty() && !trimmed.starts_with('"') && !trimmed.starts_with("pub const")
            {
                eprintln!(
                    "dict_codegen: unexpected line in {}: `{trimmed}`",
                    path.display()
                );
                process::abort();
            }
            i += 1;
        }
        i += 1; // the line that closed the array
        if !closed {
            i += 1; // the multi-line form ends with a bare `];`
        }
        i += 1; // the closing `}`
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        sections.insert(name, tags);
    }

    // The remainder must be the `ALL` block and the tests; both are
    // regenerated below.
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    if !lines
        .get(i)
        .is_some_and(|l| l.starts_with("/// All implemented tags"))
    {
        eprintln!(
            "dict_codegen: unexpected content at line {} of {}",
            i + 1,
            path.display()
        );
        process::abort();
    }

    for cat in cats.keys() {
        let mut tags: Vec<String> = items
            .iter()
            .filter(|i| i.item["category"].as_str() == Some(cat))
            .map(|i| i.tag.clone())
            .collect();
        // The legacy drift tags stay registered for output.
        for extra in drift[cat].as_array().cloned().unwrap_or_default() {
            if extra.is_string() && !tags.contains(&extra.to_string()) {
                tags.push(extra.as_str().unwrap().to_string());
            }
        }
        tags.sort();
        sections.insert(cat.clone(), tags);
    }

    let mut out = String::new();
    for h in &header {
        out.push_str(h);
        out.push('\n');
    }
    if !header.last().is_some_and(|h| h.is_empty()) {
        out.push('\n');
    }
    for (name, tags) in &sections {
        out.push_str(&format!("/// `{name}` category tags ({}).\n", tags.len()));
        let list: Vec<String> = tags.iter().map(|t| format!("\"{t}\"")).collect();
        let one_line = format!("    pub const TAGS: &[&str] = &[{}];", list.join(", "));
        out.push_str(&format!("pub mod {name} {{\n"));
        if one_line.len() <= MAX_WIDTH {
            out.push_str(&one_line);
            out.push('\n');
        } else {
            out.push_str("    pub const TAGS: &[&str] = &[\n");
            for t in tags {
                out.push_str(&format!("        \"{t}\",\n"));
            }
            out.push_str("    ];\n");
        }
        out.push_str("}\n\n");
    }
    out.push_str("/// All implemented tags, one entry per `CifTerm` tag string.\n");
    out.push_str("pub const ALL: &[&str] = &[\n");
    let mut total = 0usize;
    for (name, tags) in &sections {
        out.push_str(&format!("    // {name}\n"));
        for t in tags {
            out.push_str(&format!("    \"{t}\",\n"));
        }
        total += tags.len();
    }
    out.push_str("];\n\n");
    out.push_str("#[cfg(test)]\nmod tests {\n    use super::*;\n\n");
    out.push_str("    #[test]\n    fn category_sections_are_subsets_of_all() {\n");
    for name in sections.keys() {
        out.push_str(&format!(
            "        for t in {name}::TAGS {{\n            assert!(ALL.contains(t), \"{{t}} missing from ALL\");\n        }}\n"
        ));
    }
    out.push_str("    }\n\n");
    out.push_str("    #[test]\n    fn registry_counts() {\n");
    for (name, tags) in &sections {
        out.push_str(&format!(
            "        assert_eq!({name}::TAGS.len(), {});\n",
            tags.len()
        ));
    }
    out.push_str(&format!("        assert_eq!(ALL.len(), {total});\n"));
    out.push_str("        assert!(ALL.iter().all(|t| !t.starts_with('_')));\n");
    out.push_str("        assert_eq!(\n");
    out.push_str("            ALL.len(),\n");
    out.push_str("            ALL.iter().collect::<std::collections::HashSet<_>>().len()\n");
    out.push_str("        );\n    }\n}\n");

    if out != raw {
        fs::write(path, &out).expect("write tags.rs");
        report.push(format!(
            "updated {} ({} categories, {total} tags in ALL)",
            path.display(),
            sections.len()
        ));
    } else {
        report.push(format!("unchanged {}", path.display()));
    }
}
