use crate::{
    evaluation::{EvaluatedExpression, ValueType},
    sub_tokens::{BangType, EqualType},
    token::Token,
    token_type::TokenType,
};

pub fn handle_string_binary_operation(
    token: &Token,
    left: String,
    right: EvaluatedExpression,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    match right.value_type {
        ValueType::STRING(value) => {
            let result = match token.token_type {
                PLUS => format!("{left}{value}").into(),
                EQUAL(EqualType::EQUAL_EQUAL) => (left == value).into(),
                BANG(BangType::BANG_EQUAL) => (left != value).into(),
                _ => return Err("Invalid binary operator for string".to_string()),
            };
            Ok(result)
        }
        _ => {
            if token.token_type == EQUAL(EqualType::EQUAL_EQUAL)
                || token.token_type == BANG(BangType::BANG_EQUAL)
            {
                Ok(false.into())
            } else {
                Err("Invalid binary operator for string".to_string())
            }
        }
    }
}
