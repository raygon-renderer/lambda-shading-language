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
