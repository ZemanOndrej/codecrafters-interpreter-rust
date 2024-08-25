use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(input: &str) -> anyhow::Result<Token> {
        let token_type = TokenType::parse(input)?;
        let value = token_type.get_value();

        Ok(Token { token_type, value })
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