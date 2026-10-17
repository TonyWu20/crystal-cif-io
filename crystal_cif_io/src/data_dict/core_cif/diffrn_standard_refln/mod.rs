mod diffrn_standard_refln;
use std::fmt::Display;

use crate::data_dict::{matrix, CifTerm, LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::grammar::{CharString, Matrix, Numeric, Tag, Value};
pub use diffrn_standard_refln::{DiffrnStandardRefln, DiffrnStandardReflnRow};

/// The single-value terms of the `diffrn_standard_refln` category: one variant per
/// registry single-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnStandardReflnTerms {
    /// The `_diffrn_standard_refln_hkl` item. The variant holds the single value.
    Hkl(Matrix),
}

impl CifTerm for DiffrnStandardReflnTerms {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnStandardReflnTerms::Hkl(_) => "hkl",
        };
        Tag::new(format!("diffrn_standard_refln_{suffix}"))
    }
}

impl SingleValueTerm for DiffrnStandardReflnTerms {
    fn value(&self) -> Value {
        match self {
            DiffrnStandardReflnTerms::Hkl(m) => matrix::to_value(m),
        }
    }
}

impl Display for DiffrnStandardReflnTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_single_value_data())
    }
}

/// The loop-value terms of the `diffrn_standard_refln` category: one variant per
/// registry loop-value item, in item order.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum DiffrnStandardReflnLoopItem {
    /// The `_diffrn_standard_refln_code` item. The variant holds the column values, one per loop row.
    ///
    /// The code identifying a reflection measured as a standard reflection
    /// with the indices _diffrn_standard_refln_index_. This is the same
    /// code as the _diffrn_refln_standard_code in the _diffrn_refln_ list.
    Code(Vec<CharString>),
    /// The `_diffrn_standard_refln_index_h` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of standard reflections used in the diffraction
    /// measurement process.
    Index_h(Vec<Numeric>),
    /// The `_diffrn_standard_refln_index_k` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of standard reflections used in the diffraction
    /// measurement process.
    Index_k(Vec<Numeric>),
    /// The `_diffrn_standard_refln_index_l` item. The variant holds the column values, one per loop row.
    ///
    /// Miller indices of standard reflections used in the diffraction
    /// measurement process.
    Index_l(Vec<Numeric>),
}

impl CifTerm for DiffrnStandardReflnLoopItem {
    fn tag(&self) -> Tag {
        let suffix = match self {
            DiffrnStandardReflnLoopItem::Code(_) => "code",
            DiffrnStandardReflnLoopItem::Index_h(_) => "index_h",
            DiffrnStandardReflnLoopItem::Index_k(_) => "index_k",
            DiffrnStandardReflnLoopItem::Index_l(_) => "index_l",
        };
        Tag::new(format!("diffrn_standard_refln_{suffix}"))
    }
}

impl LoopValueTerm for DiffrnStandardReflnLoopItem {
    fn values(&self) -> Vec<Value> {
        match self {
            DiffrnStandardReflnLoopItem::Code(s) => s.iter().cloned().map(Value::from).collect(),
            DiffrnStandardReflnLoopItem::Index_h(n)
            | DiffrnStandardReflnLoopItem::Index_k(n)
            | DiffrnStandardReflnLoopItem::Index_l(n) => {
                n.iter().cloned().map(Value::from).collect()
            }
        }
    }
}
