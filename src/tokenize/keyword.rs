use crate::{token::Token, token_type::TokenType};

pub fn handle_keyword(token: Token) -> Token {
    use TokenType::*;
    let value = match &token.token_type {
        IDENTIFIER(v) => v,
        _ => return token,
    };
    match value.as_str() {
        "and" => Token::new(AND),
        "class" => Token::new(CLASS),
        "else" => Token::new(ELSE),
        "false" => Token::new(FALSE),
        "fun" => Token::new(FUN),
        "for" => Token::new(FOR),
        "if" => Token::new(IF),
        "nil" => Token::new(NIL),
        "or" => Token::new(OR),
        "print" => Token::new(PRINT),
        "return" => Token::new(RETURN),
        "super" => Token::new(SUPER),
        "this" => Token::new(THIS),
        "true" => Token::new(TRUE),
        "var" => Token::new(VAR),
        "while" => Token::new(WHILE),
        _ => token,
    }
}
