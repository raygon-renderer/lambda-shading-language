use super::*;

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Ident(Ident<'a>),
    Literal(Lit<'a>),
    Unary(UnaryOp, &'a Expression<'a>),
    Binary {
        lhs: &'a Expression<'a>,
        op: BinaryOp,
        rhs: &'a Expression<'a>,
    },
    Dereference(&'a Expression<'a>),
    Reference {
        mutable: bool,
        value: &'a Expression<'a>,
    },
    StaticAccess(Type<'a>, BumpVec<'a, Ident<'a>>),
    Cast(&'a Expression<'a>, BumpVec<'a, Type<'a>>),
    FunctionCall {
        callable: &'a Expression<'a>,
        turbofish: BumpVec<'a, Type<'a>>,
        params: BumpVec<'a, Expression<'a>>,
    },
    FieldAccess(&'a Expression<'a>, Ident<'a>),
    TupleAccess(&'a Expression<'a>, usize),
    ArrayAccess(&'a Expression<'a>, &'a Expression<'a>),
    CodeBlock(CodeBlock<'a>),
    Tuple(BumpVec<'a, Expression<'a>>),
    Array(ArrayExpr<'a>),
    Struct {
        name: Ident<'a>,
        fields: BumpVec<'a, StructureConstructField<'a>>,
    },
    IfExpr {
        condition: &'a Expression<'a>,
        body: CodeBlock<'a>,
        fallback: Option<&'a Expression<'a>>,
    },
    WhileLoop {
        label: Option<Ident<'a>>,
        condition: &'a Expression<'a>,
        body: CodeBlock<'a>,
    },
    ForLoop {
        label: Option<Ident<'a>>,
        binding: Binding<'a>,
        iterator: &'a Expression<'a>,
        body: CodeBlock<'a>,
    },
    InfiniteLoop {
        label: Option<Ident<'a>>,
        body: CodeBlock<'a>,
    },
    Break {
        label: Option<Ident<'a>>,
        value: Option<&'a Expression<'a>>,
    },
    Return(Option<&'a Expression<'a>>),
    Assign(&'a Expression<'a>, AssignOp, &'a Expression<'a>),
}

impl<'a> Expression<'a> {
    pub fn boxed(self, arena: &'a Bump) -> &'a Expression<'a> {
        arena.alloc(self)
    }
}

#[derive(Debug, Clone)]
pub enum ArrayExpr<'a> {
    Array(BumpVec<'a, Expression<'a>>),
    Splat { value: &'a Expression<'a>, len: &'a Expression<'a> },
}

pub fn expr<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Expression<'a>> {
    Ok(match pair.as_rule() {
        // expr just wraps generic expressions
        Rule::expr => expr(arena, pair.into_inner().next_token()?)?,
        Rule::ident => Expression::Ident(ident(arena, pair)?),
        Rule::literal => Expression::Literal(literal(arena, pair)?),
        Rule::infix => infix(arena, pair)?,
        Rule::code_block => Expression::CodeBlock(code_block(arena, pair)?),
        Rule::unsafe_code_block => Expression::CodeBlock(unsafe_code_block(arena, pair)?),
        Rule::assign_expr => assign(arena, pair)?,
        Rule::deref_expr => deref_expr(arena, pair)?,
        Rule::struct_construct_expr => struct_construct(arena, pair)?,
        Rule::reference_expr => {
            let mut ref_expr = pair.into_inner();

            // consume reference token
            assert_eq!(ref_expr.next_token()?.as_rule(), Rule::reference);

            let mut_keyword_or_expr = ref_expr.next_token()?;

            match mut_keyword_or_expr.as_rule() {
                Rule::mut_keyword => Expression::Reference {
                    mutable: true,
                    value: expr(arena, ref_expr.next_token()?)?.boxed(arena),
                },
                Rule::expr => Expression::Reference {
                    mutable: false,
                    value: expr(arena, mut_keyword_or_expr)?.boxed(arena),
                },
                _ => return Err(ParseError::UnexpectedToken(mut_keyword_or_expr)),
            }
        }
        Rule::prefix => {
            let mut prefix = pair.into_inner();
            let op = unary_op(prefix.next_token()?)?;
            let expr = expr(arena, prefix.next_token()?)?.boxed(arena);

            Expression::Unary(op, expr)
        }
        Rule::array_lit => {
            let mut elements = BumpVec::new_in(arena);
            for pair in pair.into_inner() {
                elements.push(expr(arena, pair)?);
            }
            Expression::Array(ArrayExpr::Array(elements))
        }
        Rule::array_splat => {
            let mut splat = pair.into_inner();

            let value = expr(arena, splat.next_token()?)?.boxed(arena);
            let len = expr(arena, splat.next_token()?)?.boxed(arena);

            Expression::Array(ArrayExpr::Splat { value, len })
        }
        Rule::tuple => {
            let mut values = BumpVec::new_in(arena);
            for pair in pair.into_inner() {
                values.push(expr(arena, pair)?);
            }
            Expression::Tuple(values)
        }
        Rule::cast_expr => {
            let mut cast_expr = pair.into_inner();
            let expr = expr(arena, cast_expr.next_token()?)?.boxed(arena);
            let mut tys = BumpVec::new_in(arena);
            for ty in cast_expr {
                tys.push(typespec(arena, ty)?);
            }
            Expression::Cast(expr, tys)
        }
        Rule::return_expr => Expression::Return(match pair.into_inner().next() {
            Some(pair) => Some(expr(arena, pair)?.boxed(arena)),
            None => None,
        }),
        Rule::if_expr => {
            let mut if_expr = pair.into_inner();

            let condition = expr(arena, if_expr.next_token()?)?.boxed(arena);
            let body = code_block(arena, if_expr.next_token()?)?;

            let fallback = match if_expr.next() {
                Some(pair) => Some(expr(arena, pair)?.boxed(arena)),
                None => None,
            };

            Expression::IfExpr { condition, body, fallback }
        }
        Rule::break_expr => {
            let mut label = None;
            let mut value = None;

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::ident => label = Some(ident(arena, pair)?),
                    Rule::expr => value = Some(expr(arena, pair)?.boxed(arena)),
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
                    Rule::ident => label = Some(ident(arena, pair)?),
                    Rule::code_block => body = Some(code_block(arena, pair)?),
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
                    Rule::ident => label = Some(ident(arena, pair)?),
                    Rule::expr => condition = Some(expr(arena, pair)?.boxed(arena)),
                    Rule::code_block => body = Some(code_block(arena, pair)?),
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
                    Rule::ident => label = Some(ident(arena, pair)?),
                    Rule::binding => loop_binding = Some(binding(arena, pair)?),
                    Rule::expr => iterator = Some(expr(arena, pair)?.boxed(arena)),
                    Rule::code_block => body = Some(code_block(arena, pair)?),
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
        Rule::static_access => {
            let mut static_access = pair.into_inner();

            let base_ty = typespec(arena, static_access.next_token()?)?;

            let mut path = BumpVec::new_in(arena);

            for pair in static_access {
                path.push(ident(arena, pair)?);
            }

            Expression::StaticAccess(base_ty, path)
        }
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn deref_expr<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Expression<'a>> {
    let mut accessor = pair.into_inner();

    let explicit_or_implicit = accessor.next_token()?;

    let mut access = match explicit_or_implicit.as_rule() {
        Rule::dereference => return Ok(Expression::Dereference(expr(arena, accessor.next_token()?)?.boxed(arena))),
        _ => expr(arena, explicit_or_implicit)?,
    };

    while let Some(next_access) = accessor.next() {
        access = match next_access.as_rule() {
            Rule::field_access => {
                let ident_or_index = next_access.into_inner().next_token()?;

                match ident_or_index.as_rule() {
                    Rule::ident => Expression::FieldAccess(access.boxed(arena), ident(arena, ident_or_index)?),
                    Rule::decinteger => Expression::TupleAccess(access.boxed(arena), ident_or_index.as_str().parse::<usize>()?),
                    _ => return Err(ParseError::UnexpectedToken(ident_or_index)),
                }
            }
            Rule::array_access => Expression::ArrayAccess(access.boxed(arena), expr(arena, next_access.into_inner().next_token()?)?.boxed(arena)),
            Rule::function_call => {
                let mut params = BumpVec::new_in(arena);
                let mut turbofish = BumpVec::new_in(arena);

                for pair in next_access.into_inner() {
                    match pair.as_rule() {
                        Rule::expr => params.push(expr(arena, pair)?),
                        Rule::turbofish => {
                            for pair in pair.into_inner() {
                                turbofish.push(typespec(arena, pair)?);
                            }
                        }
                        _ => return Err(ParseError::UnexpectedToken(pair)),
                    };
                }

                Expression::FunctionCall {
                    callable: access.boxed(arena),
                    turbofish,
                    params,
                }
            }
            _ => return Err(ParseError::UnexpectedToken(next_access)),
        };
    }

    Ok(access)
}
