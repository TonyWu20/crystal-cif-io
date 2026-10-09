use std::fmt::Display;

use crate::grammar::{CharString, Numeric, SyntacticUnit, TextField};

#[derive(Debug, Clone, Default)]
pub enum Value {
    /// '.'
    Inapplicable,
    #[default]
    /// '?'
    Unknown,
    Numeric(Numeric),
    CharString(CharString),
    TextField(TextField),
    /// A CIF 2.0 list value (`[...]`). The elements are any `Value`s,
    /// and a list may nest. Parsed only in the 2.0 dialect.
    List(Vec<Value>),
}

impl Value {
    pub fn as_numeric(&self) -> Option<&Numeric> {
        if let Self::Numeric(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_char_string(&self) -> Option<&CharString> {
        if let Self::CharString(v) = self {
            Some(v)
        } else {
            None
        }
    }

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
}

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
        }
    }
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
    fn as_list_returns_the_elements() {
        let value = Value::List(vec![Value::Inapplicable, Value::Unknown]);
        match value.as_list() {
            Some(elements) => assert_eq!(elements.len(), 2),
            None => panic!("as_list must return the element list"),
        }
        assert!(Value::Inapplicable.as_list().is_none());
    }
}
