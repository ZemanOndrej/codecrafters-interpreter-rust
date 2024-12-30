use crate::{
    evaluation::{EvaluatedExpression, ValueType},
    sub_tokens::*,
    token::Token,
    token_type::TokenType,
};

pub fn handle_number_binary_operation(
    right: EvaluatedExpression,
    token: &Token,
    left: f64,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    if let ValueType::NUMBER(right) = right.value_type {
        let result = match token.token_type {
            PLUS => (left + right).into(),
            MINUS => (left - right).into(),
            STAR => (left * right).into(),
            SLASH(SlashType::SLASH) => (left / right).into(),
            GREATER(GreaterType::GREATER) => (left > right).into(),
            GREATER(GreaterType::GREATER_EQUAL) => (left >= right).into(),
            LESS(LessType::LESS) => (left < right).into(),
            LESS(LessType::LESS_EQUAL) => (left <= right).into(),
            EQUAL(EqualType::EQUAL_EQUAL) => (left == right).into(),
            BANG(BangType::BANG_EQUAL) => (left != right).into(),
            _ => return Err("Invalid binary operator".to_string()),
        };

        Ok(result)
    } else if token.token_type == EQUAL(EqualType::EQUAL_EQUAL)
        || token.token_type == BANG(BangType::BANG_EQUAL)
    {
        Ok(false.into())
    } else {
        Err("Invalid binary operator for number".to_string())
    }
}
