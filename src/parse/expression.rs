use crate::{token::Token, token_type::TokenType};

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

impl Expression {
    pub fn evaluate(&self) -> String {
        use Expression::*;
        use TokenType::*;
        match self {
            Literal(t) => match &t.token_type {
                TRUE | FALSE | NIL => t.token_type.get_lexeme(),
                NUMBER(_) => {
                    let value = t.token_type.get_value();
                    let value = value.trim_end_matches("0");
                    let value = value.trim_end_matches(".");
                    value.to_string()
                }
                t => t.get_value(),
            },
            Binary(expression, token, expression1) => todo!(),
            Unary(token, expression) => todo!(),
            Grouping(expression) => expression.evaluate(),
        }
    }
}
