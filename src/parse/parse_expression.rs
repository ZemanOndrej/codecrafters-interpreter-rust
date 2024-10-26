use crate::{evaluate::Expression, sub_tokens::SlashType, token::Token, token_type::TokenType};

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
            // dbg!(&next);
            return Err(generate_error_message(token, end_tokens));
        };
        if end_tokens.contains(&next.token_type) {
            break;
        }
        if next.token_type == TokenType::SLASH(SlashType::COMMENT) {
            continue;
        }
        let value = parse_token(next, input, &mut stack)?
            .ok_or(generate_error_message(token, end_tokens))?;
        stack.push(value);
    }
    let inner = stack
        .pop()
        .ok_or(generate_error_message(token, end_tokens))?;
    Ok((inner, next.unwrap().clone()))
}

fn generate_error_message(token: &Token, end_tokens: &[TokenType]) -> String {
    format!(
        "Error at '{}': Expect {}",
        token.token_type.get_lexeme(),
        end_tokens
            .iter()
            .map(|x| format!("'{}'", x.get_lexeme()))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
