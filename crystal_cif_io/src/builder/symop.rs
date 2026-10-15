//! The `space_group_symop` loop builder.
//!
//! The rows are the general positions of the resolved space group.
//! `R`, `RT`, `T`, and `Seitz_matrix` derive from the Seitz list.
//! The supplied `operation_xyz` row text cross-checks against the
//! computed list. A mismatch rejects with a `Result`, never a
//! panic.
//!
//! `Symop` materializes the loop through `CifCategory::to_members`
//! (the `id` and `operation_xyz` columns). The per-row fields
//! (`r`, `rt`, `t`, `seitz_matrix`, `operation_description`) are
//! plain struct fields.

use bon::bon;
use crystallographic_group::SeitzMatrix;
use crystallographic_group::SpaceGroup as Resolved;

use crate::data_dict::core_cif::space_group_symop::Symop;
use crystal_cif_fmt::grammar::{CharString, Float, Matrix, Number, UnquotedString};

/// A build failure of the `symop` builder.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SymopError {
    /// The supplied column carries a different row count than the
    /// general-position list.
    #[error("the supplied column holds {supplied} rows, the list holds {expected}")]
    RowCount { expected: usize, supplied: usize },
    /// A supplied row text misses the computed operation.
    #[error("row {row} is {supplied:?}, the list computes {computed:?}")]
    Mismatch {
        row: usize,
        supplied: String,
        computed: String,
    },
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
    Matrix::try_new(
        (0..3)
            .map(|i| vec![Number::Float(Float(translation[i] as f64 / 12.0))])
            .collect::<Vec<_>>(),
    )
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
impl Symop {
    /// The group is needed at the `build()` call. The rows and the
    /// per-row matrices compute from its general positions. The
    /// supplied `operation_xyz` cross-checks row by row. Whitespace
    /// is ignored in the comparison.
    #[builder(start_fn = builder, builder_type = SymopBuilder, finish_fn = build)]
    pub fn build_symops(
        group: Resolved,
        operation_xyz: Option<Vec<CharString>>,
    ) -> Result<Vec<Symop>, SymopError> {
        let positions = group.general_positions();
        let full: Vec<SeitzMatrix> = positions.derive_full_sets().into_iter().flatten().collect();
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
            for (row, (supplied_row, computed_row)) in supplied.iter().zip(&operations).enumerate()
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

        let mut rows = Vec::with_capacity(full.len());
        for (i, m) in full.iter().enumerate() {
            let rotation = m.rotation_part();
            let r = matrix3([
                [rotation[(0, 0)], rotation[(0, 1)], rotation[(0, 2)]],
                [rotation[(1, 0)], rotation[(1, 1)], rotation[(1, 2)]],
                [rotation[(2, 0)], rotation[(2, 1)], rotation[(2, 2)]],
            ]);
            let rt = matrix3([
                [rotation[(0, 0)], rotation[(1, 0)], rotation[(2, 0)]],
                [rotation[(0, 1)], rotation[(1, 1)], rotation[(2, 1)]],
                [rotation[(0, 2)], rotation[(1, 2)], rotation[(2, 2)]],
            ]);
            let tr = m.translation_part();
            let t = t_column([tr[(0, 0)], tr[(1, 0)], tr[(2, 0)]]);
            let seitz = seitz4(m);
            rows.push(Symop {
                id: Some(unquoted(&ids[i])),
                operation_description: Some(unquoted(&operations[i])),
                operation_xyz: Some(unquoted(&operations[i])),
                r: Some(r),
                rt: Some(rt),
                seitz_matrix: Some(seitz),
                t: Some(t),
                ..Default::default()
            });
        }

        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_dict::core_cif::CifCategory;
    use crystal_cif_fmt::DataBlockMember;
    use crystallographic_group::database::PER_NUMBER_SPACE_GROUP_SYMBOLS;

    fn entry(number: i32) -> Resolved {
        let row = (number - 1) as usize;
        let hall = PER_NUMBER_SPACE_GROUP_SYMBOLS[2][row];
        Resolved::try_from_str(hall).expect("the group resolves")
    }

    #[test]
    fn the_p1_rows_are_the_identity() {
        let set = Symop::builder()
            .group(entry(1))
            .build()
            .expect("the rows build");
        assert_eq!(set.len(), 1);
        assert_eq!(
            set[0].operation_xyz.as_ref().expect("xyz").content(),
            "x,y,z"
        );
        assert_eq!(set[0].id.as_ref().expect("id").content(), "1");
        let r = set[0].r.as_ref().expect("r");
        assert_eq!(
            r.rows()[0]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>(),
            vec!["1", "0", "0"],
        );
        let t = set[0].t.as_ref().expect("t");
        assert_eq!(
            t.rows()[0]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>(),
            vec!["0"],
        );
        assert_eq!(set.to_members().len(), 1);
    }

    #[test]
    fn the_p2_rows_carry_the_inversion() {
        let set = Symop::builder()
            .group(entry(2))
            .build()
            .expect("the rows build");
        let xyz: Vec<&str> = set
            .iter()
            .map(|row| row.operation_xyz.as_ref().expect("xyz").content())
            .collect();
        assert_eq!(xyz, ["x,y,z", "-x,-y,-z"]);
        assert_eq!(set.len(), 2);
        let second = set[1].rt.as_ref().expect("rt");
        assert_eq!(second.shape(), (3, 3));
        assert_eq!(set[0].seitz_matrix.as_ref().expect("seitz").shape(), (4, 4));
        let members = set.to_members();
        let loop_columns = match &members[0] {
            DataBlockMember::DataItems(items) => items.as_multi_values().expect("one loop item"),
            other => panic!("expected a loop member, got {other:?}"),
        };
        let has_column = |name: &str| {
            loop_columns
                .columns()
                .iter()
                .any(|c| c.tag().as_ref() == name)
        };
        assert!(
            has_column("space_group_symop_id"),
            "the id column is present"
        );
        assert!(
            has_column("space_group_symop_operation_xyz"),
            "the operation column is present"
        );
        let id_col = loop_columns
            .columns()
            .iter()
            .find(|c| c.tag().as_ref() == "space_group_symop_id")
            .expect("the id column is present");
        assert_eq!(id_col.values().len(), 2);
    }

    #[test]
    fn a_supplied_row_text_cross_checks() {
        let set = Symop::builder()
            .group(entry(2))
            .operation_xyz(vec![unquoted("x, y, z"), unquoted("-x,-y,-z")])
            .build()
            .expect("whitespace does not break the check");
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn a_mismatching_row_text_rejects() {
        let err = Symop::builder()
            .group(entry(2))
            .operation_xyz(vec![unquoted("-x,-y,-z"), unquoted("x,y,z")])
            .build()
            .unwrap_err();
        assert!(matches!(err, SymopError::Mismatch { row: 0, .. }));
    }

    #[test]
    fn a_wrong_row_count_rejects() {
        let err = Symop::builder()
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
        let set = Symop::builder()
            .group(entry(14))
            .build()
            .expect("the rows build");
        assert_eq!(set.len(), 4);
        let ops: Vec<&str> = set
            .iter()
            .map(|row| row.operation_xyz.as_ref().expect("xyz").content())
            .collect();
        assert!(
            ops.iter().any(|op| op.contains("+1/2")),
            "the glide carries a half shift"
        );
        for row in &set {
            assert!(row.r.is_some(), "the rotation matrix is set");
            assert!(row.seitz_matrix.is_some(), "the Seitz matrix is set");
        }
        assert_eq!(set.to_members().len(), 1);
    }
}
