use super::*;

#[derive(Debug, Clone)]
pub enum Binding<'a> {
    Named { by_ref: bool, mutable: bool, name: Ident<'a> },
    Destructure(Destructure<'a>),
}

#[derive(Debug, Clone)]
pub enum Destructure<'a> {
    TupleDestructure(BumpVec<'a, Binding<'a>>),
    StructureDestructure(StructureDestructure<'a>),
    ArrayDestructure(BumpVec<'a, Binding<'a>>),
}

pub fn named_binding<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Binding<'a>> {
    let mut by_ref = false;
    let mut mutable = false;
    let mut name = None;

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::ref_keyword => by_ref = true,
            Rule::mut_keyword => mutable = true,
            Rule::ident => name = Some(ident(arena, pair)?),
            _ => return Err(ParseError::UnexpectedToken(pair)),
        }
    }
    let name = name.ok_or(ParseError::MissingToken)?;
    Ok(Binding::Named { by_ref, mutable, name })
}

pub fn binding<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Binding<'a>> {
    let mut binding = pair.into_inner().next_token()?;

    Ok(match binding.as_rule() {
        Rule::named_binding => named_binding(arena, binding)?,
        Rule::destructure => Binding::Destructure(destructure(arena, binding)?),
        _ => return Err(ParseError::UnexpectedToken(binding)),
    })
}

pub fn destructure<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Destructure<'a>> {
    let pair = pair.into_inner().next_token()?;

    Ok(match pair.as_rule() {
        Rule::tuple_destructure => Destructure::TupleDestructure(tuple_array_destructure(arena, pair)?),
        Rule::array_destructure => Destructure::ArrayDestructure(tuple_array_destructure(arena, pair)?),
        Rule::struct_destructure => Destructure::StructureDestructure(struct_destructure(arena, pair)?),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn tuple_array_destructure<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, BumpVec<'a, Binding<'a>>> {
    let mut bindings = BumpVec::new_in(arena);

    for pair in pair.into_inner().filter(|pair| pair.as_rule() == Rule::binding) {
        bindings.push(binding(arena, pair)?);
    }

    Ok(bindings)
}
