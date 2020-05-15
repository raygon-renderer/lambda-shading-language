use super::*;

#[derive(Debug, Clone)]
pub enum Type {
    Array(Box<ArrayType>),
    Tuple(Vec<Type>),
    Ptr(Box<PointerType>),
    Named(Ident),
    Inferred,
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub element: Type,
    pub len: Expression,
}

#[derive(Debug, Clone)]
pub struct PointerType {
    pub mutable: bool,
    pub ty: Type,
}

pub fn typespec(pair: Pair<Rule>) -> ParseResult<Type> {
    Ok(match pair.as_rule() {
        Rule::typespec => return typespec(pair.into_inner().next_token()?),
        Rule::ident => Type::Named(ident(pair)?),
        Rule::tuple_ty => {
            let mut tys = Vec::new();

            for pair in pair.into_inner() {
                tys.push(typespec(pair)?);
            }

            Type::Tuple(tys)
        }
        Rule::array_ty => {
            let mut array_ty = pair.into_inner();

            let element = typespec(array_ty.next_token()?)?;
            let len = expr(array_ty.next_token()?)?;

            Type::Array(Box::new(ArrayType { element, len }))
        }
        Rule::ptr_ty => {
            let mut ptr_ty = pair.into_inner();

            let mutability = ptr_ty.next_token()?;

            let mutable = match mutability.as_rule() {
                Rule::mut_keyword => true,
                Rule::const_keyword => false,
                _ => return Err(ParseError::UnexpectedToken(mutability)),
            };

            let ty = typespec(ptr_ty.next_token()?)?;

            Type::Ptr(Box::new(PointerType { mutable, ty }))
        }
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
