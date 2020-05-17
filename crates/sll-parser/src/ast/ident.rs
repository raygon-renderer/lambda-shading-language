use super::*;

pub type Ident<'a> = BumpString<'a>;

pub fn ident<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Ident<'a>> {
    assert_eq!(pair.as_rule(), Rule::ident);

    Ok(Ident::from_str_in(pair.as_str(), arena))
}
