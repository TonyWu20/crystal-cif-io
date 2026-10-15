//! CIF parser built on `chumsky` and `ariadne`.
//!
//! The grammar is the CIF 1.1 value rule, plus the CIF 2.0 value
//! rules when the file carries a 2.0 version code, either the
//! IUCr `##CIF_2.0` spelling or the escaped `#\#CIF_2.0`
//! spelling of the COMCIFS reference implementation (see
//! [`CifDialect`]). The 2.0 rules are scoped to the value layer:
//! lists parse, and `[`, `]`, `{`, `}` are illegal inside unquoted
//! values. Everything else (blocks, loops, save frames, text
//! fields) is 1.1. See `docs/roadmap-dict-items.md` section 13.
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
use chumsky::{
    error::Rich,
    extra,
    input::{Emitter, InputRef},
    prelude::*,
    recursive::{recursive, Direct, Recursive},
};

use crate::grammar::{
    CharString, CifDocument, Comments, DataBlock, DataBlockHeading, DataBlockMember, DataItems,
    DoubleQuotedString, Float, Integer, LoopColumn, LoopColumns, Number, Numeric, SaveFrame,
    SaveFrameHeading, SemiColonTextField, SingleLineData, SingleQuotedString, Tag, TextField,
    TripleDoubleQuotedString, TripleSingleQuotedString, UnquotedString, UnsignedInteger, Value,
};

type Err<'src> = extra::Err<Rich<'src, char>>;

/// The value-rule dialect of a CIF file.
///
/// A file is 2.0 only when a 2.0 version code stands as the first
/// characters of the file, after any byte order mark. Both
/// spellings count: the IUCr `##CIF_2.0` and the escaped
/// `#\#CIF_2.0` (the COMCIFS reference implementation spells the
/// version comment with an escaped hash). In the 2.0 dialect the
/// list value rule parses and `[`, `]`, `{`, `}` are illegal
/// inside unquoted values. In the 1.1 dialect the version code is
/// an inert comment line and the characters stay plain characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CifDialect {
    /// CIF 1.1 value rules. The default for any file.
    Cif1_1,
    /// CIF 2.0 value rules: lists parse, brackets stay out of
    /// unquoted values.
    Cif2_0,
}

impl CifDialect {
    /// Detect the dialect from the start of the file. A leading byte
    /// order mark is skipped. Anything but a 2.0 version code at
    /// position zero is 1.1. Both spellings of the 2.0 code count:
    /// the IUCr `##CIF_2.0` and the escaped `#\#CIF_2.0`.
    pub fn detect(input: &str) -> Self {
        let body = input.strip_prefix('\u{feff}').unwrap_or(input);
        if body.starts_with("##CIF_2.0") || body.starts_with("#\\#CIF_2.0") {
            Self::Cif2_0
        } else {
            Self::Cif1_1
        }
    }

    /// The canonical JSON schema version for this dialect
    /// (see `docs/ground-truth.md`).
    pub fn schema_version(self) -> u32 {
        match self {
            Self::Cif1_1 => 1,
            Self::Cif2_0 => 2,
        }
    }
}

