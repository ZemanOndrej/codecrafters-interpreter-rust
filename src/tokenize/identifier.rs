use crate::{token::Token, token_type::TokenType};
use std::{iter::Peekable, str::Chars};

pub fn handle_identifier(
    mut identifier: String,
    chars: &mut Peekable<Chars<'_>>,
    input: &mut String,
    line_index: usize,
) -> Result<Token, String> {
    loop {
        let char = chars.peek();
        let Some(char) = char else {
            input.clear();
            break;
        };

        if char.is_alphanumeric() || char == &'_' {
            identifier.push(chars.next().unwrap());
        } else {
            input.clear();

            break;
        }
    }

    let token = Token::new(TokenType::IDENTIFIER(identifier.to_string()), line_index);
    Ok(token)
}
