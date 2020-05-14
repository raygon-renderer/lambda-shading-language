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

    assert_eq!(item.next_token()?.as_rule(), Rule::struct_keyword); // skip keyword
    let ident = ident(item.next_token()?)?;
    let ty = ty(item.next_token()?)?;
    assert_eq!(item.next_token()?.as_rule(), Rule::assign); // skip assign token
    let value = expr(item.next_token()?)?;

    Ok(ConstItem { ident, ty, value })
}

pub fn struct_decl(item: Pair<Rule>) -> ParseResult<Structure> {
    let mut fields = Vec::new();

    for field in item.into_inner().filter(|pair| pair.as_rule() == Rule::struct_field) {
        let mut field = field.into_inner();

        fields.push(StructureField {
            ident: ident(field.next_token()?)?,
            ty: ty(field.next_token()?)?,
        });
    }

    Ok(Structure { fields })
}