/// Parse a CIF document from a string.
///
/// The dialect is detected from the file start: a 2.0 version code
/// (after any byte order mark) enables the 2.0 value rules. Both
/// spellings count: the IUCr `##CIF_2.0` and the escaped
/// `#\#CIF_2.0`. Anything else stays 1.1. A leading byte order
/// mark is elided.
///
/// `Ok` means the whole input was consumed. `Err` carries one `Rich`
/// error per failure reached; render them with [`rich_error`].
pub fn parse_cif<'a>(input: &'a str) -> Result<CifDocument, Vec<Rich<'a, char>>> {
    let source = input.strip_prefix('\u{feff}').unwrap_or(input);
    let dialect = CifDialect::detect(source);
    match cif_file(dialect).parse(source).into_result() {
        Ok(doc) => Ok(doc),
        Err(mut errors) => {
            // Re-derive the semantic diagnostics that the backtracked
            // `filter`s drop on rewind, so they reach the caller. A
            // successful parse is left untouched.
            errors.extend(semantic_diagnoses(source));
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

/// Render one syntax error as an ariadne report into a `String`.
/// The report carries the source snippet, the span, and the reason.
pub fn syntax_error_report(e: &Rich<char>, source_name: &str, src: &str) -> String {
    let report = Report::build(ReportKind::Error, (source_name, e.span().into_range()))
        .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
        .with_message(e.to_string())
        .with_label(
            Label::new((source_name, e.span().into_range()))
                .with_message(e.reason().to_string())
                .with_color(Color::Red),
        )
        .finish();
    let mut out = std::io::Cursor::new(Vec::new());
    let _ = report.write((source_name, Source::from(src)), &mut out);
    String::from_utf8(out.into_inner()).expect("the report is utf-8")
}

// --- character classes (spec Appendix A) ------------------------------------

fn is_eol(c: char) -> bool {
    // VT (0x0B) and FF (0x0C) end a line in every dialect (decision
    // D9a, roadmap section 14). The CIF 1.1 spec excludes both from
    // its character set (para. 22, 28); accepting them is a
    // recorded 1.1 deviation, pinned by fixture 26.
    matches!(c, '\n' | '\r' | '\u{0b}' | '\u{0c}')
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
///
/// The 1.1 unquoted rule widens this lead set with `[` and `]`
/// (roadmap section 13.6: in 1.1 the brackets stay plain
/// characters, fixture `17_cif1_brackets_stay_strings`).
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

/// `<EOL>`: `\r\n`, `\n`, `\r`, VT, or FF. The line endings of spec
/// para. 5, plus the VT and FF terminators of decision D9a (roadmap
/// section 14, a recorded 1.1 deviation).
fn eol<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    choice((
        just("\r\n"),
        just("\n"),
        just("\r"),
        just("\u{0b}"),
        just("\u{0c}"),
    ))
    .map(|_: &str| ())
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

fn consume_ws<'src, 'parse>(
    inp: &mut InputRef<'src, 'parse, &'src str, Err<'src>>,
    starts_at_line_start: bool,
    glued_comment_ok: bool,
) -> usize {
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
        // A comment on this line: a `#` at line start, after the
        // blanks just consumed, or, in a 2.0 container, glued to
        // the opening delimiter (the COMCIFS reference scanner
        // start condition after `[` or `{`; roadmap section 14).
        if inp.peek() == Some('#')
            && (at_line_start || blanks > 0 || (glued_comment_ok && consumed == 0))
        {
            while let Some(c) = inp.peek() {
                if is_eol(c) {
                    break;
                }
                inp.next();
                consumed += 1;
            }
        }
        match inp.peek() {
            Some('\n') | Some('\r') | Some('\u{0b}') | Some('\u{0c}') => {
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
    consumed
}

/// White space between CIF tokens: blanks, line endings, and
/// comment lines.
///
/// A `#` starts a comment only when it stands at the start of a
/// line or immediately after at least one blank of the current line
/// (spec para. 21, 45). A `#` glued to the token before the region
/// is part of that token, not a comment (fixture
/// `11_whitespace_edge`); the value parsers consume such a `#` as a
/// `<NonBlankChar>`.
///
/// The parser never fails: it consumes the maximal such region and
/// reports the number of characters consumed.
fn ws_region<'src>(starts_at_line_start: bool) -> impl Parser<'src, &'src str, usize, Err<'src>> {
    custom::<_, &'src str, usize, Err<'src>>(move |inp: &mut InputRef<'_, '_, &'src str, _>| {
        Ok(consume_ws(inp, starts_at_line_start, false))
    })
}

/// The white space region of a 2.0 container interior (after a list
/// `[` or table `{`): the same region as [`ws_region`], except a `#`
/// glued to the opening delimiter starts a comment with no leading
/// blank (upstream CIF 2.0 scanner; roadmap section 14). In the 1.1
/// dialect the region is the plain white space region.
fn container_ws<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, usize, Err<'src>> {
    let glued_comment_ok = matches!(dialect, CifDialect::Cif2_0);
    custom::<_, &'src str, usize, Err<'src>>(move |inp: &mut InputRef<'_, '_, &'src str, _>| {
        Ok(consume_ws(inp, false, glued_comment_ok))
    })
}

// --- values (spec Appendix A <Value>) ---------------------------------------

/// `<Inapplicable>`: a `.` standing as a whole token, anywhere in a
/// value position (closes the known gap of the winnow parser,
/// fixture 09). In the 2.0 dialect the token also ends at the list
/// closer `]` (see [`continues_value_token`]).
fn inapplicable<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Value, Err<'src>> {
    just('.')
        .then(
            any()
                .filter(move |&c| continues_value_token(dialect, c))
                .not(),
        )
        .map(|_| Value::Inapplicable)
}

/// `<Unknown>`: a `?` standing as a whole token, anywhere in a value
/// position. Token end is dialect-aware as in [`inapplicable`].
fn unknown<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Value, Err<'src>> {
    just('?')
        .then(
            any()
                .filter(move |&c| continues_value_token(dialect, c))
                .not(),
        )
        .map(|_| Value::Unknown)
}

/// `<Numeric>` (spec Appendix A): an optional sign, a digit run with
/// an optional decimal point and exponent, and an optional
/// `( <UnsignedInteger> )` standard uncertainty.
///
/// The whole value token is consumed first; a token that is not a
/// valid CIF numeric literal (for example `1_445`, or a `#` glued to
/// the digits) is not a number, so the choice falls through to the
/// character-string production. This keeps the token boundary
/// correct (fixtures `03` and `11`). In the 2.0 dialect the token
/// also ends at the list closer `]` (see
/// [`continues_value_token`]).
fn numeric<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Value, Err<'src>> {
    let lead = one_of::<&str, _, _>("+-.0123456789");
    lead.then(
        any()
            .filter(move |&c| continues_value_token(dialect, c))
            .repeated()
            .collect::<Vec<char>>(),
    )
    .map(|(first, rest): (char, Vec<char>)| {
        let mut token = String::with_capacity(rest.len() + 1);
        token.push(first);
        token.extend(rest);
        token
    })
    .filter(|token: &String| is_numeric_literal(token))
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
    if matches!(bytes.first(), Some(b'+') | Some(b'-')) {
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

/// `<CharString>`: unquoted, single-quoted, or double-quoted. The
/// unquoted and quoted forms are dialect-aware (see [`unquoted`],
/// [`single_quoted`]).
/// A triple-quoted character string (CIF 2.0 only, roadmap section
/// 14): an opening ''' or """ delimiter, then content up to the
/// first closing delimiter. The content may hold line terminators
/// and runs of up to two delimiters; a third delimiter closes the
/// string. The empty form (six delimiters) is a legal empty string.
/// The content is stored as written, without the delimiters.
fn triple_quoted<'src>(
    quote: char,
    single: bool,
) -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    let q = just(quote);
    let nonq = any().filter(move |c: &char| c != &quote);
    // A chunk: up to two delimiters, then a non-delimiter character
    // (the upstream NOTSQ3/NOTDQ3 form).
    let chunk = choice((
        nonq.clone().map(|c: char| c.to_string()),
        q.clone()
            .then(nonq.clone())
            .map(|(d, c): (char, char)| d.to_string() + &c.to_string()),
        q.clone()
            .then(q)
            .then(nonq)
            .map(|((d1, d2), c): ((char, char), char)| {
                d1.to_string() + &d2.to_string() + &c.to_string()
            }),
    ));
    let open = q.clone().then(q.clone()).then(q);
    let close = q.clone().then(q.clone()).then(q);
    open.map(|_o: ((char, char), char)| ())
        .then(chunk.repeated().collect::<Vec<String>>())
        .then(close.map(|_c: ((char, char), char)| ()))
        .map(move |(((), chars), ())| {
            let content: String = chars.concat();
            if single {
                CharString::TripleSingleQuoted(TripleSingleQuotedString::new(content))
            } else {
                CharString::TripleDoubleQuoted(TripleDoubleQuotedString::new(content))
            }
        })
}

