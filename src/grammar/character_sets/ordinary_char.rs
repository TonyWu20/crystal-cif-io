use std::{error::Error, fmt::Display};

use winnow::{
    combinator::alt,
    stream::{Accumulate, AsChar},
    Parser,
};

use crate::grammar::SyntacticUnit;

#[repr(u8)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum OrdinaryChar {
    Exclamation = 33,
    Quotation,
    Percent = 37,
    Ampersand,
    ParenOpen = 40,
    ParenClose,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Dot,
    Slash,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    Colon,
    LessThan = 60,
    Equal,
    GreaterThan,
    Question,
    At,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    BackSlash = 92,
    Caret = 94,
    Grave = 96,
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    CurlyBraceOpen,
    VerticalBar,
    CurlyBraceClose,
    Tilde,
}

impl Display for OrdinaryChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self as u8))
    }
}

impl TryFrom<char> for OrdinaryChar {
    type Error = OrdinaryCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '!' => Ok(OrdinaryChar::Exclamation),
            '%' => Ok(OrdinaryChar::Percent),
            '&' => Ok(OrdinaryChar::Ampersand),
            '(' => Ok(OrdinaryChar::ParenOpen),
            ')' => Ok(OrdinaryChar::ParenClose),
            '*' => Ok(OrdinaryChar::Asterisk),
            '+' => Ok(OrdinaryChar::Plus),
            ',' => Ok(OrdinaryChar::Comma),
            '-' => Ok(OrdinaryChar::Minus),
            '.' => Ok(OrdinaryChar::Dot),
            '/' => Ok(OrdinaryChar::Slash),
            '0' => Ok(OrdinaryChar::_0),
            '1' => Ok(OrdinaryChar::_1),
            '2' => Ok(OrdinaryChar::_2),
            '3' => Ok(OrdinaryChar::_3),
            '4' => Ok(OrdinaryChar::_4),
            '5' => Ok(OrdinaryChar::_5),
            '6' => Ok(OrdinaryChar::_6),
            '7' => Ok(OrdinaryChar::_7),
            '8' => Ok(OrdinaryChar::_8),
            '9' => Ok(OrdinaryChar::_9),
            ':' => Ok(OrdinaryChar::Colon),
            '<' => Ok(OrdinaryChar::LessThan),
            '=' => Ok(OrdinaryChar::Equal),
            '>' => Ok(OrdinaryChar::GreaterThan),
            '?' => Ok(OrdinaryChar::Question),
            '@' => Ok(OrdinaryChar::At),
            'A' => Ok(OrdinaryChar::A),
            'B' => Ok(OrdinaryChar::B),
            'C' => Ok(OrdinaryChar::C),
            'D' => Ok(OrdinaryChar::D),
            'E' => Ok(OrdinaryChar::E),
            'F' => Ok(OrdinaryChar::F),
            'G' => Ok(OrdinaryChar::G),
            'H' => Ok(OrdinaryChar::H),
            'I' => Ok(OrdinaryChar::I),
            'J' => Ok(OrdinaryChar::J),
            'K' => Ok(OrdinaryChar::K),
            'L' => Ok(OrdinaryChar::L),
            'M' => Ok(OrdinaryChar::M),
            'N' => Ok(OrdinaryChar::N),
            'O' => Ok(OrdinaryChar::O),
            'P' => Ok(OrdinaryChar::P),
            'Q' => Ok(OrdinaryChar::Q),
            'R' => Ok(OrdinaryChar::R),
            'S' => Ok(OrdinaryChar::S),
            'T' => Ok(OrdinaryChar::T),
            'U' => Ok(OrdinaryChar::U),
            'V' => Ok(OrdinaryChar::V),
            'W' => Ok(OrdinaryChar::W),
            'X' => Ok(OrdinaryChar::X),
            'Y' => Ok(OrdinaryChar::Y),
            'Z' => Ok(OrdinaryChar::Z),
            '\\' => Ok(OrdinaryChar::BackSlash),
            '^' => Ok(OrdinaryChar::Caret),
            '`' => Ok(OrdinaryChar::Grave),
            'a' => Ok(OrdinaryChar::a),
            'b' => Ok(OrdinaryChar::b),
            'c' => Ok(OrdinaryChar::c),
            'd' => Ok(OrdinaryChar::d),
            'e' => Ok(OrdinaryChar::e),
            'f' => Ok(OrdinaryChar::f),
            'g' => Ok(OrdinaryChar::g),
            'h' => Ok(OrdinaryChar::h),
            'i' => Ok(OrdinaryChar::i),
            'j' => Ok(OrdinaryChar::j),
            'k' => Ok(OrdinaryChar::k),
            'l' => Ok(OrdinaryChar::l),
            'm' => Ok(OrdinaryChar::m),
            'n' => Ok(OrdinaryChar::n),
            'o' => Ok(OrdinaryChar::o),
            'p' => Ok(OrdinaryChar::p),
            'q' => Ok(OrdinaryChar::q),
            'r' => Ok(OrdinaryChar::r),
            's' => Ok(OrdinaryChar::s),
            't' => Ok(OrdinaryChar::t),
            'u' => Ok(OrdinaryChar::u),
            'v' => Ok(OrdinaryChar::v),
            'w' => Ok(OrdinaryChar::w),
            'x' => Ok(OrdinaryChar::x),
            'y' => Ok(OrdinaryChar::y),
            'z' => Ok(OrdinaryChar::z),
            '{' => Ok(OrdinaryChar::CurlyBraceOpen),
            '|' => Ok(OrdinaryChar::VerticalBar),
            '}' => Ok(OrdinaryChar::CurlyBraceClose),
            '~' => Ok(OrdinaryChar::Tilde),
            _ => Err(OrdinaryCharError),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrdinaryCharError;

impl From<OrdinaryChar> for char {
    fn from(value: OrdinaryChar) -> Self {
        char::from(value as u8)
    }
}

impl Display for OrdinaryCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Not included in CIF `<OrdinaryChar>`.")
    }
}

