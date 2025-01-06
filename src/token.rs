use crate::token_type::TokenType;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line_index: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line_index: usize) -> Token {
        Token {
            value: token_type.get_value(),
            token_type,
            line_index,
        }
    }
    pub fn nil() -> Token {
        Token {
            value: "nil".to_string(),
            token_type: TokenType::NIL,
            line_index: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.token_type.get_lexeme(),
            self.value
        )
    }
}
