use std::fmt::Display;

use winnow::Parser;

use crate::grammar::SyntacticUnit;

use self::{body::LoopBody, header::LoopHeader};

mod body;
mod header;

#[derive(Debug, Clone)]
pub struct LoopUnit {
    header: LoopHeader,
    body: LoopBody,
}

impl LoopUnit {
    pub fn new(header: LoopHeader, body: LoopBody) -> Self {
        Self { header, body }
    }

    pub fn header(&self) -> &LoopHeader {
        &self.header
    }
}

impl SyntacticUnit for LoopUnit {
    type ParseResult = Self;

    type FormatOutput = String;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        (LoopHeader::parser, LoopBody::parser)
            .map(|(header, body)| LoopUnit::new(header, body))
            .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        let body_output = self
            .body
            .values()
            .chunks(self.header.num_of_tags())
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        format!("{}{}", self.header, body_output)
    }
}

impl Display for LoopUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::SyntacticUnit;

    use super::LoopUnit;

    #[test]
    fn loop_data_parsing() {
        let mut input = r#"loop_
 _atom_type_symbol
 _atom_type_description
 _atom_type_scat_dispersion_real
 _atom_type_scat_dispersion_imag
 _atom_type_scat_source
 'C'  'C'   0.0033   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'
 'H'  'H'   0.0000   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'
 'N'  'N'   0.0061   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'
 'O'  'O'   0.0106   0.0000
 'International Tables Vol C Tables 4.2.6.8 and 6.1.1.4'

_symmetry_cell_setting            triclinic
_symmetry_space_group_name_H-M    'P 1'
_symmetry_space_group_name_Hall   'P 1'
"#;
        let parse_result = LoopUnit::parser(&mut input);
        match parse_result {
            Ok(l) => {
                l.body.values().iter().for_each(|v| println!("{v:?}"));
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}
