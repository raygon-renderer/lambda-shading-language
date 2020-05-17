use super::*;

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub name: Ident<'a>,
    pub args: BumpVec<'a, FunctionArgs<'a>>,
    pub ret: Option<Type<'a>>,
    pub code: CodeBlock<'a>,
}

#[derive(Debug, Clone)]
pub enum FunctionArgs<'a> {
    SelfValue { by_ref: bool, mutable: bool },
    Binding { binding: Binding<'a>, ty: Type<'a> },
}

pub fn function_item<'a, 'i>(arena: &'a Bump, item: Pair<'i, Rule>) -> ParseResult<'i, Function<'a>> {
    let mut code = Err(ParseError::MissingToken);
    let mut is_unsafe = false;
    let mut name = Err(ParseError::MissingToken);
    let mut args = BumpVec::new_in(arena);
    let mut ret = None;

    for pair in item.into_inner() {
        match pair.as_rule() {
            Rule::unsafe_keyword => is_unsafe = true,
            Rule::ident => name = ident(arena, pair),
            Rule::function_arg => {
                let mut arg = pair.into_inner();

                let binding_or_self = arg.next_token()?;

                args.push(match binding_or_self.as_rule() {
                    Rule::function_self => {
                        let mut by_ref = false;
                        let mut mutable = false;

                        for pair in binding_or_self.into_inner() {
                            match pair.as_rule() {
                                Rule::reference => by_ref = true,
                                Rule::mut_keyword => mutable = true,
                                _ => return Err(ParseError::UnexpectedToken(pair)),
                            }
                        }

                        FunctionArgs::SelfValue { by_ref, mutable }
                    }
                    Rule::binding => {
                        let binding = binding(arena, binding_or_self)?;
                        let ty = typespec(arena, arg.next_token()?)?;

                        FunctionArgs::Binding { binding, ty }
                    }
                    _ => return Err(ParseError::UnexpectedToken(binding_or_self)),
                });
            }
            Rule::function_return => {
                ret = Some(typespec(arena, pair.into_inner().next_token()?)?);
            }
            Rule::code_block => code = code_block(arena, pair),
            _ => return Err(ParseError::UnexpectedToken(pair)),
        }
    }

    let mut code: CodeBlock = code?;
    code.is_unsafe = is_unsafe;

    Ok(Function {
        name: name?,
        args,
        ret,
        code,
    })
}
