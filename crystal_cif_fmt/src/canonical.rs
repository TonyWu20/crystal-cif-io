//! Encode the crate's parsed CIF IR into the parser-agnostic
//! "canonical CIF IR" JSON schema used by the groundtruth.
//!
//! The schema (see `docs/ground-truth.md`) is deliberately independent of
//! the parser implementation. When the crate moves from the current
//! winnow-based IR to a chumsky-based IR, this module is rewritten to
//! serialize the new IR into the same JSON shape.
//!
//! Value encoding:
//! - `inapplicable`            the `.` value
//! - `unknown`                 the `?` value
//! - `numeric`                 `{"type": "integer"|"float", "value": "<shortest
//!   form of the parsed value>", "std_uncertainty": "<digits>"|null}`
//! - `char_string`             `{"style": "unquoted"|"single_quoted"|"double_quoted"
//!                                |"triple_single_quoted"|"triple_double_quoted",
//!   "text": "<content between delimiters, as written>"}`
//! - `text_field`             `{"text": "<lines joined with \n>"}`
//! - `list`                    `{"values": [ ... value objects ... ]}`;
//!   nested lists hold nested value objects. Only in schema version 2
//!   (the CIF 2.0 dialect).
//! - `table`                   `{"pairs": [ {"key": <value object>,
//!   "value": <value object>}, ... ]}`; an ordered array keeps the
//!   key order. Only in schema version 2. A frame object gains a
//!   `frames` array only when it holds nested frames (CIF 2.0).
//!
//! Schema versions: `schema_version` 1 is the historical shape (CIF
//! 1.1 only). Schema version 2 adds the `list` and `table` value
//! kinds and the triple-quoted `char_string` styles for the CIF 2.0
//! dialect. The harness accepts both.
//!
//! Loops are encoded as rows (transposed from the column-oriented IR).
//! Tags are stored without the leading underscore. Comments and other
//! whitespace are elided: the spec states that no essential
//! machine-readable content is conveyed by comments (para. 45).

use std::collections::{BTreeSet, HashSet};
use std::convert::AsRef;

use serde_json::{json, Map, Value as Json};

use crate::{
    CharString, CifDocument, DataBlockMember, DataItems, LoopColumns, Number, Numeric, SaveFrame,
    Value,
};

