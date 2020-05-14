use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Local(Binding, Type, Expression),
    Assign(Dereferenceable, AssignOp, Expression),
    FunctionCall(Ident, Vec<Expression>),
    Item(Item),
}

pub fn statement(pair: Pair<Rule>) -> ParseResult<Statement> {
    unimplemented!()
}
