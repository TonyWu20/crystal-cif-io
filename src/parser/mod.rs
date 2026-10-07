//! CIF 1.1 parser built on `chumsky` and `ariadne`.
//!
//! The design follows `castep_cell_fmt` in the `castep-cell-io`
//! repository: one chumsky combinator per CIF production, a free
//! `parse_cif` entry point that returns a `Vec<Rich<char>>` of
//! errors, and `rich_error` to render an `ariadne` report with the
//! source text labeled. See `docs/refactor-chumsky-ariadne.md`.
//!
//! Semantic notes (CIF 1.1 "File syntax", IUCr):
//! - reserved words (`data_`, `loop_`, `save_`, `stop_`, `global_`)
//!   are case-insensitive and may not occur as unquoted values
//!   (para. 8, 11, 26);
//! - `.` and `?` standing as whole tokens are the `inapplicable` and
//!   `unknown` values, anywhere a value occurs (para. 12, 13);
//! - a `#` starts a comment only at the start of a line or after
//!   white space; a `#` glued to the previous token is part of that
//!   token (para. 21, 45);
//! - a `global_` block is forbidden (para. 33), so such a line fails
//!   the strict top level;
//! - the top level is strict: `Ok` means the whole input was
//!   consumed.

use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{error::Rich, extra, input::Emitter, prelude::*};

use crate::grammar::{
    CharString, CifDocument, DataBlock, DataBlockHeading, DataBlockMember, DataItems,
    DoubleQuotedString, Float, Integer, LoopColumn, LoopColumns, Number, Numeric, SaveFrame,
    SaveFrameHeading, SemiColonTextField, SingleLineData, SingleQuotedString, Tag, TextField,
    UnquotedString, UnsignedInteger, Value,
};

type Err<'src> = extra::Err<Rich<'src, char>>;

/// Parse a CIF 1.1 document from a string.
///
/// `Ok` means the whole input was consumed. `Err` carries one `Rich`
/// error per failure reached; render them with [`rich_error`].
pub fn parse_cif<'a>(input: &'a str) -> Result<CifDocument, Vec<Rich<'a, char>>> {
    match cif_file().parse(input).into_result() {
        Ok(doc) => Ok(doc),
        Err(mut errors) => {
            // Re-derive the semantic diagnostics that the backtracked
            // `filter`s drop on rewind, so they reach the caller. A
            // successful parse is left untouched.
            errors.extend(semantic_diagnoses(input));
            Err(errors)
        }
    }
}

pub fn rich_error(e: &Rich<char>, source_name: &str, src: &str) {
    let _ = Report::build(ReportKind::Error, (source_name, e.span().into_range()))
        .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
        .with_message(e.to_string())
        .with_label(
            Label::new((source_name, e.span().into_range()))
                .with_message(e.reason().to_string())
                .with_color(Color::Red),
        )
        .finish()
        .print((source_name, Source::from(src)));
}

// --- character classes (spec Appendix A) ------------------------------------

fn is_eol(c: char) -> bool {
    matches!(c, '\n' | '\r')
}

fn is_blank(c: char) -> bool {
    matches!(c, ' ' | '\t')
}

/// `<NonBlankChar>`: any character except white space (SP/HT/LF/CR).
fn is_nonblank(c: char) -> bool {
    !is_blank(c) && !is_eol(c)
}

/// `<OrdinaryChar>`: printable ASCII except space, tab, and the
/// special characters `#`, `$`, `"`, `'`, `;`, `[`, `]`, `_`.
fn is_ordinary(c: char) -> bool {
    matches!(c as u8, 0x21..=0x7e) && !matches!(c, '#' | '$' | '"' | '\'' | ';' | '[' | ']' | '_')
}

/// `<AnyPrintChar>`: any printable ASCII character.
fn is_printable(c: char) -> bool {
    matches!(c as u8, 0x20..=0x7e)
}

/// A bare reserved word token (`loop_`, `save_`, `stop_`, `data_`,
/// `global_`, case-insensitive). Such a token may not be a value
/// (spec para. 8, 11).
fn is_reserved_word(token: &str) -> bool {
    token.eq_ignore_ascii_case("loop_")
        || token.eq_ignore_ascii_case("save_")
        || token.eq_ignore_ascii_case("stop_")
        || token.eq_ignore_ascii_case("data_")
        || token.eq_ignore_ascii_case("global_")
}

