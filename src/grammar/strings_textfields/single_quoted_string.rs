use std::fmt::Display;

use winnow::{
    combinator::{peek, preceded, repeat_till, terminated},
    error::StrContext,
    Parser,
};

use crate::grammar::{
    character_sets::{AnyPrintChar, SingleQuote},
    whitespace_comments::WhiteSpace,
    SyntacticUnit,
};

use super::CharString;

#[derive(Debug, Clone)]
pub struct SingleQuotedString {
    content: String,
}

impl Display for SingleQuotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl SingleQuotedString {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for SingleQuotedString {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(
            SingleQuote::parser.context(StrContext::Label("<single_quote> Open")),
            repeat_till(
                0..,
                AnyPrintChar::parser.context(StrContext::Label(
                    "AnyPrintChar, single quote not following white space",
                )),
                terminated(SingleQuote::parser, peek(WhiteSpace::parser))
                    .context(StrContext::Label("<single_quote><WhiteSpace>")),
            )
            .map(|(s, _open_quote): (String, SingleQuote)| s)
            .context(StrContext::Label("AnyPrintChar")),
            // SingleQuote::parser.context(StrContext::Label("<single_quote> Close")),
        )
        .map(SingleQuotedString::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("'{}'", self.content)
    }
}

impl From<SingleQuotedString> for CharString {
    fn from(value: SingleQuotedString) -> Self {
        Self::SingleQuoted(value)
    }
}

#[cfg(test)]
mod test {
    use winnow::{
        combinator::{preceded, repeat_till, terminated},
        error::StrContext,
        Parser,
    };

    use crate::grammar::{
        character_sets::{AnyPrintChar, SingleQuote},
        whitespace_comments::WhiteSpace,
        SyntacticUnit,
    };

    use super::SingleQuotedString;

    // use super::SingleQuotedString;
    // use crate::grammar::SyntacticUnit;

    #[test]
    fn single_quoted_string() {
        let mut input = "'f scans, and w scans with k offsets Mr. Evan's things'
";
        let mut input_2 = "'C16 H38 N4 2+, C4 H4 O5 2-, 2C H4 O'";
        let mut parser = preceded(
            SingleQuote::parser.context(StrContext::Label("<single_quote> Open")),
            repeat_till(
                0..,
                AnyPrintChar::parser.context(StrContext::Label(
                    "AnyPrintChar, single quote not following white space",
                )),
                terminated(SingleQuote::parser, WhiteSpace::parser),
            )
            .map(|(s, _open_quote): (String, SingleQuote)| s)
            .context(StrContext::Label("AnyPrintChar")),
            // SingleQuote::parser.context(StrContext::Label("<single_quote> Close")),
        );

        match SingleQuotedString::parser(&mut input) {
            // match parser.parse_next(&mut input) {
            Ok(s) => {
                println!("{s}");
                dbg!(input);
            }
            Err(d) => {
                dbg!(d);
                dbg!(input);
            }
        }
        match parser.parse_next(&mut input_2) {
            // match parser.parse_next(&mut input) {
            Ok(s) => {
                println!("{s}");
                dbg!(input);
            }
            Err(d) => {
                dbg!(d);
                dbg!(input);
            }
        }
    }
}
