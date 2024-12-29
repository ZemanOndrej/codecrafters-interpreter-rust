mod handle_bool_binary_operation;
mod handle_number_binary_operation;
mod handle_string_binary_operation;

use super::{ContextRef, EvaluatedExpressionResult, Expression};
use crate::{evaluation::ValueType, sub_tokens::EqualType, token_type::TokenType};

use handle_bool_binary_operation::*;
use handle_number_binary_operation::*;
use handle_string_binary_operation::*;

pub fn handle_binary(
    context: &mut ContextRef,
    expression: &Box<Expression>,
    token: &crate::token::Token,
    expression1: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    // dbg!(expression, token, expression1);
    use Expression::*;
    use TokenType::*;

    let right = expression1.evaluate(context)?.assert_value()?;
    if token.token_type == EQUAL(EqualType::EQUAL) {
        match &**expression {
            Literal(t) => {
                if let IDENTIFIER(identifier) = &t.token_type {
                    if !context.borrow().contains_variable(identifier) {
                        return Err(format!(
                            "Undefined variable '{}'.\n[line {}]",
                            identifier, t.line_index
                        ));
                    }
                    context
                        .borrow_mut()
                        .change_variable(identifier, right.clone());
                    return Ok(right.into());
                }
            }
            Binary(e1, t, e2) => {
                dbg!(e1, t, e2);
            }
            _ => (),
        }
    }
    let left = expression.evaluate(context)?.assert_value()?;
    if token.token_type.is_bool_logic_operator() {
        return handle_bool_binary_operation(token, &left, &right).map(|v| v.into());
    }

    match left.value_type {
        ValueType::STRING => handle_string_binary_operation(token, &left, &right).map(|v| v.into()),
        ValueType::NUMBER => handle_number_binary_operation(
            right,
            token,
            left.value
                .parse::<f64>()
                .map_err(|_| "Invalid number".to_string())?,
        )
        .map(|v| v.into()),
        ValueType::BOOL => handle_bool_binary_operation(token, &left, &right).map(|v| v.into()),

        e => panic!("Invalid binary operator {:?}", e),
    }
}
