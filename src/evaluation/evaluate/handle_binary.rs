mod handle_bool_binary_operation;
mod handle_number_binary_operation;
mod handle_string_binary_operation;
mod handle_variable_assignment;

use super::{ContextRef, EvaluatedExpressionResult, Expression};
use crate::{evaluation::ValueType, sub_tokens::EqualType, Token, TokenType};
use handle_bool_binary_operation::*;
use handle_number_binary_operation::*;
use handle_string_binary_operation::*;
use handle_variable_assignment::*;

pub fn handle_binary(
    context: &mut ContextRef,
    expression: &Box<Expression>,
    token: &Token,
    expression1: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    // dbg!(expression, token, expression1);
    use TokenType::*;

    let right = expression1.evaluate(context)?.assert_value()?;
    if token.token_type == EQUAL(EqualType::EQUAL) {
        if let Some(value) = handle_variable_assignment(context, expression, &right) {
            return value;
        }
    }
    // dbg!(token, context.borrow());
    let left = expression.evaluate(context)?.assert_value()?;
    if token.token_type.is_bool_logic_operator() {
        return handle_bool_binary_operation(token, &left, &right).map(|v| v.into());
    }

    match left.value_type {
        ValueType::STRING(value) => {
            handle_string_binary_operation(token, value, right).map(|v| v.into())
        }
        ValueType::NUMBER(value) => {
            handle_number_binary_operation(right, token, value).map(|v| v.into())
        }
        ValueType::BOOL(_) => handle_bool_binary_operation(token, &left, &right).map(|v| v.into()),

        e => panic!("Invalid binary operator {:?}", e),
    }
}
