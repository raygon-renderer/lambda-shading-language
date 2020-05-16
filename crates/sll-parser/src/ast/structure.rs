use super::*;

#[derive(Debug, Clone)]
pub struct Structure {
    pub fields: Vec<StructureField>,
}

#[derive(Debug, Clone)]
pub struct StructureField {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub enum StructureConstructField {
    Captured(Ident),
    Explicit { name: Ident, value: Box<Expression> },
}

pub fn struct_decl(item: Pair<Rule>) -> ParseResult<Structure> {
    let mut fields = Vec::new();

    for field in item.into_inner().filter(|pair| pair.as_rule() == Rule::struct_field) {
        let mut field = field.into_inner();

        fields.push(StructureField {
            name: ident(field.next_token()?)?,
            ty: typespec(field.next_token()?)?,
        });
    }

    Ok(Structure { fields })
}

pub fn struct_construct(pair: Pair<Rule>) -> ParseResult<Expression> {
    let mut construct = pair.into_inner();

    let name = ident(construct.next_token()?)?;

    let mut fields = Vec::new();

    for pair in construct {
        match pair.as_rule() {
            Rule::struct_construct_field => {
                let mut construct_field = pair.into_inner();

                let name = ident(construct_field.next_token()?)?;

                fields.push(match construct_field.next() {
                    None => StructureConstructField::Captured(name),
                    Some(pair) => StructureConstructField::Explicit {
                        name,
                        value: expr(pair)?.boxed(),
                    },
                });
            }
            _ => return Err(ParseError::UnexpectedToken(pair)),
        }
    }

    Ok(Expression::Struct { name, fields })
}
