use crate::{token::Token, token_type::TokenType};
use std::str::Chars;

pub fn handle_identifier(
    mut identifier: String,
    chars: &mut Chars<'_>,
    input: &mut String,
) -> Result<Token, String> {
    loop {
        let char = chars.next();
        let Some(char) = char else {
            input.clear();
            break;
        };

        if char.is_alphanumeric() || char == '_' {
            identifier.push(char);
        } else {
            input.clear();
            if !char.is_whitespace() {
                input.push(char);
            }
            break;
        }
    }

    let token = Token::new(TokenType::IDENTIFIER(identifier.to_string()));
    Ok(token)
}
