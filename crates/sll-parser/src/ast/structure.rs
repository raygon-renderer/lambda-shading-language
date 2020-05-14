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
