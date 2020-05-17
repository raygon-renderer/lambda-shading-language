mod assign;
mod binary_op;
mod binding;
mod code_block;
mod expr;
mod function;
mod ident;
mod impls;
mod item;
mod literal;
mod statement;
mod structure;
mod types;
mod unary_op;

pub use self::{
    assign::*, binary_op::*, binding::*, code_block::*, expr::*, function::*, ident::*, impls::*, item::*, literal::*, statement::*, structure::*,
    types::*, unary_op::*,
};

pub(crate) use bumpalo::{
    collections::{String as BumpString, Vec as BumpVec},
    Bump,
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
pub struct File<'a> {
    pub items: BumpVec<'a, Item<'a>>,
}

pub fn parse<'a>(arena: &'a Bump, file: &str) -> File<'a> {
    let mut items = BumpVec::new_in(arena);

    for pair in Grammar::parse(Rule::file, file).unwrap() {
        items.push(match pair.as_rule() {
            Rule::item => item(arena, pair).unwrap(),
            Rule::EOI => break,
            _ => Err(ParseError::UnexpectedToken(pair)).unwrap(),
        });
    }

    File { items }
}
