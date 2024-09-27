use crate::{
    parse::ValueType,
    sub_tokens::{BangType, EqualType, GreaterType, LessType, SlashType},
    token::Token,
    token_type::TokenType,
};

use super::EvaluatedExpression;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Token),
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Grouping(Box<Expression>),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        use Expression::*;
        use TokenType::*;
        match self {
            Literal(v) => match &v.token_type {
                TRUE | FALSE | NIL => v.token_type.get_lexeme(),
                NUMBER(_) => v.token_type.get_value(),
                t => t.get_value(),
            },
            Binary(left, op, right) => format!(
                "({} {} {})",
                op.token_type.get_lexeme(),
                left.to_string(),
                right.to_string()
            ),
            Unary(op, right) => {
                format!("({} {})", op.token_type.get_lexeme(), right.to_string())
            }
            Grouping(expr) => format!("(group {})", expr.to_string()),
        }
    }
}

impl Into<ValueType> for TokenType {
    fn into(self) -> ValueType {
        use TokenType::*;
        match self {
            NUMBER(_) => ValueType::NUMBER,
            STRING(_) => ValueType::STRING,
            TRUE | FALSE => ValueType::BOOL,
            NIL => ValueType::NIL,
            _ => panic!("Invalid value type"),
        }
    }
}

impl Expression {
    pub fn evaluate(&self) -> Result<EvaluatedExpression, String> {
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
                t => Ok(EvaluatedExpression {
                    value: t.get_value(),
                    value_type: t.clone().into(),
                }),
            },
            Binary(expression, token, expression1) => {
                let left = expression.evaluate()?;
                let right = expression1.evaluate()?;
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
                let evalueated_expr = expression.evaluate()?;
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
            Grouping(expression) => expression.evaluate(),
        }
    }
}

fn handle_bool_binary_operation(
    token: &Token,
    left: &EvaluatedExpression,
    right: &EvaluatedExpression,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    if right.value_type == left.value_type {
        let result = match token.token_type {
            EQUAL(EqualType::EQUAL_EQUAL) => (left.value == right.value).into(),
            BANG(BangType::BANG_EQUAL) => (left.value != right.value).into(),
            _ => return Err("Invalid binary operator for bool".to_string()),
        };
        Ok(result)
    } else if token.token_type == EQUAL(EqualType::EQUAL_EQUAL)
        || token.token_type == BANG(BangType::BANG_EQUAL)
    {
        Ok(false.into())
    } else {
        Err("Invalid binary operator for bool".to_string())
    }
}

fn handle_string_binary_operation(
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

fn handle_number_binary_operation(
    right: EvaluatedExpression,
    token: &Token,
    left: f64,
) -> Result<EvaluatedExpression, String> {
    use TokenType::*;

    if right.value_type == ValueType::NUMBER {
        let right = right
            .value
            .parse::<f64>()
            .map_err(|_| "Invalid number".to_string())?;
        let result = match token.token_type {
            PLUS => (left + right).into(),
            MINUS => (left - right).into(),
            STAR => (left * right).into(),
            SLASH(SlashType::SLASH) => (left / right).into(),
            GREATER(GreaterType::GREATER) => (left > right).into(),
            GREATER(GreaterType::GREATER_EQUAL) => (left >= right).into(),
            LESS(LessType::LESS) => (left < right).into(),
            LESS(LessType::LESS_EQUAL) => (left <= right).into(),
            EQUAL(EqualType::EQUAL_EQUAL) => (left == right).into(),
            BANG(BangType::BANG_EQUAL) => (left != right).into(),
            _ => return Err("Invalid binary operator".to_string()),
        };

        Ok(result)
    } else if token.token_type == EQUAL(EqualType::EQUAL_EQUAL)
        || token.token_type == BANG(BangType::BANG_EQUAL)
    {
        Ok(false.into())
    } else {
        Err("Invalid binary operator for number".to_string())
    }
}
