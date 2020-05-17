use super::*;

#[derive(Debug, Clone)]
pub struct CodeBlock<'a> {
    pub is_unsafe: bool,
    pub stmts: BumpVec<'a, Statement<'a>>,
}

pub fn code_block<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, CodeBlock<'a>> {
    let mut stmts = BumpVec::new_in(arena);

    for stmt in pair.into_inner() {
        stmts.push(statement(arena, stmt)?);
    }

    Ok(CodeBlock { is_unsafe: false, stmts })
}

pub fn unsafe_code_block<'a, 'i>(arena: &'a Bump, pair: Pair<'i, Rule>) -> ParseResult<'i, CodeBlock<'a>> {
    let mut block = pair.into_inner();

    let unsafe_keyword_or_code_block = block.next_token()?;

    Ok(match unsafe_keyword_or_code_block.as_rule() {
        Rule::unsafe_keyword => {
            let mut code_block = code_block(arena, block.next_token()?)?;
            code_block.is_unsafe = true;
            code_block
        }
        Rule::code_block => code_block(arena, unsafe_keyword_or_code_block)?,
        _ => return Err(ParseError::UnexpectedToken(unsafe_keyword_or_code_block)),
    })
}
