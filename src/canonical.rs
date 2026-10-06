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
//! - `char_string`             `{"style": "unquoted"|"single_quoted"|"double_quoted",
//!   "text": "<content between delimiters, as written>"}`
//! - `text_field`             `{"text": "<lines joined with \n>"}`
//!
//! Loops are encoded as rows (transposed from the column-oriented IR).
//! Tags are stored without the leading underscore. Comments and other
//! whitespace are elided: the spec states that no essential
//! machine-readable content is conveyed by comments (para. 45).

use std::collections::{BTreeSet, HashSet};
use std::convert::AsRef;

use serde_json::{json, Map, Value as Json};

use crate::{
    CharString, CifDocument, DataBlockMember, DataItems, LoopColumns, Numeric, Number, Value,
};

/// Serialize a parsed document into the canonical JSON shape.
pub fn document_json(doc: &CifDocument, fixture: &str) -> Json {
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
        "schema_version": 1,
        "fixture": fixture,
        "blocks": blocks,
    })
}

fn member_json(member: &DataBlockMember) -> Json {
    match member {
        DataBlockMember::DataItems(items) => items_json(items),
        DataBlockMember::SaveFrame(frame) => {
            json!({
                "kind": "save_frame",
                "name": frame.heading().as_ref(),
                "members": frame.data_items().iter().map(items_json).collect::<Vec<_>>(),
            })
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
    let tags = cols.iter().map(|c| json!(c.tag().as_str())).collect::<Vec<_>>();
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
    DuplicateBlockName(String),
    DuplicateTagInBlock {
        block: String,
        tag: String,
    },
    DuplicateSaveFrameCode {
        block: String,
        code: String,
    },
    RaggedLoop {
        block: String,
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

    pub fn to_string(&self) -> String {
        match self {
            Self::DuplicateBlockName(name) => format!("duplicate data block name {name}"),
            Self::DuplicateTagInBlock { block, tag } => {
                format!("duplicate tag {tag} in block or save frame {block}")
            }
            Self::DuplicateSaveFrameCode { block, code } => {
                format!("duplicate save frame code {code} in block {block}")
            }
            Self::RaggedLoop { block, tags } => {
                format!("ragged loop {tags:?} in block {block}")
            }
        }
    }
}

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
                    let mut frame_tags: HashSet<String> = HashSet::new();
                    for item in frame.data_items() {
                        match item {
                            DataItems::SingleValue(single) => {
                                record_tag(&mut out, &code, single.tag().as_str(), &mut frame_tags);
                            }
                            DataItems::MultiValues(lc) => {
                                check_loop(&mut out, &code, lc, "save frame");
                                record_tags(&mut out, &code, lc, &mut frame_tags);
                            }
                        }
                    }
                }
            }
        }
    }
    out
}

fn record_tag(
    out: &mut Vec<Violation>,
    scope: &str,
    tag: &str,
    seen: &mut HashSet<String>,
) {
    let key = tag.to_lowercase();
    if !seen.insert(key.clone()) {
        out.push(Violation::DuplicateTagInBlock {
            block: scope.to_string(),
            tag: key,
        });
    }
}

fn record_tags(out: &mut Vec<Violation>, scope: &str, lc: &LoopColumns, seen: &mut HashSet<String>) {
    for col in lc.columns() {
        record_tag(out, scope, col.tag().as_str(), seen);
    }
}

fn check_loop(out: &mut Vec<Violation>, scope: &str, lc: &LoopColumns, _where: &str) {
    let lens: BTreeSet<usize> = lc.columns().iter().map(|c| c.values().len()).collect();
    if lens.len() > 1 {
        let tags = lc.columns().iter().map(|c| c.tag().as_str().to_string()).collect();
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
                && e.iter().all(|(k, ev)| a.get(k).map(|av| trees_equal(ev, av)).unwrap_or(false))
        }
        (Json::Array(e), Json::Array(a)) => {
            e.len() == a.len() && e.iter().zip(a.iter()).all(|(ev, av)| trees_equal(ev, av))
        }
        (e, a) => e == a,
    }
}
