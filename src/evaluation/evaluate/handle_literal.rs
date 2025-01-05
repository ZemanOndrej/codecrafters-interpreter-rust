use super::{ContextRef, EvaluatedExpressionResult};
use crate::{
    evaluation::{EvaluatedExpression, ValueType},
    token_type::TokenType,
};

pub fn handle_literal(
    context: &mut ContextRef,
    t: &crate::token::Token,
) -> Result<EvaluatedExpressionResult, String> {
    use TokenType::*;

    match &t.token_type {
        TRUE | FALSE | NIL => Ok(EvaluatedExpression {
            value_type: t.token_type.clone().into(),
        }
        .into()),

        NUMBER(_) => {
            let value = t.token_type.get_value().parse::<f64>().unwrap();
            Ok(EvaluatedExpression {
                value_type: ValueType::NUMBER(value).into(),
            }
            .into())
        }
        IDENTIFIER(identifier) => {
            if let Some(value) = context.borrow().get_declaration(identifier) {
                match value.value_type {
                    ValueType::FUNCTION {
                        name,
                        params,
                        body,
                        context,
                    } => Ok(EvaluatedExpression {
                        value_type: ValueType::FUNCTION {
                            name: name.to_string(),
                            params: params.iter().map(|a| a.to_string()).collect(),
                            body: body.clone(),
                            context: context.clone(),
                        },
                    }
                    .into()),
                    value => Ok(EvaluatedExpression { value_type: value }.into()),
                }
            } else {
                Err(format!(
                    "Undefined variable '{}'.\n[line {}]",
                    identifier, t.line_index
                ))
            }
        }
        t => Ok(EvaluatedExpression {
            value_type: t.clone().into(),
        }
        .into()),
    }
}
