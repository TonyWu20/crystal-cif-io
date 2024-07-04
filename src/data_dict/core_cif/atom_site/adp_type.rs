use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum AdpType {
    Uani,
    Uiso,
    Uovl,
    Umpe,
    Bani,
    Biso,
    Bovl,
}

impl Display for AdpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = format!("{:?}", self);
        write!(f, "{}", content)
    }
}
