use std::fmt::Display;

use crate::grammar::{CharString, Numeric, SyntacticUnit, TextField};

/// A CIF value in one of its forms. The value rules are the
/// CIF 1.1 set, plus the CIF 2.0 list and table forms.
#[derive(Debug, Clone, Default)]
pub enum Value {
    /// The inapplicable value `.`.
    Inapplicable,
    #[default]
    /// The unknown value `?`.
    Unknown,
    /// A numeric value, with the uncertainty digits.
    Numeric(Numeric),
    /// A character string in one of its quoted forms.
    CharString(CharString),
    /// A `;`-delimited text field.
    TextField(TextField),
    /// A CIF 2.0 list value (`[...]`). The elements are any `Value`s,
    /// and a list may nest. Parsed only in the 2.0 dialect.
    List(Vec<Value>),
    /// A CIF 2.0 table value (`{...}`). The ordered pairs hold a key
    /// and a value; the key is a value (the parser produces a
    /// char_string key). Parsed only in the 2.0 dialect (roadmap
    /// section 14). No core dictionary item uses a table (roadmap
    /// section 13.3).
    Table(Vec<(Value, Value)>),
}

impl Value {
    /// The numeric payload, when the value is numeric.
    pub fn as_numeric(&self) -> Option<&Numeric> {
        if let Self::Numeric(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// The char string payload, when the value is a char string.
    pub fn as_char_string(&self) -> Option<&CharString> {
        if let Self::CharString(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// The text field payload, when the value is a text field.
    pub fn as_text_field(&self) -> Option<&TextField> {
        if let Self::TextField(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// The element list of a `Value::List`.
    pub fn as_list(&self) -> Option<&Vec<Value>> {
        if let Self::List(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// The ordered key-value pairs of a `Value::Table`.
    pub fn as_table(&self) -> Option<&Vec<(Value, Value)>> {
        if let Self::Table(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// Marker trait for the types that carry CIF values.
///
/// Implemented on the numeric and text field types here; the
/// data layer adds its own value types in `crystal-cif-io`.
pub trait CIFValue {}

impl CIFValue for Numeric {}
impl CIFValue for TextField {}

impl crate::grammar::SyntacticUnit for Value {
    type FormatOutput = String;

    fn formatted_output(&self) -> Self::FormatOutput {
        match self {
            Value::Inapplicable => ".".to_string(),
            Value::Unknown => "?".to_string(),
            Value::Numeric(numeric) => numeric.to_string(),
            Value::CharString(char_string) => char_string.to_string(),
            Value::TextField(text_field) => text_field.to_string(),
            Value::List(values) => list_text(values),
            Value::Table(pairs) => table_text(pairs),
        }
    }
}

/// Render a list value as CIF 2.0 text: `[`, the values joined by a
/// blank, and `]`. An empty list renders as `[]`.
fn list_text(values: &[Value]) -> String {
    let items = values
        .iter()
        .map(|v| v.formatted_output())
        .collect::<Vec<_>>()
        .join(" ");
    format!("[{items}]")
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Inapplicable => write!(f, "."),
            Value::Unknown => write!(f, "?"),
            Value::Numeric(n) => write!(f, "{n}"),
            Value::CharString(cs) => write!(f, "{cs}"),
            Value::TextField(tf) => write!(f, "{tf}"),
            Value::List(values) => write!(f, "{}", list_text(values)),
            Value::Table(pairs) => write!(f, "{}", table_text(pairs)),
        }
    }
}

/// Render a table value as CIF 2.0 text: `{`, the pairs joined by a
/// blank, and `}`. Each pair is `key:value`. An empty table renders
/// as `{}`.
fn table_text(pairs: &[(Value, Value)]) -> String {
    let items = pairs
        .iter()
        .map(|(k, v)| format!("{}:{}", k.formatted_output(), v.formatted_output()))
        .collect::<Vec<_>>()
        .join(" ");
    format!("{{{items}}}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_display_round_trips() {
        let value = Value::List(vec![
            Value::Numeric(Numeric::from(1i32)),
            Value::CharString(CharString::Unquoted(crate::grammar::UnquotedString::new(
                "abc".to_string(),
            ))),
            Value::List(vec![Value::Unknown]),
        ]);
        assert_eq!(value.to_string(), "[1 abc [?]]");
        let empty = Value::List(Vec::new());
        assert_eq!(empty.to_string(), "[]");
    }

    #[test]
    fn table_display_round_trips() {
        let pairs = vec![(
            Value::CharString(CharString::SingleQuoted(
                crate::grammar::SingleQuotedString::new("zero".to_string()),
            )),
            Value::Numeric(Numeric::from(0i32)),
        )];
        let value = Value::Table(pairs);
        assert_eq!(value.to_string(), "{'zero':0}");
        let empty = Value::Table(Vec::new());
        assert_eq!(empty.to_string(), "{}");
    }

    #[test]
    fn as_list_returns_the_elements() {
        let value = Value::List(vec![Value::Inapplicable, Value::Unknown]);
        match value.as_list() {
            Some(elements) => assert_eq!(elements.len(), 2),
            None => panic!("as_list must return the element list"),
        }
        assert!(Value::Inapplicable.as_list().is_none());
    }
}
