use super::ValueType;
use crate::{token::Token, token_type::TokenType};

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Token),
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Variable(String, Box<Expression>),
    Grouping(Box<Expression>),
    Function(Token, Vec<Expression>),
    Scope(Token, Vec<Expression>),
    IfElse {
        condition: Box<Expression>,
        then: Box<Expression>,
        else_expr: Option<Box<Expression>>,
    },
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
            Function(name, args) => {
                format!(
                    "function {}:{}",
                    name.to_string(),
                    args.iter()
                        .map(|s| s.to_string())
                        .reduce(|cur: String, nxt: String| format!("{}, {}", cur, &nxt))
                        .unwrap()
                )
            }
            Scope(name, exprs) => {
                format!(
                    "scope {}:{}",
                    name.to_string(),
                    exprs
                        .iter()
                        .map(|s| s.to_string())
                        .reduce(|cur: String, nxt: String| cur + &nxt)
                        .unwrap()
                )
            }
            Variable(tok, expr) => {
                format!("variable declaration {:?}:{}", tok, expr.to_string())

                // format!(
                //     "variable {} declared:{} ",
                //     name.iter()
                //         .map(|v| v.to_string())
                //         .collect::<Vec<String>>()
                //         .join(","),
                //     expr.to_string()
                // )
            }
            IfElse {
                condition,
                then,
                else_expr,
            } => {
                let else_expr = match else_expr {
                    Some(expr) => format!("else {}", expr.to_string()),
                    None => "".to_string(),
                };
                format!(
                    "if {} then {} {}",
                    condition.to_string(),
                    then.to_string(),
                    else_expr
                )
            }
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
            IDENTIFIER(_) => ValueType::STRING,
            _ => panic!("Invalid value type"),
        }
    }
}
