use super::*;

pub type Ident = String;

pub fn ident(pair: Pair<Rule>) -> ParseResult<Ident> {
    assert_eq!(pair.as_rule(), Rule::ident);

    Ok(Ident::from(pair.as_str()))
}