/// Serialize a parsed document into the canonical JSON shape.
///
/// `schema_version` is the schema generation of the expected file:
/// 1 for CIF 1.1 fixtures, 2 for CIF 2.0 fixtures (the `list` value
/// kind). See `docs/ground-truth.md`.
pub fn document_json(doc: &CifDocument, fixture: &str, schema_version: u32) -> Json {
    let blocks = doc
        .data_blocks()
        .map(|blocks| {
            blocks
                .iter()
                .map(|b| {
                    json!({
                        "name": b.heading(),
                        "members": b.members().iter().map(member_json).collect::<Vec<_>>(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    json!({
        "schema_version": schema_version,
        "fixture": fixture,
        "blocks": blocks,
    })
}

fn member_json(member: &DataBlockMember) -> Json {
    match member {
        DataBlockMember::DataItems(items) => items_json(items),
        DataBlockMember::SaveFrame(frame) => {
            // The `frames` key appears only when the frame holds
            // nested frames (CIF 2.0, roadmap section 14). Empty
            // frames keep the 1.1 shape, so the iucr and cod pins
            // stay byte-identical.
            let mut object = serde_json::Map::new();
            object.insert("kind".to_string(), json!("save_frame"));
            object.insert("name".to_string(), json!(frame.heading().as_ref()));
            object.insert(
                "members".to_string(),
                json!(frame
                    .data_items()
                    .iter()
                    .map(items_json)
                    .collect::<Vec<_>>()),
            );
            if !frame.frames().is_empty() {
                object.insert(
                    "frames".to_string(),
                    json!(frame
                        .frames()
                        .iter()
                        .map(|child| member_json(&DataBlockMember::SaveFrame(child.clone())))
                        .collect::<Vec<_>>()),
                );
            }
            Json::Object(object)
        }
    }
}

fn items_json(items: &DataItems) -> Json {
    match items {
        DataItems::SingleValue(item) => json!({
            "kind": "item",
            "tag": item.tag().as_str(),
            "value": value_json(item.value()),
        }),
        DataItems::MultiValues(columns) => loop_json(columns),
    }
}

fn loop_json(columns: &LoopColumns) -> Json {
    let cols = columns.columns();
    let tags = cols
        .iter()
        .map(|c| json!(c.tag().as_str()))
        .collect::<Vec<_>>();
    let row_len = cols.iter().map(|c| c.values().len()).max().unwrap_or(0);
    let rows = (0..row_len)
        .map(|row| {
            cols.iter()
                .map(|col| col.values().get(row).map(value_json).unwrap_or(Json::Null))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    json!({
        "kind": "loop",
        "tags": tags,
        "rows": rows,
    })
}

fn value_json(value: &Value) -> Json {
    match value {
        Value::Inapplicable => json!({
            "kind": "inapplicable",
        }),
        Value::Unknown => json!({
            "kind": "unknown",
        }),
        Value::Numeric(n) => numeric_json(n),
        Value::CharString(cs) => {
            let (style, text) = match cs {
                CharString::Unquoted(u) => ("unquoted", u.as_ref().to_string()),
                CharString::SingleQuoted(s) => ("single_quoted", s.as_ref().to_string()),
                CharString::DoubleQuoted(d) => ("double_quoted", d.as_ref().to_string()),
                CharString::TripleSingleQuoted(t) => {
                    ("triple_single_quoted", t.as_ref().to_string())
                }
                CharString::TripleDoubleQuoted(t) => {
                    ("triple_double_quoted", t.as_ref().to_string())
                }
            };
            json!({
                "kind": "char_string",
                "style": style,
                "text": text,
            })
        }
        Value::TextField(tf) => json!({
            "kind": "text_field",
            "text": tf.raw_string(),
        }),
        Value::List(values) => json!({
            "kind": "list",
            "values": values.iter().map(value_json).collect::<Vec<_>>(),
        }),
        Value::Table(pairs) => json!({
            "kind": "table",
            "pairs": pairs
                .iter()
                .map(|(key, value)| {
                    json!({
                        "key": value_json(key),
                        "value": value_json(value),
                    })
                })
                .collect::<Vec<_>>(),
        }),
    }
}

fn numeric_json(n: &Numeric) -> Json {
    let (type_name, value) = match n.number() {
        Number::Integer(i) => ("integer", i.0.to_string()),
        Number::Float(f) => ("float", f.0.to_string()),
    };
    let mut m = Map::new();
    m.insert("kind".to_string(), json!("numeric"));
    m.insert("type".to_string(), json!(type_name));
    m.insert("value".to_string(), json!(value));
    let uncertainty = match n.std_uncertainty() {
        Some(u) => json!(u.0.to_string()),
        None => Json::Null,
    };
    m.insert("std_uncertainty".to_string(), uncertainty);
    Json::Object(m)
}

/// Spec-level (para. 6, 7, 31) validation of a fully parsed document.
/// The parser is syntactic; these semantic checks close the gap between
/// "parses" and "is a valid CIF".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    /// Two data blocks carry the same name.
    DuplicateBlockName(String),
    /// One block or save frame carries a tag twice.
    DuplicateTagInBlock {
        /// The block or frame name.
        block: String,
        /// The tag, without the leading underscore.
        tag: String,
    },
    /// One block carries two save frames with the same code.
    DuplicateSaveFrameCode {
        /// The block name.
        block: String,
        /// The frame code.
        code: String,
    },
    /// A loop column holds more values than its rows.
    RaggedLoop {
        /// The block name.
        block: String,
        /// The loop tags, in loop order.
        tags: Vec<String>,
    },
}

impl Violation {
    /// Stable machine-readable code, used by the negative fixtures.
    pub fn code(&self) -> &'static str {
        match self {
            Self::DuplicateBlockName(_) => "duplicate_block_name",
            Self::DuplicateTagInBlock { .. } => "duplicate_tag_in_block",
            Self::DuplicateSaveFrameCode { .. } => "duplicate_save_frame_code",
            Self::RaggedLoop { .. } => "ragged_loop",
        }
    }
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateBlockName(name) => {
                write!(f, "duplicate data block name {name}")
            }
            Self::DuplicateTagInBlock { block, tag } => {
                write!(f, "duplicate tag {tag} in block or save frame {block}")
            }
            Self::DuplicateSaveFrameCode { block, code } => {
                write!(f, "duplicate save frame code {code} in block {block}")
            }
            Self::RaggedLoop { block, tags } => {
                write!(f, "ragged loop {tags:?} in block {block}")
            }
        }
    }
}

/// Run the spec-level checks on a parsed document. The vector
/// holds one violation per finding; an empty vector means the
/// document is spec-valid.
pub fn validate(doc: &CifDocument) -> Vec<Violation> {
    let mut out: Vec<Violation> = Vec::new();
    let Some(blocks) = doc.data_blocks() else {
        return out;
    };
    let mut seen_block_names: HashSet<String> = HashSet::new();
    for block in blocks {
        let block_name = block.heading().to_lowercase();
        if !seen_block_names.insert(block_name.clone()) {
            out.push(Violation::DuplicateBlockName(block_name.clone()));
        }
        // Block-level tag scope: block-level items and loops only.
        // Save frame contents have their own scope (spec para. 7).
        let mut seen_tags: HashSet<String> = HashSet::new();
        let mut seen_frame_codes: HashSet<String> = HashSet::new();
        for member in block.members() {
            match member {
                DataBlockMember::DataItems(DataItems::SingleValue(item)) => {
                    record_tag(&mut out, &block_name, item.tag().as_str(), &mut seen_tags);
                }
                DataBlockMember::DataItems(DataItems::MultiValues(lc)) => {
                    check_loop(&mut out, &block_name, lc, "block");
                    record_tags(&mut out, &block_name, lc, &mut seen_tags);
                }
                DataBlockMember::SaveFrame(frame) => {
                    let code = frame.heading().as_ref().to_lowercase();
                    if !seen_frame_codes.insert(code.clone()) {
                        out.push(Violation::DuplicateSaveFrameCode {
                            block: block_name.clone(),
                            code: code.clone(),
                        });
                    }
                    validate_frame_contents(&mut out, &code, frame);
                }
            }
        }
    }
    out
}

/// The tag scope and frame-code scope of one save frame's direct
/// content, then the same for each nested frame (CIF 2.0, roadmap
/// section 14). Frame codes must be unique among their siblings in
/// one container; the container is the block at the top level and
/// the parent frame below it.
fn validate_frame_contents(out: &mut Vec<Violation>, scope: &str, frame: &SaveFrame) {
    let mut frame_tags: HashSet<String> = HashSet::new();
    for item in frame.data_items() {
        match item {
            DataItems::SingleValue(single) => {
                record_tag(out, scope, single.tag().as_str(), &mut frame_tags);
            }
            DataItems::MultiValues(lc) => {
                check_loop(out, scope, lc, "save frame");
                record_tags(out, scope, lc, &mut frame_tags);
            }
        }
    }
    let mut seen_frame_codes: HashSet<String> = HashSet::new();
    for child in frame.frames() {
        let child_code = child.heading().as_ref().to_lowercase();
        if !seen_frame_codes.insert(child_code.clone()) {
            out.push(Violation::DuplicateSaveFrameCode {
                block: scope.to_string(),
                code: child_code.clone(),
            });
        }
        validate_frame_contents(out, &child_code, child);
    }
}

fn record_tag(out: &mut Vec<Violation>, scope: &str, tag: &str, seen: &mut HashSet<String>) {
    let key = tag.to_lowercase();
    if !seen.insert(key.clone()) {
        out.push(Violation::DuplicateTagInBlock {
            block: scope.to_string(),
            tag: key,
        });
    }
}

fn record_tags(
    out: &mut Vec<Violation>,
    scope: &str,
    lc: &LoopColumns,
    seen: &mut HashSet<String>,
) {
    for col in lc.columns() {
        record_tag(out, scope, col.tag().as_str(), seen);
    }
}

fn check_loop(out: &mut Vec<Violation>, scope: &str, lc: &LoopColumns, _where: &str) {
    let lens: BTreeSet<usize> = lc.columns().iter().map(|c| c.values().len()).collect();
    if lens.len() > 1 {
        let tags = lc
            .columns()
            .iter()
            .map(|c| c.tag().as_str().to_string())
            .collect();
        out.push(Violation::RaggedLoop {
            block: scope.to_string(),
            tags,
        });
    }
}

/// Compare two canonical JSON trees. Numeric values compare their
/// string forms exactly (the encoder emits the shortest form of the
/// parsed value, so equality is deterministic).
pub fn trees_equal(expected: &Json, actual: &Json) -> bool {
    match (expected, actual) {
        (Json::Object(e), Json::Object(a)) => {
            e.len() == a.len()
                && e.iter()
                    .all(|(k, ev)| a.get(k).map(|av| trees_equal(ev, av)).unwrap_or(false))
        }
        (Json::Array(e), Json::Array(a)) => {
            e.len() == a.len() && e.iter().zip(a.iter()).all(|(ev, av)| trees_equal(ev, av))
        }
        (e, a) => e == a,
    }
}
