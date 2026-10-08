//! Regenerates `tests/dict_inventory.json` from `tests/dict/cif_core.dic`.
//!
//! The inventory records the DDL1 core dictionary item set: one record
//! per item with its name, category, type, loop flag, and the
//! enumeration constraint when the dictionary marks one. The item
//! count in the JSON is the recorded source of truth for the
//! coverage gate. See `docs/roadmap-dict-items.md` (section 11)
//! and `tests/dict/SOURCE.md` for the version note.
//!
//! Usage: `cargo run --example dict_inventory_gen`.

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};

/// The DDL1 dictionary is a valid CIF file. The generator parses it
/// line by line instead of running the crate parser:
///
/// - a `data_<block>` line starts a dictionary block;
/// - a line whose first non-whitespace character is `;` opens or
///   closes a semi-enclosed text field. Text fields hold example
///   data. The generator skips their content.
/// - a line whose first non-whitespace character is `#` is a
///   comment.
/// - a `loop_ _name` section lists the numbered variants of an item
///   family. The generator expands them into individual records.
/// - a `loop_ _enumeration` section lists a closed code set.
///
/// Block classification:
/// - `data_on_this_dictionary` is the dictionary metadata block.
/// - `data_<cat>_[]` with `_category category_overview` is a
///   category overview. It names a category and holds no items.
/// - a block with a `loop_ _name` section is an item family. Each
///   listed variant becomes one item record, inheriting the block
///   type, category, and loop flag.
/// - every other block is a single item, named by its `_name` tag.
///
/// `is_loop` is true when the block is a family block or when its
/// `_list` tag has the value `yes` or `both`.

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dic_path = manifest.join("tests/dict/cif_core.dic");
    let out_path = manifest.join("tests/dict_inventory.json");
    let text = fs::read_to_string(&dic_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", dic_path.display()));
    let lines: Vec<&str> = text.lines().collect();

    let blocks = split_blocks(&lines);
    let mut items: Vec<Value> = Vec::new();
    let mut categories: Vec<String> = Vec::new();
    let mut dict_name: Option<String> = None;
    let mut dict_version: Option<String> = None;
    let mut dict_update: Option<String> = None;
    let mut family_blocks = 0usize;
    let mut single_blocks = 0usize;
    let mut overview_blocks = 0usize;

    for (name, start, end) in &blocks {
        let block_lines: Vec<String> = lines[*start + 1..=*end]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let meta = analyze_block(&block_lines);
        if name == "on_this_dictionary" {
            dict_name = meta.dictionary_name.clone();
            dict_version = meta.dictionary_version.clone();
            dict_update = meta.dictionary_update.clone();
            continue;
        }
        if name.ends_with("[]") {
            overview_blocks += 1;
            let cat = name
                .strip_suffix("_[]")
                .unwrap_or(name)
                .to_string();
            if !categories.contains(&cat) {
                categories.push(cat);
            }
            continue;
        }
        let category = meta
            .category
            .clone()
            .unwrap_or_else(|| panic!("block data_{name} has no _category tag"));
        if !categories.contains(&category) {
            categories.push(category.clone());
        }
        let enumeration = enumeration_json(&meta);
        if !meta.family_variants.is_empty() {
            family_blocks += 1;
            for variant in &meta.family_variants {
                items.push(item_json(
                    variant,
                    &category,
                    meta.type_.as_deref().unwrap_or_default(),
                    true,
                    &enumeration,
                ));
            }
        } else {
            single_blocks += 1;
            let item_name = meta.item_name.clone().unwrap_or_else(|| {
                panic!("block data_{name} is neither a family block nor named by _name")
            });
            let is_loop = matches!(meta.list.as_deref(), Some("yes") | Some("both"));
            items.push(item_json(
                &item_name,
                &category,
                meta.type_.as_deref().unwrap_or_default(),
                is_loop,
                &enumeration,
            ));
        }
    }
    let _ = overview_blocks;

    categories.sort();
    items.sort_by(|a, b| {
        let key_a = (a["category"].as_str().unwrap(), a["name"].as_str().unwrap());
        let key_b = (b["category"].as_str().unwrap(), b["name"].as_str().unwrap());
        key_a.cmp(&key_b)
    });
    let mut seen = std::collections::HashSet::new();
    for item in &items {
        let key = (item["category"].as_str().unwrap(), item["name"].as_str().unwrap());
        if !seen.insert(key) {
            panic!("duplicate inventory item {key:?}");
        }
    }
    let cat_counts: Vec<Value> = categories
        .iter()
        .map(|cat| {
            json!({
                "name": cat,
                "item_count": items.iter().filter(|i| i["category"] == *cat).count(),
            })
        })
        .collect();

    let total = items.len();
    let doc = json!({
        "schema_version": 1,
        "source": {
            "file": "tests/dict/cif_core.dic",
            "url": "https://raw.githubusercontent.com/COMCIFS/DDL1-legacy-dictionaries/main/dictionaries/cif_core.dic",
            "retrieved": "2026-10-08",
            "dictionary_name": dict_name,
            "dictionary_version": dict_version,
            "dictionary_update": dict_update,
        },
        "note": "Recorded from the IUCr-managed DDL1 archive (COMCIFS/DDL1-legacy-dictionaries). The roadmap pins DDL1 v2.4.5 (734 items, 56 categories). When the recorded counts differ, the coverage gate uses the recorded counts.",
        "counts": {
            "total_items": total,
            "categories": categories.len(),
            "family_blocks": family_blocks,
            "family_variants": total - single_blocks,
            "single_items": single_blocks,
        },
        "categories": cat_counts,
        "items": items,
    });
    let rendered = serde_json::to_string_pretty(&doc).expect("serialize inventory");
    fs::write(&out_path, rendered + "\n").unwrap_or_else(|e| panic!("write {}: {e}", out_path.display()));
    println!(
        "wrote {}: {total} items in {} categories ({} family blocks, {} single blocks)",
        out_path.display(),
        categories.len(),
        family_blocks,
        single_blocks
    );
    if total != 734 {
        println!(
            "note: recorded total {total} differs from the roadmap v2.4.5 pin (734); the coverage gate uses the recorded total"
        );
    }
}

