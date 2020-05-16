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

#[derive(Debug, Clone)]
pub struct StructureDestructureField {
    pub ident: Ident,
    pub binding: Binding,
}

#[derive(Debug, Clone)]
pub struct StructureDestructure {
    pub ident: Ident,
    pub fields: Vec<StructureDestructureField>,
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

pub fn struct_destructure(pair: Pair<Rule>) -> ParseResult<StructureDestructure> {
    let mut fields = Vec::new();

    //println!("{:#?}", pair);

    let mut struct_destructure = pair.into_inner();

    let name = ident(struct_destructure.next_token()?)?;

    for pair in struct_destructure {
        match pair.as_rule() {
            Rule::struct_destructure_field => {
                let mut struct_destructure_field = pair.into_inner();

                let mut_keyword_or_ident = struct_destructure_field.next_token()?;

                match mut_keyword_or_ident.as_rule() {
                    Rule::mut_keyword => {
                        let field_name = ident(struct_destructure_field.next_token()?)?;

                        fields.push(StructureDestructureField {
                            ident: field_name.clone(),
                            binding: Binding::Named {
                                mutable: true,
                                ident: field_name,
                            },
                        });
                    }
                    Rule::ident => {
                        let field_name = ident(mut_keyword_or_ident)?;

                        // check if there is a binding
                        match struct_destructure_field.next() {
                            Some(pair) => {
                                fields.push(StructureDestructureField {
                                    ident: field_name,
                                    binding: binding(pair)?,
                                });
                            }
                            None => fields.push(StructureDestructureField {
                                ident: field_name.clone(),
                                binding: Binding::Named {
                                    mutable: false,
                                    ident: field_name,
                                },
                            }),
                        }
                    }
                    _ => return Err(ParseError::UnexpectedToken(mut_keyword_or_ident)),
                }
            }
            _ => return Err(ParseError::UnexpectedToken(pair)),
        }
    }

    Ok(StructureDestructure { ident: name, fields })
}
