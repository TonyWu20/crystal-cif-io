//! Regenerates `tests/dict_inventory_shell.json` from
//! `tests/dict/cif_core_shell.dic` (the CIF_CORE shell, DDL 4.2.0).
//!
//! This is the T6 sibling of `dict_inventory_gen.rs` (the DDL1
//! generator). The shell dictionary is a valid CIF 2.0 file: items
//! are `save_<category>.<item>` blocks. Item metadata lives in
//! `_name.category_id`, `_name.object_id`, `_type.container`,
//! `_type.contents`, `_type.dimension`, `_enumeration.range`, and
//! the `_enumeration_set.state` loop. The DDL1 generator cannot read
//! this format, so the shell generator is a sibling example. See
//! `docs/roadmap-dict-items.md` (section 11), `tests/dict/SOURCE.md`,
//! and `docs/codegen.md` (T6 dual-inventory step).
//!
//! Each item record carries:
//! - `name`: the DDL1-style item name without the leading
//!   underscore. It is the first `_alias.definition_id` when the
//!   block records one (single value, or first column of the first
//!   data row of an alias loop). Items without an alias fall back to
//!   `category.object_id`.
//! - `category`: the recorded `_name.category_id` (conventional
//!   case, as written in the dictionary).
//! - `type`: `char` or `numb`, mapped from `_type.contents`
//!   (Text/Word/Code/Iri/Uri/Date/DateTime/Symop/Tag map to
//!   `char`; Real/Integer/Complex map to `numb`). Items without
//!   type metadata resolve the type through `_name.linked_item_id`
//!   (the `_su` uncertainty items link to their base item), and the
//!   imported numbered families (`mat_*`, `vec_*`, `beta_*`,
//!   `U_*`, `B_*`, `Cromer_Mann_*`, `hi_ang_Fox_*`) fall back to
//!   `numb`. `model_site.symop` (a Symop content with no type
//!   metadata) falls back to `char`.
//! - `is_loop`: true when `_type.container` is `Loop`. The shell
//!   records no `Loop` containers, so every record is `false`.
//! - `matrix_shape`: the `_type.dimension` string when
//!   `_type.container` is `Matrix` (for example `[3,3]`).
//! - `enumeration`: the closed code set recorded by an
//!   `_enumeration_set.state` loop, or the `_enumeration.range`
//!   constraint, matching the DDL1 generator's shape.
//!
//! The header records the source and version. `counts` records the
//! actual totals (the roadmap expects 1139 items in 91
//! categories) and the T6 split against the standard inventory:
//! `t6_items` (names absent from `tests/dict_inventory.json`),
//! `t6_su_items` (the `_su` items skipped per decision D2), and
//! `t6_implemented` (the rest, the T6 implementation set).
//!
//! Usage: `cargo run --example dict_inventory_shell_gen`.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};

