use super::*;

#[derive(Debug, Clone)]
pub struct Structure<'a> {
    pub fields: BumpVec<'a, StructureField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructureField<'a> {
    pub name: Ident<'a>,
    pub ty: Type<'a>,
}

#[derive(Debug, Clone)]
pub enum StructureConstructField<'a> {
    Captured(Ident<'a>),
    Explicit { name: Ident<'a>, value: &'a Expression<'a> },
}

#[derive(Debug, Clone)]
pub struct StructureDestructureField<'a> {
    pub name: Ident<'a>,
    pub binding: Binding<'a>,
}

#[derive(Debug, Clone)]
pub struct StructureDestructure<'a> {
    pub name: Ident<'a>,
    pub fields: BumpVec<'a, StructureDestructureField<'a>>,
}

pub fn struct_decl<'a, 'i>(arena: &'a Bump, item: Pair<'i, Rule>) -> ParseResult<'i, Structure<'a>> {
    let mut fields = BumpVec::new_in(arena);

    for field in item.into_inner().filter(|pair| pair.as_rule() == Rule::struct_field) {
        let mut field = field.into_inner();

        fields.push(StructureField {
            name: ident(arena, field.next_token()?)?,
            ty: typespec(arena, field.next_token()?)?,
        });
    }

    Ok(Structure { fields })
}

pub fn struct_construct<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Expression<'a>> {
    let mut construct = pair.into_inner();

    let name = ident(arena, construct.next_token()?)?;

    let mut fields = BumpVec::new_in(arena);

    for pair in construct {
        match pair.as_rule() {
            Rule::struct_construct_field => {
                let mut construct_field = pair.into_inner();

                let name = ident(arena, construct_field.next_token()?)?;

                fields.push(match construct_field.next() {
                    None => StructureConstructField::Captured(name),
                    Some(pair) => StructureConstructField::Explicit {
                        name,
                        value: expr(arena, pair)?.boxed(arena),
                    },
                });
            }
            _ => return Err(ParseError::UnexpectedToken(pair)),
        }
    }

    Ok(Expression::Struct { name, fields })
}

pub fn struct_destructure<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, StructureDestructure<'a>> {
    let mut struct_destructure = pair.into_inner();

    let name = ident(arena, struct_destructure.next_token()?)?;

    let mut fields = BumpVec::new_in(arena);

    for pair in struct_destructure {
        if pair.as_rule() != Rule::struct_destructure_field {
            return Err(ParseError::UnexpectedToken(pair));
        }

        let mut struct_destructure_field = pair.into_inner();

        let ident_or_named_binding = struct_destructure_field.next_token()?;

        fields.push(match ident_or_named_binding.as_rule() {
            Rule::named_binding => {
                let named_binding = named_binding(arena, ident_or_named_binding)?;

                match named_binding {
                    Binding::Named { ref name, .. } => StructureDestructureField {
                        name: name.clone(),
                        binding: named_binding,
                    },
                    _ => panic!("Expected Binding::Named, found {:?} instead", named_binding),
                }
            }
            // if it's not an implicit named binding, and starts with an ident, it's assumed to be an explicit binding
            Rule::ident => StructureDestructureField {
                name: ident(arena, ident_or_named_binding)?,
                binding: binding(arena, struct_destructure_field.next_token()?)?,
            },
            _ => return Err(ParseError::UnexpectedToken(ident_or_named_binding)),
        });
    }

    Ok(StructureDestructure { name, fields })
}