/// A reserved word stem with an attached code (for example
/// `save_foo` or `Data_bar`). Such a token starts a block or save
/// frame and never begins an unquoted value (strict reading of spec
/// para. 6, 8, 11, 26, 35; fixture `12_mixed_case`).
fn starts_with_reserved_stem(token: &str) -> bool {
    let lower = token.to_ascii_lowercase();
    ["loop_", "save_", "data_", "stop_", "global_"]
        .iter()
        .any(|w| lower.starts_with(w))
}

// --- low-level producers -----------------------------------------------------

/// `<EOL>`: `\r\n`, `\n`, or `\r` (LF/CR line endings, spec para. 5).
fn eol<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    choice((just("\r\n"), just("\n"), just("\r"))).map(|_: &str| ())
}

/// `<NonBlankChar>`: one non-white-space character.
fn nonblank<'src>() -> impl Parser<'src, &'src str, char, Err<'src>> {
    any().filter(|&c| is_nonblank(c))
}

/// The `data_` reserved word stem, matched case-insensitively
/// (spec para. 26).
fn data_prefix<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    one_of::<&str, _, _>("dD")
        .then(one_of("aA"))
        .then(one_of("tT"))
        .then(one_of("aA"))
        .then(just('_'))
        .map(|_| ())
}

/// The `loop_` reserved word stem, matched case-insensitively.
fn loop_prefix<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    one_of::<&str, _, _>("lL")
        .then(one_of("oO"))
        .then(one_of("oO"))
        .then(one_of("pP"))
        .then(just('_'))
        .map(|_| ())
}

/// The `save_` reserved word stem, matched case-insensitively.
fn save_prefix<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    one_of::<&str, _, _>("sS")
        .then(one_of("aA"))
        .then(one_of("vV"))
        .then(one_of("eE"))
        .then(just('_'))
        .map(|_| ())
}

/// White space between CIF tokens: blanks, line endings, and comment
/// lines.
///
/// A `#` starts a comment only when it stands at the start of a line
/// or immediately after at least one blank of the current line
/// (spec para. 21, 45). A `#` glued to the token before the region
/// is part of that token, not a comment (fixture
/// `11_whitespace_edge`); the value parsers consume such a `#` as a
/// `<NonBlankChar>`.
///
/// The parser never fails: it consumes the maximal such region and
/// reports the number of characters consumed.
fn ws_region<'src>(starts_at_line_start: bool) -> impl Parser<'src, &'src str, usize, Err<'src>> {
    custom::<_, &'src str, usize, Err<'src>>(move |inp| {
        let mut consumed = 0usize;
        let mut at_line_start = starts_at_line_start;
        loop {
            // Blanks on the current line.
            let mut blanks = 0usize;
            while let Some(c) = inp.peek() {
                if is_blank(c) {
                    inp.next();
                    blanks += 1;
                    consumed += 1;
                } else {
                    break;
                }
            }
            // A comment on this line: a `#` at line start, or after
            // the blanks just consumed.
            if inp.peek() == Some('#') && (at_line_start || blanks > 0) {
                while let Some(c) = inp.peek() {
                    if is_eol(c) {
                        break;
                    }
                    inp.next();
                    consumed += 1;
                }
            }
            match inp.peek() {
                Some('\n') | Some('\r') => {
                    let is_cr = inp.peek() == Some('\r');
                    inp.next();
                    consumed += 1;
                    if is_cr && inp.peek() == Some('\n') {
                        inp.next();
                        consumed += 1;
                    }
                    at_line_start = true;
                }
                _ => break,
            }
        }
        Ok(consumed)
    })
}

// --- values (spec Appendix A <Value>) ---------------------------------------

/// `<Inapplicable>`: a `.` standing as a whole token, anywhere in a
/// value position (closes the known gap of the winnow parser,
/// fixture 09).
fn inapplicable<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    just('.')
        .then(nonblank().not())
        .map(|_| Value::Inapplicable)
}

