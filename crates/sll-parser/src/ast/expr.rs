use super::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Ident(Ident),
    Unary(UnaryOp, Box<Expression>),
    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>,
    },
    Cast(Box<Expression>, Vec<Type>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    FieldAccess(Box<Expression>, Ident),
    TupleAccess(Box<Expression>, usize),
    ArrayAccess(Box<Expression>, Box<Expression>),
    CodeBlock(CodeBlock),
    Tuple(Vec<Expression>),
    Array(ArrayExpr),
    Literal(Lit),
    IfExpr {
        condition: Box<Expression>,
        body: CodeBlock,
        fallback: Option<Box<Expression>>,
    },
    WhileLoop {
        label: Option<Ident>,
        expr: Box<Expression>,
        body: CodeBlock,
    },
    ForLoop {
        label: Option<Ident>,
        binding: Binding,
        iterator: Box<Expression>,
        body: CodeBlock,
    },
    InfiniteLoop {
        label: Option<Ident>,
        body: CodeBlock,
    },
    Break {
        label: Option<Ident>,
        value: Option<Box<Expression>>,
    },
    Return(Option<Box<Expression>>),
    Assign(Box<Expression>, AssignOp, Box<Expression>),
}

impl Expression {
    pub fn boxed(self) -> Box<Expression> {
        Box::new(self)
    }
}

#[derive(Debug, Clone)]
pub enum ArrayExpr {
    Array(Vec<Expression>),
    Splat { value: Box<Expression>, len: Box<Expression> },
}

pub fn expr(pair: Pair<Rule>) -> ParseResult<Expression> {
    Ok(match pair.as_rule() {
        // expr just wraps generic expressions
        Rule::expr => expr(pair.into_inner().next_token()?)?,
        Rule::ident => Expression::Ident(ident(pair)?),
        Rule::literal => Expression::Literal(literal(pair)?),
        Rule::infix => infix(pair)?,
        Rule::unsafe_code_block => Expression::CodeBlock(unsafe_code_block(pair)?),
        Rule::assign_expr => assign(pair)?,
        Rule::deref_expr => deref_expr(pair)?,
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn deref_expr(pair: Pair<Rule>) -> ParseResult<Expression> {
    let mut accessor = pair.into_inner();

    // get base expression to start with
    let mut access = expr(accessor.next_token()?)?;

    while let Some(next_access) = accessor.next() {
        access = match next_access.as_rule() {
            Rule::field_access => {
                let ident_or_index = next_access.into_inner().next_token()?;

                match ident_or_index.as_rule() {
                    Rule::ident => Expression::FieldAccess(access.boxed(), ident(ident_or_index)?),
                    Rule::decinteger => Expression::TupleAccess(access.boxed(), ident_or_index.as_str().parse::<usize>()?),
                    _ => return Err(ParseError::UnexpectedToken(ident_or_index)),
                }
            }
            Rule::array_access => Expression::ArrayAccess(access.boxed(), expr(next_access.into_inner().next_token()?)?.boxed()),
            Rule::function_call => {
                let mut params = Vec::new();
                for param in next_access.into_inner() {
                    params.push(expr(param)?);
                }
                Expression::FunctionCall(access.boxed(), params)
            }
            _ => return Err(ParseError::UnexpectedToken(next_access)),
        };
    }

    Ok(access)
}
