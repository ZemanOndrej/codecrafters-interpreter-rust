use crate::{
    builtin_fns,
    evaluate::{
        handle_bool_binary_operation, handle_number_binary_operation,
        handle_string_binary_operation, ValueType,
    },
    sub_tokens::*,
    token_type::TokenType,
};

use super::{Context, ContextRef, EvaluatedExpression, Expression};

impl Expression {
    pub fn evaluate(&self, context: &mut ContextRef) -> Result<EvaluatedExpression, String> {
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
                    let Some(value) = context.borrow().get_variable(identifier) else {
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
                // dbg!(expression, token, expression1);
                let right = expression1.evaluate(context)?;
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
                                return Ok(right);
                            }
                        }
                        Binary(e1, t, e2) => {
                            dbg!(e1, t, e2);
                        }
                        _ => (),
                    }
                }
                let left = expression.evaluate(context)?;
                if token.token_type.is_bool_logic_operator() {
                    return handle_bool_binary_operation(token, &left, &right);
                }

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

                    e => panic!("Invalid binary operator {:?}", e),
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
            Variable(name, expr) => {
                let value = expr.evaluate(context)?;
                context.borrow_mut().set_variable(name.clone(), value);
                Ok(EvaluatedExpression {
                    value: "".to_string(),
                    value_type: ValueType::NIL,
                })
            }
            Grouping(expression) => expression.evaluate(context),
            Function(t, args) => {
                let builtin_fns = builtin_fns::get_builtin_fns();
                let fn_name = t.token_type.get_lexeme();
                if let Some(builtin_fn) = builtin_fns.get(fn_name.as_str()) {
                    return (builtin_fn.function)(args, context);
                }

                match t.token_type {
                    PRINT => builtin_fns::print(args, context),
                    _ => Err(format!(
                        "Undefined function '{}'",
                        t.token_type.get_lexeme()
                    )),
                }
            }
            Scope(_, exprs) => {
                let mut child_context = Context::new(context.clone());
                for expr in exprs {
                    expr.evaluate(&mut child_context)?;
                }
                Ok(EvaluatedExpression {
                    value: "".to_string(),
                    value_type: ValueType::NIL,
                })
            }
            IfElse {
                condition,
                then,
                else_expr,
            } => {
                let condition = condition.evaluate(context)?;

                if condition.to_bool() {
                    then.evaluate(context)
                } else {
                    if let Some(expr) = else_expr {
                        expr.evaluate(context)
                    } else {
                        Ok(EvaluatedExpression {
                            value: "".to_string(),
                            value_type: ValueType::NIL,
                        })
                    }
                }
            }
            For {
                declaration,
                condition,
                increment,
                then,
            } => {
                declaration.evaluate(context)?;
                loop {
                    let condition = condition.evaluate(context)?;
                    if !condition.to_bool() {
                        break;
                    }
                    then.evaluate(context)?;
                    increment.evaluate(context)?;
                }
                Ok(EvaluatedExpression {
                    value: "".to_string(),
                    value_type: ValueType::NIL,
                })
            }
            While { condition, then } => {
                loop {
                    if !condition.evaluate(context)?.to_bool() {
                        break;
                    }
                    then.evaluate(context)?;
                }
                Ok(EvaluatedExpression {
                    value: "".to_string(),
                    value_type: ValueType::NIL,
                })
            }
        }
    }
}
