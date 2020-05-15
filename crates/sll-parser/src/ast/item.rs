use super::*;

#[derive(Debug, Clone)]
pub enum Item {
    Struct(Structure),
    Function(Function),
    Const(ConstItem),
}

#[derive(Debug, Clone)]
pub struct ConstItem {
    pub ident: Ident,
    pub ty: Type,
    pub value: Expression,
}

pub fn item(item: Pair<Rule>) -> ParseResult<Item> {
    let item = item.into_inner().next_token()?;

    Ok(match item.as_rule() {
        Rule::struct_decl => Item::Struct(struct_decl(item)?),
        Rule::const_item => Item::Const(const_item(item)?),
        Rule::function => Item::Function(function_item(item)?),
        _ => return Err(ParseError::UnexpectedToken(item)),
    })
}

pub fn const_item(item: Pair<Rule>) -> ParseResult<ConstItem> {
    let mut item = item.into_inner();

    assert_eq!(item.next_token()?.as_rule(), Rule::const_keyword); // skip keyword

    let ident = ident(item.next_token()?)?;
    let ty = typespec(item.next_token()?)?;

    assert_eq!(item.next_token()?.as_rule(), Rule::assign); // skip assign token

    let value = expr(item.next_token()?)?;

    Ok(ConstItem { ident, ty, value })
}
