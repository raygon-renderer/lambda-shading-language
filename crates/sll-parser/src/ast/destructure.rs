use super::*;

#[derive(Debug, Clone)]
pub enum Destructure {
    TupleDestructure(Vec<Binding>),
    StructDestructure(StructDestructure),
    ArrayDestructure(Vec<Binding>),
}

#[derive(Debug, Clone)]
pub struct StructDestructureField {
    pub ident: Ident,
    pub binding: Binding,
}

#[derive(Debug, Clone)]
pub struct StructDestructure {
    pub ident: Ident,
    pub fields: Vec<StructDestructureField>,
}

pub fn destructure(pair: Pair<Rule>) -> ParseResult<Destructure> {
    let pair = pair.into_inner().next_token()?;

    Ok(match pair.as_rule() {
        Rule::tuple_destructure => Destructure::TupleDestructure(tuple_array_destructure(pair)?),
        Rule::array_destructure => Destructure::ArrayDestructure(tuple_array_destructure(pair)?),
        Rule::struct_destructure => Destructure::StructDestructure(struct_destructure(pair)?),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}

pub fn tuple_array_destructure(pair: Pair<Rule>) -> ParseResult<Vec<Binding>> {
    let mut bindings = Vec::new();

    for pair in pair.into_inner().filter(|pair| pair.as_rule() == Rule::binding) {
        bindings.push(binding(pair)?);
    }

    Ok(bindings)
}

pub fn struct_destructure(pair: Pair<Rule>) -> ParseResult<StructDestructure> {
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

                        fields.push(StructDestructureField {
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
                                fields.push(StructDestructureField {
                                    ident: field_name,
                                    binding: binding(pair)?,
                                });
                            }
                            None => fields.push(StructDestructureField {
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

    Ok(StructDestructure { ident: name, fields })
}
