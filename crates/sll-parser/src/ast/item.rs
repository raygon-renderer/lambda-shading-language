use super::*;

#[derive(Debug, Clone)]
pub enum Item<'a> {
    Struct(Structure<'a>),
    Function(Function<'a>),
    Const(ConstItem<'a>),
}

#[derive(Debug, Clone)]
pub struct ConstItem<'a> {
    pub ident: Ident<'a>,
    pub ty: Type<'a>,
    pub value: Expression<'a>,
}

pub fn item<'a, 'i>(arena: &'a Bump, item: Pair<'i, Rule>) -> ParseResult<'i, Item<'a>> {
    let item = item.into_inner().next_token()?;

    Ok(match item.as_rule() {
        Rule::struct_decl => Item::Struct(struct_decl(arena, item)?),
        Rule::const_item => Item::Const(const_item(arena, item)?),
        Rule::function => Item::Function(function_item(arena, item)?),
        _ => return Err(ParseError::UnexpectedToken(item)),
    })
}

pub fn const_item<'a, 'i>(arena: &'a Bump, item: Pair<'i, Rule>) -> ParseResult<'i, ConstItem<'a>> {
    let mut item = item.into_inner();

    assert_eq!(item.next_token()?.as_rule(), Rule::const_keyword); // skip keyword

    let ident = ident(arena, item.next_token()?)?;
    let ty = typespec(arena, item.next_token()?)?;

    assert_eq!(item.next_token()?.as_rule(), Rule::assign); // skip assign token

    let value = expr(arena, item.next_token()?)?;

    Ok(ConstItem { ident, ty, value })
}
