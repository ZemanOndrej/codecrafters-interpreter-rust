use crate::{
    evaluation::{EvaluatedExpression, ValueType},
    sub_tokens::BangType,
    token_type::TokenType,
};

use super::{Context, EvaluatedExpressionResult, Expression};

pub fn handle_unary(
    context: &mut std::rc::Rc<std::cell::RefCell<Context>>,
    token: &crate::token::Token,
    expression: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    use TokenType::*;
    let evalueated_expr = expression.evaluate(context)?.assert_value()?;

    match token.token_type {
        MINUS => {
            let right = evalueated_expr
                .value
                .parse::<f64>()
                .map_err(|_| "Invalid number".to_string())?;
            let expr: EvaluatedExpression = (-right).into();
            Ok(expr.into())
        }
        BANG(BangType::BANG) => {
            let bool_value = if evalueated_expr.value_type == ValueType::NIL {
                false
            } else if evalueated_expr.value_type == ValueType::BOOL {
                evalueated_expr
                    .value
                    .parse::<bool>()
                    .map_err(|_| "Invalid number".to_string())?
            } else if let Ok(number) = evalueated_expr.value.parse::<f64>() {
                if number != 0.0 {
                    true
                } else {
                    false
                }
            } else {
                true
            };
            let expr: EvaluatedExpression = (!bool_value).into();
            Ok(expr.into())
        }
        _ => panic!("Invalid unary operator"),
    }
}
