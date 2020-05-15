use super::*;

#[derive(Debug, Clone)]
pub enum Binding {
    Named { mutable: bool, ident: Ident },
    Destructure(Destructure),
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