fn char_string<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Value, Err<'src>> {
    // The triple-quoted form is a 2.0 production (roadmap section
    // 14): in a 1.1 file the delimiters fall through to the single
    // and double quote productions (embedded-quote runs).
    let triple = choice((triple_quoted('\'', true), triple_quoted('"', false)))
        .filter(move |_: &CharString| dialect == CifDialect::Cif2_0)
        .boxed();
    choice((
        triple,
        single_quoted(dialect),
        double_quoted(dialect),
        unquoted(dialect),
    ))
    .map(Value::CharString)
}

/// An unquoted character string: `<OrdinaryChar><NonBlankChar>*`,
/// where the whole token is not a bare reserved word (spec para. 8,
/// 11; fixture `neg_reserved_word_value`).
///
/// Dialect split (roadmap section 13.3):
/// - 1.1: `[`, `]`, `{`, `}` are plain characters anywhere in the
///   token, and `[` / `]` may also lead it.
/// - 2.0: a token containing any of `[`, `]`, `{`, `}` is a parse
///   error; the lead set stays the strict `<OrdinaryChar>` set.
fn unquoted<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    any()
        .filter(move |&c| is_unquoted_lead(dialect, c))
        .then(
            any()
                .filter(move |&c| continues_value_token(dialect, c))
                .repeated(),
        )
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
        .filter(move |token| {
            if is_reserved_word(token) || starts_with_reserved_stem(token) {
                return false;
            }
            match dialect {
                CifDialect::Cif1_1 => true,
                CifDialect::Cif2_0 => !token.contains(['[', ']', '{', '}']),
            }
        })
        .map(|token| CharString::Unquoted(UnquotedString::new(token.to_string())))
}

/// A leading character of an unquoted value, per dialect.
///
/// 1.1: `<OrdinaryChar>` plus `[` and `]` (the brackets stay plain
/// characters, fixture `17_cif1_brackets_stay_strings`). The 1.1
/// character set is ASCII only (fixture 20 pins the rejection of
/// non-ASCII values). 2.0: the `<OrdinaryChar>` set, and any
/// non-ASCII character (roadmap section 14). The container
/// delimiters stay excluded: the 2.0 token filter rejects a token
/// that holds one.
fn is_unquoted_lead(dialect: CifDialect, c: char) -> bool {
    if is_ordinary(c) {
        return true;
    }
    match c {
        '[' | ']' => dialect == CifDialect::Cif1_1,
        _ => !c.is_ascii() && dialect == CifDialect::Cif2_0,
    }
}

/// A character of a block code, a frame code, or a tag name, per
/// dialect.
///
/// 1.1: the character set is ASCII only (roadmap section 14). 2.0:
/// any `<NonBlankChar>`, non-ASCII included (roadmap section 14).
fn is_name_char(dialect: CifDialect, c: char) -> bool {
    if !is_nonblank(c) {
        return false;
    }
    match dialect {
        CifDialect::Cif1_1 => c.is_ascii(),
        CifDialect::Cif2_0 => true,
    }
}

/// True when `c` continues the current value token, per dialect.
///
/// 1.1: any `<NonBlankChar>` continues the token, as today. 2.0:
/// the container delimiters `[`, `]`, `{`, and `}` end the token
/// (roadmap sections 13.2 and 14): the upstream scanner treats all
/// four as separate tokens, so `[1 2 3]` tokenizes as three numeric
/// values and a closer, and `{ 'k':. }` tokenizes a table pair
/// with an inapplicable value.
fn continues_value_token(dialect: CifDialect, c: char) -> bool {
    if !is_nonblank(c) {
        return false;
    }
    match dialect {
        CifDialect::Cif1_1 => true,
        CifDialect::Cif2_0 => !matches!(c, '[' | ']' | '{' | '}'),
    }
}

/// A single-quoted character string (spec para. 15, 16).
///
/// 1.1: the content may hold any printable character. An embedded
/// delimiter is legal (kept verbatim): a quote closes the string
/// only when it is followed by a blank (SP/HT), a line terminator,
/// or end of input; a quote followed by a non-blank character is
/// content (spec para. 15). The empty form is a legal empty
/// string.
/// 2.0: the EBNF quote-content excludes only the closing quote, so
/// blanks inside the quotes are legal (COMCIFS CIF2-EBNF,
/// `quote-content`). The run stops at the first closing quote,
/// which must be present. The empty form `''` is a legal empty
/// string. (Review d8 P1-1.)
fn single_quoted<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    match dialect {
        CifDialect::Cif1_1 => {
            // An embedded quote (spec para. 15): a quote followed by
            // a non-blank printable character is content. A quote
            // followed by a blank (SP/HT), a line terminator, or end
            // of input closes the string.
            let embedded = just('\'')
                .then(any().filter(|&c| is_printable(c) && !is_eol(c) && !is_blank(c)))
                .map(|(q, c): (char, char)| vec![q, c])
                .boxed();
            let content = choice((
                any()
                    .filter(|&c| is_printable(c) && !is_eol(c) && c != '\'')
                    .map(|c: char| vec![c])
                    .boxed(),
                embedded,
            ));
            just('\'')
                .then(content.repeated().collect::<Vec<Vec<char>>>())
                .then(just('\''))
                .map(|((_, content), _): ((char, Vec<Vec<char>>), char)| {
                    let mut s = String::new();
                    for part in &content {
                        s.extend(part);
                    }
                    CharString::SingleQuoted(SingleQuotedString::new(s))
                })
                .boxed()
        }
        CifDialect::Cif2_0 => {
            let run = any()
                .filter(|&c| is_printable(c) && !is_eol(c) && c != '\'')
                .repeated()
                .to_slice();
            just('\'')
                .then(run)
                .then(just('\''))
                .map(|((_q, content), _q2): ((char, &str), char)| {
                    CharString::SingleQuoted(SingleQuotedString::new(content.to_string()))
                })
                .boxed()
        }
    }
}

