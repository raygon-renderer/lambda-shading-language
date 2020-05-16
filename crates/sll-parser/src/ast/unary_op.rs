use super::*;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Absolute,
    Negate,
    Not,
}

pub fn unary_op(pair: Pair<Rule>) -> ParseResult<UnaryOp> {
    Ok(match pair.as_rule() {
        Rule::plus => UnaryOp::Absolute,
        Rule::minus => UnaryOp::Negate,
        Rule::logical_not => UnaryOp::Not,
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
