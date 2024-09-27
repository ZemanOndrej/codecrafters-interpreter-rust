use crate::{
    sub_tokens::{BangType, EqualType, GreaterType, LessType, SlashType},
    token::Token,
    token_type::TokenType,
};

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

pub struct EvaluatedExpression {
    pub value: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    NUMBER,
    STRING,
    BOOL,
    NIL,
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
                            return EvaluatedExpression {
                                value: format!("{}{}", left.value, right.value),
                                value_type: ValueType::STRING,
                            };
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
                        EvaluatedExpression {
                            value: (-right).to_string(),
                            value_type: ValueType::NUMBER,
                        }
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
                        EvaluatedExpression {
                            value: (!bool_value).to_string(),
                            value_type: ValueType::BOOL,
                        }
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
        PLUS => left + right,
        MINUS => left - right,
        STAR => left * right,
        SLASH(SlashType::SLASH) => left / right,
        GREATER(GreaterType::GREATER) => (left > right) as i32 as f64,
        GREATER(GreaterType::GREATER_EQUAL) => (left >= right) as i32 as f64,
        LESS(LessType::LESS) => (left < right) as i32 as f64,
        LESS(LessType::LESS_EQUAL) => (left <= right) as i32 as f64,
        EQUAL(EqualType::EQUAL_EQUAL) => (left == right) as i32 as f64,
        BANG(BangType::BANG_EQUAL) => (left != right) as i32 as f64,
        _ => panic!("Invalid binary operator"),
    };
    EvaluatedExpression {
        value: result.to_string(),
        value_type: ValueType::NUMBER,
    }
}
