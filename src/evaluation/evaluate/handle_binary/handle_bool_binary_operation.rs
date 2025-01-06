use crate::{evaluation::EvaluatedExpression, sub_tokens::{BangType, EqualType}, token::Token, token_type::TokenType};

pub fn handle_bool_binary_operation(
    token: &Token,
    left: &EvaluatedExpression,
    right: &EvaluatedExpression,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    let left_bool = left.to_bool();
    let right_bool = right.to_bool();
    let result = match token.token_type {
        EQUAL(EqualType::EQUAL_EQUAL) => (left_bool == right_bool).into(),
        BANG(BangType::BANG_EQUAL) => (left_bool != right_bool).into(),
        OR => {
            if left_bool {
                left.clone()
            } else if right_bool {
                right.clone()
            } else {
                false.into()
            }
        }
        AND => {
            if left_bool && right_bool {
                right.clone()
            } else {
                false.into()
            }
        }
        _ => {
            return Err(format!(
                "Invalid binary operator for bool '{}'",
                token.token_type.get_lexeme()
            ))
        }
    };
    Ok(result)
}
