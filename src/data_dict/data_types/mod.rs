use std::fmt::{Debug, Display};

pub mod num_value;

pub trait DataLabel: Clone {
    fn category_prefix(&self) -> String;
    fn tag(&self) -> String;
    fn value_string(&self) -> String;
    fn full_label(&self) -> String {
        let label = format!("_{}_{}", self.category_prefix(), self.tag());
        format!("{label:<32}")
    }
    fn single_line_output(&self) -> String {
        format!("{}  {}", self.full_label(), self.value_string())
    }
}

pub trait LoopDataLabel: DataLabel {}

#[derive(Debug, Clone)]
pub struct SingleValueSection<T: DataLabel> {
    entries: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct SingleValueSectionBuilder<T: DataLabel> {
    entries: Vec<T>,
}

impl<T: DataLabel> Default for SingleValueSectionBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: DataLabel> SingleValueSectionBuilder<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add_entry(mut self, entry: T) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn finish(self) -> SingleValueSection<T> {
        SingleValueSection::new(self.entries)
    }
}

impl<T: DataLabel> Display for SingleValueSection<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = self
            .entries
            .iter()
            .map(|entry| entry.single_line_output())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{content}")
    }
}

impl<T: DataLabel> SingleValueSection<T> {
    pub fn new(entries: Vec<T>) -> Self {
        Self { entries }
    }
    pub fn init_builder() -> SingleValueSectionBuilder<T> {
        SingleValueSectionBuilder::new()
    }

    pub fn entries(&self) -> &[T] {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut Vec<T> {
        &mut self.entries
    }

    pub fn set_entries(&mut self, entries: Vec<T>) {
        self.entries = entries;
    }
}

#[derive(Debug, Clone)]
pub struct LoopDataEntry<T: LoopDataLabel> {
    entries: Vec<T>,
}

impl<T: LoopDataLabel> LoopDataEntry<T> {
    pub fn new(entries: Vec<T>) -> Self {
        Self { entries }
    }

    pub fn init_builder() -> LoopDataEntryBuilder<T> {
        LoopDataEntryBuilder::new()
    }

    pub fn relevant_labels(&self) -> String {
        self.entries
            .iter()
            .map(|entry| entry.full_label())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct LoopDataEntryBuilder<T: LoopDataLabel> {
    entries: Vec<T>,
}

impl<T: LoopDataLabel> Default for LoopDataEntryBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: LoopDataLabel> LoopDataEntryBuilder<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(mut self, entry: T) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn finish(self) -> LoopDataEntry<T> {
        LoopDataEntry::new(self.entries)
    }
}

impl<T: LoopDataLabel> Display for LoopDataEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = self
            .entries
            .iter()
            .map(|entry| entry.value_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{line}")
    }
}

#[derive(Debug, Clone)]
pub struct LoopData<T: LoopDataLabel> {
    entries: Vec<LoopDataEntry<T>>,
}

impl<T: LoopDataLabel> Display for LoopData<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries_content = self
            .entries()
            .iter()
            .map(|line| format!("{line}"))
            .collect::<Vec<String>>()
            .join("\n");
        let output = [
            "loop_".to_string(),
            self.entry_labels().to_string(),
            entries_content,
        ]
        .join("\n");
        write!(f, "{output}")
    }
}

impl<T: LoopDataLabel> LoopData<T> {
    pub fn new(entries: Vec<LoopDataEntry<T>>) -> Self {
        Self { entries }
    }

    pub fn init_builder() -> LoopDataBuilder<T> {
        LoopDataBuilder::new()
    }

    pub fn entries(&self) -> &[LoopDataEntry<T>] {
        &self.entries
    }

    pub fn entry_labels(&self) -> String {
        self.entries[0].relevant_labels()
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoopDataBuilder<T: LoopDataLabel> {
    entries: Vec<LoopDataEntry<T>>,
}

impl<T: LoopDataLabel> LoopDataBuilder<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(mut self, entry: LoopDataEntry<T>) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn finish(self) -> LoopData<T> {
        LoopData::new(self.entries)
    }
}
