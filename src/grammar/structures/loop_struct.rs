use std::fmt::Display;

use winnow::Parser;

use crate::{
    data_dict::CifTerm,
    grammar::{tags_values::Value, SyntacticUnit, Tag},
};

use self::{body::LoopBody, header::LoopHeader};

pub use body::LoopColumn;

mod body;
mod header;

#[derive(Debug, Clone)]
pub struct LoopUnit {
    header: LoopHeader,
    body: LoopBody,
}

#[derive(Debug, Clone, Default)]
pub struct LoopUnitBuilder {
    value_columns: Option<Vec<LoopColumn>>,
    column_length: usize,
}

#[derive(Debug, Clone, Default)]
pub struct LoopColumns {
    columns: Vec<LoopColumn>,
}

impl LoopUnitBuilder {
    pub fn with_value_columns(mut self, value_columns: Vec<LoopColumn>) -> Self {
        self.column_length = value_columns[0].as_ref().len();
        self.value_columns = Some(value_columns);
        self
    }

    pub fn build(self) -> LoopUnit {
        let header = self
            .value_columns
            .as_ref()
            .map(|columns| {
                columns
                    .iter()
                    .map(|c| c.tag().clone())
                    .collect::<Vec<Tag>>()
            })
            .unwrap_or_default();
        let header = LoopHeader::new(header);
        let body = LoopBody::from_columns(&self.value_columns.unwrap(), self.column_length);
        LoopUnit::new(header, body)
    }
}

impl LoopUnit {
    pub fn new(header: LoopHeader, body: LoopBody) -> Self {
        Self { header, body }
    }

    pub fn builder() -> LoopUnitBuilder {
        LoopUnitBuilder::default()
    }

    pub fn header(&self) -> &LoopHeader {
        &self.header
    }

    pub fn find_loop_column_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<LoopColumn> {
        self.header.get_tag_index(&tag).map(|index| {
            let values = self
                .body
                .nth_column_values(index, self.header.num_of_tags());
            LoopColumn::new(Tag::new(tag.as_ref().to_string()), values)
        })
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

impl LoopColumns {
    pub fn find_loop_column_by_tag<T: AsRef<str>>(&self, tag: T) -> Option<&LoopColumn> {
        self.columns
            .iter()
            .find(|col| col.tag().as_ref() == tag.as_ref())
    }
}

impl From<LoopColumns> for LoopUnit {
    fn from(value: LoopColumns) -> Self {
        LoopUnit::builder()
            .with_value_columns(value.columns)
            .build()
    }
}

impl From<&LoopColumns> for LoopUnit {
    fn from(value: &LoopColumns) -> Self {
        LoopUnit::builder()
            .with_value_columns(value.columns.clone())
            .build()
    }
}

impl From<LoopUnit> for LoopColumns {
    fn from(value: LoopUnit) -> Self {
        let column_width = value.header().tags().len();
        LoopColumns {
            columns: value
                .header()
                .tags()
                .iter()
                .enumerate()
                .map(|(i, tag)| {
                    LoopColumn::new(tag.clone(), value.body.nth_column_values(i, column_width))
                })
                .collect::<Vec<LoopColumn>>(),
        }
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
