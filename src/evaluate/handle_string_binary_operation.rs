use crate::{sub_tokens::*, token::Token, token_type::TokenType};

use super::EvaluatedExpression;

pub fn handle_string_binary_operation(
    token: &Token,
    left: &EvaluatedExpression,
    right: &EvaluatedExpression,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    if left.value_type == right.value_type {
        let result = match token.token_type {
            PLUS => format!("{}{}", left.value, right.value).into(),
            EQUAL(EqualType::EQUAL_EQUAL) => (left.value == right.value).into(),
            BANG(BangType::BANG_EQUAL) => (left.value != right.value).into(),

            _ => return Err("Invalid binary operator for string".to_string()),
        };
        Ok(result)
    } else if token.token_type == EQUAL(EqualType::EQUAL_EQUAL)
        || token.token_type == BANG(BangType::BANG_EQUAL)
    {
        Ok(false.into())
    } else {
        Err("Invalid binary operator for string".to_string())
    }
}
