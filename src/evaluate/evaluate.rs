use crate::{
    builtin_fns,
    evaluate::{
        handle_bool_binary_operation, handle_number_binary_operation,
        handle_string_binary_operation, ValueType,
    },
    handlers::Context,
    sub_tokens::*,
    token_type::TokenType,
};

use super::{EvaluatedExpression, Expression};

impl Expression {
    pub fn evaluate(&self, context: &mut Context) -> Result<EvaluatedExpression, String> {
        use Expression::*;
        use TokenType::*;

        match self {
            Literal(t) => match &t.token_type {
                TRUE | FALSE | NIL => Ok(EvaluatedExpression {
                    value: t.token_type.get_lexeme(),
                    value_type: t.token_type.clone().into(),
                }),
                NUMBER(_) => {
                    let value = t.token_type.get_value();
                    let value = value.trim_end_matches("0");
                    let value = value.trim_end_matches(".");
                    Ok(EvaluatedExpression {
                        value: value.to_string(),
                        value_type: t.token_type.clone().into(),
                    })
                }
                IDENTIFIER(identifier) => {
                    let Some(value) = context.variables.get(identifier) else {
                        return Err(format!(
                            "Undefined variable '{}'.\n[line {}]",
                            identifier, t.line_index
                        ));
                    };

                    Ok(value.clone())
                }
                t => Ok(EvaluatedExpression {
                    value: t.get_value(),
                    value_type: t.clone().into(),
                }),
            },
            Binary(expression, token, expression1) => {
                let left = expression.evaluate(context)?;
                let right = expression1.evaluate(context)?;
                match left.value_type {
                    ValueType::STRING => handle_string_binary_operation(token, &left, &right),
                    ValueType::NUMBER => handle_number_binary_operation(
                        right,
                        token,
                        left.value
                            .parse::<f64>()
                            .map_err(|_| "Invalid number".to_string())?,
                    ),
                    ValueType::BOOL => handle_bool_binary_operation(token, &left, &right),

                    _ => panic!("Invalid binary operator"),
                }
            }
            Unary(token, expression) => {
                let evalueated_expr = expression.evaluate(context)?;
                match token.token_type {
                    MINUS => {
                        let right = evalueated_expr
                            .value
                            .parse::<f64>()
                            .map_err(|_| "Invalid number".to_string())?;
                        Ok((-right).into())
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
                        Ok((!bool_value).into())
                    }
                    _ => panic!("Invalid unary operator"),
                }
            }
            Variable(name, _, expr) => {
                // let value = expr.evaluate(context)?;
                let value = expr.evaluate(context)?;
                context
                    .variables
                    .insert(name.token_type.get_lexeme(), value);
                Ok(EvaluatedExpression {
                    value: "".to_string(),
                    value_type: ValueType::NIL,
                })
            }
            Grouping(expression) => expression.evaluate(context),
            Function(_, args) => builtin_fns::print(args, context),
            Scope(_, _) => {
                todo!()
            }
        }
    }
}
