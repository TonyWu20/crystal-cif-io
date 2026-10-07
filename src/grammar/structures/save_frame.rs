use std::fmt::Display;

use crate::grammar::SyntacticUnit;

use crate::grammar::structures::data_items::DataItems;

#[derive(Debug, Clone)]
pub struct SaveFrame {
    heading: SaveFrameHeading,
    data_items: Vec<DataItems>,
}

#[derive(Debug, Clone)]
pub struct SaveFrameHeading {
    code: String,
}

impl AsRef<str> for SaveFrameHeading {
    #[inline]
    fn as_ref(&self) -> &str {
        <String as AsRef<str>>::as_ref(&self.code)
    }
}

impl SaveFrame {
    pub fn from_heading_items(heading_items: (SaveFrameHeading, Vec<DataItems>)) -> Self {
        let (heading, data_items) = heading_items;
        Self {
            heading,
            data_items,
        }
    }

    pub fn heading(&self) -> &SaveFrameHeading {
        &self.heading
    }

    pub fn data_items(&self) -> &[DataItems] {
        &self.data_items
    }

    pub fn data_items_mut(&mut self) -> &mut Vec<DataItems> {
        &mut self.data_items
    }
}

impl crate::grammar::SyntacticUnit for SaveFrame {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        let items = self
            .data_items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        format!("save_{}\n{}\nsave_", self.heading.as_ref(), items)
    }
}

impl Display for SaveFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}

impl SaveFrameHeading {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

impl crate::grammar::SyntacticUnit for SaveFrameHeading {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        format!("save_{}\n", self.as_ref())
    }
}

impl Display for SaveFrameHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_output())
    }
}
