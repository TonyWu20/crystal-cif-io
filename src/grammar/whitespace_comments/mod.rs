mod comments;

use std::fmt::Display;

pub use comments::{Comments, TokenizedComments};
use winnow::{
    combinator::{alt, repeat},
    stream::{Accumulate, AsChar},
    Parser,
};

use super::{character_sets::LeadingBlank, SyntacticUnit};

enum WhiteSpaceItem {
    LeadingBlank(LeadingBlank),
    TokenizedComments(TokenizedComments),
}

impl Accumulate<WhiteSpaceItem> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: WhiteSpaceItem) {
        match acc {
            WhiteSpaceItem::LeadingBlank(lb) => self.push(lb.as_char()),
            WhiteSpaceItem::TokenizedComments(t) => {
                *self = [self.clone(), t.formatted_output()].concat()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhiteSpace {
    content: String,
}

impl WhiteSpace {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl SyntacticUnit for WhiteSpace {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        repeat(
            1..,
            alt((
                TokenizedComments::parser.map(WhiteSpaceItem::TokenizedComments),
                LeadingBlank::parser.map(WhiteSpaceItem::LeadingBlank),
            )),
        )
        .map(WhiteSpace::new)
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        self.content.to_owned()
    }
}

impl Display for WhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {

    use crate::grammar::SyntacticUnit;

    use super::{comments::TokenizedComments, Comments};

    #[test]
    fn comments() {
        let mut input = "#==============================================================
# 1. SUBMISSION DETAILS
#===================================================================
";
        let comment = Comments::parser(&mut input);
        if let Ok(comment) = comment {
            println!("{}", comment);
        }
        let mut input = " #File kfb29v03.cif
";
        let tokenized_comment = TokenizedComments::parser(&mut input);
        if let Ok(comment) = tokenized_comment {
            println!("{}", comment);
        }
    }
}
