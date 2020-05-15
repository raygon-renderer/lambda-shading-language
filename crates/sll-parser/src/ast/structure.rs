use super::*;

#[derive(Debug, Clone)]
pub struct Structure {
    pub fields: Vec<StructureField>,
}

#[derive(Debug, Clone)]
pub struct StructureField {
    pub ident: Ident,
    pub ty: Type,
}

pub fn struct_decl(item: Pair<Rule>) -> ParseResult<Structure> {
    let mut fields = Vec::new();

    for field in item.into_inner().filter(|pair| pair.as_rule() == Rule::struct_field) {
        let mut field = field.into_inner();

        fields.push(StructureField {
            ident: ident(field.next_token()?)?,
            ty: typespec(field.next_token()?)?,
        });
    }

    Ok(Structure { fields })
}
