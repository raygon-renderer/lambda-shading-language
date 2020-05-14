use super::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Unary(UnaryOp, Box<Expression>),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Cast(Box<Expression>, Type),
    FunctionCall(Ident, Vec<Expression>),
    CodeBlock(CodeBlock),
    Tuple(Vec<Expression>),
    Literal(Lit),
}

#[derive(Debug, Clone)]
pub enum ArrayExpr {
    Array(Vec<Expression>),
    Splat { value: Expression, len: Expression },
}

pub fn expr(pair: Pair<Rule>) -> ParseResult<Expression> {
    Ok(match pair.as_rule() {
        // expr just wraps generic expressions
        Rule::expr => expr(pair.into_inner().next_token()?)?,
        Rule::literal => Expression::Literal(literal(pair)?),
        Rule::infix => infix(pair)?,
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
