mod assign;
mod binary_op;
mod binding;
mod code_block;
mod expr;
mod function;
mod ident;
mod item;
mod literal;
mod statement;
mod structure;
mod types;
mod unary_op;

pub use self::{
    assign::*, binary_op::*, binding::*, code_block::*, expr::*, function::*, ident::*, item::*, literal::*, statement::*, structure::*, types::*,
    unary_op::*,
};

pub(crate) use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

pub(crate) use crate::{
    error::{ParseError, ParseResult},
    iterators::PairsExt,
    Grammar, Rule,
};

#[derive(Debug, Clone)]
pub struct File {
    pub items: Vec<Item>,
}

pub fn parse(file: &str) -> ParseResult<File> {
    let mut items = Vec::new();

    for pair in Grammar::parse(Rule::file, file)? {
        items.push(match pair.as_rule() {
            Rule::item => item(pair)?,
            Rule::EOI => break,
            _ => return Err(ParseError::UnexpectedToken(pair)),
        });
    }

    Ok(File { items })
}
