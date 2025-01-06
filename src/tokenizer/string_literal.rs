use crate::{token::Token, token_type::TokenType};
use std::{iter::Peekable, str::Chars};

pub fn handle_string_literal(
    v: String,
    chars: &mut Peekable<Chars<'_>>,
    input: &mut String,
    line_index: usize,
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

    let token = Token::new(TokenType::STRING(string), line_index);
    input.clear();
    Ok(token)
}
