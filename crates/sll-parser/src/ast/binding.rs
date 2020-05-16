use super::*;

#[derive(Debug, Clone)]
pub enum Binding {
    Named { mutable: bool, ident: Ident },
    Destructure(Destructure),
}

#[derive(Debug, Clone)]
pub enum Destructure {
    TupleDestructure(Vec<Binding>),
    StructureDestructure(StructureDestructure),
    ArrayDestructure(Vec<Binding>),
}

pub fn binding(pair: Pair<Rule>) -> ParseResult<Binding> {
    let mut binding = pair.into_inner();

    let first = binding.next_token()?;

    Ok(match first.as_rule() {
        Rule::mut_keyword => Binding::Named {
            mutable: true,
            ident: ident(binding.next_token()?)?,
        },
        Rule::ident => Binding::Named {
            mutable: false,
            ident: ident(first)?,
        },
        Rule::destructure => Binding::Destructure(destructure(first)?),
        _ => return Err(ParseError::UnexpectedToken(first)),
    })
}

pub fn destructure(pair: Pair<Rule>) -> ParseResult<Destructure> {
    let pair = pair.into_inner().next_token()?;

    Ok(match pair.as_rule() {
        Rule::tuple_destructure => Destructure::TupleDestructure(tuple_array_destructure(pair)?),
        Rule::array_destructure => Destructure::ArrayDestructure(tuple_array_destructure(pair)?),
        Rule::struct_destructure => Destructure::StructureDestructure(struct_destructure(pair)?),
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
