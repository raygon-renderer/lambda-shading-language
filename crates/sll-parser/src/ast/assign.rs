use super::*;

#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

pub fn assign(pair: Pair<Rule>) -> ParseResult<Expression> {
    let mut assign = pair.into_inner();

    let ident_or_deref = assign.next_token()?;

    let access = match ident_or_deref.as_rule() {
        Rule::ident => Expression::Ident(ident(ident_or_deref)?),
        Rule::deref_expr => deref_expr(ident_or_deref)?,
        _ => return Err(ParseError::UnexpectedToken(ident_or_deref)),
    };

    let op = assign_op(assign.next_token()?)?;
    let value = expr(assign.next_token()?)?;

    Ok(Expression::Assign(access.boxed(), op, value.boxed()))
}

pub fn assign_op(pair: Pair<Rule>) -> ParseResult<AssignOp> {
    assert_eq!(pair.as_rule(), Rule::assign_operator);

    let mut assign_operator = pair.into_inner().next_token()?;

    Ok(match assign_operator.as_rule() {
        Rule::assign => AssignOp::Assign,
        Rule::multiply => AssignOp::Mul,
        Rule::divide => AssignOp::Div,
        Rule::modulus => AssignOp::Rem,
        Rule::plus => AssignOp::Add,
        Rule::minus => AssignOp::Sub,
        Rule::shift_left => AssignOp::ShiftLeft,
        Rule::shift_right => AssignOp::ShiftRight,
        Rule::bitwise_and => AssignOp::BitwiseAnd,
        Rule::bitwise_or => AssignOp::BitwiseOr,
        Rule::bitwise_xor => AssignOp::BitwiseXor,
        _ => return Err(ParseError::UnexpectedToken(assign_operator)),
    })
}
