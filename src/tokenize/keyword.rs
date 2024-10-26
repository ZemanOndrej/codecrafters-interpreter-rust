use crate::{token::Token, token_type::TokenType};

pub fn handle_keyword(token: Token, line_index: usize) -> Token {
    use TokenType::*;
    let value = match &token.token_type {
        IDENTIFIER(v) => v,
        _ => return token,
    };
    match value.as_str() {
        "and" => Token::new(AND, line_index),
        "class" => Token::new(CLASS, line_index),
        "else" => Token::new(ELSE, line_index),
        "false" => Token::new(FALSE, line_index),
        "fun" => Token::new(FUN, line_index),
        "for" => Token::new(FOR, line_index),
        "if" => Token::new(IF, line_index),
        "nil" => Token::new(NIL, line_index),
        "or" => Token::new(OR, line_index),
        "print" => Token::new(PRINT, line_index),
        "return" => Token::new(RETURN, line_index),
        "super" => Token::new(SUPER, line_index),
        "this" => Token::new(THIS, line_index),
        "true" => Token::new(TRUE, line_index),
        "var" => Token::new(VAR, line_index),
        "while" => Token::new(WHILE, line_index),
        _ => token,
    }
}
