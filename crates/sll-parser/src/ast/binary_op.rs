use super::*;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Rem,
    Mul,
    Div,
    ShiftLeft,
    ShiftRight,
    LessThanEqual,
    LessThan,
    GreaterThanEqual,
    GreaterThan,
    NotEqual,
    Equal,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Range,
}

lazy_static::lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec![
            Operator::new(Rule::range, Assoc::Right),
            Operator::new(Rule::logical_or, Assoc::Left),
            Operator::new(Rule::logical_and, Assoc::Left),
            Operator::new(Rule::equal, Assoc::Right)
                | Operator::new(Rule::not_equal, Assoc::Right)
                | Operator::new(Rule::greater_than_or_equal, Assoc::Left)
                | Operator::new(Rule::less_than_or_equal, Assoc::Left)
                | Operator::new(Rule::greater_than, Assoc::Left)
                | Operator::new(Rule::less_than, Assoc::Left),
            Operator::new(Rule::bitwise_or, Assoc::Left),
            Operator::new(Rule::bitwise_xor, Assoc::Left),
            Operator::new(Rule::bitwise_and, Assoc::Left),
            Operator::new(Rule::shift_right, Assoc::Left) | Operator::new(Rule::shift_left, Assoc::Left),
            Operator::new(Rule::plus, Assoc::Left) | Operator::new(Rule::minus, Assoc::Left),
            Operator::new(Rule::modulus, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left) | Operator::new(Rule::multiply, Assoc::Left),
        ])
    };
}

pub fn infix<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Expression<'a>> {
    PREC_CLIMBER.climb(
        pair.into_inner(),
        |pair| expr(arena, pair),
        |lhs: ParseResult<Expression<'a>>, op: Pair<'i, Rule>, rhs: ParseResult<Expression<'a>>| {
            Ok(Expression::Binary {
                lhs: lhs?.boxed(arena),
                op: binary_op(op)?,
                rhs: rhs?.boxed(arena),
            })
        },
    )
}

pub fn binary_op(pair: Pair<Rule>) -> ParseResult<BinaryOp> {
    Ok(match pair.as_rule() {
        Rule::multiply => BinaryOp::Mul,
        Rule::divide => BinaryOp::Div,
        Rule::modulus => BinaryOp::Rem,
        Rule::plus => BinaryOp::Add,
        Rule::minus => BinaryOp::Sub,
        Rule::shift_left => BinaryOp::ShiftLeft,
        Rule::shift_right => BinaryOp::ShiftRight,
        Rule::less_than_or_equal => BinaryOp::LessThanEqual,
        Rule::less_than => BinaryOp::LessThan,
        Rule::greater_than_or_equal => BinaryOp::GreaterThanEqual,
        Rule::greater_than => BinaryOp::GreaterThan,
        Rule::not_equal => BinaryOp::NotEqual,
        Rule::equal => BinaryOp::Equal,
        Rule::logical_and => BinaryOp::LogicalAnd,
        Rule::logical_or => BinaryOp::LogicalOr,
        Rule::bitwise_and => BinaryOp::BitwiseAnd,
        Rule::bitwise_or => BinaryOp::BitwiseOr,
        Rule::bitwise_xor => BinaryOp::BitwiseXor,
        Rule::range => BinaryOp::Range,
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
