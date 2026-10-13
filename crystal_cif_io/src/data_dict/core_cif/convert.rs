//! The value-layer conversion helpers of the typed document layer
//! (roadmap D12).
//!
//! Each reader takes a block value and returns the field payload or a
//! [`CifError`]. An inapplicable (`.`) or unknown (`?`) value carries
//! no data and reads as `None` without an error. A value of the wrong
//! class records a [`CifError::ValueClass`]. A matrix of the wrong
//! shape records a [`CifError::Shape`]. A closed-set value outside the
//! set records a [`CifError::ValueClass`].

use crystal_cif_fmt::grammar::{
    CharString, Float, Matrix, MatrixError, Number, Numeric, UnsignedInteger, Value,
};

use crate::data_dict::core_cif::cell::CellAngle;
use crate::data_dict::core_cif::chemical_formula::FormulaWeight;
use crate::data_dict::core_cif::space_group::ITNumber;
use crate::data_dict::matrix;
use crate::core_cif::CifError;

/// The value class name used in a [`CifError::ValueClass`] report.
pub(crate) fn value_class_name(v: &Value) -> &'static str {
    match v {
        Value::Inapplicable => "inapplicable",
        Value::Unknown => "unknown",
        Value::Numeric(_) => "Numeric",
        Value::CharString(_) => "CharString",
        Value::TextField(_) => "TextField",
        Value::List(_) => "List",
        Value::Table(_) => "Table",
    }
}

/// An inapplicable (`.`) or unknown (`?`) value carries no data.
pub(crate) fn is_absent(v: &Value) -> bool {
    matches!(v, Value::Inapplicable | Value::Unknown)
}

/// Read a numeric item. A missing value reads as `None`.
pub(crate) fn read_numeric(tag: &str, v: &Value) -> Result<Option<Numeric>, CifError> {
    if is_absent(v) {
        return Ok(None);
    }
    match v {
        Value::Numeric(n) => Ok(Some(*n)),
        other => Err(value_class_error(tag, "Numeric", other)),
    }
}

/// Read a character item. A missing value reads as `None`.
pub(crate) fn read_char(tag: &str, v: &Value) -> Result<Option<CharString>, CifError> {
    if is_absent(v) {
        return Ok(None);
    }
    match v {
        Value::CharString(c) => Ok(Some(c.clone())),
        other => Err(value_class_error(tag, "CharString", other)),
    }
}

/// Read a matrix item of the dictionary shape. A missing value reads
/// as `None`. A wrong shape records a [`CifError::Shape`].
pub(crate) fn read_matrix(
    tag: &str,
    v: &Value,
    shape: (usize, usize),
) -> Result<Option<Matrix>, CifError> {
    if is_absent(v) {
        return Ok(None);
    }
    match v {
        Value::List(_) => match matrix::from_value(v, Some(shape)) {
            Ok(m) => Ok(Some(m)),
            Err(MatrixError::ShapeMismatch { expected, actual }) => Err(CifError::Shape {
                tag: tag.to_string(),
                expected,
                found: actual,
            }),
            Err(_) => Err(CifError::ValueClass {
                tag: tag.to_string(),
                expected: "a numeric matrix",
                found: "List",
            }),
        },
        other => Err(value_class_error(tag, "a numeric matrix", other)),
    }
}

/// Read a `CellAngle` item. The value is a number, clamped to `0:180`.
pub(crate) fn read_cell_angle(tag: &str, v: &Value) -> Result<Option<CellAngle>, CifError> {
    let n = read_numeric(tag, v)?;
    Ok(n.map(|n| CellAngle::new(number_to_float(&n))))
}

/// Read an `ITNumber` item. The value is an integer in `1:230`.
pub(crate) fn read_it_number(tag: &str, v: &Value) -> Result<Option<ITNumber>, CifError> {
    let n = read_numeric(tag, v)?;
    // Saturate before the `u8` cast so the guard sees a value above the
    // range instead of a wrapped one.
    Ok(n.map(|n| {
        let raw = number_to_u64(&n).min(u8::MAX as u64);
        ITNumber::new(raw as u8)
    }))
}

/// Read an unsigned-integer item.
pub(crate) fn read_unsigned(tag: &str, v: &Value) -> Result<Option<UnsignedInteger>, CifError> {
    let n = read_numeric(tag, v)?;
    // Saturate before the `u32` cast so a huge value reads as the
    // max instead of wrapping low.
    Ok(n.map(|n| UnsignedInteger(number_to_u64(&n).min(u32::MAX as u64) as u32)))
}

/// Read a `SymMultiplicity` item. The value is an integer in `1:192`.
pub(crate) fn read_sym_multiplicity(
    tag: &str,
    v: &Value,
) -> Result<Option<crate::data_dict::core_cif::atom_site::SymMultiplicity>, CifError> {
    use crate::data_dict::core_cif::atom_site::SymMultiplicity;
    let n = read_numeric(tag, v)?;
    // Saturate before the `u8` cast so the guard sees a value above the
    // range instead of a wrapped one.
    Ok(n.map(|n| {
        let raw = number_to_u64(&n).min(u8::MAX as u64);
        SymMultiplicity::new(raw as u8)
    }))
}

/// Read a `FormulaWeight` item. The recorded value stores as written;
/// the range guard applies at the write.
pub(crate) fn read_formula_weight(tag: &str, v: &Value) -> Result<Option<FormulaWeight>, CifError> {
    let n = read_numeric(tag, v)?;
    Ok(n.map(FormulaWeight::from_numeric))
}