/// Split the file into `save_<block>` blocks. A block runs to the
/// next `save_` line or to the end of the file. A bare `save_` line
/// is an empty save-frame marker; it ends the current block.
fn split_blocks(lines: &[&str]) -> Vec<(String, usize, usize)> {
    let mut out: Vec<(String, usize, usize)> = Vec::new();
    let mut start: Option<(String, usize)> = None;
    for (i, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("save_") else {
            continue;
        };
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

/// One parsed block: the single-value tags, and the loops as
/// (header tags, data rows). Text-field content is skipped.
struct BlockData {
    tags: BTreeMap<String, String>,
    loops: Vec<(Vec<String>, Vec<Vec<String>>)>,
}

fn parse_block(lines: &[String]) -> BlockData {
    let mut tags: BTreeMap<String, String> = BTreeMap::new();
    let mut loops: Vec<(Vec<String>, Vec<Vec<String>>)> = Vec::new();
    let mut in_field = false;
    let mut in_loop = false;
    let mut header: Vec<String> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();
    // True when the physical line just handled pushed a one-cell row.
    // It guards the detail-column merge (a detail line merges only
    // when it sits directly under its state line).
    let mut row_just_pushed = false;

    for raw in lines {
        let line = raw.trim_end();
        let t = line.trim();
        if in_field {
            row_just_pushed = false;
            if t == ";" {
                in_field = false;
            }
            continue;
        }
        if t.starts_with(';') {
            in_field = true;
            continue;
        }
        if in_loop {
            // The loop ends at a new `loop_`, a comment, a save
            // marker, or a block-level tag line (4-space indent).
            let ends = t.starts_with("loop_")
                || t.starts_with('#')
                || t.starts_with("save_")
                || (line.len() > 4 && line.starts_with("    _"));
            if ends {
                loops.push((std::mem::take(&mut header), std::mem::take(&mut rows)));
                in_loop = false;
                row_just_pushed = false;
            } else {
                if t.starts_with('_') {
                    // Column header line. One line may hold several
                    // column tags.
                    for tok in t.split_whitespace() {
                        header.push(unquote(tok));
                    }
                    row_just_pushed = false;
                } else {
                    // A data row: split into cells. Quoted cells keep
                    // interior blanks. A one-cell quoted line that
                    // follows a one-cell row is the detail column of
                    // that row (the COMCIFS two-column layout): merge
                    // it in so the detail is not read as a new row.
                    let cells = split_cells(t);
                    if !cells.is_empty() {
                        let merge = cells.len() == 1
                            && first_cell_quoted(t)
                            && header.len() >= 2
                            && row_just_pushed
                            && rows.last().is_some_and(|r| r.len() == 1);
                        if merge {
                            if let (Some(last), Some(cell)) =
                                (rows.last_mut(), cells.into_iter().next())
                            {
                                last.push(cell);
                            }
                        } else {
                            rows.push(cells);
                        }
                        row_just_pushed = rows.last().is_some_and(|r| r.len() == 1);
                    }
                }
                continue;
            }
        }
        if t.starts_with("loop_") {
            in_loop = true;
            for tok in t[5..].split_whitespace() {
                if tok.starts_with('_') {
                    header.push(unquote(tok));
                }
            }
            continue;
        }
        if t.starts_with('#') || t.is_empty() {
            continue;
        }
        // A block-level tag line: `_tag.value   value`.
        if let Some((key, value)) = tag_value(line) {
            tags.insert(key.to_string(), unquote(value));
        }
    }
    if in_loop {
        loops.push((header, rows));
    }
    BlockData { tags, loops }
}

/// Split a `_tag value` line into the tag key (without the leading
/// underscore) and the value text.
fn tag_value(line: &str) -> Option<(&str, &str)> {
    let rest = line.trim_start();
    let rest = rest.strip_prefix('_')?;
    let mut parts = rest.splitn(2, char::is_whitespace);
    let key = parts.next()?;
    let key_ok = key.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
        && key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_');
    if !key_ok {
        return None;
    }
    let value = parts.next()?.trim();
    if value.is_empty() {
        return None;
    }
    Some((key, value))
}

/// Drop one layer of matching single quotes.
fn unquote(tok: &str) -> String {
    let t = tok.trim();
    t.strip_prefix('\'')
        .and_then(|inner| inner.strip_suffix('\''))
        .map(|s| s.to_string())
        .unwrap_or_else(|| t.to_string())
}

/// Split a loop row into cells. A quoted cell keeps its interior
/// blanks: `'Space group tables'` is one cell, not three tokens.
/// The quotes are dropped from the cell. An unterminated quote
/// leaves the rest of the line as plain tokens.
fn split_cells(line: &str) -> Vec<String> {
    let mut cells: Vec<String> = Vec::new();
    let mut rest = line;
    while let Some(pos) = rest.find(|c: char| c == '\'' || c == '"') {
        let before = &rest[..pos];
        for tok in before.split_whitespace() {
            cells.push(tok.to_string());
        }
        let q = rest.as_bytes()[pos] as char;
        let after = &rest[pos + 1..];
        match after.find(q) {
            Some(end) => {
                cells.push(after[..end].to_string());
                rest = &after[end + 1..];
            }
            None => {
                for tok in after.split_whitespace() {
                    cells.push(tok.to_string());
                }
                rest = "";
            }
        }
    }
    for tok in rest.split_whitespace() {
        cells.push(tok.to_string());
    }
    cells
}

/// True when the first cell of `line` is a quoted cell.
fn first_cell_quoted(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('\'') || t.starts_with('"')
}

/// The recorded type of the item from its `_type.contents`, when
/// the metadata is present.
fn contents_type(contents: Option<&str>) -> Option<&'static str> {
    match contents {
        Some("Real") | Some("Integer") | Some("Complex") => Some("numb"),
        Some("Text") | Some("Word") | Some("Code") | Some("Iri") | Some("Uri") | Some("Date")
        | Some("DateTime") | Some("Symop") | Some("Tag") => Some("char"),
        _ => None,
    }
}

/// The type fallback for imported items that carry no type
/// metadata. The numbered anisotropic and scattering families are
/// numeric; `model_site.symop` is a Symop text.
fn imported_type(object: &str) -> &'static str {
    if object == "symop" {
        "char"
    } else {
        "numb"
    }
}

