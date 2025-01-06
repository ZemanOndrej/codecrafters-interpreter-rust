use crate::{token::Token, token_type::TokenType};
use std::{iter::Peekable, str::Chars};

pub fn handle_number_literal(
    number: String,
    chars: &mut Peekable<Chars<'_>>,
    input: &mut String,
    line_index: usize,
) -> Result<Token, String> {
    let mut number = number.to_string();
    loop {
        let next = chars.peek();
        let Some(char) = next else {
            input.clear();
            break;
        };
        if char.is_ascii_digit() || char == &'.' {
            number.push(chars.next().unwrap());
        } else {
            input.clear();
            break;
        }
    }

    let Ok(_) = number.parse::<f64>() else {
        return Err("Invalid number.".to_string());
    };

    let token = Token::new(TokenType::NUMBER(number), line_index);
    Ok(token)
}
