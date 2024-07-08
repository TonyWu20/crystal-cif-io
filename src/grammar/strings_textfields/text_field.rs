use std::fmt::Display;

use winnow::{
    combinator::{delimited, opt, preceded, repeat, terminated},
    error::StrContext,
    PResult, Parser,
};

use crate::grammar::{
    character_sets::{AnyPrintChar, Eol, TextLeadChar},
    whitespace_comments::WhiteSpace,
    SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct SemiColonTextField {
    lines: Vec<String>,
}

/// In the present revision the production for <TextField> is a trivial
/// equivalence to <SemiColonTextField>. The redundancy is retained to permit
/// possible future extensions to text fields, in particular the possible
/// introduction of a bracket-delimited text value.
#[derive(Debug, Clone)]
pub enum TextField {
    SemiColonTextField(SemiColonTextField),
}

impl Display for TextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextField::SemiColonTextField(t) => write!(f, "{t}"),
        }
    }
}

impl SemiColonTextField {
    pub fn new(lines: Vec<String>) -> Self {
        Self { lines }
    }
}

impl SyntacticUnit for SemiColonTextField {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(
            opt(WhiteSpace::parser),
            delimited(
                ';'.context(StrContext::Label(";Begins")),
                semicolon_text_field.context(StrContext::Label("Text field")),
                ';'.context(StrContext::Label(";close")),
            )
            .context(StrContext::Label("<SemiColonTextField>")),
        )
        .map(SemiColonTextField::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("\n;{}\n;", self.lines.join("\n"))
    }
}

impl Display for SemiColonTextField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

fn semicolon_text_field(input: &mut &str) -> PResult<Vec<String>> {
    (
        terminated(repeat(0.., AnyPrintChar::parser), Eol::parser)
            .context(StrContext::Label("{<AnyPrintChar>}*<eol>")),
        repeat(
            0..,
            terminated(
                repeat(
                    0..=1,
                    (TextLeadChar::parser, repeat(0.., AnyPrintChar::parser))
                        .context(StrContext::Label("<TextLeadChar>{<AnyPrintChar>}*"))
                        .map(|(lead, follows): (TextLeadChar, String)| format!("{lead}{follows}")),
                )
                .context(StrContext::Label("{<TextLeadChar>{<AnyPrintChar>}*}?"))
                .map(|line: Vec<String>| line.concat()),
                Eol::parser.context(StrContext::Label("Each line eol")),
            )
            .context(StrContext::Label("{<TextLeadChar>{<AnyPrintChar>}*}?<eol>")),
        )
        .context(StrContext::Label(
            "{{<TextLeadChar>{<AnyPrintChar>}*}?<eol>}*",
        )),
    )
        .map(|(first_line, following)| [vec![first_line], following].concat())
        .parse_next(input)
}

impl SyntacticUnit for TextField {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> PResult<Self::ParseResult> {
        SemiColonTextField::parser
            .map(TextField::SemiColonTextField)
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            TextField::SemiColonTextField(f) => f.to_string(),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::grammar::{strings_textfields::text_field::SemiColonTextField, SyntacticUnit};

    #[test]
    fn text_field_test() {
        let mut test = r#";
meso-5,5,7,12,12,14-Hexamethyl-4,11-diaza-1,8-diazoniacyclotetradecane 
(S)-malate(2-) methanol disolvate
Equimolar quantities of tet-a (Hay et al., 1975) and (S)-malic acid 
(purchased from Aldrich) were separately dissolved in methanol. The 
solutions were mixed, and the mixture was set aside to crystallise, 
providing analytically pure (I). Analysis found: C 54.7, H 11.0, N 11.7%; 
C~22~H~50~N~4~O~7~ requires: C 54.7, H 10.4, N 11.6%. Crystals of (I) 
suitable for single-crystal X-ray diffraction were selected directly 
from the sample as prepared.
;
_vrf_PLAT_113_I
;
PROBLEM: _A ADDSYM Suggests Possible Pseudo/New Spacegroup       P-1          
RESPONSE: .See above
;
"#;
        match SemiColonTextField::parser(&mut test) {
            Ok(t) => println!("{t}"),
            Err(e) => {
                dbg!(e);
                println!("{test}")
            }
        }
    }
}