struct ItemRec {
    definition_id: String,
    category: String,
    object: String,
    name: String,
    container: Option<String>,
    contents: Option<String>,
    dimension: Option<String>,
    enum_range: Option<String>,
    enum_codes: Vec<String>,
    linked: Option<String>,
    item_type: Option<String>,
}

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dic_path = manifest.join("tests/dict/cif_core_shell.dic");
    let out_path = manifest.join("tests/dict_inventory_shell.json");
    let text = fs::read_to_string(&dic_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", dic_path.display()));
    let lines: Vec<&str> = text.lines().collect();

    let blocks = split_blocks(&lines);

    let mut items: Vec<ItemRec> = Vec::new();
    let dict_name: Option<String>;
    let dict_version: Option<String>;
    let dict_update: Option<String>;

    // The file-level `data_CIF_CORE` block (before the first
    // `save_` line) holds the dictionary metadata.
    {
        let mut head: Vec<String> = Vec::new();
        for line in &lines {
            if line.starts_with("save_") {
                break;
            }
            head.push(line.to_string());
        }
        let data = parse_block(&head);
        dict_name = data.tags.get("dictionary.title").cloned();
        dict_version = data.tags.get("dictionary.version").cloned();
        dict_update = data.tags.get("dictionary.date").cloned();
    }

    for (name, start, end) in &blocks {
        let block_lines: Vec<String> = lines[*start + 1..=*end]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let data = parse_block(&block_lines);
        // A bare `save_` marker (empty block name) ends a save frame.
        if name.is_empty() {
            continue;
        }
        // Category blocks record `_definition.scope Category`. They
        // name a category and hold no items.
        if data.tags.get("definition.scope").is_some() {
            continue;
        }
        let category = data
            .tags
            .get("name.category_id")
            .unwrap_or_else(|| panic!("block save_{name} has no _name.category_id"));
        let object = data
            .tags
            .get("name.object_id")
            .unwrap_or_else(|| panic!("block save_{name} has no _name.object_id"));
        let definition_id = data.tags.get("definition.id").cloned();
        let mut alias = data.tags.get("alias.definition_id").cloned();
        if alias.is_none() {
            for (header, rows) in &data.loops {
                if header.iter().any(|h| h == "_alias.definition_id") {
                    if let Some(first) = rows.first().and_then(|r| r.first()) {
                        alias = Some(first.clone());
                    }
                    break;
                }
            }
        }
        let item_name = match alias {
            Some(a) => a.trim_start_matches('_').to_string(),
            // No alias: the item is shell-native. The crate tag is
            // the flat `category_object` spelling, matching the DDL1
            // style of the standard inventory.
            None => format!("{category}_{object}"),
        };
        let mut codes: Vec<String> = Vec::new();
        for (header, rows) in &data.loops {
            if header.iter().any(|h| h == "_enumeration_set.state") {
                for row in rows {
                    if let Some(code) = row.first() {
                        if !codes.iter().any(|c| c == code) {
                            codes.push(code.clone());
                        }
                    }
                }
            }
        }
        items.push(ItemRec {
            definition_id: definition_id.unwrap_or_else(|| format!("_{category}.{object}")),
            category: category.clone(),
            object: object.clone(),
            name: item_name,
            container: data.tags.get("type.container").cloned(),
            contents: data.tags.get("type.contents").cloned(),
            dimension: data.tags.get("type.dimension").cloned(),
            enum_range: data.tags.get("enumeration.range").cloned(),
            enum_codes: codes,
            linked: data.tags.get("name.linked_item_id").cloned(),
            item_type: None,
        });
    }

    // Resolve the type of every item. Items without type metadata
    // follow the linked item; imported families fall back to the
    // name-pattern rule.
    // The resolved type of every item, keyed by definition id: its
    // own `_type.contents`, else the imported-family fallback.
    let def_types: BTreeMap<String, &'static str> = items
        .iter()
        .map(|i| {
            let ty =
                contents_type(i.contents.as_deref()).unwrap_or_else(|| imported_type(&i.object));
            (i.definition_id.to_ascii_lowercase(), ty)
        })
        .collect();
    for item in &mut items {
        let direct = contents_type(item.contents.as_deref());
        let Some(ty) = direct.or_else(|| {
            item.linked
                .as_deref()
                .and_then(|l| def_types.get(&l.to_ascii_lowercase()).copied())
        }) else {
            let fallback = imported_type(&item.object);
            item.item_type = Some(fallback.to_string());
            continue;
        };
        item.item_type = Some(ty.to_string());
    }

    // The T6 split against the standard inventory: items whose name
    // is absent from `tests/dict_inventory.json`.
    let std_path = manifest.join("tests/dict_inventory.json");
    let std_raw = fs::read_to_string(&std_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", std_path.display()));
    let std_doc: Value = serde_json::from_str(&std_raw).expect("standard inventory JSON");
    let std_names: std::collections::BTreeSet<String> = std_doc["items"]
        .as_array()
        .expect("standard inventory items array")
        .iter()
        .map(|i| i["name"].as_str().expect("item name").to_ascii_lowercase())
        .collect();
    let t6_items = items
        .iter()
        .filter(|i| !std_names.contains(&i.name.to_ascii_lowercase()))
        .count();
    let t6_su_items = items
        .iter()
        .filter(|i| {
            !std_names.contains(&i.name.to_ascii_lowercase())
                && i.name.to_ascii_lowercase().ends_with("_su")
        })
        .count();

    let mut categories: Vec<String> = items.iter().map(|i| i.category.clone()).collect();
    categories.sort();
    categories.dedup();

    let total = items.len();
    let doc = json!({
        "schema_version": 1,
        "source": {
            "file": "tests/dict/cif_core_shell.dic",
            "url": "https://raw.githubusercontent.com/COMCIFS/cif_core/main/cif_core.dic",
            "retrieved": "2026-10-09",
            "dictionary_name": dict_name,
            "dictionary_version": dict_version,
            "dictionary_update": dict_update,
            "ddl_conformance": "4.2.0",
        },
        "note": "Recorded from the IUCr core shell (COMCIFS/cif_core, DDL 4.2.0). \
                 The T6 set is the items whose name is absent from \
                 tests/dict_inventory.json. The _su uncertainty items are skipped \
                 per decision D2; the uncertainty stays inside Numeric.",
        "counts": {
            "total_items": total,
            "categories": categories.len(),
            "t6_items": t6_items,
            "t6_su_items": t6_su_items,
            "t6_implemented": t6_items - t6_su_items,
        },
        "categories": json!(categories
            .iter()
            .map(|cat| {
                json!({
                    "name": cat,
                    "item_count": items
                        .iter()
                        .filter(|i| i.category == *cat)
                        .count(),
                })
            })
            .collect::<Vec<Value>>()),
        "items": json!(items
            .iter()
            .map(item_json)
            .collect::<Vec<Value>>()),
    });

    let mut rendered_items: Vec<Value> = doc["items"].as_array().cloned().unwrap();
    rendered_items.sort_by(|a, b| {
        let key_a = (
            a["category"].as_str().unwrap().to_ascii_lowercase(),
            a["name"].as_str().unwrap().to_ascii_lowercase(),
        );
        let key_b = (
            b["category"].as_str().unwrap().to_ascii_lowercase(),
            b["name"].as_str().unwrap().to_ascii_lowercase(),
        );
        key_a.cmp(&key_b)
    });
    let mut dup_guard: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for item in &rendered_items {
        let key = format!(
            "{}/{}",
            item["category"].as_str().unwrap(),
            item["name"].as_str().unwrap()
        );
        assert!(!dup_guard.contains(&key), "duplicate inventory item {key}");
        dup_guard.insert(key);
    }

    let mut doc = doc;
    doc["items"] = Value::Array(rendered_items);
    let rendered = serde_json::to_string_pretty(&doc).expect("serialize inventory");
    fs::write(&out_path, rendered + "\n")
        .unwrap_or_else(|e| panic!("write {}: {e}", out_path.display()));
    println!(
        "wrote {}: {total} items in {} categories (T6 split: {t6_items} items, \
         {t6_su_items} _su skipped, {} to implement)",
        out_path.display(),
        categories.len(),
        t6_items - t6_su_items
    );
    if total != 1139 || categories.len() != 91 {
        println!(
            "note: recorded {total} items in {} categories differs from the \
             roadmap pin (1139 items, 91 categories)",
            categories.len()
        );
    }
}

fn item_json(item: &ItemRec) -> Value {
    let mut rec = json!({
        "name": item.name,
        "category": item.category,
        "type": item.item_type,
        "is_loop": item.container.as_deref() == Some("Loop"),
        "enumeration": enumeration_value(&item),
    });
    if item.container.as_deref() == Some("Matrix") {
        let dim = item
            .dimension
            .clone()
            .unwrap_or_else(|| panic!("matrix item {} has no _type.dimension", item.name));
        rec["matrix_shape"] = json!(dim);
    }
    rec
}

fn enumeration_value(item: &ItemRec) -> Value {
    if !item.enum_codes.is_empty() {
        return json!({ "kind": "closed", "values": item.enum_codes });
    }
    if let Some(range) = &item.enum_range {
        return json!({ "kind": "range", "range": range });
    }
    Value::Null
}
