use super::*;

#[derive(Debug, Clone)]
pub enum Lit<'a> {
    String(BumpString<'a>),
    RawString(BumpString<'a>),
    Integer(BumpString<'a>, IntegerBase),
    Float(BumpString<'a>),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum IntegerBase {
    Dec,
    Hex,
    Bin,
    Oct,
}

#[rustfmt::skip]
pub fn literal<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Lit<'a>> {
    let pair = pair.into_inner().next_token()?;

    Ok(match pair.as_rule() {
        Rule::decinteger        => Lit::Integer(BumpString::from_str_in(pair.as_str(), arena), IntegerBase::Dec),
        Rule::hexinteger        => Lit::Integer(BumpString::from_str_in(pair.as_str(), arena), IntegerBase::Hex),
        Rule::octinteger        => Lit::Integer(BumpString::from_str_in(pair.as_str(), arena), IntegerBase::Oct),
        Rule::bininteger        => Lit::Integer(BumpString::from_str_in(pair.as_str(), arena), IntegerBase::Bin),
        Rule::float             => Lit::Float  (BumpString::from_str_in(pair.as_str(), arena)),
        Rule::string_content    => Lit::String (BumpString::from_str_in(pair.as_str(), arena)),
        Rule::raw_string      => Lit::RawString(BumpString::from_str_in(pair.into_inner().next_token()?.as_str(), arena)),
        Rule::boolean_true      => Lit::Boolean(true),
        Rule::boolean_false     => Lit::Boolean(false),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
