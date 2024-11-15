use std::{iter::Peekable, slice::Iter};

use crate::{
    evaluate::Expression,
    parse::{create_error, parse_expression, process_precedence::parse_precedence},
    sub_tokens::*,
    token::Token,
    token_type::TokenType,
};
pub type InputIter<'a> = Peekable<Iter<'a, Token>>;

pub fn parse_token(
    token: &Token,
    input: &mut InputIter,
    stack: &mut Vec<Expression>,
) -> Result<Option<Expression>, String> {
    use TokenType::*;
    let expr = match &token.token_type {
        BANG(BangType::BANG) => {
            let right = parse_token(input.next().unwrap(), input, stack)?.unwrap();

            Expression::Unary(token.clone(), Box::new(right)).into()
        }
        FALSE | TRUE | NUMBER(_) | NIL | STRING(_) => Expression::Literal(token.clone()).into(),
        MINUS => {
            let left = stack.pop();
            let value = match left {
                Some(left) => {
                    let right = parse_precedence(token, input, stack)?.unwrap();
                    Expression::Binary(Box::new(left), token.clone(), Box::new(right))
                }
                None => {
                    let right = parse_token(input.next().unwrap(), input, stack)?.unwrap();
                    Expression::Unary(token.clone(), Box::new(right))
                }
            };
            value.into()
        }

        PLUS
        | GREATER(GreaterType::GREATER)
        | SLASH(SlashType::SLASH)
        | LESS(LessType::LESS)
        | LESS(LessType::LESS_EQUAL)
        | BANG(BangType::BANG_EQUAL)
        | GREATER(GreaterType::GREATER_EQUAL)
        | EQUAL(EqualType::EQUAL)
        | EQUAL(EqualType::EQUAL_EQUAL) => {
            let left = stack.pop().ok_or_else(|| create_error(token))?;
            let right = parse_precedence(token, input, stack)?.unwrap();

            Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into()
        }
        STAR => {
            let left = stack.pop().ok_or_else(|| create_error(token))?;
            let right = parse_token(input.next().unwrap(), input, stack)?.unwrap();
            Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into()
        }

        LEFT_PAREN => {
            let (inner, _) = parse_expression(input, token, &[RIGHT_PAREN])?;
            let r = Expression::Grouping(Box::new(inner)).into();
            r
        }
        RIGHT_PAREN => {
            let right = stack.pop().ok_or_else(|| create_error(token))?;

            stack.pop().unwrap();
            Expression::Grouping(right.into()).into()
        }
        IDENTIFIER(_) => {
            let value = Expression::Literal(token.clone()).into();
            value
        }
        PRINT => {
            let mut arguments = Vec::new();
            loop {
                let (arg, next) = parse_expression(input, token, &[COMMA, SEMICOLON])?;
                arguments.push(arg);

                if next.token_type == SEMICOLON {
                    break;
                }
            }

            Expression::Function(token.clone(), arguments).into()
        }
        SEMICOLON => None,
        LEFT_BRACE => {
            let mut arguments = Vec::new();
            loop {
                let (arg, next) = parse_expression(input, token, &[RIGHT_BRACE])?;
                arguments.push(arg);

                if next.token_type == RIGHT_BRACE {
                    break;
                }
            }

            Expression::Function(token.clone(), arguments).into()
        }

        VAR => {
            let name = input.next().unwrap();

            let next_token = input.peek().unwrap();
            if matches!(next_token.token_type, EQUAL(EqualType::EQUAL)) {
                input.next().unwrap();
                let (expr, _) = parse_expression(input, token, &[SEMICOLON])?;
                Expression::Variable(name.clone(), token.clone(), Box::new(expr)).into()
            } else {
                Expression::Variable(
                    name.clone(),
                    token.clone(),
                    Box::new(Expression::Literal(Token::new(
                        TokenType::NIL,
                        token.line_index,
                    )))
                    .into(),
                )
                .into()
            }
        }

        EOF => None,
        _ => {
            panic!("Invalid token type");
        }
    };
    Ok(expr)
}
