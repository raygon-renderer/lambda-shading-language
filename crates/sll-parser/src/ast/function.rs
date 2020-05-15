use super::*;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Ident,
    pub args: Vec<FunctionArgs>,
    pub ret: Option<Type>,
    pub code: CodeBlock,
}

#[derive(Debug, Clone)]
pub struct FunctionArgs {
    pub binding: Binding,
    pub ty: Type,
}

pub fn function_item(item: Pair<Rule>) -> ParseResult<Function> {
    let mut code = Err(ParseError::MissingToken);
    let mut is_unsafe = false;
    let mut name = Err(ParseError::MissingToken);
    let mut args = Vec::new();
    let mut ret = None;

    for pair in item.into_inner() {
        match pair.as_rule() {
            Rule::unsafe_keyword => is_unsafe = true,
            Rule::ident => name = ident(pair),
            Rule::function_arg => {
                let mut arg = pair.into_inner();
                let binding = binding(arg.next_token()?)?;
                let ty = typespec(arg.next_token()?)?;

                args.push(FunctionArgs { binding, ty });
            }
            Rule::function_return => {
                ret = Some(typespec(pair.into_inner().next_token()?)?);
            }
            Rule::code_block => code = code_block(pair),
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
