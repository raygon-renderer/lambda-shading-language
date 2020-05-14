use super::*;

#[derive(Debug, Clone)]
pub enum Type {
    Array(Box<ArrayType>),
    Tuple(Vec<Type>),
    Ptr(Box<PointerType>),
    Named(Ident),
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

pub fn ty(pair: Pair<Rule>) -> ParseResult<Type> {
    Ok(match pair.as_rule() {
        Rule::ident => Type::Named(ident(pair)?),
        Rule::tuple_ty => {
            let mut tys = Vec::new();

            for pair in pair.into_inner().filter(|pair| pair.as_rule() == Rule::ty) {
                tys.push(ty(pair)?);
            }

            Type::Tuple(tys)
        }
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
