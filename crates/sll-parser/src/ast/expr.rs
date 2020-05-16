use super::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Ident(Ident),
    Literal(Lit),
    Unary(UnaryOp, Box<Expression>),
    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>,
    },
    Dereference(Box<Expression>),
    Reference {
        mutable: bool,
        value: Box<Expression>,
    },
    Cast(Box<Expression>, Vec<Type>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    FieldAccess(Box<Expression>, Ident),
    TupleAccess(Box<Expression>, usize),
    ArrayAccess(Box<Expression>, Box<Expression>),
    CodeBlock(CodeBlock),
    Tuple(Vec<Expression>),
    Array(ArrayExpr),
    Struct {
        name: Ident,
        fields: Vec<StructureConstructField>,
    },
    IfExpr {
        condition: Box<Expression>,
        body: CodeBlock,
        fallback: Option<Box<Expression>>,
    },
    WhileLoop {
        label: Option<Ident>,
        condition: Box<Expression>,
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
        Rule::code_block => Expression::CodeBlock(code_block(pair)?),
        Rule::unsafe_code_block => Expression::CodeBlock(unsafe_code_block(pair)?),
        Rule::assign_expr => assign(pair)?,
        Rule::deref_expr => deref_expr(pair)?,
        Rule::struct_construct_expr => struct_construct(pair)?,
        Rule::reference_expr => {
            let mut ref_expr = pair.into_inner();

            let mut_keyword_or_expr = ref_expr.next_token()?;

            match mut_keyword_or_expr.as_rule() {
                Rule::mut_keyword => Expression::Reference {
                    mutable: true,
                    value: expr(ref_expr.next_token()?)?.boxed(),
                },
                Rule::expr => Expression::Reference {
                    mutable: false,
                    value: expr(mut_keyword_or_expr)?.boxed(),
                },
                _ => return Err(ParseError::UnexpectedToken(mut_keyword_or_expr)),
            }
        }
        Rule::prefix => {
            let mut prefix = pair.into_inner();
            let op = unary_op(prefix.next_token()?)?;
            let expr = expr(prefix.next_token()?)?.boxed();

            Expression::Unary(op, expr)
        }
        Rule::array_lit => {
            let mut elements = Vec::new();
            for pair in pair.into_inner() {
                elements.push(expr(pair)?);
            }
            Expression::Array(ArrayExpr::Array(elements))
        }
        Rule::array_splat => {
            let mut splat = pair.into_inner();

            let value = expr(splat.next_token()?)?.boxed();
            let len = expr(splat.next_token()?)?.boxed();

            Expression::Array(ArrayExpr::Splat { value, len })
        }
        Rule::tuple => {
            let mut values = Vec::new();
            for pair in pair.into_inner() {
                values.push(expr(pair)?);
            }
            Expression::Tuple(values)
        }
        Rule::cast_expr => {
            let mut cast_expr = pair.into_inner();
            let expr = expr(cast_expr.next_token()?)?;
            let mut tys = Vec::new();
            for ty in cast_expr {
                tys.push(typespec(ty)?);
            }
            Expression::Cast(expr.boxed(), tys)
        }
        Rule::return_expr => Expression::Return(match pair.into_inner().next() {
            Some(pair) => Some(expr(pair)?.boxed()),
            None => None,
        }),
        Rule::if_expr => {
            let mut if_expr = pair.into_inner();

            let condition = expr(if_expr.next_token()?)?.boxed();
            let body = code_block(if_expr.next_token()?)?;

            let fallback = match if_expr.next() {
                Some(pair) => Some(expr(pair)?.boxed()),
                None => None,
            };

            Expression::IfExpr { condition, body, fallback }
        }
        Rule::break_expr => {
            let mut label = None;
            let mut value = None;

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::ident => label = Some(ident(pair)?),
                    Rule::expr => value = Some(expr(pair)?.boxed()),
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }

            Expression::Break { label, value }
        }
        Rule::inf_loop => {
            let mut label = None;
            let mut body = None;

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::ident => label = Some(ident(pair)?),
                    Rule::code_block => body = Some(code_block(pair)?),
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }

            Expression::InfiniteLoop {
                label,
                body: body.ok_or(ParseError::MissingToken)?,
            }
        }
        Rule::while_loop => {
            let mut label = None;
            let mut condition = None;
            let mut body = None;
            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::ident => label = Some(ident(pair)?),
                    Rule::expr => condition = Some(expr(pair)?.boxed()),
                    Rule::code_block => body = Some(code_block(pair)?),
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }

            Expression::WhileLoop {
                label,
                condition: condition.ok_or(ParseError::MissingToken)?,
                body: body.ok_or(ParseError::MissingToken)?,
            }
        }
        Rule::for_loop => {
            let mut label = None;
            let mut loop_binding = None;
            let mut iterator = None;
            let mut body = None;

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::ident => label = Some(ident(pair)?),
                    Rule::binding => loop_binding = Some(binding(pair)?),
                    Rule::expr => iterator = Some(expr(pair)?.boxed()),
                    Rule::code_block => body = Some(code_block(pair)?),
                    _ => return Err(ParseError::UnexpectedToken(pair)),
                }
            }

            Expression::ForLoop {
                label,
                binding: loop_binding.ok_or(ParseError::MissingToken)?,
                iterator: iterator.ok_or(ParseError::MissingToken)?,
                body: body.ok_or(ParseError::MissingToken)?,
            }
        }
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn deref_expr(pair: Pair<Rule>) -> ParseResult<Expression> {
    let mut accessor = pair.into_inner();

    let explicit_or_implicit = accessor.next_token()?;

    let mut access = match explicit_or_implicit.as_rule() {
        Rule::dereference => return Ok(Expression::Dereference(expr(accessor.next_token()?)?.boxed())),
        _ => expr(explicit_or_implicit)?,
    };

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
