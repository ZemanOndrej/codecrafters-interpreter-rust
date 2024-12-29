use crate::{
    evaluation::{EvaluatedExpression, ValueType},
    token_type::TokenType,
};
use super::{ContextRef, EvaluatedExpressionResult};

pub fn handle_literal(
    context: &mut ContextRef,
    t: &crate::token::Token,
) -> Result<EvaluatedExpressionResult, String> {
    use TokenType::*;

    match &t.token_type {
        TRUE | FALSE | NIL => Ok(EvaluatedExpression {
            value: t.token_type.get_lexeme(),
            value_type: t.token_type.clone().into(),
        }
        .into()),
        NUMBER(_) => {
            let value = t.token_type.get_value();
            let value = value.trim_end_matches("0");
            let value = value.trim_end_matches(".");
            Ok(EvaluatedExpression {
                value: value.to_string(),
                value_type: t.token_type.clone().into(),
            }
            .into())
        }
        IDENTIFIER(identifier) => {
            if let Some(value) = context.borrow().get_variable(identifier) {
                return Ok(value.clone().into());
            }
            if let Some(function) = context.borrow_mut().get_function(identifier) {
                return Ok(EvaluatedExpression {
                    value: function.to_string(),
                    value_type: ValueType::STRING,
                }
                .into());
            } else {
                return Err(format!(
                    "Undefined variable '{}'.\n[line {}]",
                    identifier, t.line_index
                ));
            }
        }
        t => Ok(EvaluatedExpression {
            value: t.get_value(),
            value_type: t.clone().into(),
        }
        .into()),
    }
}