/// `<Unknown>`: a `?` standing as a whole token, anywhere in a value
/// position.
fn unknown<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    just('?').then(nonblank().not()).map(|_| Value::Unknown)
}

/// `<Numeric>` (spec Appendix A): an optional sign, a digit run with
/// an optional decimal point and exponent, and an optional
/// `( <UnsignedInteger> )` standard uncertainty.
///
/// The whole non-blank token is consumed first; a token that is not a
/// valid CIF numeric literal (for example `1_445`, or a `#` glued to
/// the digits) is not a number, so the choice falls through to the
/// character-string production. This keeps the token boundary
/// correct (fixtures `03` and `11`).
fn numeric<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    let lead = one_of::<&str, _, _>("+-.0123456789");
    lead.then(nonblank().repeated().collect::<Vec<char>>())
        .map(|(first, rest): (char, Vec<char>)| {
            let mut token = String::with_capacity(rest.len() + 1);
            token.push(first);
            token.extend(rest);
            token
        })
        .filter(|token: &String| is_numeric_literal(&token))
        .map(|token: String| numeric_value(&token))
}

/// True when the token is a valid CIF numeric literal:
/// `[+-]?( d+(.d*)? | .d+ ) [eE[+-]?d+ ]? [( d+ )] ?` where the
/// number part contains no characters other than digits, dots, and
/// the exponent marker.
fn is_numeric_literal(token: &str) -> bool {
    let (number_part, uncertainty_part) = match token.split_once('(') {
        Some((n, rest))
            if rest.ends_with(')')
                && rest.len() > 1
                && rest[..rest.len() - 1].chars().all(|c| c.is_ascii_digit()) =>
        {
            (n, Some(&rest[..rest.len() - 1]))
        }
        _ => (token, None),
    };
    if uncertainty_part.is_some() && number_part.is_empty() {
        return false;
    }
    let bytes = number_part.as_bytes();
    let mut i = 0;
    if matches!(bytes.get(0), Some(b'+') | Some(b'-')) {
        i = 1;
    }
    let mut digits = 0usize;
    let mut dot = false;
    let mut exp = false;
    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => {
                digits += 1;
                i += 1;
            }
            b'.' if !dot && !exp => {
                dot = true;
                i += 1;
            }
            b'e' | b'E' if !exp && digits > 0 => {
                exp = true;
                i += 1;
                if matches!(bytes.get(i), Some(b'+') | Some(b'-')) {
                    i += 1;
                }
                let exp_start = i;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                if i == exp_start {
                    return false;
                }
            }
            _ => return false,
        }
    }
    digits > 0
}

/// Classify and build a `<Numeric>` value from a token known to be a
/// valid numeric literal. The number part containing `.` or an
/// exponent is a float; a plain signed digit run is an integer.
fn numeric_value(token: &str) -> Value {
    let (number_part, uncertainty_part) = match token.split_once('(') {
        Some((n, rest)) => (n, Some(rest.trim_end_matches(')'))),
        None => (token, None),
    };
    let is_float =
        number_part.contains('.') || number_part.contains('e') || number_part.contains('E');
    let number = if is_float {
        Number::Float(Float(number_part.parse::<f64>().unwrap_or(f64::NAN)))
    } else if let Ok(v) = number_part.parse::<i32>() {
        Number::Integer(Integer(v))
    } else {
        Number::Float(Float(number_part.parse::<f64>().unwrap_or(f64::NAN)))
    };
    let std_uncertainty = uncertainty_part
        .and_then(|digits| digits.parse::<u32>().ok())
        .map(UnsignedInteger);
    Value::Numeric(Numeric::new(number, std_uncertainty))
}

/// `<CharString>`: unquoted, single-quoted, or double-quoted.
fn char_string<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    choice((single_quoted(), double_quoted(), unquoted())).map(Value::CharString)
}

