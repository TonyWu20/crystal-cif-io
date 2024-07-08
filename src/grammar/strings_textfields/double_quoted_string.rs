use std::fmt::Display;

use winnow::{
    combinator::{peek, preceded, repeat_till, terminated},
    error::StrContext,
    Parser,
};

use crate::grammar::{
    character_sets::{AnyPrintChar, DoubleQuote},
    whitespace_comments::WhiteSpace,
    SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct DoubleQuotedString {
    content: String,
}

impl Display for DoubleQuotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl DoubleQuotedString {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for DoubleQuotedString {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(
            DoubleQuote::parser.context(StrContext::Label("<single_quote> Open")),
            repeat_till(
                0..,
                AnyPrintChar::parser.context(StrContext::Label(
                    "AnyPrintChar, single quote not following white space",
                )),
                terminated(DoubleQuote::parser, peek(WhiteSpace::parser))
                    .context(StrContext::Label("<single_quote><WhiteSpace>")),
            )
            .map(|(s, _open_quote): (String, DoubleQuote)| s)
            .context(StrContext::Label("AnyPrintChar")),
            // DoubleQuote::parser.context(StrContext::Label("<single_quote> Close")),
        )
        .map(DoubleQuotedString::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        format!(r#""{}""#, self.content)
    }
}

#[cfg(test)]
mod test {
    // use winnow::{
    //     combinator::{preceded, repeat_till, terminated},
    //     error::StrContext,
    //     Parser,
    // };

    // use crate::grammar::{
    //     character_sets::{AnyPrintChar, DoubleQuote},
    //     whitespace_comments::WhiteSpace,
    //     SyntacticUnit,
    // };

    use super::DoubleQuotedString;
    use crate::grammar::SyntacticUnit;

    #[test]
    fn double_quoted_string() {
        let mut input = "\"\\f scans, and \\w scans with \\k offsets Mr. Evan\"s things\"
";
        // let mut parser = preceded(
        //     DoubleQuote::parser.context(StrContext::Label("<single_quote> Open")),
        //     repeat_till(
        //         0..,
        //         AnyPrintChar::parser.context(StrContext::Label(
        //             "AnyPrintChar, single quote not following white space",
        //         )),
        //         terminated(DoubleQuote::parser, WhiteSpace::parser),
        //     )
        //     .map(|(s, _open_quote): (String, DoubleQuote)| s)
        //     .context(StrContext::Label("AnyPrintChar")),
        //     // DoubleQuote::parser.context(StrContext::Label("<single_quote> Close")),
        // );

        match DoubleQuotedString::parser(&mut input) {
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
