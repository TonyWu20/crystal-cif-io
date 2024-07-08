use std::fmt::Display;

use winnow::{
    combinator::{alt, repeat},
    stream::AsChar,
    PResult, Parser,
};

use crate::grammar::{
    character_sets::{Eol, NonBlankChar, NotEol, OrdinaryChar},
    SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct UnquotedString {
    content: String,
}

impl AsRef<str> for UnquotedString {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.content)
    }
}

impl UnquotedString {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

fn not_eol_unquoted(input: &mut &str) -> PResult<UnquotedString> {
    (
        NotEol::parser.verify(|c| c.char() != '_'),
        alt((OrdinaryChar::parser.map(|oc| oc.as_char()), ';')),
        repeat::<_, _, String, _, _>(0.., NonBlankChar::parser),
    )
        .map(|(_not_eol, c, content)| UnquotedString::new(format!("{c}{content}")))
        .parse_next(input)
}

fn eol_unquoted(input: &mut &str) -> PResult<UnquotedString> {
    (
        Eol::parser,
        OrdinaryChar::parser,
        repeat::<_, _, String, _, _>(0.., NonBlankChar::parser),
    )
        .map(|(_eol, oc, content)| UnquotedString::new(format!("{oc}{content}")))
        .parse_next(input)
}

pub fn pure_unquoted(input: &mut &str) -> PResult<UnquotedString> {
    (
        OrdinaryChar::parser,
        repeat::<_, _, String, _, _>(0.., NonBlankChar::parser),
    )
        .map(|(oc, content)| UnquotedString::new(format!("{oc}{content}")))
        .parse_next(input)
}

impl SyntacticUnit for UnquotedString {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((not_eol_unquoted, eol_unquoted)).parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.content.to_owned()
    }
}

impl Display for UnquotedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::{whitespace_comments::WhiteSpace, SyntacticUnit};

    use super::UnquotedString;

    #[test]
    fn unquoted_string() {
        let mut input = " rm # known chiral centre
";
        let value = UnquotedString::parser(&mut input).unwrap();
        let white_space = WhiteSpace::parser(&mut input).unwrap();
        println!("{value}{white_space}");
    }
}
