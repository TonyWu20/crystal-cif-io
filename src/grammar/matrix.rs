//! A matrix value for the value layer.
//!
//! A matrix never appears as one token in a CIF 1.1 file. The
//! dictionary writes it as a numbered item family (nine items for a
//! 3 x 3 matrix). This type is the public value-layer container for
//! such data. It is not a `Value` variant. A `Value` is one CIF
//! 1.1 token. Matrix items bind as typed views in the data-dictionary
//! layer. See `docs/roadmap-dict-items.md` section 12.

use std::fmt::Display;

use super::numeric_values::Number;

/// A matrix of `Number` values. All rows share one length.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: Vec<Vec<Number>>,
}

/// The error returned by [`Matrix::try_new`].
#[derive(Debug)]
pub struct MatrixError {
    reason: &'static str,
}

impl MatrixError {
    fn empty() -> Self {
        Self {
            reason: "matrix has no rows",
        }
    }

    fn ragged() -> Self {
        Self {
            reason: "matrix rows do not share one length",
        }
    }
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.reason)
    }
}

impl std::error::Error for MatrixError {}

impl Matrix {
    /// Build a matrix from rows. All rows must share one length,
    /// with at least one row. Ragged input fails.
    pub fn try_new(rows: Vec<Vec<Number>>) -> Result<Self, MatrixError> {
        if rows.is_empty() {
            return Err(MatrixError::empty());
        }
        let width = rows[0].len();
        if rows.iter().any(|row| row.len() != width) {
            return Err(MatrixError::ragged());
        }
        Ok(Self { rows })
    }

    /// The rows, in order.
    pub fn rows(&self) -> &[Vec<Number>] {
        &self.rows
    }

    /// The shape as `(rows, columns)`.
    pub fn shape(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }

    /// One row by index.
    pub fn row(&self, i: usize) -> Option<&[Number]> {
        self.rows.get(i).map(|row| row.as_slice())
    }

    /// One column by index, in row order.
    pub fn col(&self, i: usize) -> Option<Vec<Number>> {
        if i >= self.rows[0].len() {
            return None;
        }
        Some(self.rows.iter().map(|row| row[i]).collect())
    }

    /// Shape-checked view for a fixed 3 x 3 shape.
    pub fn as_3x3(&self) -> Option<[[&Number; 3]; 3]> {
        if self.shape() != (3, 3) {
            return None;
        }
        Some([
            [
                &self.rows[0][0],
                &self.rows[0][1],
                &self.rows[0][2],
            ],
            [
                &self.rows[1][0],
                &self.rows[1][1],
                &self.rows[1][2],
            ],
            [
                &self.rows[2][0],
                &self.rows[2][1],
                &self.rows[2][2],
            ],
        ])
    }

    /// Shape-checked view for a 3-vector. A vector is three items,
    /// held as a 3 x 1 or 1 x 3 matrix.
    pub fn as_vector(&self) -> Option<Vec<Number>> {
        match self.shape() {
            (3, 1) => Some(self.rows.iter().flat_map(|row| row.iter().cloned()).collect()),
            (1, 3) => Some(self.rows[0].clone()),
            _ => None,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(&rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Float, Integer};

    fn matrix_3x3() -> Matrix {
        let one = Number::Integer(Integer(1));
        let two = Number::Integer(Integer(2));
        let three = Number::Float(Float(3.0));
        Matrix::try_new(vec![
            vec![one.clone(), one, one],
            vec![one, two, two],
            vec![two, three.clone(), three],
        ])
        .expect("3x3 matrix")
    }

    #[test]
    fn try_new_rejects_ragged_rows() {
        let m = Matrix::try_new(vec![
            vec![Number::Integer(Integer(1))],
            vec![
                Number::Integer(Integer(1)),
                Number::Integer(Integer(2)),
            ],
        ]);
        assert!(m.is_err());
    }

    #[test]
    fn try_new_rejects_no_rows() {
        let m = Matrix::try_new(Vec::new());
        assert!(m.is_err());
        assert_eq!(m.unwrap_err().to_string(), "matrix has no rows");
    }

    #[test]
    fn shape_views() {
        let m = matrix_3x3();
        assert_eq!(m.shape(), (3, 3));
        assert_eq!(m.rows().len(), 3);
        assert_eq!(m.row(1).map(|r| r.len()), Some(3));
        assert_eq!(m.row(3), None);
        let col0 = m.col(0).expect("column 0");
        assert_eq!(col0.len(), 3);
        assert_eq!(m.col(3), None);
    }

    #[test]
    fn as_3x3_view() {
        let m = matrix_3x3();
        let v = m.as_3x3().expect("3x3 view");
        assert_eq!(v[0][0], &Number::Integer(Integer(1)));
        assert_eq!(v[2][2], &Number::Float(Float(3.0)));
        let wide = Matrix::try_new(vec![
            vec![Number::Integer(Integer(1))],
            vec![Number::Integer(Integer(2))],
        ])
        .expect("2x1 matrix");
        assert!(wide.as_3x3().is_none());
    }

    #[test]
    fn as_vector_view() {
        let col3 = Matrix::try_new(vec![
            vec![Number::Float(Float(1.5))],
            vec![Number::Integer(Integer(2))],
            vec![Number::Integer(Integer(3))],
        ])
        .expect("3x1 matrix");
        assert_eq!(
            col3.as_vector(),
            Some(vec![
                Number::Float(Float(1.5)),
                Number::Integer(Integer(2)),
                Number::Integer(Integer(3)),
            ])
        );
        let row3 = Matrix::try_new(vec![vec![
            Number::Integer(Integer(1)),
            Number::Integer(Integer(2)),
            Number::Integer(Integer(3)),
        ]])
        .expect("1x3 matrix");
        assert_eq!(row3.as_vector().as_ref().map(|v| v.len()), Some(3));
        assert!(matrix_3x3().as_vector().is_none());
    }

    #[test]
    fn display_round_trips() {
        let m = matrix_3x3();
        let text = m.to_string();
        let tokens = text
            .split(|c| c == ' ' || c == '\n')
            .filter(|t| !t.is_empty())
            .collect::<Vec<&str>>();
        assert_eq!(tokens.len(), 9);
        let expected: Vec<f64> = vec![
            1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0,
        ];
        let parsed: Vec<f64> = tokens
            .iter()
            .map(|t| t.parse::<f64>().expect("token parses"))
            .collect();
        assert_eq!(parsed, expected);
        // Round trip through the f64 shortest form for a float entry.
        let floaty = Matrix::try_new(vec![vec![Number::Float(Float(0.1))]])
            .expect("1x1 matrix");
        let text = floaty.to_string();
        assert_eq!(text, "0.1");
        assert_eq!(text.parse::<f64>(), Ok(0.1_f64));
    }
}
