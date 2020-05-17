use super::*;

#[derive(Debug, Clone)]
pub struct BoundedType<'a> {
    pub ty: Type<'a>,
    pub bounds: BumpVec<'a, ()>,
}

#[derive(Debug, Clone)]
pub enum Type<'a> {
    /// Array type with an element type and constant expression for length
    Array { element: &'a Type<'a>, len: &'a Expression<'a> },

    /// Tuple type made up of made types
    Tuple(BumpVec<'a, Type<'a>>),

    /// Reference type
    Ref { mutable: bool, ty: &'a Type<'a> },

    /// Named type, including primitives
    Named(Ident<'a>),

    /// Generic type with specified parameter types
    Generic { base: &'a Type<'a>, params: BumpVec<'a, Type<'a>> },

    /// Inferred type to be deduced by the compiler
    Inferred,
}

impl<'a> Type<'a> {
    pub fn boxed(self, arena: &'a Bump) -> &'a Type<'a> {
        arena.alloc(self)
    }
}

pub fn typespec<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Type<'a>> {
    Ok(match pair.as_rule() {
        Rule::typespec => return typespec(arena, pair.into_inner().next_token()?),
        Rule::ident => Type::Named(ident(arena, pair)?),
        Rule::tuple_ty => {
            let mut tys = BumpVec::new_in(arena);

            for pair in pair.into_inner() {
                tys.push(typespec(arena, pair)?);
            }

            Type::Tuple(tys)
        }
        Rule::array_ty => {
            let mut array_ty = pair.into_inner();

            let element = typespec(arena, array_ty.next_token()?)?.boxed(arena);
            let len = expr(arena, array_ty.next_token()?)?.boxed(arena);

            Type::Array { element, len }
        }
        Rule::ref_ty => {
            let mut mutable = false;
            let mut ty = None;

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::mut_keyword => mutable = true,
                    _ => ty = Some(typespec(arena, pair)?.boxed(arena)),
                }
            }

            Type::Ref {
                mutable,
                ty: ty.ok_or(ParseError::MissingToken)?,
            }
        }
        Rule::generic_ty => {
            let mut generic_ty = pair.into_inner();

            let base = typespec(arena, generic_ty.next_token()?)?.boxed(arena);
            let mut params = BumpVec::new_in(arena);

            for pair in generic_ty {
                params.push(typespec(arena, pair)?);
            }

            Type::Generic { base, params }
        }
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