/// Split the file into `data_<block>` blocks. A block runs to the
/// next `data_` line or to the end of the file.
fn split_blocks(lines: &[&str]) -> Vec<(String, usize, usize)> {
    let mut out: Vec<(String, usize, usize)> = Vec::new();
    let mut start: Option<(String, usize)> = None;
    for (i, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("data_") else {
            continue;
        };
        if rest.is_empty() || rest.starts_with(char::is_whitespace) {
            continue;
        }
        if let Some((n, s)) = start.take() {
            out.push((n, s, i - 1));
        }
        start = Some((rest.trim().to_string(), i));
    }
    if let Some((n, s)) = start {
        out.push((n, s, lines.len() - 1));
    }
    out
}

#[derive(Debug, Default)]
struct BlockMeta {
    category: Option<String>,
    type_: Option<String>,
    list: Option<String>,
    item_name: Option<String>,
    family_variants: Vec<String>,
    enum_codes: Vec<String>,
    enum_range: Option<String>,
    dictionary_name: Option<String>,
    dictionary_version: Option<String>,
    dictionary_update: Option<String>,
}

/// Extract the item metadata from one dictionary block. Text-field
/// content (example data) is skipped.
fn analyze_block(lines: &[String]) -> BlockMeta {
    let mut meta = BlockMeta::default();
    let mut in_field = false;
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();
        if in_field {
            if t.starts_with(';') {
                in_field = false;
            }
            i += 1;
            continue;
        }
        if t.starts_with(';') {
            in_field = true;
            i += 1;
            continue;
        }
        if t.starts_with('#') {
            i += 1;
            continue;
        }
        if let Some(rest) = t.strip_prefix("loop_") {
            let toks: Vec<&str> = rest.split_whitespace().collect();
            match toks.first().copied() {
                Some("_name") => {
                    collect_quoted(&mut meta.family_variants, &toks[1..]);
                    i += 1;
                    while i < lines.len() {
                        let c = lines[i].trim();
                        if c.starts_with("'_") {
                            let toks: Vec<&str> = c.split_whitespace().collect();
                            collect_quoted(&mut meta.family_variants, &toks);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    continue;
                }
                Some("_enumeration") => {
                    for tok in &toks[1..] {
                        // The code column ends at the detail column
                        // name or at the first quoted detail token.
                        if tok.starts_with('_') || tok.starts_with('\'') {
                            break;
                        }
                        push_enum_code(&mut meta, *tok);
                    }
                    i += 1;
                    while i < lines.len() {
                        let c = lines[i].trim();
                        if c.starts_with('#') {
                            i += 1;
                            continue;
                        }
                        if c.starts_with(';') || c.starts_with("loop_") {
                            break;
                        }
                        if let Some(rest) = c.strip_prefix("_enumeration_detail") {
                            if let Some(code) = rest.split_whitespace().next() {
                                push_enum_code(&mut meta, code);
                            }
                            i += 1;
                            continue;
                        }
                        if c.starts_with('_') {
                            break;
                        }
                        if let Some(code) = c.split_whitespace().next() {
                            push_enum_code(&mut meta, code);
                        }
                        i += 1;
                    }
                    continue;
                }
                _ => {
                    i += 1;
                    continue;
                }
            }
        }
        if let Some((tag, value)) = tag_value(t) {
            let value = unquote(value).unwrap_or_else(|| value.to_string());
            match tag {
                "category" => meta.category = Some(value),
                "type" => meta.type_ = Some(value),
                "list" => meta.list = Some(value),
                "name" => meta.item_name = Some(stripped_item_name(&value)),
                "enumeration_range" => meta.enum_range = Some(value),
                "dictionary_name" => meta.dictionary_name = Some(value),
                "dictionary_version" => meta.dictionary_version = Some(value),
                "dictionary_update" => meta.dictionary_update = Some(value),
                _ => {}
            }
        }
        i += 1;
    }
    meta
}

/// Split a trimmed `_tag value` line into the tag name (without the
/// leading underscore) and the value text.
fn tag_value(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix('_')?;
    let mut parts = rest.splitn(2, char::is_whitespace);
    let tag = parts.next()?;
    if tag.is_empty() {
        return None;
    }
    let value = parts.next()?;
    Some((tag, value.trim()))
}

/// Collect the single-quoted names from a token slice.
fn collect_quoted(out: &mut Vec<String>, toks: &[&str]) {
    for tok in toks {
        if let Some(name) = unquote(tok) {
            out.push(stripped_item_name(&name));
        }
    }
}

/// Push one enumeration code, dropping duplicates. Codes are
/// usually unquoted; a fully quoted token is accepted as well.
fn push_enum_code(meta: &mut BlockMeta, tok: &str) {
    let t = tok.trim();
    if t.is_empty() || t.starts_with('_') {
        return;
    }
    let code = if t.starts_with('\'') && t.ends_with('\'') && t.len() >= 2 {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    };
    if !meta.enum_codes.iter().any(|c| c == &code) {
        meta.enum_codes.push(code);
    }
}

/// Strip matching single quotes from a token.
fn unquote(tok: &str) -> Option<String> {
    let t = tok.trim();
    let inner = t.strip_prefix('\'')?.strip_suffix('\'')?;
    Some(inner.to_string())
}

/// Drop the leading underscore of a dictionary item name.
fn stripped_item_name(name: &str) -> String {
    name.strip_prefix('_').unwrap_or(name).to_string()
}

fn item_json(name: &str, category: &str, r#type: &str, is_loop: bool, enumeration: &Value) -> Value {
    json!({
        "name": name,
        "category": category,
        "type": r#type,
        "is_loop": is_loop,
        "enumeration": enumeration,
    })
}

fn enumeration_json(meta: &BlockMeta) -> Value {
    if !meta.enum_codes.is_empty() {
        return json!({ "kind": "closed", "values": meta.enum_codes });
    }
    if let Some(range) = &meta.enum_range {
        return json!({ "kind": "range", "range": range });
    }
    Value::Null
}
