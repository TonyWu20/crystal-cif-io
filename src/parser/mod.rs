use pest_derive::Parser;

mod ast;

#[derive(Parser)]
#[grammar = "cif.pest"]
pub struct CIFParser;

// // 需要公开pest的Error类型
// pub type Error = pest::error::Error<Rule>;
// pub type Result<T> = std::result::Result<T, Error>;
