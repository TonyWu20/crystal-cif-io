//! The `space_group_symop` loop builder.
//!
//! The rows are the general positions of the resolved space group.
//! `R`, `RT`, `T`, and `Seitz_matrix` derive from the Seitz list.
//! The supplied `operation_xyz` row text cross-checks against the
//! computed list. A mismatch rejects with a `Result`, never a
//! panic.
//!
//! `to_data_items` materializes the loop (the `id` and
//! `operation_xyz` columns). The per-row single-value terms
//! (`r`, `rt`, `t`, `seitz_matrix`, `operation_description`) are
//! available from `terms` through `SingleValueTerm`.

use bon::bon;
use crystallographic_group::SeitzMatrix;

use crate::DataItems;
use crate::data_dict::core_cif::space_group_symop::{
    SpaceGroupSymopLoopItem, SpaceGroupSymopTerms,
};
use crate::data_dict::LoopValueTerm;
use crate::grammar::{CharString, Float, LoopColumns, Matrix, Number, UnquotedString};

use super::space_group::SpaceGroup;

/// A build failure of the `symop` builder.
#[derive(Debug, Clone, PartialEq)]
pub enum SymopError {
    /// The supplied column carries a different row count than the
    /// general-position list.
    RowCount { expected: usize, supplied: usize },
    /// A supplied row text misses the computed operation.
    Mismatch { row: usize, supplied: String, computed: String },
}

impl std::fmt::Display for SymopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RowCount { expected, supplied } => {
                write!(f, "the supplied column holds {supplied} rows, the list holds {expected}")
            }
            Self::Mismatch {
                row,
                supplied,
                computed,
            } => write!(f, "row {row} is {supplied:?}, the list computes {computed:?}"),
        }
    }
}

impl std::error::Error for SymopError {}

/// The materialized `space_group_symop` category: the general
/// positions and the per-row derived matrices.
#[derive(Debug, Clone)]
pub struct SymopSet {
    group: SpaceGroup,
    operations: Vec<String>,
    ids: Vec<String>,
    r_matrices: Vec<Matrix>,
    rt_matrices: Vec<Matrix>,
    t_matrices: Vec<Matrix>,
    seitz_matrices: Vec<Matrix>,
    terms: Vec<SpaceGroupSymopTerms>,
}

impl SymopSet {
    /// The resolved group the rows come from.
    pub fn group(&self) -> &SpaceGroup {
        &self.group
    }

    /// The `operation_xyz` row text, in list order.
    pub fn operations(&self) -> &[String] {
        &self.operations
    }

    /// The row ordinals, one per row.
    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    /// The per-row 3 x 3 rotation matrices.
    pub fn r_matrices(&self) -> &[Matrix] {
        &self.r_matrices
    }

    /// The per-row transposed rotation matrices.
    pub fn rt_matrices(&self) -> &[Matrix] {
        &self.rt_matrices
    }

    /// The per-row 3 x 1 translation columns. The entries are
    /// fractions of the 12-fold translation base, held as `Float`.
    pub fn t_matrices(&self) -> &[Matrix] {
        &self.t_matrices
    }

    /// The per-row 4 x 4 Seitz matrices. The translation part is
    /// divided by the 12-fold base.
    pub fn seitz_matrices(&self) -> &[Matrix] {
        &self.seitz_matrices
    }

    /// The per-row single-value terms: `operation_description`,
    /// `r`, `rt`, `t`, and `seitz_matrix`.
    pub fn terms(&self) -> &[SpaceGroupSymopTerms] {
        &self.terms
    }

    /// The loop, materialized as one `MultiValues` item over the
    /// `id` and `operation_xyz` columns.
    pub fn to_data_items(&self) -> Vec<DataItems> {
        vec![DataItems::MultiValues(LoopColumns::new(vec![
            SpaceGroupSymopLoopItem::Id(
                self.ids
                    .iter()
                    .map(|id| unquoted(id))
                    .collect(),
            )
            .to_loop_column(),
            SpaceGroupSymopLoopItem::Operation_xyz(
                self.operations
                    .iter()
                    .map(|op| unquoted(op))
                    .collect(),
            )
            .to_loop_column(),
        ]))]
    }
}

fn unquoted(text: &str) -> CharString {
    CharString::Unquoted(UnquotedString::new(text.to_string()))
}