/// An unquoted character string: `<OrdinaryChar><NonBlankChar>*`,
/// where the whole token is not a bare reserved word (spec para. 8,
/// 11; fixture `neg_reserved_word_value`).
fn unquoted<'src>() -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    any()
        .filter(|&c| is_ordinary(c))
        .then(nonblank().repeated())
        .to_slice()
        .validate(|token: &'src str, map, emit| {
            if is_reserved_word(token) {
                emit.emit(Rich::custom(
                    map.span(),
                    format!("the reserved word `{token}` may not be used as a value"),
                ));
            } else if starts_with_reserved_stem(token) {
                emit.emit(Rich::custom(
                    map.span(),
                    format!("a reserved word stem with a code, `{token}`, may not begin a value"),
                ));
            }
            token
        })
        .filter(|token| !is_reserved_word(token) && !starts_with_reserved_stem(token))
        .map(|token| CharString::Unquoted(UnquotedString::new(token.to_string())))
}

/// A single-quoted character string (spec para. 15, 16). The content
/// may hold any printable character, including embedded quotes (kept
/// verbatim). The value token ends at the first white space or EOL;
/// the last character of the token must be the closing quote.
fn single_quoted<'src>() -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    let run = any()
        .filter(|&c| is_printable(c) && !is_eol(c))
        .repeated()
        .to_slice();
    just('\'')
        .then(run)
        .map(|(_q, run): (char, &str)| {
            run.trim_end_matches(|c: char| matches!(c, ' ' | '\t'))
                .to_string()
        })
        .filter(|trimmed: &String| !trimmed.is_empty() && trimmed.ends_with('\''))
        .map(|trimmed: String| {
            CharString::SingleQuoted(SingleQuotedString::new(
                trimmed[..trimmed.len() - 1].to_string(),
            ))
        })
}

/// A double-quoted character string. The content may hold any
/// printable character, including embedded quotes (kept verbatim).
/// The value token ends at the first white space or EOL; the last
/// character of the token must be the closing quote.
fn double_quoted<'src>() -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    let run = any()
        .filter(|&c| is_printable(c) && !is_eol(c))
        .repeated()
        .to_slice();
    just('"')
        .then(run)
        .map(|(_q, run): (char, &str)| {
            run.trim_end_matches(|c: char| matches!(c, ' ' | '\t'))
                .to_string()
        })
        .filter(|trimmed: &String| !trimmed.is_empty() && trimmed.ends_with('"'))
        .map(|trimmed: String| {
            CharString::DoubleQuoted(DoubleQuotedString::new(
                trimmed[..trimmed.len() - 1].to_string(),
            ))
        })
}

/// `<SemiColonTextField>` (spec para. 17): an opening `;`, then the
/// rest of that line (which may be empty), then full lines up to a
/// line that starts with the closing `;`. The stored lines are the
/// content without the delimiters; `raw_string` joins them with a
/// newline.
fn text_field<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    let line_content = any().filter(|&c| !is_eol(c)).repeated().to_slice();
    let first_line = line_content.clone().then(eol());
    let full_line = choice((
        // A content line: a first character that is not `;`, then any
        // non-EOL characters to the EOL.
        any()
            .filter(|&c| !is_eol(c) && c != ';')
            .then(
                any()
                    .filter(|&c| !is_eol(c))
                    .repeated()
                    .collect::<Vec<char>>(),
            )
            .then(eol())
            .map(|((first, rest), _): ((char, Vec<char>), ())| {
                let mut line = String::with_capacity(rest.len() + 1);
                line.push(first);
                line.extend(rest);
                line
            }),
        // An empty line.
        eol().map(|_| String::new()),
    ));
    just(';')
        .then(first_line.map(|(line, _): (&str, ())| line.to_string()))
        .then(full_line.repeated().collect::<Vec<String>>())
        .then(just(';'))
        .map(|(((_, first), lines), _close)| {
            let mut all = vec![first];
            all.extend(lines);
            Value::TextField(TextField::SemiColonTextField(SemiColonTextField::new(all)))
        })
}

/// `<DataName>`: `_` followed by one or more non-blank characters.
/// The stored tag name drops the leading underscore.
fn tag<'src>() -> impl Parser<'src, &'src str, Tag, Err<'src>> {
    just('_')
        .then(nonblank().repeated().at_least(1).to_slice())
        .map(|(_underscore, name)| Tag::new(name.to_string()))
}