/// A double-quoted character string.
///
/// 1.1: the content may hold any printable character. An embedded
/// delimiter is legal (kept verbatim): a double quote closes the
/// string only when it is followed by a blank (SP/HT), a line
/// terminator, or end of input; a double quote followed by a
/// non-blank character is content (spec para. 15). The empty form
/// is a legal empty string.
/// 2.0: the EBNF quote-content excludes only the closing quote, so
/// blanks inside the quotes are legal. The run stops at the first
/// closing quote, which must be present. The empty form `""` is a
/// legal empty string. (Review d8 P1-1.)
fn double_quoted<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, CharString, Err<'src>> {
    match dialect {
        CifDialect::Cif1_1 => {
            // An embedded delimiter (spec para. 15): a `"` followed
            // by a non-blank printable character is content. A `"`
            // followed by a blank (SP/HT), a line terminator, or
            // end of input closes the string.
            let embedded = just('"')
                .then(any().filter(|&c| is_printable(c) && !is_eol(c) && !is_blank(c)))
                .map(|(q, c): (char, char)| vec![q, c])
                .boxed();
            let content = choice((
                any()
                    .filter(|&c| is_printable(c) && !is_eol(c) && c != '"')
                    .map(|c: char| vec![c])
                    .boxed(),
                embedded,
            ));
            just('"')
                .then(content.repeated().collect::<Vec<Vec<char>>>())
                .then(just('"'))
                .map(|((_, content), _): ((char, Vec<Vec<char>>), char)| {
                    let mut s = String::new();
                    for part in &content {
                        s.extend(part);
                    }
                    CharString::DoubleQuoted(DoubleQuotedString::new(s))
                })
                .boxed()
        }
        CifDialect::Cif2_0 => {
            let run = any()
                .filter(|&c| is_printable(c) && !is_eol(c) && c != '"')
                .repeated()
                .to_slice();
            just('"')
                .then(run)
                .then(just('"'))
                .map(|((_q, content), _q2): ((char, &str), char)| {
                    CharString::DoubleQuoted(DoubleQuotedString::new(content.to_string()))
                })
                .boxed()
        }
    }
}

/// `<SemiColonTextField>` (spec para. 17): an opening `;`, then the
/// rest of that line (which may be empty), then full lines up to a
/// line that starts with the closing `;`. The stored lines are the
/// content without the delimiters; `raw_string` joins them with a
/// newline.
fn text_field<'src>() -> impl Parser<'src, &'src str, Value, Err<'src>> {
    let line_content = any().filter(|&c| !is_eol(c)).repeated().to_slice();
    let first_line = line_content.then(eol());
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
fn tag<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Tag, Err<'src>> {
    just('_')
        .then(
            any()
                .filter(move |c: &char| is_name_char(dialect, *c))
                .repeated()
                .at_least(1)
                .to_slice(),
        )
        .map(|(_underscore, name): (char, &'src str)| Tag::new(name.to_string()))
}

/// `<DataItems>`, single-value form: a tag, white space (at least
/// one character, spec para. 7), and a value.
fn tag_value_items<'src>(
    dialect: CifDialect,
) -> impl Parser<'src, &'src str, DataItems, Err<'src>> {
    tag(dialect)
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
        .then(value(dialect))
        .map(|(tag, value)| DataItems::SingleValue(SingleLineData::from_tag_value((tag, value))))
}

/// `<DataItems>`, loop form: `loop_`, one or more tags, and a
/// non-empty value stream transposed into columns (spec para. 7).
/// One header tag, preceded by the white space that separates it
/// from the `loop_` reserved word or the previous tag (spec para. 7:
/// a data name must be separated from the preceding token by white
/// space).
fn loop_tag<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Tag, Err<'src>> {
    ws_region(false)
        .map(|_| ())
        .then(tag(dialect))
        .map(|(_ws, tag)| tag)
}

fn loop_items<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, DataItems, Err<'src>> {
    loop_prefix()
        .then(loop_tag(dialect).repeated().collect::<Vec<Tag>>())
        .then(
            ws_region(false)
                .map(|_| ())
                .then(value(dialect))
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
fn tag_value<'src>(
    dialect: CifDialect,
) -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    tag_value_items(dialect).map(DataBlockMember::DataItems)
}

/// A loop data item, wrapped as a block member.
fn loop_struct<'src>(
    dialect: CifDialect,
) -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    loop_items(dialect).map(DataBlockMember::DataItems)
}