impl Error for OrdinaryCharError {}

impl SyntacticUnit for OrdinaryChar {
    type ParseResult = OrdinaryChar;
    type FormatOutput = char;

    fn parser(input: &mut &str) -> winnow::prelude::PResult<Self::ParseResult> {
        alt((
            alt(('!', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/')),
            alt(('0', '1', '2', '3', '4', '5', '6', '7', '8', '9')),
            alt((
                ':', '<', '=', '>', '?', '@', '\\', '^', '`', '{', ',', '}', '~',
            )),
            alt((
                alt(('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I')),
                alt(('J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q')),
                alt(('R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z')),
            )),
            alt((
                alt(('a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i')),
                alt(('j', 'k', 'l', 'm', 'n', 'o', 'p', 'q')),
                alt(('r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z')),
            )),
        ))
        .map(|c| Self::try_from(c).unwrap())
        .parse_next(input)
    }

    fn formatted_output(&self) -> Self::FormatOutput {
        char::from(*self as u8)
    }
}

impl AsChar for OrdinaryChar {
    fn as_char(self) -> char {
        char::from(self)
    }

    fn is_alpha(self) -> bool {
        (65..91).contains(&(self as u8)) || (97..123).contains(&(self as u8))
    }

    fn is_alphanum(self) -> bool {
        (65..91).contains(&(self as u8))
            || (97..123).contains(&(self as u8))
            || (48..58).contains(&(self as u8))
    }

    fn is_dec_digit(self) -> bool {
        (48..58).contains(&(self as u8))
    }

    fn is_hex_digit(self) -> bool {
        (48..54).contains(&(self as u8))
    }

    fn is_oct_digit(self) -> bool {
        (48..56).contains(&(self as u8))
    }

    fn len(self) -> usize {
        1
    }

    fn is_space(self) -> bool {
        false
    }

    fn is_newline(self) -> bool {
        false
    }
}

impl Accumulate<OrdinaryChar> for String {
    fn initial(capacity: Option<usize>) -> Self {
        match capacity {
            Some(capacity) => String::with_capacity(capacity.clamp(0, 65536)),
            None => String::new(),
        }
    }

    fn accumulate(&mut self, acc: OrdinaryChar) {
        self.push(acc.as_char())
    }
}
