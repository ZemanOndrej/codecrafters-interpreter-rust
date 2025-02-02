use crate::{
    evaluation::{EvaluatedExpression, ValueType},
    sub_tokens::BangType,
    token_type::TokenType,
};

use super::{Context, EvaluatedExpressionResult, Expression};

pub fn handle_unary(
    context: &mut std::rc::Rc<std::cell::RefCell<Context>>,
    token: &crate::token::Token,
    expression: &Expression,
) -> Result<EvaluatedExpressionResult, String> {
    use TokenType::*;
    let evalueated_expr = expression.evaluate(context)?.assert_value()?;

    match token.token_type {
        MINUS => {
            let ValueType::NUMBER(right) = evalueated_expr.value_type else {
                return Err("Invalid number".to_string());
            };

            let expr: EvaluatedExpression = (-right).into();
            Ok(expr.into())
        }
        BANG(BangType::BANG) => {
            let bool_value = if evalueated_expr.value_type == ValueType::NIL {
                false
            } else if let ValueType::BOOL(value) = evalueated_expr.value_type {
                value
            } else if let ValueType::NUMBER(number) = evalueated_expr.value_type {
                number != 0.0
            } else {
                true
            };
            let expr: EvaluatedExpression = (!bool_value).into();
            Ok(expr.into())
        }
        _ => panic!("Invalid unary operator"),
    }
}