/// `<Value>`: one of inapplicable, unknown, numeric, text field,
/// character string. The 2.0 `list` production comes first
/// (roadmap section 13.4: `list | <current choice>`).
///
/// The production is self-recursive: a list holds values, and a
/// value may be a list. The body is therefore built with chumsky's
/// `recursive` (one `Rc` per build, no per-token allocation). In the
/// 1.1 dialect the trailing filter rejects the list production, so
/// the 1.1 value rule stays the historical choice.
fn value<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, Value, Err<'src>> {
    recursive(
        move |rec: Recursive<Direct<'src, 'src, &'src str, Value, Err<'src>>>| {
            // `<List>` (CIF 2.0 only, roadmap section 13.2): `[`, zero
            // or more values separated by white space (which may span
            // lines), and `]`. Lists may nest. An empty list is the
            // empty `[]` form.
            //
            // The sub-parsers are `.boxed()` (chumsky's `Rc`-backed
            // `Boxed`) so the recursive `choice` satisfies chumsky's
            // `Clone` bound: the sibling productions are opaque `impl
            // Parser` types, which rustc does not prove `Clone` through.
            // The EBNF list production allows optional white space
            // (blanks, comments, and new lines) before the closer,
            // so `[1 ]` and `[ 1 ]` parse (review d8 P1-2). The first
            // white space region is the container region: a `#` glued
            // to the `[` starts a comment (roadmap section 14). A
            // text field may stand as a list member (the `;` form of
            // the value rule; the 1.1 list stays unchanged).
            let list = just('[')
                .then(container_ws(dialect).boxed().map(|_| ()))
                .then(
                    rec.clone()
                        .separated_by(container_ws(dialect).boxed().map(|_| ()))
                        .collect::<Vec<Value>>(),
                )
                .then(container_ws(dialect).boxed().map(|_| ()))
                .then(just(']'))
                .map(|((((_open, _lead), values), _tail), _close)| Value::List(values))
                .filter(move |_: &Value| dialect == CifDialect::Cif2_0);

            // `<Table>` (CIF 2.0 only, roadmap section 14): `{`, zero
            // or more white space separated pairs, and `}`. A pair is
            // a quoted key (any quote style; the upstream KEY token),
            // a colon, and a value of any kind. The value may sit
            // directly after the colon, or after white space (which
            // may hold a comment or a line break). The pairs keep
            // their order; the key is stored as a value.
            let table_key = choice((
                triple_quoted('\'', true),
                triple_quoted('"', false),
                single_quoted(dialect),
                double_quoted(dialect),
            ))
            .filter(move |_: &CharString| dialect == CifDialect::Cif2_0)
            .map(Value::CharString);
            let entry = table_key
                .then(just(':'))
                .then(choice((
                    rec.clone(),
                    ws_region(false)
                        .map(|_| ())
                        .then(rec.clone())
                        .map(|(_ws, v): (_, Value)| v),
                )))
                .map(|((key, _colon), value): ((Value, char), Value)| (key, value));
            let table = just('{')
                .then(container_ws(dialect).boxed().map(|_| ()))
                .then(
                    entry
                        .separated_by(ws_region(false).boxed().map(|_| ()))
                        .collect::<Vec<(Value, Value)>>(),
                )
                .then(ws_region(false).boxed().map(|_| ()))
                .then(just('}'))
                .map(|(((_open, pairs), _tail), _close)| Value::Table(pairs))
                .filter(move |_: &Value| dialect == CifDialect::Cif2_0)
                .boxed();
            choice((
                list,
                table,
                inapplicable(dialect).boxed(),
                unknown(dialect).boxed(),
                numeric(dialect).boxed(),
                text_field().boxed(),
                char_string(dialect).boxed(),
            ))
        },
    )
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
/// `<SaveFrame>`: `save_<code>`, frame content (data items and, in
/// CIF 2.0, nested frames), and the `save_` terminator.
///
/// Nested frames are a 2.0 production (roadmap section 14): the
/// upstream C reference accepts them at unlimited depth, and the
/// 1.1 spec forbids them (para. 6), so the nested-frame branch is
/// filtered to the 2.0 dialect.
fn save_frame<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, SaveFrame, Err<'src>> {
    recursive(
        move |rec: Recursive<Direct<'src, 'src, &'src str, SaveFrame, Err<'src>>>| {
            save_prefix()
                .boxed()
                .then(
                    any()
                        .filter(move |c: &char| is_name_char(dialect, *c))
                        .repeated()
                        .at_least(1)
                        .to_slice()
                        .boxed(),
                )
                .then(
                    ws_region(false)
                        .map(|_| ())
                        .then(frame_member(dialect, rec))
                        .map(|(_ws, member)| member)
                        .repeated()
                        .collect::<Vec<DataBlockMember>>()
                        .boxed(),
                )
                .then(
                    ws_region(false)
                        .map(|_| ())
                        .then(save_prefix().boxed())
                        .then(nonblank().not())
                        .map(|_| ())
                        .boxed(),
                )
                .map(|((((), code), members), _close)| {
                    let mut items: Vec<DataItems> = Vec::new();
                    let mut frames: Vec<SaveFrame> = Vec::new();
                    for member in members {
                        match member {
                            DataBlockMember::DataItems(items_member) => items.push(items_member),
                            DataBlockMember::SaveFrame(frame) => frames.push(frame),
                        }
                    }
                    SaveFrame::from_heading_items_frames((
                        SaveFrameHeading::new(code.to_string()),
                        items,
                        frames,
                    ))
                })
        },
    )
}

/// Frame content members: a `<DataItems>` unit or, in CIF 2.0 only,
/// a nested save frame (roadmap section 14).
fn frame_member<'src>(
    dialect: CifDialect,
    rec: Recursive<Direct<'src, 'src, &'src str, SaveFrame, Err<'src>>>,
) -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    choice((
        inner_item(dialect).map(DataBlockMember::DataItems),
        rec.clone()
            .filter(move |_: &SaveFrame| dialect == CifDialect::Cif2_0)
            .map(DataBlockMember::SaveFrame),
    ))
}

/// Members inside a save frame: single-value items and loops, as
/// `<DataItems>` (spec para. 6).
fn inner_item<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, DataItems, Err<'src>> {
    choice((loop_items(dialect), tag_value_items(dialect)))
}

/// Members of a data block: save frames, loops, and tag-value items.
fn block_member<'src>(
    dialect: CifDialect,
) -> impl Parser<'src, &'src str, DataBlockMember, Err<'src>> {
    choice((
        save_frame(dialect).map(DataBlockMember::SaveFrame),
        loop_struct(dialect),
        tag_value(dialect),
    ))
}