/// `<DataItems>`, single-value form: a tag, white space (at least
/// one character, spec para. 7), and a value.
fn tag_value_items<'src>() -> impl Parser<'src, &'src str, DataItems, Err<'src>> {
    tag()
        .then(ws_region(false))
        .validate(|(tag, n), map, emit| {
            if n == 0 {
                emit.emit(Rich::custom(
                    map.span(),
                    "a data name must be followed by white space and a value",
                ));
            }
            tag
        })
        .then(value())
        .map(|(tag, value)| DataItems::SingleValue(SingleLineData::from_tag_value((tag, value))))
}

/// `<DataItems>`, loop form: `loop_`, one or more tags, and a
/// non-empty value stream transposed into columns (spec para. 7).
/// One header tag, preceded by the white space that separates it
/// from the `loop_` reserved word or the previous tag (spec para. 7:
/// a data name must be separated from the preceding token by white
/// space).
fn loop_tag<'src>() -> impl Parser<'src, &'src str, Tag, Err<'src>> {
    ws_region(false)
        .map(|_| ())
        .then(tag())
        .map(|(_ws, tag)| tag)
}

fn loop_items<'src>() -> impl Parser<'src, &'src str, DataItems, Err<'src>> {
    loop_prefix()
        .then(loop_tag().repeated().collect::<Vec<Tag>>())
        .then(
            ws_region(false)
                .map(|_| ())
                .then(value())
                .map(|(_ws, v): (_, Value)| v)
                .separated_by(ws_region(false).map(|_| ()))
                .collect::<Vec<Value>>(),
        )
        .validate(
            |((_, tags), values), map, emit: &mut Emitter<Rich<'src, char>>| {
                if tags.is_empty() || values.is_empty() {
                    emit.emit(Rich::<'src, char>::custom(
                        map.span(),
                        "a loop must name at least one tag and hold at least one value",
                    ));
                }
                (tags, values)
            },
        )
        .filter(|(tags, values)| !tags.is_empty() && !values.is_empty())
        .map(|(tags, values)| {
            let columns = into_columns(tags, values);
            DataItems::MultiValues(columns)
        })
}

/// A single-value data item, wrapped as a block member.
fn tag_value<'src>() -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    tag_value_items().map(DataBlockMember::DataItems)
}

/// A loop data item, wrapped as a block member.
fn loop_struct<'src>() -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    loop_items().map(DataBlockMember::DataItems)
}

/// `<Value>`: one of inapplicable, unknown, numeric, text field,
/// character string.
fn value<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    choice((
        inapplicable(),
        unknown(),
        numeric(),
        text_field(),
        char_string(),
    ))
}

/// Transpose a flat loop body into per-tag columns (the loop body is
/// a row-major value stream, spec para. 7).
fn into_columns(tags: Vec<Tag>, values: Vec<Value>) -> LoopColumns {
    let n = tags.len();
    let columns = (0..n)
        .map(|i| {
            let col_values = values.iter().skip(i).step_by(n).cloned().collect();
            LoopColumn::new(tags[i].clone(), col_values)
        })
        .collect();
    LoopColumns::new(columns)
}

/// `<SaveFrame>`: `save_<code>`, inner data items (save frames do not
/// nest, spec para. 6), closed by a bare `save_`.
fn save_frame<'src>() -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    save_prefix()
        .then(nonblank().repeated().at_least(1).to_slice())
        .then(
            ws_region(false)
                .map(|_| ())
                .then(inner_item())
                .map(|(_ws, item)| item)
                .repeated()
                .collect::<Vec<DataItems>>(),
        )
        .then(
            ws_region(false)
                .map(|_| ())
                .then(save_prefix())
                .then(nonblank().not())
                .map(|_| ()),
        )
        .map(|((((), code), items), _close)| {
            DataBlockMember::SaveFrame(SaveFrame::from_heading_items((
                SaveFrameHeading::new(code.to_string()),
                items,
            )))
        })
}

/// Members inside a save frame: tag-value items and loops, but not
/// nested save frames (spec para. 6).
/// Members inside a save frame: single-value items and loops, as
/// `<DataItems>` (save frames do not nest, spec para. 6).
fn inner_item<'src>() -> impl Parser<'src, &'src str, DataItems, Err<'src>> {
    choice((loop_items(), tag_value_items()))
}

