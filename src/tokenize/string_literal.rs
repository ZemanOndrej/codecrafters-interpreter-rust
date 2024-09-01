use crate::{token::Token, token_type::TokenType};
use std::str::Chars;

pub fn handle_string_literal(
    v: String,
    chars: &mut Chars<'_>,
    input: &mut String,
) -> Result<Token, String> {
    let mut string = v.clone();
    loop {
        let char = chars.next();
        let Some(char) = char else {
            return Err("Unterminated string.".to_string());
        };
        if char == '"' {
            break;
        }
        string.push(char);
    }

    let token = Token::new(TokenType::STRING(string));
    input.clear();
    Ok(token)
}