/// `<DataBlock>`: a `data_` heading plus block members.
fn data_block<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, DataBlock, Err<'src>> {
    data_prefix()
        .then(
            any()
                .filter(move |c: &char| is_name_char(dialect, *c))
                .repeated()
                .at_least(1)
                .to_slice(),
        )
        .then(
            ws_region(false)
                .map(|_| ())
                .then(block_member(dialect))
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
/// para. 33) and any other garbage fail the strict `end()`. The
/// 1.1 magic-code line `##CIF_2.0` is an inert comment here; the
/// 2.0 rules ride on the `dialect` flag.
/// The leading white space and comment region of the file, held verbatim
/// for the document-level writer hook. The region runs from the start of
/// the file to the first content line. It is captured from the source the
/// parser consumed, so it never diverges from the white space scan.
fn head_region<'src>() -> impl Parser<'src, &'src str, Option<Comments>, Err<'src>> {
    custom::<_, &'src str, Option<Comments>, Err<'src>>(move |inp| {
        let full = inp.full_slice();
        let consumed_chars = consume_ws(inp, true, false);
        let region: String = full.chars().take(consumed_chars).collect();
        Ok(if region.trim().is_empty() {
            None
        } else {
            Some(Comments::new(region))
        })
    })
}

fn cif_file<'src>(dialect: CifDialect) -> impl Parser<'src, &'src str, CifDocument, Err<'src>> {
    head_region()
        .then(
            data_block(dialect)
                .separated_by(ws_region(false).map(|_| ()))
                .collect::<Vec<DataBlock>>(),
        )
        .then(ws_region(false).map(|_| ()))
        .then(end())
        .map(|(((comments, blocks), _tail), _end)| CifDocument::new(comments, Some(blocks)))
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
fn reserved_word_as_value(source: &str, toks: &[CifTok]) -> Vec<(Range<usize>, String)> {
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
fn loop_without_tags(source: &str, toks: &[CifTok]) -> Vec<(Range<usize>, String)> {
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

    /// The single-value item of `tag` in the first block that holds
    /// it.
    fn single_item<'a>(doc: &'a CifDocument, tag: &'a str) -> &'a Value {
        for block in doc.data_blocks().expect("data blocks") {
            for member in block.members() {
                if let DataBlockMember::DataItems(DataItems::SingleValue(item)) = member {
                    if item.tag().as_str() == tag {
                        return item.value();
                    }
                }
            }
        }
        panic!("tag {tag} not found")
    }

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
            msgs.iter()
                .any(|m| m.contains("a loop must name at least one tag")),
            "expected the empty-loop diagnostic, got {msgs:?}"
        );
    }

    #[test]
    fn parse_cif_surfaces_the_custom_diagnostics() {
        let src = "data_d\n_a save_foo\n";
        let errs = parse_cif(src).expect_err("must fail");
        assert!(
            errs.iter()
                .any(|e| e.reason().to_string().contains("may not begin a value")),
            "the custom diagnostic must reach the caller, got {errs:?}"
        );
    }

    #[test]
    fn a_valid_document_stays_ok() {
        let src = "data_ok\n _a 1\n _b 2.5\n";
        assert!(parse_cif(src).is_ok(), "a valid document must parse");
    }

    #[test]
    fn dialect_detection_reads_the_magic_code() {
        assert_eq!(
            CifDialect::detect("##CIF_2.0\ndata_d\n_a 1\n"),
            CifDialect::Cif2_0
        );
        // The byte order mark precedes the magic code.
        assert_eq!(
            CifDialect::detect("\u{feff}##CIF_2.0\ndata_d\n_a 1\n"),
            CifDialect::Cif2_0
        );
        // Anything else is 1.1, including a magic-code-looking line
        // that is not at position zero.
        assert_eq!(
            CifDialect::detect("##CIF_1.1\ndata_d\n_a 1\n"),
            CifDialect::Cif1_1
        );
        assert_eq!(CifDialect::detect("data_d\n_a 1\n"), CifDialect::Cif1_1);
        assert_eq!(
            CifDialect::detect("# a line\n##CIF_2.0\ndata_d\n_a 1\n"),
            CifDialect::Cif1_1
        );
        // The escaped COMCIFS spelling of the 2.0 version code, with
        // and without a byte order mark.
        assert_eq!(
            CifDialect::detect("#\\#CIF_2.0\ndata_d\n_a 1\n"),
            CifDialect::Cif2_0
        );
        assert_eq!(
            CifDialect::detect("\u{feff}#\\#CIF_2.0\ndata_d\n_a 1\n"),
            CifDialect::Cif2_0
        );
        // The escaped 1.1 spelling stays 1.1, which the default
        // already is.
        assert_eq!(
            CifDialect::detect("#\\#CIF_1.1\ndata_d\n_a 1\n"),
            CifDialect::Cif1_1
        );
    }

    #[test]
    fn schema_versions_follow_the_dialect() {
        assert_eq!(CifDialect::Cif1_1.schema_version(), 1);
        assert_eq!(CifDialect::Cif2_0.schema_version(), 2);
    }

    #[test]
    fn a_2_0_file_parses_lists() {
        let src = "##CIF_2.0\ndata_d\n_flat [1 2.5 -3]\n_nested [[1 2] [3 4]]\n_matrix [[1 2 3] [4 5 6] [7 8 9]]\n_empty []\n_strings ['a' \"b\"]\n";
        let doc = parse_cif(src).expect("2.0 lists parse");
        match single_item(&doc, "empty") {
            Value::List(values) => assert!(values.is_empty()),
            other => panic!("expected an empty list, got {other:?}"),
        }
        match single_item(&doc, "nested") {
            Value::List(rows) => {
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0].as_list().expect("row is a list").len(), 2);
            }
            other => panic!("expected a nested list, got {other:?}"),
        }
        match single_item(&doc, "matrix") {
            Value::List(rows) => {
                assert_eq!(rows.len(), 3);
                for row in rows {
                    assert_eq!(row.as_list().expect("row is a list").len(), 3);
                }
            }
            other => panic!("expected a matrix list, got {other:?}"),
        }
        match single_item(&doc, "strings") {
            Value::List(values) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0].as_char_string().expect("quoted").as_ref(), "a");
            }
            other => panic!("expected a string list, got {other:?}"),
        }
    }

    #[test]
    fn a_2_0_table_holds_key_value_pairs() {
        let src = "##CIF_2.0
data_t
_a { 'zero':0 'one':. }
_e {}
";
        let doc = parse_cif(src).expect("a 2.0 table must parse");
        match single_item(&doc, "a") {
            Value::Table(pairs) => {
                assert_eq!(pairs.len(), 2);
                match &pairs[0].0 {
                    Value::CharString(cs) => assert_eq!(cs.as_ref(), "zero"),
                    other => panic!("expected a key, got {other:?}"),
                }
            }
            other => panic!("expected a table, got {other:?}"),
        }
        match single_item(&doc, "e") {
            Value::Table(pairs) => assert!(pairs.is_empty()),
            other => panic!("expected an empty table, got {other:?}"),
        }
    }

    #[test]
    fn a_2_0_triple_quoted_string_keeps_content_verbatim() {
        let src = "##CIF_2.0\ndata_t\n_a '''first\nsecond'''\n_b ''''four'''\n_c ''''''\n";
        let doc = parse_cif(src).expect("a 2.0 triple-quoted string must parse");
        match single_item(&doc, "a") {
            Value::CharString(cs) => assert_eq!(cs.as_ref(), "first\nsecond"),
            other => panic!("expected a triple string, got {other:?}"),
        }
        match single_item(&doc, "b") {
            Value::CharString(cs) => assert_eq!(cs.as_ref(), "'four"),
            other => panic!("expected a triple string, got {other:?}"),
        }
        match single_item(&doc, "c") {
            Value::CharString(cs) => assert_eq!(cs.as_ref(), ""),
            other => panic!("expected an empty triple string, got {other:?}"),
        }
    }

    #[test]
    fn a_1_1_file_rejects_a_table() {
        let src = "data_t
_a { 'zero':0 }
";
        let errs = parse_cif(src).expect_err("a table in a 1.1 file must be rejected");
        assert!(!errs.is_empty(), "a Rich error must reach the caller");
    }

    #[test]
    fn a_2_0_file_nests_save_frames() {
        let src = "##CIF_2.0\ndata_t\nsave_a\nsave_b\nsave_\n_level 1\nsave_\n";
        let doc = parse_cif(src).expect("a 2.0 nested frame must parse");
        let block = doc
            .data_blocks()
            .expect("one block")
            .first()
            .expect("a block");
        match block.members().first().expect("one member") {
            DataBlockMember::SaveFrame(frame) => {
                assert_eq!(frame.data_items().len(), 1);
                let child = frame.frames().first().expect("a nested frame");
                assert!(child.frames().is_empty());
            }
            other => panic!("expected a save frame, got {other:?}"),
        }
    }

    #[test]
    fn a_2_0_file_allows_unicode_in_codes_tags_and_values() {
        let src = "##CIF_2.0\ndata_ünïcode\nsave_§\n_αβγ Ω\n_π 𐘾\nsave_\n";
        let doc = parse_cif(src).expect("a 2.0 unicode file must parse");
        let block = doc
            .data_blocks()
            .expect("one block")
            .first()
            .expect("a block");
        assert_eq!(block.heading(), "ünïcode");
    }

    #[test]
    fn a_1_1_file_rejects_unicode_in_a_block_code() {
        let src = "data_ñ\n_a x\n";
        let errs = parse_cif(src).expect_err("unicode in a 1.1 block code must fail");
        assert!(!errs.is_empty(), "a Rich error must reach the caller");
    }

    #[test]
    fn a_1_1_file_rejects_nested_save_frames() {
        let src = "data_t\nsave_a\nsave_b\nsave_\nsave_\n";
        let errs = parse_cif(src).expect_err("a nested frame in a 1.1 file must fail");
        assert!(!errs.is_empty(), "a Rich error must reach the caller");
    }

    #[test]
    fn a_2_0_file_rejects_brackets_in_unquoted_values() {
        let src = "##CIF_2.0\ndata_d\n_v a[1,2]\n";
        let errs = parse_cif(src).expect_err("a bracket in an unquoted 2.0 value must fail");
        assert!(!errs.is_empty(), "a Rich error must reach the caller");
    }

    #[test]
    fn a_2_0_file_rejects_a_curly_brace_in_unquoted_values() {
        let src = "##CIF_2.0\ndata_d\n_v a{x}\n";
        assert!(
            parse_cif(src).is_err(),
            "a brace in an unquoted 2.0 value must fail"
        );
    }

    #[test]
    fn a_2_0_list_allows_a_glued_comment_and_a_text_member() {
        // Roadmap section 14: a `#` glued to the `[` starts a
        // comment (fixture 24, pinned by `list_data.cif`). A
        // semi-enclosed text field may stand as a list member.
        let src = "##CIF_2.0\ndata_d\n_a [# comment\n 1\n]\n_b [ x\n;t\n;\n ?\n]\n";
        let doc = parse_cif(src).expect("list comment and text member parse");
        match single_item(&doc, "a") {
            Value::List(values) => {
                assert_eq!(values.len(), 1);
                assert_eq!(
                    values[0].as_numeric().expect("numeric"),
                    &Numeric::from(1i32)
                );
            }
            other => panic!("expected a list, got {other:?}"),
        }
        match single_item(&doc, "b") {
            Value::List(values) => {
                assert_eq!(values.len(), 3);
                assert!(matches!(values[1], Value::TextField(_)));
            }
            other => panic!("expected a list, got {other:?}"),
        }
    }

    #[test]
    fn vt_and_ff_end_lines_in_a_1_1_file() {
        // Decision D9a (roadmap section 14): VT (0x0B) and FF
        // (0x0C) act as line terminators in every dialect. The 1.1
        // spec excludes both characters (para. 22, 28); the crate
        // accepts them, which fixture 26 pins.
        let src =
            "data_26_vtff\u{0c}_a 'x'\u{0c}_b 1.5e+02\n_t \u{0c};first\n;\n\n_v\u{0b}\u{0b}7\n";
        let doc = parse_cif(src).expect("VT and FF must end lines in 1.1");
        assert_eq!(
            single_item(&doc, "b")
                .as_numeric()
                .expect("a numeric value"),
            &Numeric::from(150.0f64),
            "the exponent form 1.5e+02 must parse as 150.0",
        );
        assert_eq!(
            single_item(&doc, "t")
                .as_text_field()
                .expect("a text field")
                .raw_string(),
            "first",
        );
        assert_eq!(
            single_item(&doc, "v")
                .as_numeric()
                .expect("a numeric value"),
            &Numeric::from(7i32),
        );
    }

    /// The `LoopColumns` of the first loop in the document.
    fn first_loop<'a>(doc: &'a CifDocument) -> &'a LoopColumns {
        for block in doc.data_blocks().expect("data blocks") {
            for member in block.members() {
                if let DataBlockMember::DataItems(items) = member {
                    if let Some(columns) = items.as_multi_values() {
                        return columns;
                    }
                }
            }
        }
        panic!("no loop found in the document")
    }

    #[test]
    fn a_cif1_1_loop_line_may_mix_quoted_and_bare_values() {
        // IUCr `atom_type` loop shape: a line holds quoted values
        // and bare values; the last column lands on its own line.
        let src = "data_d\nloop_\n_a\n_b\n_c\n_d\n_e\n'1' '2' 3 4\n5\n";
        let doc = parse_cif(src).expect("a mixed loop value line must parse");
        let columns = first_loop(&doc);
        let a = columns.find_loop_column_by_tag("a").expect("tag a");
        assert_eq!(
            a.as_ref()[0].as_char_string().expect("quoted").as_ref(),
            "1",
        );
        let b = columns.find_loop_column_by_tag("b").expect("tag b");
        assert_eq!(
            b.as_ref()[0].as_char_string().expect("quoted").as_ref(),
            "2",
        );
        let c = columns.find_loop_column_by_tag("c").expect("tag c");
        assert!(c.as_ref()[0].as_numeric().is_some(), "tag c is bare");
        let e = columns.find_loop_column_by_tag("e").expect("tag e");
        assert!(e.as_ref()[0].as_numeric().is_some(), "tag e is bare");
    }

    #[test]
    fn a_cif1_1_quoted_value_may_end_before_a_blank() {
        let src = "data_d\n_x 'C19 H19 N O3' \n";
        let doc = parse_cif(src).expect("a quoted value with a trailing blank must parse");
        let value = single_item(&doc, "x");
        assert_eq!(
            value.as_char_string().expect("quoted").as_ref(),
            "C19 H19 N O3",
        );
    }

    #[test]
    fn a_cif1_1_embedded_quote_is_kept_as_content() {
        let src = "data_d\n_sq 'don't rock the boat'\n_dq \"He said \"\"hi\"\"\"\n";
        let doc = parse_cif(src).expect("an embedded quote must parse");
        assert_eq!(
            single_item(&doc, "sq")
                .as_char_string()
                .expect("quoted")
                .as_ref(),
            "don't rock the boat",
        );
        // All four quotes are content except the final closer: the
        // quote after `He said` is followed by a quote, and each
        // inner quote is followed by a non-blank character.
        assert_eq!(
            single_item(&doc, "dq")
                .as_char_string()
                .expect("quoted")
                .as_ref(),
            "He said \"\"hi\"\"",
        );
    }

    #[test]
    fn a_cif1_1_empty_quoted_string_is_legal() {
        let src = "data_d\n_a ''\n_b \"\"\n";
        let doc = parse_cif(src).expect("empty quoted strings must parse");
        assert_eq!(
            single_item(&doc, "a")
                .as_char_string()
                .expect("quoted")
                .as_ref(),
            ""
        );
        assert_eq!(
            single_item(&doc, "b")
                .as_char_string()
                .expect("quoted")
                .as_ref(),
            ""
        );
    }

    #[test]
    fn a_cif1_1_unterminated_quoted_string_fails_to_parse() {
        let src = "data_d\n_x 'oops\n";
        assert!(
            parse_cif(src).is_err(),
            "an unterminated quoted string must fail to parse",
        );
    }

    #[test]
    fn a_1_1_file_keeps_brackets_as_plain_characters() {
        let src = "data_d\n_square [1,2,3]\n_curly {a}\n_lead ]x[\n";
        let doc = parse_cif(src).expect("1.1 brackets stay plain characters");
        for tag in ["square", "curly", "lead"] {
            match single_item(&doc, tag) {
                Value::CharString(cs) => assert!(
                    matches!(cs, CharString::Unquoted(_)),
                    "{tag} must stay an unquoted char string, got {cs:?}"
                ),
                other => panic!("expected an unquoted char string, got {other:?}"),
            }
        }
        assert_eq!(
            single_item(&doc, "square")
                .as_char_string()
                .expect("char string")
                .as_ref(),
            "[1,2,3]"
        );
    }

    #[test]
    fn the_magic_code_is_an_inert_comment_in_1_1() {
        // The magic-code line after a comment line is not at position
        // zero: the file stays 1.1 and the brackets stay characters.
        let src = "# note\n##CIF_2.0\ndata_d\n_v [1,2,3]\n";
        let doc = parse_cif(src).expect("an inert magic-code line must not flip the dialect");
        assert_eq!(
            single_item(&doc, "v")
                .as_char_string()
                .expect("char string")
                .as_ref(),
            "[1,2,3]"
        );
    }

    #[test]
    fn a_byte_order_mark_does_not_hide_the_magic_code() {
        let src = "\u{feff}##CIF_2.0\ndata_d\n_v []\n";
        let doc = parse_cif(src).expect("a BOM-prefixed 2.0 file must parse");
        match single_item(&doc, "v") {
            Value::List(values) => assert!(values.is_empty()),
            other => panic!("expected a list, got {other:?}"),
        }
    }

    #[test]
    fn a_list_spanning_lines_in_2_0_parses() {
        let src = "##CIF_2.0\ndata_d\n_v [1\n # a comment inside the list\n 2]\n";
        let doc = parse_cif(src).expect("a multi-line 2.0 list must parse");
        match single_item(&doc, "v") {
            Value::List(values) => assert_eq!(values.len(), 2),
            other => panic!("expected a two-element list, got {other:?}"),
        }
    }
}
