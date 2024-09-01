use crate::{token::Token, token_type::TokenType};
use std::str::Chars;

pub fn handle_number_literal(
    number: String,
    chars: &mut Chars<'_>,
    input: &mut String,
) -> Result<Token, String> {
    let mut number = number.to_string();
    loop {
        let char = chars.next();
        let Some(char) = char else {
            input.clear();
            break;
        };
        if let Some(_) = char.to_digit(10) {
            number.push(char);
        } else {
            if char == '.' {
                number.push(char);
            } else {
                input.clear();
                if !char.is_whitespace() {
                    input.push(char);
                }
                break;
            }
        }
    }

    let Ok(_) = number.parse::<f64>() else {
        return Err("Invalid number.".to_string());
    };

    let token = Token::new(TokenType::NUMBER(number));
    Ok(token)
}
