use crate::token_type::TokenType;

#[derive(Debug, Clone, Default)]
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

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.token_type.to_string(),
            self.token_type.get_lexeme(),
            self.value
        )
    }
}
