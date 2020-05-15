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
    FunctionCall(Ident, Vec<Expression>),
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
    Assign(AccessorExpr, AssignOp, Box<Expression>),
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

#[derive(Debug, Clone)]
pub enum AccessExpr {
    FieldAccess(Ident),
    TupleAccess(usize),
    ArrayAccess(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum AccessorExpr {
    Local(Ident),
    Accessed { base: Box<Expression>, accesses: Vec<AccessExpr> },
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
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn assign(pair: Pair<Rule>) -> ParseResult<Expression> {
    let mut assign = pair.into_inner();

    let ident_or_accessor = assign.next_token()?;

    let accessor = match ident_or_accessor.as_rule() {
        Rule::ident => AccessorExpr::Local(ident(ident_or_accessor)?),
        Rule::accessor_expr => accessor_expr(ident_or_accessor)?,
        _ => return Err(ParseError::UnexpectedToken(ident_or_accessor)),
    };

    let op = assign_op(assign.next_token()?)?;
    let value = expr(assign.next_token()?)?;

    Ok(Expression::Assign(accessor, op, Box::new(value)))
}

pub fn accessor_expr(pair: Pair<Rule>) -> ParseResult<AccessorExpr> {
    let mut accessor = pair.into_inner();

    let base = expr(accessor.next_token()?)?.boxed();

    let mut accesses = Vec::new();

    while let Some(field_or_array_access) = accessor.next() {
        accesses.push(match field_or_array_access.as_rule() {
            Rule::field_access => {
                let ident_or_index = field_or_array_access.into_inner().next_token()?;

                match ident_or_index.as_rule() {
                    Rule::ident => AccessExpr::FieldAccess(ident(ident_or_index)?),
                    Rule::decinteger => AccessExpr::TupleAccess(ident_or_index.as_str().parse::<usize>()?),
                    _ => return Err(ParseError::UnexpectedToken(ident_or_index)),
                }
            }
            Rule::array_access => AccessExpr::ArrayAccess(expr(field_or_array_access.into_inner().next_token()?)?.boxed()),
            _ => return Err(ParseError::UnexpectedToken(field_or_array_access)),
        });
    }

    Ok(AccessorExpr::Accessed { base, accesses })
}
