use crate::{sub_tokens::BangType, token::Token, token_type::TokenType};

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
            Binary(expression, token, expression1) => todo!(),
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
