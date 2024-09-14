use crate::{sub_tokens::SlashType, token::Token, token_type::TokenType};

pub fn parse(results: Vec<Token>) {
    let mut stack = Vec::new();

    let mut iterator = results.iter();
    while let Some(token) = iterator.next() {
        use TokenType::*;
        // dbg!(&token);
        match &token.token_type {
            FALSE | TRUE | NUMBER(_) | NIL | STRING(_) => {
                stack.push(Expression::Literal(token.clone()))
            }

            PLUS | MINUS | STAR | SLASH(SlashType::SLASH) => {
                let left = stack.pop().unwrap();
                let right = iterator.next().unwrap();

                let right = match right.token_type {
                    NUMBER(_) => Expression::Literal(right.clone()),
                    _ => panic!("Invalid token type"),
                };
                stack.push(Expression::Binary(
                    Box::new(left),
                    token.clone(),
                    Box::new(right),
                ));
            }

            LEFT_PAREN => {
                stack.push(Expression::Grouping(Box::new(Expression::Literal(
                    token.clone(),
                ))));
            }
            RIGHT_PAREN => {
                let right = stack.pop().unwrap();
                stack.pop().unwrap();
                stack.push(Expression::Grouping(right.into()));
            }
            EOF => {
                break;
            }
            _ => {
                panic!("Invalid token type");
            }
        }
    }
    for i in stack.iter() {
        println!("{}", i.to_string());
    }
}

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
