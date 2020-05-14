use super::*;

#[derive(Debug, Clone)]
pub enum Destructure {
    TupleDestructure(Vec<Binding>),
    StructDestructure(StructDestructure),
    ArrayDestructure(Vec<Binding>),
}

#[derive(Debug, Clone)]
pub struct StructDestructureField {
    pub ident: Ident,
    pub binding: Binding,
}

#[derive(Debug, Clone)]
pub struct StructDestructure {
    pub ident: Ident,
    pub fields: Vec<StructDestructureField>,
}

pub fn destructure(pair: Pair<Rule>) -> ParseResult<Destructure> {
    let pair = pair.into_inner().next_token()?;

    Ok(match pair.as_rule() {
        Rule::tuple_destructure => Destructure::TupleDestructure(tuple_array_destructure(pair)?),
        Rule::array_destructure => Destructure::ArrayDestructure(tuple_array_destructure(pair)?),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn tuple_array_destructure(pair: Pair<Rule>) -> ParseResult<Vec<Binding>> {
    let mut bindings = Vec::new();

    for pair in pair.into_inner().filter(|pair| pair.as_rule() == Rule::binding) {
        bindings.push(binding(pair)?);
    }

    Ok(bindings)
}
