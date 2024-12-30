use super::ValueType;
use crate::{token::Token, token_type::TokenType};
mod create_for;

type Parameter = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Token),
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Variable(String, Box<Expression>),
    Grouping(Box<Expression>),
    FunctionCall(Token, Vec<Expression>),
    FunctionDeclaration {
        name: Token,
        args: Vec<Parameter>,
        body: Box<Expression>,
    },
    Scope(Token, Vec<Expression>),
    IfElse {
        condition: Box<Expression>,
        then: Box<Expression>,
        else_expr: Option<Box<Expression>>,
    },
    While {
        condition: Box<Expression>,
        then: Box<Expression>,
    },
    For {
        declaration: Box<Expression>,
        condition: Box<Expression>,
        increment: Box<Expression>,
        then: Box<Expression>,
    },
    Return(Box<Expression>),
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
            FunctionCall(name, args) => {
                format!(
                    "function {}:{}",
                    name.to_string(),
                    args.iter()
                        .map(|s| s.to_string())
                        .reduce(|cur: String, nxt: String| format!("{}, {}", cur, &nxt))
                        .unwrap()
                )
            }
            FunctionDeclaration {
                name,
                args: _,
                body: _,
            } => {
                format!("<fn {}>", name.token_type.get_lexeme(),)
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
            While { condition, then } => {
                format!("while {} then {}", condition.to_string(), then.to_string())
            }
            For {
                declaration,
                condition,
                increment,
                then,
            } => format!(
                "for {} {} {:?} then {}",
                declaration.to_string(),
                condition.to_string(),
                increment,
                then.to_string()
            ),
            Return(expr) => format!("return {}", expr.to_string()),
        }
    }
}

impl Into<ValueType> for TokenType {
    fn into(self) -> ValueType {
        use TokenType::*;
        match self {
            NUMBER(v) => ValueType::NUMBER(v.parse().unwrap()),
            STRING(v) => ValueType::STRING(v),
            TRUE => ValueType::BOOL(true),
            FALSE => ValueType::BOOL(false),
            NIL => ValueType::NIL,
            IDENTIFIER(v) => ValueType::STRING(v),
            _ => panic!("Invalid value type"),
        }
    }
}
impl Expression {
    pub fn nil() -> Expression {
        Expression::Literal(Token::nil())
    }
}
