use super::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Local(Binding, Type, Option<Expression>),
    Expr(Expression),
    Item(Item),
}

pub fn statement(pair: Pair<Rule>) -> ParseResult<Statement> {
    Ok(match pair.as_rule() {
        Rule::local => {
            let mut local = pair.into_inner();

            let binding = binding(local.next_token()?)?;

            let mut ty = Type::Inferred;
            let mut assignment = None;

            loop {
                if let Some(typespec_or_assignment) = local.next() {
                    match typespec_or_assignment.as_rule() {
                        Rule::typespec => {
                            ty = typespec(typespec_or_assignment)?;
                            continue;
                        }
                        Rule::assign => {
                            assignment = Some(expr(local.next_token()?)?);
                            break;
                        }
                        _ => return Err(ParseError::UnexpectedToken(typespec_or_assignment)),
                    }
                }

                break;
            }

            Statement::Local(binding, ty, assignment)
        }
        Rule::expr => Statement::Expr(expr(pair)?),
        Rule::item => Statement::Item(item(pair)?),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
