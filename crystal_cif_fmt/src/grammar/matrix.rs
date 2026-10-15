//! A matrix value for the value layer.
//!
//! A matrix never appears as one token in a CIF 1.1 file. The
//! standard dictionary writes it as a numbered item family (nine
//! items for a 3 x 3 matrix). This type is the public value-layer
//! container for such data. Matrix items bind as typed views in the
//! data-dictionary layer. See `docs/roadmap-dict-items.md`
//! section 12.
//!
//! In a CIF 2.0 file a matrix item is a `Value::List` of row lists
//! of `Numeric` values. Build the matrix from that list with
//! [`Matrix::from_list`]. See `docs/roadmap-dict-items.md`
//! section 13.

use std::fmt::Display;

use super::numeric_values::Number;
use super::tags_values::Value;

/// A matrix of `Number` values. All rows share one length.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: Vec<Vec<Number>>,
}

/// The error returned by [`Matrix::try_new`] and
/// [`Matrix::from_list`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MatrixError {
    /// The input holds no rows.
    #[error("matrix has no rows")]
    Empty,
    /// The rows do not share one length.
    #[error("matrix rows do not share one length")]
    Ragged,
    /// A row holds a value that is not numeric.
    #[error("a matrix row holds a non-numeric value")]
    NonNumeric,
    /// A matrix element is not a list of numeric values.
    #[error("a matrix element is not a list of numeric values")]
    BadRow,
    /// A dictionary shape was given and the parsed matrix does not
    /// match it.
    #[error("matrix shape is {actual:?}, the dictionary expects {expected:?}")]
    ShapeMismatch {
        expected: (usize, usize),
        actual: (usize, usize),
    },
}

impl Matrix {
    /// Build a matrix from rows. All rows must share one length,
    /// with at least one row. Ragged input fails.
    pub fn try_new(rows: Vec<Vec<Number>>) -> Result<Self, MatrixError> {
        if rows.is_empty() {
            return Err(MatrixError::Empty);
        }
        let width = rows[0].len();
        if rows.iter().any(|row| row.len() != width) {
            return Err(MatrixError::Ragged);
        }
        Ok(Self { rows })
    }

