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
    let no_write = std::env::args().any(|a| a == "--no-write");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let inventory = load_json(&manifest.join("tests/dict_inventory.json"));
    // The T6 dual-inventory step: when the shell inventory is
    // recorded, the shell items join the per-category item set.
    let shell_path = manifest.join("tests/dict_inventory_shell.json");
    let shell = shell_path.is_file().then(|| load_json(&shell_path));
    let annotations = load_json(
        &manifest
            .parent()
            .expect("workspace root")
            .join("codegen/annotations.json"),
    );
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

    let report = emit(
        &manifest,
        &inventory,
        shell.as_ref(),
        &annotations,
        &drift,
        no_write,
    );
    if no_write {
        println!("dict_codegen: --no-write check mode, no files written");
    }
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
    /// True when the range type carries a `from_numeric` constructor.
    numeric: bool,
    /// An optional override of the closed-set doc header.
    doc: Option<String>,
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
    no_write: bool,
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
        write_category(manifest, plan, &mut report, no_write);
    }

    let core_cif_mod = manifest.join("src/data_dict/core_cif/mod.rs");
    let ann_cats: Vec<String> = plans.iter().map(|p| p.cat.clone()).collect();
    update_core_cif_mod(&core_cif_mod, &ann_cats, &mut report, no_write);
    let tags = manifest.join("src/data_dict/tags.rs");
    update_tags_registry(&tags, &cats, &items, &drift, &mut report, no_write);
    inject_item_docs(manifest, inventory, shell, &cats, &mut report, no_write);
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
            let file = entry
                .get("file")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| snake_case(&name));
            types.push(TypeFile {
                file,
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
            numeric: entry
                .get("numeric")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            doc: entry
                .get("doc")
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
fn write_category(manifest: &Path, plan: &CatPlan, report: &mut Vec<String>, no_write: bool) {
    let dir = manifest.join("src/data_dict/core_cif").join(&plan.cat);
    if !no_write {
        fs::create_dir_all(&dir).expect("create category directory");
    }

    for t in &plan.types {
        let path = dir.join(format!("{}.rs", t.file));
        let body = if t.kind == "closed" {
            emit_closed(plan, t)
        } else {
            emit_range(plan, t)
        };
        write_if_changed(manifest, &path, &body, report, no_write);
    }

    let mod_path = dir.join("mod.rs");
    let existing = existing_mod_body(&dir);
    let body = emit_mod(plan, &existing);
    write_if_changed(manifest, &mod_path, &body, report, no_write);
}

/// Read the existing `mod.rs` body of one category. An empty string when
/// the file is absent. The layer lines are recovered from it, so the
/// generated file round-trips the hand-written layer declarations.
fn existing_mod_body(dir: &Path) -> String {
    let path = dir.join("mod.rs");
    fs::read_to_string(path).unwrap_or_default()
}

/// Dump the generated body next to the real tree when
/// `DICT_CODEGEN_DUMP` names a directory. This is the diff side
/// channel: it writes the body to `<dump>/<path under the manifest>`
/// so the drift can be inspected without touching the tree.
fn dump_body(manifest: &Path, path: &Path, body: &str) {
    let dump = match std::env::var("DICT_CODEGEN_DUMP") {
        Ok(d) => d,
        Err(_) => return,
    };
    let rel = path.strip_prefix(manifest).unwrap_or(path);
    let target = std::path::Path::new(&dump).join(rel);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("create dump directory");
    }
    fs::write(&target, body).expect("write dump file");
}

fn write_if_changed(
    manifest: &Path,
    path: &Path,
    body: &str,
    report: &mut Vec<String>,
    no_write: bool,
) {
    let same = fs::read_to_string(path)
        .map(|old| old == body)
        .unwrap_or(false);
    dump_body(manifest, path, body);
    if same {
        report.push(format!("unchanged {}", path.display()));
    } else if no_write {
        report.push(format!("would-write {}", path.display()));
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
    out.push_str("use crystal_cif_fmt::grammar::{CharString, UnquotedString, Value};\n\n");
    if let Some(doc) = &plan_item.doc {
        for line in doc.lines() {
            out.push_str(&format!("/// {line}\n"));
        }
    } else {
        let doc = format!(
            "/// The closed code set of `_{item}`. The codes are recorded in\n\
             /// the dictionary `_enumeration` (see `tests/dict_inventory.json`).\n"
        );
        out.push_str(&doc);
    }
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
    // The closed-set parse method: a code to a variant, None when absent.
    out.push_str("\n");
    out.push_str(&format!("impl {} {{\n", t.name));
    out.push_str(
        "    /// Parse a closed-set code into a variant. None when the code is not in the set.\n",
    );
    out.push_str("    pub fn parse(code: &str) -> Option<Self> {\n");
    out.push_str("        match code {\n");
    for code in codes {
        let variant = code_variant(code);
        let line = format!("            \"{code}\" => Some(Self::{variant}),\n");
        out.push_str(&line);
    }
    out.push_str("            _ => None,\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n");
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
    out.push_str("use crystal_cif_fmt::grammar::{Float, Number, Numeric, Value};\n\n");
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
    out.push_str("    /// Build the guarded value: the input clamped to the dictionary range.\n");
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
    out.push_str("    /// The recorded value, stored as written.\n");
    out.push_str("    pub fn value(&self) -> Float {\n        self.0\n    }\n");
    if plan_item.numeric {
        out.push_str("\n");
        out.push_str("    /// The recorded value, stored as written. The range guard applies at\n");
        out.push_str("    /// the write.\n");
        out.push_str("    pub fn from_numeric(n: Numeric) -> Self {\n");
        out.push_str("        let raw = match n.number() {\n");
        out.push_str("            Number::Integer(i) => i.0 as f64,\n");
        out.push_str("            Number::Float(f) => f.0,\n");
        out.push_str("        };\n");
        out.push_str("        Self(Float(raw))\n");
        out.push_str("    }\n");
    }
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
fn emit_mod(plan: &CatPlan, existing: &str) -> String {
    let cat = &plan.cat;
    let single_name = format!("{}Terms", default_type_name(cat));
    let loop_name = format!("{}LoopItem", default_type_name(cat));
    let has_single = !plan.single.is_empty();
    let has_loop = !plan.loop_items.is_empty();

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

    // The generated body: the `Terms` and `LoopItem` enums and their
    // trait impls. This is the part the generator owns.
    let mut body = String::new();
    if has_single {
        body.push_str(&emit_enum(&single_name, &plan.single, false));
        body.push('\n');
        body.push_str(&emit_cif_term(cat, &single_name, &plan.single));
        body.push('\n');
        body.push_str(&emit_value(&single_name, &plan.single));
        body.push('\n');
        body.push_str(&format!("impl Display for {single_name} {{\n"));
        body.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
        body.push_str("        write!(f, \"{}\", self.to_single_value_data())\n");
        body.push_str("    }\n}\n");
    }
    if has_loop {
        if has_single {
            body.push('\n');
        }
        body.push_str(&emit_enum(&loop_name, &plan.loop_items, true));
        body.push('\n');
        body.push_str(&emit_cif_term(cat, &loop_name, &plan.loop_items));
        body.push('\n');
        body.push_str(&emit_loop_values(&loop_name, &plan.loop_items));
    }

    // The header: the module declarations, imports, and re-exports. The
    // committed files are hand-maintained with per-file layout, so an
    // existing header is preserved verbatim. A uniform header is built
    // only for a brand-new category.
    if !existing.is_empty() {
        if let Some(idx) = existing.lines().position(|l| l.starts_with("#[derive")) {
            // The header preserves the hand-maintained lines. The
            // generator's own enum doc lines from a previous run stay
            // out: the body re-emits them, so they are stripped from
            // the captured header.
            let mut header_lines: Vec<&str> = existing.lines().take(idx).collect();
            while header_lines.last().is_some_and(|l| l.starts_with("///")) {
                header_lines.pop();
            }
            let header: String = header_lines.join("\n");
            return format!("{header}\n{body}");
        }
    }
    let mut head = String::new();
    let mut mods: Vec<String> = plan.types.iter().map(|t| t.file.clone()).collect();
    let layer = existing.lines().find(|l| {
        l.starts_with(&format!("pub use self::{cat}::"))
            || l.starts_with(&format!("pub use {cat}::"))
    });
    if layer.is_some() {
        mods.push(cat.clone());
    }
    mods.sort();
    for m in &mods {
        head.push_str(&format!("mod {m};\n"));
    }
    if !mods.is_empty() {
        head.push('\n');
    }
    if has_single {
        head.push_str("use std::fmt::Display;\n\n");
    }
    head.push_str(&format!(
        "use crate::data_dict::{{{}}};\n",
        traits.join(", ")
    ));
    head.push_str(&format!(
        "use crystal_cif_fmt::grammar::{{{}}};\n",
        gram.join(", ")
    ));
    head.push('\n');
    let mut exports: Vec<(String, String)> = plan
        .types
        .iter()
        .map(|t| {
            (
                t.file.clone(),
                format!("pub use self::{}::{};", t.file, t.name),
            )
        })
        .collect();
    if let Some(line) = layer {
        exports.push((cat.clone(), line.to_string()));
    }
    exports.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, line) in &exports {
        head.push_str(line);
        head.push('\n');
    }
    if !exports.is_empty() {
        head.push('\n');
    }
    head.push_str(&body);
    head
}

fn emit_enum(enum_name: &str, plans: &[Plan], loop_form: bool) -> String {
    let kind = if loop_form {
        "loop-value"
    } else {
        "single-value"
    };
    let cat = plans
        .first()
        .map(|p| p.item.item["category"].as_str().unwrap_or_default())
        .unwrap_or_default();
    let mut out = String::new();
    out.push_str(&format!(
        "/// The {kind} terms of the `{cat}` category: one variant per\n"
    ));
    out.push_str(&format!("/// registry {kind} item, in item order.\n"));
    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("#[allow(non_camel_case_types, clippy::upper_case_acronyms)]\n");
    let head = format!("pub enum {enum_name} {{\n");
    out.push_str(&head);
    for p in plans {
        out.push_str(&item_variant_docs(p, loop_form));
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

/// The doc lines of one term variant: the CIF item name, the
/// dictionary definition, and the units. A bare item keeps the
/// one-line name doc.
fn item_variant_docs(p: &Plan, loop_form: bool) -> String {
    let mut out = String::new();
    let item = &p.item.name;
    let def = p.item.item["definition"].as_str();
    let units = p.item.item["units"].as_str();
    let units_detail = p.item.item["units_detail"].as_str();
    let holds = if loop_form {
        "The variant holds the column values, one per loop row."
    } else {
        "The variant holds the single value."
    };
    out.push_str(&format!("    /// The `_{item}` item. {holds}\n"));
    if let Some(d) = def {
        out.push_str("    ///\n");
        for line in wrap(d, 68) {
            out.push_str(&format!("    /// {line}\n"));
        }
    }
    if units.is_some() || units_detail.is_some() {
        let mut u = String::new();
        if let Some(u_) = units {
            u.push_str(u_);
        }
        if let Some(d) = units_detail {
            u.push_str(&format!(" ({d})"));
        }
        out.push_str(&format!("    /// Units: {u}.\n"));
    }
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
/// Escape one dictionary text for rustdoc. The definition and
/// note text is data, not the crate's markdown: angle brackets
/// become HTML entities, a bare URL becomes a backtick span.
fn sanitize_doc(text: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < text.len() {
        if text[i..].starts_with("http://") || text[i..].starts_with("https://") {
            let rest = &text[i..];
            let start = rest.find("://").unwrap() + 3;
            let mut j = start;
            while j < rest.len() {
                let ch = rest.as_bytes()[j] as char;
                if ch.is_whitespace() || matches!(ch, ')' | ']' | ',') {
                    break;
                }
                j += 1;
            }
            out.push_str(&format!("`{}`", &rest[..j]));
            i += j;
            continue;
        }
        match text.as_bytes()[i] {
            b'<' => out.push_str("&lt;"),
            b'>' => out.push_str("&gt;"),
            _ => out.push(text[i..].chars().next().unwrap()),
        }
        i += 1;
    }
    out
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let text = sanitize_doc(text);
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
            // The standard inventory already records the item. When
            // the standard record carries no enumeration but the
            // shell record does, merge the shell enumeration in so
            // a closed set recorded in the shell dictionary is not
            // flagged as drift.
            if let Some(std_item) = out.iter_mut().find(|i| i.tag == name) {
                if std_item.item["enumeration"].is_null() && record["enumeration"].is_object() {
                    std_item.item["enumeration"] = record["enumeration"].clone();
                }
            }
            continue;
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
/// A line that declares a module: `pub mod name;`
fn is_pub_mod(line: &str) -> Option<String> {
    line.trim()
        .strip_prefix("pub mod ")
        .and_then(|rest| rest.strip_suffix(';'))
        .map(|n| n.to_string())
}

/// Add the missing `pub mod` lines to `core_cif/mod.rs` without touching
/// any other line. The file also carries the typed document layer (the
/// `CifCategory` trait block, the `convert` module, the imports). Those
/// lines are preserved as-is. When nothing is missing the file is
/// reported unchanged and not rewritten.
fn update_core_cif_mod(path: &Path, cats: &[String], report: &mut Vec<String>, no_write: bool) {
    let raw = fs::read_to_string(path).expect("read core_cif/mod.rs");
    let lines: Vec<String> = raw.lines().map(|l| l.to_string()).collect();
    let mut existing: BTreeSet<String> = BTreeSet::new();
    for l in &lines {
        if let Some(n) = is_pub_mod(l) {
            existing.insert(n);
        }
    }
    let mut missing: Vec<&String> = cats.iter().filter(|c| !existing.contains(*c)).collect();

    // The module docs: every category `pub mod` line carries a doc
    // line, whether the category is annotated or hand-maintained.
    // The doc line is the uniform category description.
    let mut new_lines: Vec<String> = Vec::new();
    let mut doc_added = 0;
    for (idx, line) in lines.iter().enumerate() {
        if let Some(n) = is_pub_mod(line) {
            let has_doc = idx > 0 && lines[idx - 1].starts_with("/// ");
            if !has_doc {
                new_lines.push(format!(
                    "/// The `{n}` CIF category of the typed document layer."
                ));
                doc_added += 1;
            }
        }
        new_lines.push(line.clone());
    }
    if missing.is_empty() && doc_added == 0 {
        report.push(format!("unchanged {}", path.display()));
        return;
    }
    missing.sort();
    // Insert at the end of the leading run of `pub mod` / blank lines, so the
    // module declarations stay grouped above the layer block.
    let mut insert_at = 0;
    while insert_at < new_lines.len() {
        if is_pub_mod(&new_lines[insert_at]).is_some() || new_lines[insert_at].trim().is_empty() {
            insert_at += 1;
        } else {
            break;
        }
    }
    for c in &missing {
        new_lines.insert(
            insert_at,
            format!("/// The `{c}` CIF category of the typed document layer."),
        );
        new_lines.insert(insert_at + 1, format!("pub mod {c};"));
        insert_at += 2;
    }
    let new_raw = new_lines.join("\n") + "\n";
    if no_write {
        report.push(format!(
            "would-update {} (+{} pub mod lines, +{} module docs)",
            path.display(),
            missing.len(),
            doc_added
        ));
    } else {
        fs::write(path, &new_raw).expect("write core_cif/mod.rs");
        report.push(format!(
            "updated {} (+{} pub mod lines, +{} module docs)",
            path.display(),
            missing.len(),
            doc_added
        ));
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
    no_write: bool,
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
            // The doc lines of the `TAGS` constant carry no tags.
            if !trimmed.is_empty()
                && !trimmed.starts_with('"')
                && !trimmed.starts_with("pub const")
                && !trimmed.starts_with("///")
            {
                // The registry carries content the parser does not model.
                // Preserve it as-is rather than clobber the file.
                report.push(format!(
                    "skipped {} (unexpected content, preserved as-is)",
                    path.display()
                ));
                return;
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
        // The tail is not the expected `ALL` block. Preserve the file as-is.
        report.push(format!(
            "skipped {} (unexpected tail, preserved as-is)",
            path.display()
        ));
        return;
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
        let const_doc = "    /// The registered tag strings of the category, one per implemented item. No leading underscore.";
        let one_line = format!("    pub const TAGS: &[&str] = &[{}];", list.join(", "));
        out.push_str(&format!("pub mod {name} {{\n"));
        out.push_str(const_doc);
        out.push('\n');
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

    if out == raw {
        report.push(format!("unchanged {}", path.display()));
    } else if no_write {
        report.push(format!(
            "would-update {} ({} categories, {total} tags in ALL)",
            path.display(),
            sections.len()
        ));
    } else {
        fs::write(path, &out).expect("write tags.rs");
        report.push(format!(
            "updated {} ({} categories, {total} tags in ALL)",
            path.display(),
            sections.len()
        ));
    }
}

/// The dictionary item docs, injected into the hand-maintained
/// category files: the struct fields, the term-enum variants, and
/// the struct headers. The generated type files and the
/// regenerated `mod.rs` bodies carry their docs from the emitters;
/// this step only touches what the emitters do not own.
///
/// The mapping is by the lowercased item suffix: the struct field
/// `length_a` of the `cell` category maps to the item
/// `cell_length_a`. A field the inventory does not name is left
/// as-is and reported.
fn inject_item_docs(
    manifest: &Path,
    inventory: &Value,
    shell: Option<&Value>,
    ann_cats: &BTreeMap<String, BTreeMap<String, Value>>,
    report: &mut Vec<String>,
    no_write: bool,
) {
    let mut errors = Vec::new();
    let core: Vec<Item> = inv_items(inventory, &mut errors);
    let shell_items: Vec<Item> = match shell {
        Some(sh) => inv_items(sh, &mut errors),
        None => Vec::new(),
    };
    if !errors.is_empty() {
        report.push(format!("inject_item_docs: inventory errors: {errors:?}"));
        return;
    }
    let base = manifest.join("src/data_dict/core_cif");
    let mut dirs: Vec<String> = Vec::new();
    let entries = fs::read_dir(&base).expect("read the category directory");
    for e in entries.flatten() {
        if e.path().is_dir() {
            dirs.push(e.file_name().to_string_lossy().to_string());
        }
    }
    dirs.sort();
    for cat in &dirs {
        let dir = base.join(cat);
        let cat_core: Vec<&Item> = core
            .iter()
            .filter(|i| i.item["category"].as_str() == Some(cat.as_str()))
            .collect();
        let cat_shell: Vec<&Item> = shell_items
            .iter()
            .filter(|i| i.item["category"].as_str() == Some(cat.as_str()))
            .collect();
        // The field map: the normalized suffix key (the item suffix
        // with dots, hyphens, and case folded). The core inventory
        // wins over the shell on a key collision; a true collision
        // inside one source is a drift error.
        let norm = |name: &str| {
            name.replace('.', "_")
                .replace('-', "_")
                .replace('/', "_")
                .to_ascii_lowercase()
        };
        // The field key: the Rust spellings map back to the CIF item
        // spellings. `r#type` is the keyword-escaped `type`, and a
        // trailing underscore escapes a keyword (`abstract_`).
        let field_key = |name: &str| {
            let plain = name.strip_prefix("r#").unwrap_or(name);
            let plain = plain.strip_suffix('_').unwrap_or(plain);
            plain.replace('-', "_").to_ascii_lowercase()
        };
        // One map per source: a collision inside one source is a
        // drift error; on a key collision across sources the core
        // inventory wins (the standard category owns the field).
        let mut field_map: std::collections::HashMap<String, &Item> =
            std::collections::HashMap::new();
        let mut shell_map: std::collections::HashMap<String, &Item> =
            std::collections::HashMap::new();
        for i in &cat_core {
            let key = norm(&i.suffix);
            if field_map.insert(key.clone(), i).is_some() {
                report.push(format!(
                    "inject_item_docs: category `{cat}`: the normalized suffix `{key}` \
                     matches more than one item in the core inventory"
                ));
            }
        }
        for i in &cat_shell {
            let key = norm(&i.suffix);
            if shell_map.insert(key.clone(), i).is_some() {
                report.push(format!(
                    "inject_item_docs: category `{cat}`: the normalized suffix `{key}` \
                     matches more than one item in the shell inventory"
                ));
            }
        }
        // The global fallback: a field the category maps do not
        // own (a cross-category field, or a sub-category item like
        // `cell_measurement_refln`). It is usable only when the full
        // item name names exactly one item, so an ambiguous key
        // stays untouched and reported.
        let all_items: Vec<&Item> = core.iter().chain(shell_items.iter()).map(|i| i).collect();
        let lookup = |key: &str, scope: Option<&str>| -> Option<&Item> {
            if let Some(i) = field_map.get(key).copied() {
                return Some(i);
            }
            if let Some(i) = shell_map.get(key).copied() {
                return Some(i);
            }
            let hits: Vec<&Item> = find_all_full_items(&all_items, key)
                // The `_su` uncertainty items stay out: the base
                // item carries the value (decision D2).
                .into_iter()
                .filter(|i| !i.name.ends_with("_su"))
                .collect();
            // The core and the shell may record one item each; the
            // name deduplicates them.
            let mut seen = std::collections::HashSet::new();
            let hits: Vec<&Item> = hits
                .into_iter()
                .filter(|i| seen.insert(i.name.clone()))
                .collect();
            if hits.len() == 1 {
                return Some(hits[0]);
            }
            if let Some(scope) = scope {
                let scoped: Vec<&Item> = hits
                    .iter()
                    .filter(|i| {
                        i.name
                            .replace('.', "_")
                            .replace('-', "_")
                            .replace('/', "_")
                            .to_ascii_lowercase()
                            .starts_with(scope)
                    })
                    .copied()
                    .collect();
                let mut seen2 = std::collections::HashSet::new();
                let scoped: Vec<&Item> = scoped
                    .into_iter()
                    .filter(|i| seen2.insert(i.name.clone()))
                    .collect();
                if scoped.len() == 1 {
                    return Some(scoped[0]);
                }
            }
            None
        };
        let mut files: Vec<std::path::PathBuf> = Vec::new();
        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().is_some_and(|ext| ext == "rs") {
                    files.push(p);
                }
            }
        }
        files.sort();
        for file in files {
            let is_ann = ann_cats.contains_key(cat);
            let body = match fs::read_to_string(&file) {
                Ok(b) => b,
                Err(_) => continue,
            };
            let lines: Vec<&str> = body.lines().collect();
            let mut out: Vec<String> = Vec::new();
            let mut field_docs = 0;
            let mut enum_docs = 0;
            let mut struct_docs = 0;
            let mut in_struct = false;
            let mut in_enum = false;
            let mut enum_ann = false;
            let mut cur_struct: Option<String> = None;
            // The attribute run above one item: the header docs insert
            // before the `#[derive]` block, and a doc line above the
            // block counts as documented.
            let mut attr_start: Option<usize> = None;
            // A header doc block, with attribute lines between the
            // docs and the item allowed.
            let has_doc_above = |idx: usize| -> bool {
                let mut j = idx;
                while j > 0 {
                    let prev = lines[j - 1].trim_start();
                    if prev.is_empty() {
                        return false;
                    }
                    if prev.starts_with("///") {
                        return true;
                    }
                    if prev.starts_with("#[") {
                        j -= 1;
                        continue;
                    }
                    return false;
                }
                false
            };
            for (idx, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                // Indented docs count too: the struct fields and the
                // enum variants carry four-space-indented `///` lines.
                let prev_doc = idx > 0 && lines[idx - 1].trim_start().starts_with("///");
                if trimmed.starts_with("#[") {
                    if attr_start.is_none() {
                        attr_start = Some(out.len());
                    }
                }
                if trimmed == "}" {
                    in_struct = false;
                    in_enum = false;
                    enum_ann = false;
                    cur_struct = None;
                }
                if trimmed.starts_with("pub struct ") {
                    in_struct = !trimmed.ends_with(';');
                    in_enum = false;
                    enum_ann = false;
                    cur_struct = trimmed
                        .strip_prefix("pub struct ")
                        .and_then(|r| r.split_whitespace().next())
                        .map(|n| n.to_string());
                    if in_struct && !has_doc_above(idx) {
                        let name = trimmed
                            .strip_prefix("pub struct ")
                            .and_then(|r| r.split_whitespace().next())
                            .unwrap_or_default();
                        let is_cat_struct = name == default_type_name(cat.as_str());
                        let is_row = name.ends_with("Row");
                        if is_cat_struct || is_row {
                            let doc = if is_cat_struct {
                                format!(
                                    "/// The `{cat}` category struct. One field per registry item."
                                )
                            } else {
                                format!(
                                    "/// One loop row of the `{cat}` category: the per-observation values of the loop items."
                                )
                            };
                            let pos = attr_start.take().unwrap_or(out.len());
                            out.insert(pos, doc);
                            struct_docs += 1;
                        }
                    }
                }
                if trimmed.starts_with("pub enum ") {
                    in_enum = true;
                    in_struct = false;
                    // The term-enum header docs: only the hand-written
                    // categories. The regenerated `mod.rs` bodies
                    // carry the docs from `emit_enum`.
                    let name = trimmed
                        .strip_prefix("pub enum ")
                        .and_then(|r| r.split_whitespace().next())
                        .unwrap_or_default();
                    let is_term_enum = name.ends_with("Terms") || name.ends_with("LoopItem");
                    enum_ann = is_term_enum && !is_ann && !has_doc_above(idx);
                    if enum_ann {
                        let kind = if name.ends_with("LoopItem") {
                            "loop-value"
                        } else {
                            "single-value"
                        };
                        let doc1 = format!(
                            "/// The {kind} terms of the `{cat}` category: one variant per\n"
                        );
                        let doc2 = format!("/// registry {kind} item, in item order.");
                        let pos = attr_start.take().unwrap_or(out.len());
                        out.insert(pos, doc1);
                        out.insert(pos + 1, doc2);
                        enum_docs += 1;
                    }
                }
                // The struct fields and the term-enum variants.
                let field = if in_struct {
                    line_match::is_struct_field(line)
                } else {
                    None
                };
                let variant = if in_enum && enum_ann {
                    line_match::is_enum_variant(line)
                } else {
                    None
                };
                if let Some(name) = field.or(variant) {
                    let indent = line.len() - line.trim_start().len();
                    if !prev_doc {
                        let key = field_key(name);
                        // The scope disambiguates a key the whole
                        // inventory shares: a struct field is scoped
                        // to its own name (`ReflnsDResolution` owns
                        // `reflns_d_resolution_*`), a variant to the
                        // category.
                        let scope: Option<String> = if in_enum {
                            Some(cat.clone())
                        } else {
                            cur_struct.as_deref().map(snake_case)
                        };
                        let item = lookup(&key, scope.as_deref());
                        if let Some(item) = item {
                            for dl in item_field_docs(item, in_enum) {
                                out.push(format!("{}{dl}", " ".repeat(indent)));
                            }
                            field_docs += 1;
                        } else if name == "rows" && line.contains("Vec<") {
                            out.push(format!(
                                "{}/// The loop rows of the category: one row per observation.",
                                " ".repeat(indent)
                            ));
                            field_docs += 1;
                        } else {
                            report.push(format!(
                                "inject_item_docs: {} field or variant `{name}` has no inventory item",
                                file.display()
                            ));
                        }
                    }
                }
                if !trimmed.starts_with("#[") && !trimmed.starts_with("pub ") {
                    attr_start = None;
                }
                out.push(line.to_string());
            }
            let new_body = out.join("\n") + "\n";
            if new_body != body {
                write_if_changed(manifest, &file, &new_body, report, no_write);
                let verb = if no_write {
                    "would-doc-inject"
                } else {
                    "doc-injected"
                };
                if field_docs + enum_docs + struct_docs > 0 {
                    report.push(format!(
                        "{verb} {} (+{} field docs, +{} enum docs, +{} struct docs)",
                        file.display(),
                        field_docs,
                        enum_docs,
                        struct_docs
                    ));
                }
            } else {
                report.push(format!("unchanged {}", file.display()));
            }
        }
    }
}

/// The doc lines of one item field or term variant: the CIF item
/// name, the dictionary definition, and the units. No leading
/// indentation: the caller adds it.
fn item_field_docs(item: &Item, enum_form: bool) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let name = &item.name;
    let holds = if enum_form {
        "The variant holds the column values, one per loop row."
    } else if item.is_loop {
        "The field holds the column values, one per loop row."
    } else {
        "The field holds the single value."
    };
    out.push(format!("/// The `_{name}` item. {holds}"));
    if let Some(def) = item.item["definition"].as_str() {
        out.push("///".to_string());
        out.extend(wrap(def, 68).into_iter().map(|l| format!("/// {l}")));
    }
    let units = item.item["units"].as_str();
    let units_detail = item.item["units_detail"].as_str();
    if units.is_some() || units_detail.is_some() {
        let mut u = String::new();
        if let Some(u_) = units {
            u.push_str(u_);
        }
        if let Some(d) = units_detail {
            u.push_str(&format!(" ({d})"));
        }
        out.push(format!("/// Units: {u}."));
    }
    out
}

/// The items whose normalized full name ends with the field or
/// variant name. The hand-written term enums may keep part of the
/// item name in the variant name. The caller uses the result only
/// when it is a single item, so an ambiguous key reports instead of
/// guessing.
fn find_all_full_items<'a>(items: &'a [&'a Item], key: &str) -> Vec<&'a Item> {
    items
        .iter()
        .copied()
        .filter(|i| {
            let full = i
                .name
                .replace('.', "_")
                .replace('-', "_")
                .replace('/', "_")
                .to_ascii_lowercase();
            full.ends_with(key)
        })
        .collect()
}

/// The line-shape matchers of the injector.
mod line_match {
    /// A struct field line: an indented `pub <field>:` with a
    /// lowercase field. The unindented `pub struct` header does not
    /// match.
    pub fn is_struct_field(line: &str) -> Option<&str> {
        if !line.starts_with(' ') {
            return None;
        }
        let t = line.trim_start();
        let rest = t.strip_prefix("pub ")?;
        let field = rest.split(|c: char| c == ':' || c.is_whitespace()).next()?;
        if !field.chars().next().is_some_and(|c| c.is_ascii_lowercase()) {
            return None;
        }
        Some(field)
    }

    /// A term-enum variant line: an indented `Variant(` with an
    /// uppercase name. The unindented `pub enum` header does not
    /// match.
    pub fn is_enum_variant(line: &str) -> Option<&str> {
        if !line.starts_with(' ') {
            return None;
        }
        let t = line.trim_start();
        let v = t.split('(').next()?;
        if !v.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
            return None;
        }
        Some(v)
    }
}
