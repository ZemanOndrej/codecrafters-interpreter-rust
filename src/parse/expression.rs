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
    pub fn evaluate(&self) -> EvaluatedExpression {
        use Expression::*;
        use TokenType::*;
        match self {
            Literal(t) => match &t.token_type {
                TRUE | FALSE | NIL => EvaluatedExpression {
                    value: t.token_type.get_lexeme(),
                    value_type: t.token_type.clone().into(),
                },
                NUMBER(_) => {
                    let value = t.token_type.get_value();
                    let value = value.trim_end_matches("0");
                    let value = value.trim_end_matches(".");
                    EvaluatedExpression {
                        value: value.to_string(),
                        value_type: t.token_type.clone().into(),
                    }
                }
                t => EvaluatedExpression {
                    value: t.get_value(),
                    value_type: t.clone().into(),
                },
            },
            Binary(expression, token, expression1) => {
                let left = expression.evaluate();
                let right = expression1.evaluate();
                match left.value_type {
                    ValueType::STRING => match token.token_type {
                        PLUS => {
                            return format!("{}{}", left.value, right.value).into();
                        }
                        _ => panic!("Invalid binary operator for string"),
                    },
                    ValueType::NUMBER => {
                        handle_number_binary_operation(right, token, left.value.parse::<f64>().unwrap())
                    }

                    _ => panic!("Invalid binary operator"),
                }
            }
            Unary(token, expression) => {
                let evalueated_expr = expression.evaluate();
                match token.token_type {
                    MINUS => {
                        let right = evalueated_expr.value.parse::<f64>().unwrap();
                        (-right).into()
                    }
                    BANG(BangType::BANG) => {
                        let bool_value = if evalueated_expr.value_type == ValueType::NIL {
                            false
                        } else if evalueated_expr.value_type == ValueType::BOOL {
                            evalueated_expr.value.parse::<bool>().unwrap()
                        } else if let Ok(number) = evalueated_expr.value.parse::<f64>() {
                            if number != 0.0 {
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        };
                        (!bool_value).into()
                    }
                    _ => panic!("Invalid unary operator"),
                }
            }
            Grouping(expression) => expression.evaluate(),
        }
    }
}

fn handle_number_binary_operation(right: EvaluatedExpression, token: &Token, left: f64) -> EvaluatedExpression {
    use TokenType::*;
    let right = right.value.parse::<f64>().unwrap();
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
        _ => panic!("Invalid binary operator"),
    };

    result
}
