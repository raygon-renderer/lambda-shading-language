use super::*;

#[derive(Debug, Clone)]
pub enum Lit {
    String(String),
    RawString(String),
    Integer(String, IntegerBase),
    Float(String),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy)]
pub enum IntegerBase {
    Dec,
    Hex,
    Bin,
    Oct,
}

pub fn literal(pair: Pair<Rule>) -> ParseResult<Lit> {
    let pair = pair.into_inner().next_token()?;

    Ok(match pair.as_rule() {
        Rule::decinteger => Lit::Integer(pair.as_str().to_owned(), IntegerBase::Dec),
        Rule::hexinteger => Lit::Integer(pair.as_str().to_owned(), IntegerBase::Hex),
        Rule::octinteger => Lit::Integer(pair.as_str().to_owned(), IntegerBase::Oct),
        Rule::bininteger => Lit::Integer(pair.as_str().to_owned(), IntegerBase::Bin),
        Rule::float => Lit::Float(pair.as_str().to_owned()),
        Rule::string_content => Lit::String(pair.as_str().to_owned()),
        Rule::raw_string => Lit::RawString(pair.into_inner().next_token()?.as_str().to_owned()),
        Rule::boolean_true => Lit::Boolean(true),
        Rule::boolean_false => Lit::Boolean(false),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
