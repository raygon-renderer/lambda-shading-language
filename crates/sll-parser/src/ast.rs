pub mod assign;
pub mod binary_op;
pub mod binding;
pub mod code_block;
pub mod destructure;
pub mod expr;
pub mod function;
pub mod ident;
pub mod item;
pub mod literal;
pub mod statement;
pub mod structure;
pub mod types;
pub mod unary_op;

pub use self::{
    assign::*, binary_op::*, binding::*, code_block::*, destructure::*, expr::*, function::*, ident::*, item::*, literal::*, statement::*,
    structure::*, types::*, unary_op::*,
};

pub(crate) use crate::error::{ParseError, ParseResult};
pub(crate) use crate::iterators::PairsExt;

pub(crate) use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

pub(crate) use crate::{Grammar, Rule};

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

pub fn parse(file: &str) -> ParseResult<File> {
    let mut items = Vec::new();

    for pair in Grammar::parse(Rule::file, file)? {
        items.push(match pair.as_rule() {
            Rule::item => item(pair)?,
            Rule::EOI => continue,
            _ => return Err(ParseError::UnexpectedToken(pair)),
        });
    }

    Ok(File { items })
}
