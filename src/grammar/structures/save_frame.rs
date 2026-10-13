use std::fmt::Display;

use crate::grammar::SyntacticUnit;

use crate::grammar::structures::data_items::DataItems;

#[derive(Debug, Clone)]
pub struct SaveFrame {
    heading: SaveFrameHeading,
    data_items: Vec<DataItems>,
    /// Nested save frames (CIF 2.0 only, roadmap section 14). Empty
    /// in every 1.1 file.
    frames: Vec<SaveFrame>,
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
            frames: Vec::new(),
        }
    }

    /// Build a frame with its items and its nested frames (CIF 2.0
    /// only, roadmap section 14). The items and frames are the
    /// frame content, split by kind.
    pub fn from_heading_items_frames(
        (heading, data_items, frames): (SaveFrameHeading, Vec<DataItems>, Vec<SaveFrame>),
    ) -> Self {
        Self {
            heading,
            data_items,
            frames,
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

    /// The nested frames of this frame (empty in 1.1 files).
    pub fn frames(&self) -> &[SaveFrame] {
        &self.frames
    }
}

impl crate::grammar::SyntacticUnit for SaveFrame {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        // The items come before the nested frames. Both splits
        // re-parse to the same frame, so the round trip is stable.
        let items = self
            .data_items
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let frames = self
            .frames
            .iter()
            .map(|frame| frame.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let mut body = items;
        if !frames.is_empty() {
            if !body.is_empty() {
                body.push_str("\n");
            }
            body.push_str(&frames);
        }
        format!("save_{}\n{}\nsave_", self.heading.as_ref(), body)
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
