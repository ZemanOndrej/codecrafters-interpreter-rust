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
    FunctionCallLambda(Box<Expression>, Vec<Expression>),
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
impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        use TokenType::*;
        let value = match self {
            Literal(v) => match &v.token_type {
                TRUE | FALSE | NIL => v.token_type.get_lexeme(),
                NUMBER(_) => v.token_type.get_value(),
                t => t.get_value(),
            },
            Binary(left, op, right) => {
                format!("({} {} {})", op.token_type.get_lexeme(), left, right)
            }
            Unary(op, right) => {
                format!("({} {})", op.token_type.get_lexeme(), right)
            }
            Grouping(expr) => format!("(group {})", expr),
            FunctionCall(name, args) => {
                format!(
                    "function {}:{}",
                    name,
                    args.iter()
                        .map(|s| s.to_string())
                        .reduce(|cur: String, nxt: String| format!("{}, {}", cur, &nxt))
                        .unwrap()
                )
            }
            FunctionCallLambda(expr, args) => {
                format!(
                    "function lambda {}:{}",
                    expr,
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
                    name,
                    exprs
                        .iter()
                        .map(|s| s.to_string())
                        .reduce(|cur: String, nxt: String| cur + &nxt)
                        .unwrap()
                )
            }
            Variable(tok, expr) => {
                format!("variable declaration {:?}:{}", tok, expr)
            }
            IfElse {
                condition,
                then,
                else_expr,
            } => {
                let else_expr = match else_expr {
                    Some(expr) => format!("else {}", expr),
                    None => "".to_string(),
                };
                format!("if {} then {} {}", condition, then, else_expr)
            }
            While { condition, then } => {
                format!("while {} then {}", condition, then)
            }
            For {
                declaration,
                condition,
                increment,
                then,
            } => format!(
                "for {} {} {:?} then {}",
                declaration, condition, increment, then
            ),
            Return(expr) => format!("return {}", expr),
        };
        write!(f, "{}", value)
    }
}

impl From<TokenType> for ValueType {
    fn from(val: TokenType) -> Self {
        use TokenType::*;
        match val {
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
