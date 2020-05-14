use super::*;

#[derive(Debug, Clone)]
pub enum Dereferenceable {
    Field,
    ArrayIndex,
    Ident(Ident),
}

#[derive(Debug, Clone, Copy)]
pub enum AssignOp {
    Assign,
    Mul,
    Div,
    Rem,
    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}
