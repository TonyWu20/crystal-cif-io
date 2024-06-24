use crate::data_dict::{data_types::DataLabel, NumValue};

use super::AtomSite;

#[derive(Debug, Clone, Copy)]
pub enum Fract {
    X(NumValue<f64>),
    Y(NumValue<f64>),
    Z(NumValue<f64>),
}

impl Fract {
    pub fn as_x(&self) -> Option<&NumValue<f64>> {
        if let Self::X(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_y(&self) -> Option<&NumValue<f64>> {
        if let Self::Y(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_z(&self) -> Option<&NumValue<f64>> {
        if let Self::Z(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl DataLabel for Fract {
    type C = AtomSite;

    fn tag(&self) -> String {
        match self {
            Fract::X(_) => "fract_x".to_string(),
            Fract::Y(_) => "fract_y".to_string(),
            Fract::Z(_) => "fract_z".to_string(),
        }
    }

    fn value_string(&self) -> String {
        match self {
            Fract::X(x) => format!("{x}"),
            Fract::Y(y) => format!("{y}"),
            Fract::Z(z) => format!("{z}"),
        }
    }

    fn clone_boxed(&self) -> Box<dyn DataLabel<C = Self::C>> {
        Box::new(*self)
    }
}
