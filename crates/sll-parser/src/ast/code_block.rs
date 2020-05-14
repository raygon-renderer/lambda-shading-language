use super::*;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub is_unsafe: bool,
    pub stmts: Vec<Statement>,
}

pub fn code_block(pair: Pair<Rule>) -> ParseResult<CodeBlock> {
    let mut stmts = Vec::new();

    for stmt in pair.into_inner() {
        stmts.push(statement(stmt)?);
    }

    Ok(CodeBlock { is_unsafe: false, stmts })
}

pub fn unsafe_code_block(pair: Pair<Rule>) -> ParseResult<CodeBlock> {
    let mut block = pair.into_inner();

    let unsafe_keyword_or_code_block = block.next_token()?;

    Ok(match unsafe_keyword_or_code_block.as_rule() {
        Rule::unsafe_keyword => {
            let mut code_block = code_block(block.next_token()?)?;
            code_block.is_unsafe = true;
            code_block
        }
        Rule::code_block => code_block(block.next_token()?)?,
        _ => return Err(ParseError::UnexpectedToken(unsafe_keyword_or_code_block)),
    })
}
