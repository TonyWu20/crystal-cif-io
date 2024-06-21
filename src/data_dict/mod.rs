mod core_cif;
mod symmetry_cif;

pub trait DataLabel {
    type C: DataCategory;
    fn tag(&self) -> String;
    fn full_label(&self) -> String {
        let label = format!("_{}_{}", Self::C::category_prefix(), self.tag());
        format!("{label:<34}")
    }
}

/// As a category of the data
pub trait DataCategory {
    /// The tag of the category.
    /// # Note
    /// Do not add "_" manually in implementations!
    /// The `full_label` func in trait `DataLabel` will handle the leading underscores for both
    /// the category and tag.
    fn category_prefix() -> String;
}
