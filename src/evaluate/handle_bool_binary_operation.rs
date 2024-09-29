use super::EvaluatedExpression;
use crate::{sub_tokens::*, token::Token, token_type::TokenType};

pub fn handle_bool_binary_operation(
    token: &Token,
    left: &EvaluatedExpression,
    right: &EvaluatedExpression,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    if right.value_type == left.value_type {
        let result = match token.token_type {
            EQUAL(EqualType::EQUAL_EQUAL) => (left.value == right.value).into(),
            BANG(BangType::BANG_EQUAL) => (left.value != right.value).into(),
            _ => return Err("Invalid binary operator for bool".to_string()),
        };
        Ok(result)
    } else if token.token_type == EQUAL(EqualType::EQUAL_EQUAL)
        || token.token_type == BANG(BangType::BANG_EQUAL)
    {
        Ok(false.into())
    } else {
        Err("Invalid binary operator for bool".to_string())
    }
}
