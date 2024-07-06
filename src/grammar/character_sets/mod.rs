mod any_print_char;
mod non_blank_char;
mod ordinary_char;
mod special_tokens;
mod text_lead_char;

pub use any_print_char::AnyPrintChar;
pub use non_blank_char::NonBlankChar;
pub use ordinary_char::OrdinaryChar;
pub use special_tokens::*;
pub use text_lead_char::TextLeadChar;

#[cfg(test)]
mod test {
    use crate::grammar::character_sets::ordinary_char::OrdinaryChar;

    #[test]
    fn ordinary_char() {
        let c1 = OrdinaryChar::a;
        println!("{}", c1)
    }
}
