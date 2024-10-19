use crate::{evaluate::Expression, token::Token, token_type::TokenType};

use super::{parse_token::parse_token, InputIter};

pub fn parse_expression(
    input: &mut InputIter,
    token: &Token,
    end_tokens: &[TokenType],
) -> Result<(Expression, Token), String> {
    let mut stack = Vec::new();
    let mut next;
    loop {
        next = input.next();
        let Some(next) = next else {
            dbg!(&next);
            return Err(format!(
                "Error at '{}': Expect closing bracket",
                token.token_type.get_lexeme()
            ));
        };
        if end_tokens.contains(&next.token_type) {
            break;
        }
        let value = parse_token(next, input, &mut stack)?.unwrap();
        stack.push(value);
    }
    let inner = stack.pop().ok_or("Error: Missing parameters")?;
    Ok((inner, next.unwrap().clone()))
}