/// Members of a data block: save frames, loops, and tag-value items.
fn block_member<'src>() -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    choice((save_frame(), loop_struct(), tag_value()))
}

/// `<DataBlock>`: a `data_` heading plus block members.
fn data_block<'src>() -> impl Parser<'src, &'src str, DataBlock, Err<'src>> {
    data_prefix()
        .then(nonblank().repeated().at_least(1).to_slice())
        .then(
            ws_region(false)
                .map(|_| ())
                .then(block_member())
                .map(|(_, m)| m)
                .repeated()
                .collect::<Vec<DataBlockMember>>(),
        )
        .map(|((_, heading), members)| {
            DataBlock::from_heading_members((DataBlockHeading::new(heading.to_string()), members))
        })
}

/// The top level: white space, a sequence of data blocks, trailing
/// white space, and end of input. A `global_` line (forbidden, spec
/// para. 33) and any other garbage fail the strict `end()`.
fn cif_file<'src>() -> impl Parser<'src, &'src str, CifDocument, Err<'src>> {
    ws_region(true)
        .map(|_| ())
        .then(
            data_block()
                .separated_by(ws_region(false).map(|_| ()))
                .collect::<Vec<DataBlock>>(),
        )
        .then(ws_region(false).map(|_| ()))
        .then(end())
        .map(|((((), blocks), _tail), _end)| CifDocument::new(None, Some(blocks)))
}

// --- post-parse semantic enrichment ----------------------------------------
//
// The chumsky parser enforces three CIF 1.1 semantic rules with
// backtracked `filter`s whose `validate`-emitted diagnostics chumsky
// truncates on rewind, so those helpful messages never reach the caller.
// This section re-derives the three rules from the source and returns them
// as `Rich::custom` errors. It runs only on a failed parse; a successful
// parse is untouched (the `validate`-stage groundtruth fixtures rely on
// that).

/// A CIF token with a byte range and a line index.
struct CifTok {
    start: usize,
    end: usize,
    line: usize,
}

/// Split the source into maximal non-blank, non-comment tokens, mirroring
/// `ws_region`'s comment rules (a `#` starts a comment only at the start of
/// a line or after at least one blank). Returns the byte range and line of
/// each token.
fn cif_tokens(source: &str) -> Vec<CifTok> {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut out: Vec<CifTok> = Vec::new();
    let mut i = 0usize;
    let mut line = 0usize;
    let mut at_line_start = true;
    while i < len {
        let mut blanks = 0usize;
        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
            blanks += 1;
        }
        if i < len && bytes[i] == b'#' && (at_line_start || blanks > 0) {
            while i < len && bytes[i] != b'\n' && bytes[i] != b'\r' {
                i += 1;
            }
        }
        if i < len && (bytes[i] == b'\n' || bytes[i] == b'\r') {
            if bytes[i] == b'\r' && i + 1 < len && bytes[i + 1] == b'\n' {
                i += 2;
            } else {
                i += 1;
            }
            line += 1;
            at_line_start = true;
            continue;
        }
        if i >= len {
            break;
        }
        let start = i;
        while i < len
            && bytes[i] != b' '
            && bytes[i] != b'\t'
            && bytes[i] != b'\n'
            && bytes[i] != b'\r'
        {
            i += 1;
        }
        out.push(CifTok {
            start,
            end: i,
            line,
        });
        at_line_start = false;
    }
    out
}

fn tok_text<'s>(source: &'s str, t: &CifTok) -> &'s str {
    &source[t.start..t.end]
}

/// A reserved word or reserved stem standing where a data name's value
/// belongs (spec para. 8, 11). High precision: only a stem token that
/// immediately follows a data name on the same line is a value.
fn reserved_word_as_value(
    source: &str,
    toks: &[CifTok],
) -> Vec<(Range<usize>, String)> {
    let mut out: Vec<(Range<usize>, String)> = Vec::new();
    for k in 1..toks.len() {
        let t = &toks[k];
        let text = tok_text(source, t);
        if !starts_with_reserved_stem(text) {
            continue;
        }
        let prev = &toks[k - 1];
        let prev_text = tok_text(source, prev);
        if !prev_text.starts_with('_') || prev.line != t.line {
            continue;
        }
        let msg = if is_reserved_word(text) {
            format!("the reserved word `{text}` may not be used as a value")
        } else {
            format!("a reserved word stem with a code, `{text}`, may not begin a value")
        };
        out.push((t.start..t.end, msg));
    }
    out
}