fn squeeze(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

fn matrix3(rows: [[i32; 3]; 3]) -> Matrix {
    Matrix::try_new(vec![
        rows[0].map(|v| Number::from(v)).into_iter().collect(),
        rows[1].map(|v| Number::from(v)).into_iter().collect(),
        rows[2].map(|v| Number::from(v)).into_iter().collect(),
    ])
    .expect("a 3 x 3 matrix builds")
}

fn t_column(translation: [i32; 3]) -> Matrix {
    Matrix::try_new((0..3)
        .map(|i| vec![Number::Float(Float(translation[i] as f64 / 12.0))])
        .collect::<Vec<_>>())
    .expect("a 3 x 1 column builds")
}

fn seitz4(m: &SeitzMatrix) -> Matrix {
    let f64 = m.to_f64_mat();
    Matrix::try_new(vec![
        (0..4).map(|i| Number::Float(Float(f64[(i, 0)]))).collect(),
        (0..4).map(|i| Number::Float(Float(f64[(i, 1)]))).collect(),
        (0..4).map(|i| Number::Float(Float(f64[(i, 2)]))).collect(),
        (0..4).map(|i| Number::Float(Float(f64[(i, 3)]))).collect(),
    ])
    .expect("a 4 x 4 matrix builds")
}

#[bon]
impl SymopSet {
    /// The group is needed at the `build()` call. The rows and the
    /// per-row matrices compute from its general positions. The
    /// supplied `operation_xyz` cross-checks row by row. Whitespace
    /// is ignored in the comparison.
    #[builder(start_fn = builder, builder_type = SymopBuilder, finish_fn = build)]
    pub fn build_symops(
        group: SpaceGroup,
        operation_xyz: Option<Vec<CharString>>,
    ) -> Result<SymopSet, SymopError> {
        let positions = group.inner().general_positions();
        let full: Vec<SeitzMatrix> = positions
            .derive_full_sets()
            .into_iter()
            .flatten()
            .collect();
        let operations: Vec<String> = positions.formulas();
        if full.len() != operations.len() {
            return Err(SymopError::RowCount {
                expected: operations.len(),
                supplied: full.len(),
            });
        }
        let ids: Vec<String> = (1..=operations.len()).map(|i| i.to_string()).collect();

        if let Some(supplied) = &operation_xyz {
            if supplied.len() != operations.len() {
                return Err(SymopError::RowCount {
                    expected: operations.len(),
                    supplied: supplied.len(),
                });
            }
            for (row, (supplied_row, computed_row)) in
                supplied.iter().zip(&operations).enumerate()
            {
                if squeeze(supplied_row.content()) != squeeze(computed_row) {
                    return Err(SymopError::Mismatch {
                        row,
                        supplied: supplied_row.content().to_string(),
                        computed: computed_row.clone(),
                    });
                }
            }
        }

        let mut r_matrices = Vec::with_capacity(full.len());
        let mut rt_matrices = Vec::with_capacity(full.len());
        let mut t_matrices = Vec::with_capacity(full.len());
        let mut seitz_matrices = Vec::with_capacity(full.len());
        let mut terms = Vec::with_capacity(full.len() * 5);
        for (i, m) in full.iter().enumerate() {
            let rotation = m.rotation_part();
            let r = matrix3([[
                rotation[(0, 0)],
                rotation[(0, 1)],
                rotation[(0, 2)],
            ], [
                rotation[(1, 0)],
                rotation[(1, 1)],
                rotation[(1, 2)],
            ], [
                rotation[(2, 0)],
                rotation[(2, 1)],
                rotation[(2, 2)],
            ]]);
            let rt = matrix3([[
                rotation[(0, 0)],
                rotation[(1, 0)],
                rotation[(2, 0)],
            ], [
                rotation[(0, 1)],
                rotation[(1, 1)],
                rotation[(2, 1)],
            ], [
                rotation[(0, 2)],
                rotation[(1, 2)],
                rotation[(2, 2)],
            ]]);
            let tr = m.translation_part();
            let t = t_column([tr[(0, 0)], tr[(1, 0)], tr[(2, 0)]]);
            let seitz = seitz4(m);
            r_matrices.push(r.clone());
            rt_matrices.push(rt.clone());
            t_matrices.push(t.clone());
            seitz_matrices.push(seitz.clone());
            terms.push(SpaceGroupSymopTerms::Operation_description(unquoted(&operations[i])));
            terms.push(SpaceGroupSymopTerms::R(r));
            terms.push(SpaceGroupSymopTerms::Rt(rt));
            terms.push(SpaceGroupSymopTerms::T(t));
            terms.push(SpaceGroupSymopTerms::Seitz_matrix(seitz));
        }

        Ok(SymopSet {
            group,
            operations,
            ids,
            r_matrices,
            rt_matrices,
            t_matrices,
            seitz_matrices,
            terms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_dict::core_cif::space_group::ITNumber;
    use crate::data_dict::CifTerm;

    fn entry(number: i32) -> SpaceGroup {
        SpaceGroup::by_it_number(ITNumber::from(number)).expect("the group resolves")
    }

    #[test]
    fn the_p1_rows_are_the_identity() {
        let set = SymopSet::builder()
            .group(entry(1))
            .build()
            .expect("the rows build");
        assert_eq!(set.operations(), &["x,y,z".to_string()]);
        assert_eq!(set.ids(), &["1".to_string()]);
        let r = &set.r_matrices()[0];
        assert_eq!(
            r.rows()[0].iter().map(|n| n.to_string()).collect::<Vec<_>>(),
            vec!["1", "0", "0"],
        );
        let t = &set.t_matrices()[0];
        assert_eq!(
            t.rows()[0].iter().map(|n| n.to_string()).collect::<Vec<_>>(),
            vec!["0"],
        );
        assert_eq!(set.to_data_items().len(), 1);
    }

    #[test]
    fn the_p2_rows_carry_the_inversion() {
        let set = SymopSet::builder()
            .group(entry(2))
            .build()
            .expect("the rows build");
        assert_eq!(
            set.operations(),
            &["x,y,z".to_string(), "-x,-y,-z".to_string()],
        );
        assert_eq!(set.r_matrices().len(), 2);
        let second = &set.rt_matrices()[1];
        assert_eq!(second.shape(), (3, 3));
        assert_eq!(set.seitz_matrices()[0].shape(), (4, 4));
        let items = set.to_data_items();
        let loop_columns = items
            .first()
            .and_then(|item| item.as_multi_values())
            .expect("one loop item");
        assert_eq!(loop_columns.columns().len(), 2);
        assert_eq!(
            loop_columns.columns()[0].tag().as_ref(),
            "space_group_symop_id"
        );
        assert_eq!(
            loop_columns.columns()[1].tag().as_ref(),
            "space_group_symop_operation_xyz"
        );
        assert_eq!(loop_columns.columns()[0].values().len(), 2);
    }

    #[test]
    fn a_supplied_row_text_cross_checks() {
        let set = SymopSet::builder()
            .group(entry(2))
            .operation_xyz(vec![unquoted("x, y, z"), unquoted("-x,-y,-z")])
            .build()
            .expect("whitespace does not break the check");
        assert_eq!(set.operations().len(), 2);
    }

    #[test]
    fn a_mismatching_row_text_rejects() {
        let err = SymopSet::builder()
            .group(entry(2))
            .operation_xyz(vec![unquoted("-x,-y,-z"), unquoted("x,y,z")])
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            SymopError::Mismatch { row: 0, .. }
        ));
    }

    #[test]
    fn a_wrong_row_count_rejects() {
        let err = SymopSet::builder()
            .group(entry(2))
            .operation_xyz(vec![unquoted("x,y,z")])
            .build()
            .unwrap_err();
        assert!(matches!(
            err,
            SymopError::RowCount {
                expected: 2,
                supplied: 1
            }
        ));
    }

    #[test]
    fn the_p21c_rows_match_the_seitz_list() {
        let set = SymopSet::builder()
            .group(entry(14))
            .build()
            .expect("the rows build");
        assert_eq!(set.operations().len(), 4);
        let joined: Vec<&str> = set.operations().iter().map(|s| s.as_str()).collect();
        assert!(joined.iter().any(|op| op.contains("+1/2")), "the glide carries a half shift");
        assert_eq!(set.terms().len(), 20);
        let tags: Vec<String> = set
            .terms()
            .iter()
            .take(5)
            .map(|t| t.tag().as_ref().to_string())
            .collect();
        assert_eq!(
            tags,
            vec![
                "space_group_symop_operation_description".to_string(),
                "space_group_symop_r".to_string(),
                "space_group_symop_rt".to_string(),
                "space_group_symop_t".to_string(),
                "space_group_symop_seitz_matrix".to_string(),
            ],
        );
    }
}
