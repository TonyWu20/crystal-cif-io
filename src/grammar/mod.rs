use std::fmt::Display;

use winnow::{
    combinator::{opt, preceded, repeat, terminated},
    PResult, Parser,
};

use self::{
    structures::DataBlock,
    whitespace_comments::{Comments, WhiteSpace},
};

mod character_sets;
mod numeric_values;
mod reserved_words;
mod strings_textfields;
mod structures;
mod tags_values;
mod whitespace_comments;

pub trait SyntacticUnit {
    type ParseResult;
    type FormatOutput: Display;
    fn parser(input: &mut &str) -> PResult<Self::ParseResult>;
    fn formatted_output(&self) -> Self::FormatOutput;
}

#[derive(Debug, Clone)]
pub struct CifDocument {
    comments: Option<Comments>,
    data_blocks: Option<Vec<DataBlock>>,
}

impl CifDocument {
    pub fn new(comments: Option<Comments>, data_blocks: Option<Vec<DataBlock>>) -> Self {
        Self {
            comments,
            data_blocks,
        }
    }
}

impl SyntacticUnit for CifDocument {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> PResult<Self::ParseResult> {
        let comments = opt(Comments::parser).parse_next(input)?;
        opt(WhiteSpace::parser).parse_next(input)?;
        let data_blocks_parsing = |input: &mut &str| -> PResult<Vec<DataBlock>> {
            terminated(
                (
                    DataBlock::parser,
                    repeat(0.., preceded(WhiteSpace::parser, DataBlock::parser)),
                ),
                opt(WhiteSpace::parser),
            )
            .map(|(first, mut others): (DataBlock, Vec<DataBlock>)| {
                others.insert(0, first);
                others
            })
            .parse_next(input)
        };

        let data_blocks = opt(data_blocks_parsing).parse_next(input)?;
        Ok(Self::new(comments, data_blocks))
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        let comment = match &self.comments {
            Some(c) => c.to_string(),
            None => "#\\#CIF_1.1".to_string(),
        };
        let data_blocks = match &self.data_blocks {
            Some(blocks) => blocks
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            None => String::new(),
        };
        [comment, data_blocks].join("\n")
    }
}

impl Display for CifDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use std::fs::{read_to_string, write};

    use super::{CifDocument, SyntacticUnit};

    #[test]
    fn cif_doc_parsing() {
        let home = std::env::var("HOME").unwrap();
        let example = format!("{home}/Downloads/example.cif");
        let content = read_to_string(example).expect("Reading error");
        match CifDocument::parser(&mut content.as_str()) {
            Ok(cif) => {
                let output_path = "cif_parse_test.cif";
                write(output_path, cif.to_string()).expect("Error during writing test output.")
            }
            Err(e) => {
                dbg!(e);
            }
        }
        let content_2 = read_to_string("cif_parse_test.cif").expect("written out cif");
        match CifDocument::parser(&mut content_2.as_str()) {
            Ok(cif) => {
                let output_path = "cif_parse_test_2.cif";
                write(output_path, cif.to_string()).expect("Error during writing test output.")
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
