//! The matrix payload helpers (roadmap decision D3).
//!
//! A matrix data item carries a 2.0-style nested list value:
//! a row-major list of rows, each row a list of numbers. The
//! value-layer form is a nested list of `Value::Numeric`
//! values. The shape is not part of the value: it is recorded
//! in the shell dictionary (`_type.dimension`) and checked on
//! the read path, not the write path.

use crystal_cif_fmt::grammar::{Matrix, MatrixError, Numeric, Value};

/// Re-export the read-path error so callers name one type.
pub use crystal_cif_fmt::grammar::MatrixError as MatrixReadError;

/// Convert a `Matrix` to its value-layer form: a row-major
/// nested list of numeric values.
pub fn to_value(m: &Matrix) -> Value {
    Value::List(
        m.rows()
            .iter()
            .map(|row| {
                Value::List(
                    row.iter()
                        .map(|n| Value::Numeric(Numeric::new(n.clone(), None)))
                        .collect(),
                )
            })
            .collect(),
    )
}

/// Read a value into a `Matrix`. `shape` bounds the row and
/// column count; `None` accepts any rectangular matrix.
///
/// The value must be a 2.0-style nested list of numeric
/// leaves. A value of any other shape is an error.
pub fn from_value(v: &Value, shape: Option<(usize, usize)>) -> Result<Matrix, MatrixError> {
    match v {
        Value::List(rows) => Matrix::from_list(rows, shape),
        // A scalar or text value is not a matrix element.
        _ => Err(MatrixError::BadRow),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::{Number, UnquotedString};

    fn nested(rows: &[&[Number]]) -> Value {
        Value::List(
            rows.iter()
                .map(|row| {
                    Value::List(
                        row.iter()
                            .map(|n| Value::Numeric(Numeric::new(n.clone(), None)))
                            .collect(),
                    )
                })
                .collect(),
        )
    }

    #[test]
    fn round_trip_holds() {
        let rows = vec![
            vec![Number::from(1), Number::from(2)],
            vec![Number::from(3), Number::from(4)],
        ];
        let m = Matrix::try_new(rows).unwrap();
        let back = from_value(&to_value(&m), Some((2, 2))).unwrap();
        assert_eq!(back.shape(), (2, 2));
    }

    #[test]
    fn shape_mismatch_is_an_error() {
        let v = nested(&[&[Number::from(1), Number::from(2)]]);
        let err = from_value(&v, Some((3, 3))).unwrap_err();
        assert!(matches!(err, MatrixError::ShapeMismatch { .. }));
    }

    #[test]
    fn non_numeric_leaf_is_an_error() {
        let v = Value::List(vec![Value::List(vec![
            Value::Numeric(Numeric::new(Number::from(1), None)),
            Value::CharString(crystal_cif_fmt::grammar::CharString::Unquoted(UnquotedString::new(
                "x".to_string(),
            ))),
        ])]);
        let err = from_value(&v, None).unwrap_err();
        assert!(matches!(err, MatrixError::NonNumeric));
    }

    #[test]
    fn scalar_value_is_not_a_matrix() {
        let v = Value::Numeric(Numeric::new(Number::from(1), None));
        let err = from_value(&v, None).unwrap_err();
        assert!(matches!(err, MatrixError::BadRow));
    }
}