/// A `loop_` reserved word with no header tags (the clear empty-loop case,
/// spec para. 7). The "tags but zero body values" sub-case is not detected:
/// it needs a full parse and is unreachable in the current fixtures.
fn loop_without_tags(
    source: &str,
    toks: &[CifTok],
) -> Vec<(Range<usize>, String)> {
    let mut out: Vec<(Range<usize>, String)> = Vec::new();
    for k in 0..toks.len() {
        let t = &toks[k];
        let text = tok_text(source, t);
        if !text.eq_ignore_ascii_case("loop_") {
            continue;
        }
        let has_header_tag = toks
            .get(k + 1)
            .map(|n| tok_text(source, n).starts_with('_'))
            .unwrap_or(false);
        if !has_header_tag {
            out.push((
                t.start..t.end,
                "a loop must name at least one tag and hold at least one value".to_string(),
            ));
        }
    }
    out
}

/// Re-derive the two scoped semantic checks (reserved word or reserved stem
/// in a value position, and an empty loop) and return them as `Rich::custom`
/// errors with byte-accurate spans. Deduplicated by span.
fn semantic_diagnoses<'a>(source: &'a str) -> Vec<Rich<'a, char>> {
    let toks = cif_tokens(source);
    let mut found: Vec<(Range<usize>, String)> = Vec::new();
    found.extend(reserved_word_as_value(source, &toks));
    found.extend(loop_without_tags(source, &toks));
    found.sort_by_key(|(r, _)| (r.start, r.end));
    found.dedup_by(|a, b| a.0 == b.0);
    found
        .into_iter()
        .map(|(range, msg)| Rich::custom(SimpleSpan::from(range), msg))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn messages(source: &str) -> Vec<String> {
        semantic_diagnoses(source)
            .into_iter()
            .map(|e| e.reason().to_string())
            .collect()
    }

    #[test]
    fn reserved_word_as_value_is_detected() {
        let src = "data_d\n_a save_foo\n";
        let msgs = messages(src);
        assert!(
            msgs.iter()
                .any(|m| m.contains("a reserved word stem with a code, `save_foo`")),
            "expected the stem diagnostic, got {msgs:?}"
        );
    }

    #[test]
    fn bare_reserved_word_value_is_detected() {
        let src = "data_bad\n\n_some_tag  loop_\n";
        let msgs = messages(src);
        assert!(
            msgs.iter().any(|m| m.contains("the reserved word `loop_`")),
            "expected the reserved-word diagnostic, got {msgs:?}"
        );
    }

    #[test]
    fn a_loop_value_that_opens_a_frame_is_not_flagged() {
        // `sAvE_fc` after a loop is a frame header, not a value.
        let src = "data_ii\nloop_\n _case_tag\n v1\n v2\nsAvE_fc\n _fc_a 1\nSAVE_\n";
        let msgs = messages(src);
        assert!(
            msgs.is_empty(),
            "a legal document must yield no semantic diagnoses, got {msgs:?}"
        );
    }

    #[test]
    fn loop_without_tags_is_detected() {
        let src = "data_bad\nloop_ _x\nloop_\nsave_f\n _a 1\nSAVE_\n";
        let msgs = messages(src);
        assert!(
            msgs.iter().any(|m| m.contains("a loop must name at least one tag")),
            "expected the empty-loop diagnostic, got {msgs:?}"
        );
    }

    #[test]
    fn parse_cif_surfaces_the_custom_diagnostics() {
        let src = "data_d\n_a save_foo\n";
        let errs = parse_cif(src).err().expect("must fail");
        assert!(
            errs.iter().any(|e| e.reason().to_string().contains("may not begin a value")),
            "the custom diagnostic must reach the caller, got {errs:?}"
        );
    }

    #[test]
    fn a_valid_document_stays_ok() {
        let src = "data_ok\n _a 1\n _b 2.5\n";
        assert!(parse_cif(src).is_ok(), "a valid document must parse");
    }
}