    /// Build a matrix from the element list of a CIF 2.0 list value.
    ///
    /// `rows` is the element list of a `Value::List` (the outer list).
    /// Each element must be a nested `Value::List` of `Value::Numeric`
    /// values: one row per element. Non-numeric row elements, elements
    /// that are not lists, ragged shapes, and a missing row are
    /// rejected. When `shape` is given (the dictionary
    /// `_type.dimension`), the parsed `(rows, cols)` must equal it.
    pub fn from_list(rows: &[Value], shape: Option<(usize, usize)>) -> Result<Self, MatrixError> {
        if rows.is_empty() {
            return Err(MatrixError::Empty);
        }
        let mut built: Vec<Vec<Number>> = Vec::with_capacity(rows.len());
        for element in rows {
            let row = match element {
                Value::List(inner) => inner,
                _ => return Err(MatrixError::BadRow),
            };
            let mut numbers: Vec<Number> = Vec::with_capacity(row.len());
            for value in row {
                match value {
                    Value::Numeric(n) => numbers.push(n.number()),
                    _ => return Err(MatrixError::NonNumeric),
                }
            }
            built.push(numbers);
        }
        let matrix = Self::try_new(built)?;
        if let Some(expected) = shape {
            let actual = matrix.shape();
            if actual != expected {
                return Err(MatrixError::ShapeMismatch { expected, actual });
            }
        }
        Ok(matrix)
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
            [&self.rows[0][0], &self.rows[0][1], &self.rows[0][2]],
            [&self.rows[1][0], &self.rows[1][1], &self.rows[1][2]],
            [&self.rows[2][0], &self.rows[2][1], &self.rows[2][2]],
        ])
    }

    /// Shape-checked view for a 3-vector. A vector is three items,
    /// held as a 3 x 1 or 1 x 3 matrix.
    pub fn as_vector(&self) -> Option<Vec<Number>> {
        match self.shape() {
            (3, 1) => Some(
                self.rows
                    .iter()
                    .flat_map(|row| row.iter().cloned())
                    .collect(),
            ),
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
    use crate::grammar::{CharString, Float, Integer, Numeric, UnquotedString, Value};

    fn matrix_3x3() -> Matrix {
        let one = Number::Integer(Integer(1));
        let two = Number::Integer(Integer(2));
        let three = Number::Float(Float(3.0));
        Matrix::try_new(vec![
            vec![one, one, one],
            vec![one, two, two],
            vec![two, three, three],
        ])
        .expect("3x3 matrix")
    }

    #[test]
    fn try_new_rejects_ragged_rows() {
        let m = Matrix::try_new(vec![
            vec![Number::Integer(Integer(1))],
            vec![Number::Integer(Integer(1)), Number::Integer(Integer(2))],
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

    fn num(v: i32) -> Value {
        Value::Numeric(Numeric::from(v))
    }

    fn matrix_list_3x3() -> Value {
        Value::List(vec![
            Value::List(vec![num(1), num(2), num(3)]),
            Value::List(vec![num(4), num(5), num(6)]),
            Value::List(vec![num(7), num(8), num(9)]),
        ])
    }

    #[test]
    fn from_list_builds_a_3x3_matrix() {
        let list = matrix_list_3x3();
        let m =
            Matrix::from_list(list.as_list().expect("list"), Some((3, 3))).expect("3x3 from list");
        assert_eq!(m.shape(), (3, 3));
        assert_eq!(
            m.row(1).expect("row 1"),
            &[
                Number::Integer(Integer(4)),
                Number::Integer(Integer(5)),
                Number::Integer(Integer(6))
            ]
        );
    }

    #[test]
    fn from_list_rejects_non_numeric_rows() {
        let list = Value::List(vec![Value::List(vec![
            num(1),
            Value::CharString(CharString::Unquoted(UnquotedString::new("x".to_string()))),
        ])]);
        let err = Matrix::from_list(list.as_list().expect("list"), None).unwrap_err();
        assert_eq!(err, MatrixError::NonNumeric);
    }

    #[test]
    fn from_list_rejects_elements_that_are_not_lists() {
        let list = Value::List(vec![num(1)]);
        let err = Matrix::from_list(list.as_list().expect("list"), None).unwrap_err();
        assert_eq!(err, MatrixError::BadRow);
    }

    #[test]
    fn from_list_rejects_ragged_shapes() {
        let list = Value::List(vec![
            Value::List(vec![num(1), num(2)]),
            Value::List(vec![num(3)]),
        ]);
        let err = Matrix::from_list(list.as_list().expect("list"), None).unwrap_err();
        assert_eq!(err, MatrixError::Ragged);
    }

    #[test]
    fn from_list_rejects_an_empty_list() {
        let list = Value::List(Vec::new());
        let err = Matrix::from_list(list.as_list().expect("list"), None).unwrap_err();
        assert_eq!(err, MatrixError::Empty);
    }

    #[test]
    fn from_list_checks_the_dictionary_shape() {
        let list = matrix_list_3x3();
        let err = Matrix::from_list(list.as_list().expect("list"), Some((4, 4))).unwrap_err();
        assert_eq!(
            err,
            MatrixError::ShapeMismatch {
                expected: (4, 4),
                actual: (3, 3)
            }
        );
        assert!(Matrix::from_list(list.as_list().expect("list"), None).is_ok());
    }

    #[test]
    fn display_round_trips() {
        let m = matrix_3x3();
        let text = m.to_string();
        let tokens = text
            .split(|c: char| c.is_ascii_whitespace())
            .filter(|t| !t.is_empty())
            .collect::<Vec<&str>>();
        assert_eq!(tokens.len(), 9);
        let expected: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0];
        let parsed: Vec<f64> = tokens
            .iter()
            .map(|t| t.parse::<f64>().expect("token parses"))
            .collect();
        assert_eq!(parsed, expected);
        // Round trip through the f64 shortest form for a float entry.
        let floaty = Matrix::try_new(vec![vec![Number::Float(Float(0.1))]]).expect("1x1 matrix");
        let text = floaty.to_string();
        assert_eq!(text, "0.1");
        assert_eq!(text.parse::<f64>(), Ok(0.1_f64));
    }
}
