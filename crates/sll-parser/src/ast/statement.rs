use super::*;

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Local(Binding<'a>, Type<'a>, Option<Expression<'a>>),
    Expr(Expression<'a>),
    Item(Item<'a>),
}

pub fn statement<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, Statement<'a>> {
    Ok(match pair.as_rule() {
        Rule::local => {
            let mut local = pair.into_inner();

            let binding = binding(arena, local.next_token()?)?;

            let mut ty = Type::Inferred;
            let mut assignment = None;

            loop {
                if let Some(typespec_or_assignment) = local.next() {
                    match typespec_or_assignment.as_rule() {
                        Rule::typespec => {
                            ty = typespec(arena, typespec_or_assignment)?;
                            continue;
                        }
                        Rule::assign => {
                            assignment = Some(expr(arena, local.next_token()?)?);
                            break;
                        }
                        _ => return Err(ParseError::UnexpectedToken(typespec_or_assignment)),
                    }
                }

                break;
            }

            Statement::Local(binding, ty, assignment)
        }
        Rule::expr => Statement::Expr(expr(arena, pair)?),
        Rule::item => Statement::Item(item(arena, pair)?),
        _ => return Err(ParseError::UnexpectedToken(pair)),
    })
}
