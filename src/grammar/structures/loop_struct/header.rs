use std::fmt::Display;

use winnow::{
    combinator::{preceded, repeat},
    Parser,
};

use crate::grammar::{
    reserved_words::Loop, tags_values::Tag, whitespace_comments::WhiteSpace, SyntacticUnit,
};

#[derive(Debug, Clone)]
pub struct LoopHeader {
    tags: Vec<Tag>,
}

impl LoopHeader {
    pub fn new(tags: Vec<Tag>) -> Self {
        Self { tags }
    }
    pub fn num_of_tags(&self) -> usize {
        self.tags.len()
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn get_tag_index<T: AsRef<str>>(&self, tag: T) -> Option<usize> {
        self.tags().iter().position(|t| t.as_ref() == tag.as_ref())
    }
}

impl SyntacticUnit for LoopHeader {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        preceded(
            Loop::parser,
            repeat(1.., preceded(WhiteSpace::parser, Tag::parser)),
        )
        .map(LoopHeader::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        let tags = self
            .tags
            .iter()
            .map(|t| format!("{t}"))
            .collect::<Vec<String>>()
            .join("\n");
        format!("loop_\n{tags}")
    }
}

impl Display for LoopHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::{structures::loop_struct::header::LoopHeader, SyntacticUnit};

    #[test]
    fn loop_header() {
        let mut input = "loop_
 _atom_site_aniso_label
 _atom_site_aniso_U_11
 _atom_site_aniso_U_22
 _atom_site_aniso_U_33
 _atom_site_aniso_U_23
 _atom_site_aniso_U_13
 _atom_site_aniso_U_12
";
        println!("{}", LoopHeader::parser(&mut input).unwrap());
    }
}
