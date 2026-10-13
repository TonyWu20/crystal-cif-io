//! Lexical and structural feature coverage guard (test axis 2).
//!
//! Each feature in the table below must be present in at least one
//! corpus file. The guard fails when a corpus file that is the only
//! evidence for a feature is removed. The evidence is printed so a
//! reviewer can see which file pins which feature.
//!
//! The feature table is the contract. The corpus files are the
//! evidence. Extend the table when a new value form is added to
//! the parser.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

fn corpus_files() -> Vec<(String, String)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut out: Vec<(String, String)> = Vec::new();
    for dir in [
        "tests/real_data/iucr",
        "tests/real_data/cod",
        "tests/real_data/cod_sg",
        "tests/real_data/cif_api",
        "tests/fixtures",
    ] {
        for entry in
            fs::read_dir(root.join(dir)).unwrap_or_else(|e| panic!("{dir}: {e}"))
        {
            let path = entry.unwrap().path();
            let Some(name) = path.file_name() else {
                continue;
            };
            let name = name.to_string_lossy().to_string();
            if !name.ends_with(".cif") || name.starts_with("neg_") {
                continue;
            }
            let content = fs::read_to_string(&path).unwrap();
            out.push((format!("{dir}/{name}"), content));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Does the first value token of the line look like a symmetry
/// code with an index, such as `2_767` or `x_1`?
fn has_symmetry_code_token(content: &str) -> bool {
    content.lines().any(|line| {
        line.trim_start()
            .split_whitespace()
            .next()
            .map_or(false, |token| {
                let Some(pos) = token.find('_') else {
                    return false;
                };
                let prefix = &token[..pos];
                let suffix = &token[pos + 1..];
                !prefix.is_empty()
                    && prefix.chars().all(|c| c.is_ascii_alphanumeric())
                    && !suffix.is_empty()
                    && suffix.chars().all(|c| c.is_ascii_digit())
            })
    })
}

/// A hash inside a quoted string value. The quote opens a value
/// token (line start or after a space) and closes after the hash.
fn has_hash_in_quoted_string(content: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') {
            return false;
        }
        let start = trimmed.find(|c| c == '\'' || c == '"');
        let Some(start) = start else {
            return false;
        };
        let preceded =
            start == 0 || trimmed.as_bytes()[start - 1].is_ascii_whitespace();
        if !preceded {
            return false;
        }
        let quote = trimmed.as_bytes()[start] as char;
        let rest = &trimmed[start..];
        let Some(hash) = rest.find('#') else {
            return false;
        };
        rest[hash..].contains(quote)
    })
}

struct Feature {
    name: &'static str,
    check: fn(&str) -> bool,
}

const FEATURES: &[Feature] = &[
    Feature {
        name: "unknown value '?'",
        check: |c| c.lines().any(|l| l.trim() == "?"),
    },
    Feature {
        name: "inapplicable value '.'",
        check: |c| c.lines().any(|l| {
            !l.starts_with('#') && l.trim_end().split_whitespace().last() == Some(".")
        }),
    },
    Feature {
        name: "sub/superscript ~",
        check: |c| c.lines().any(|l| !l.starts_with('#') && l.contains('~')),
    },
    Feature {
        name: "backslash letter escape",
        check: |c| c.lines().any(|l| !l.starts_with('#') && l.contains('\\')),
    },
    Feature {
        name: "symmetry code with index",
        check: has_symmetry_code_token,
    },
    Feature {
        name: "semicolon text field",
        check: |c| c.lines().any(|l| l.trim() == ";"),
    },
    Feature {
        name: "comment line",
        check: |c| c.lines().any(|l| l.starts_with('#')),
    },
    Feature {
        name: "hash inside quoted string",
        check: has_hash_in_quoted_string,
    },
    Feature {
        name: "CRLF line endings",
        check: |c| c.contains("\r\n"),
    },
    Feature {
        name: "non-ASCII (UTF-8) content",
        check: |c| c.chars().any(|ch| ch > '\u{7e}'),
    },
    Feature {
        name: "multiple data blocks",
        check: |c| c.lines().filter(|l| l.starts_with("data_")).count() >= 2,
    },
    Feature {
        name: "save frame",
        check: |c| c.lines().any(|l| l.starts_with("save_")),
    },
    Feature {
        name: "large file (100 KB and up)",
        check: |c| c.len() >= 100_000,
    },
];

#[test]
fn feature_coverage_guard() {
    let files = corpus_files();
    assert!(!files.is_empty(), "corpus is empty");

    let mut missing: Vec<String> = Vec::new();
    let mut evidence: BTreeMap<&str, Vec<String>> = BTreeMap::new();

    for feature in FEATURES {
        let hit: Vec<String> = files
            .iter()
            .filter(|(_, content)| (feature.check)(content))
            .map(|(name, _)| name.clone())
            .collect();
        if hit.is_empty() {
            missing.push(feature.name.to_string());
        }
        let table = evidence.get_mut(feature.name);
        match table {
            Some(list) => list.extend(hit),
            None => {
                evidence.insert(feature.name, hit);
            }
        }
    }

    for (name, files_hit) in &evidence {
        println!(
            "{name}: {} file(s){}",
            files_hit.len(),
            if files_hit.len() <= 4 {
                format!(" ({files_hit:?})")
            } else {
                String::new()
            }
        );
    }

    assert!(
        missing.is_empty(),
        "features with no corpus evidence: {missing:?}",
    );
}
