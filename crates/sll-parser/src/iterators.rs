use pest::iterators::{Pair, Pairs};

use crate::{
    error::{ParseError, ParseResult},
    Rule,
};

pub trait PairsExt<'i> {
    fn next_token(&mut self) -> Result<Pair<'i, Rule>, ParseError<'i>>;
}

impl<'i> PairsExt<'i> for Pairs<'i, Rule>
where
    Pairs<'i, Rule>: Iterator<Item = Pair<'i, Rule>>,
{
    fn next_token(&mut self) -> Result<Pair<'i, Rule>, ParseError<'i>> {
        self.next().ok_or(ParseError::MissingToken)
    }
}
