use super::*;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Reference { mutable: bool },
    Dereference,
    Absolute,
    Negate,
    Not,
}