/// Read a closed-set item. The value is a character string in the
/// closed set. A value outside the set records a value-class error.
pub(crate) fn read_closed_set<T, F: Fn(&str) -> Option<T>>(
    tag: &str,
    v: &Value,
    parse: F,
    type_name: &'static str,
) -> Result<Option<T>, CifError> {
    let c = read_char(tag, v)?;
    match c {
        None => Ok(None),
        Some(c) => match parse(c.content()) {
            Some(t) => Ok(Some(t)),
            None => Err(CifError::ValueClass {
                tag: tag.to_string(),
                expected: type_name,
                found: "CharString",
            }),
        },
    }
}

/// The error of a value-class mismatch of a typed item.
fn value_class_error(tag: &str, expected: &'static str, found: &Value) -> CifError {
    CifError::ValueClass {
        tag: tag.to_string(),
        expected,
        found: value_class_name(found),
    }
}

/// A number as `Float`.
fn number_to_float(n: &Numeric) -> Float {
    match n.number() {
        Number::Integer(i) => Float(i.0 as f64),
        Number::Float(f) => f,
    }
}

/// A number as `u64` (truncating the fractional part).
fn number_to_u64(n: &Numeric) -> u64 {
    match n.number() {
        Number::Integer(i) => i.0 as u64,
        Number::Float(f) => f.0 as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crystal_cif_fmt::grammar::UnquotedString;

    fn char_value(text: &str) -> Value {
        Value::CharString(CharString::Unquoted(UnquotedString::new(text.to_string())))
    }

    #[test]
    fn read_numeric_reads_a_number_and_flags_a_wrong_class() {
        assert!(read_numeric("t", &Value::Numeric(Numeric::from(1.5))).unwrap().is_some());
        assert!(matches!(
            read_numeric("t", &char_value("x")).unwrap_err(),
            CifError::ValueClass { .. }
        ));
        assert!(read_numeric("t", &Value::Inapplicable).unwrap().is_none());
    }

    #[test]
    fn read_char_reads_a_string_and_flags_a_wrong_class() {
        let c = read_char("t", &char_value("ab")).unwrap();
        assert_eq!(c.as_ref().map(|c| c.content()), Some("ab"));
        assert!(matches!(
            read_char("t", &Value::Numeric(Numeric::from(1.0))).unwrap_err(),
            CifError::ValueClass { .. }
        ));
        assert!(read_char("t", &Value::Unknown).unwrap().is_none());
    }

    #[test]
    fn read_matrix_checks_the_shape_and_the_class() {
        let row = |vals: [f64; 3]| Value::List(vals.map(|v| Value::Numeric(Numeric::from(v))).into());
        let good = Value::List(vec![row([1.0, 0.0, 0.0]), row([0.0, 1.0, 0.0]), row([0.0, 0.0, 1.0])]);
        let m = read_matrix("t", &good, (3, 3)).unwrap().expect("the matrix reads");
        assert_eq!(m.shape(), (3, 3));
        let short = Value::List(vec![row([1.0, 0.0, 0.0]), row([0.0, 1.0, 0.0])]);
        assert!(matches!(
            read_matrix("t", &short, (3, 3)).unwrap_err(),
            CifError::Shape { .. }
        ));
        assert!(matches!(
            read_matrix("t", &Value::Numeric(Numeric::from(1.0)), (3, 3)).unwrap_err(),
            CifError::ValueClass { .. }
        ));
    }

    #[test]
    fn read_closed_set_types_the_set_and_flags_the_rest() {
        use crate::data_dict::core_cif::atom_site::AdpType;
        assert!(matches!(
            read_closed_set("t", &char_value("Uiso"), AdpType::parse, "AdpType").unwrap(),
            Some(AdpType::Uiso)
        ));
        assert!(matches!(
            read_closed_set("t", &char_value("nope"), AdpType::parse, "AdpType").unwrap_err(),
            CifError::ValueClass { .. }
        ));
        assert!(read_closed_set("t", &Value::Inapplicable, AdpType::parse, "AdpType").unwrap().is_none());
    }

    #[test]
    fn the_guarded_newtypes_clamp_at_the_read() {
        let angle = read_cell_angle("t", &Value::Numeric(Numeric::from(200.0f64))).unwrap().expect("reads");
        assert_eq!(angle.angle().0, 180.0, "the angle clamps to 180");
        let it = read_it_number("t", &Value::Numeric(Numeric::from(300.0f64))).unwrap().expect("reads");
        assert_eq!(it.to_string(), "230", "the IT number clamps to 230");
        let mult = read_sym_multiplicity("t", &Value::Numeric(Numeric::from(300.0f64))).unwrap().expect("reads");
        assert_eq!(mult.to_string(), "192", "the multiplicity clamps to 192");
    }

    #[test]
    fn the_formula_weight_read_keeps_the_recorded_value() {
        let n = crate::data_dict::core_cif::range_check::number_as_f64;
        let w = read_formula_weight("t", &Value::Numeric(Numeric::from(0.5f64))).unwrap().expect("reads");
        // The range guard applies at the write, not the read. A recorded
        // value below the `1.0:` floor stores as written.
        assert_eq!(n(&w.value().number()), 0.5);
    }
}
