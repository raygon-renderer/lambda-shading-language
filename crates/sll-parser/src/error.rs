use thiserror::Error;

use pest::error::Error as PestError;

use super::Rule;

pub type ParseResult<'i, T> = Result<T, ParseError<'i>>;

#[derive(Debug, Error)]
pub enum ParseError<'i> {
    #[error("Grammar Parse Error {0}")]
    GrammarError(#[from] PestError<Rule>),

    #[error("Missing token when converting to AST")]
    MissingToken,

    #[error("Unespected token when converting to AST: {0:?}")]
    UnexpectedToken(pest::iterators::Pair<'i, Rule>),
}
