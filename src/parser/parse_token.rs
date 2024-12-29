mod handle_assignment;
mod handle_conditionals;
mod handle_for;
mod handle_fun;
mod handle_identifier;
mod handle_while;

use crate::{
    evaluation::Expression,
    parser::{create_error, parse_expression, parse_expressions, parse_precedence},
    sub_tokens::*,
    token::Token,
    token_type::TokenType,
};
use handle_assignment::*;
use handle_conditionals::*;
use handle_for::*;
use handle_fun::*;
use handle_identifier::*;
use handle_while::*;
use std::{iter::Peekable, slice::Iter};
pub type InputIter<'a> = Peekable<Iter<'a, Token>>;

pub fn parse_token(
    token: &Token,
    input: &mut InputIter,
    expression_stack: &mut Vec<Expression>,
) -> Result<Option<Expression>, String> {
    use TokenType::*;
    let expr = match &token.token_type {
        SLASH(SlashType::COMMENT) => {
            let Some(next) = input.next() else {
                return Ok(None);
            };
            let res = parse_token(next, input, expression_stack);
            return res;
        }
        BANG(BangType::BANG) => {
            let right = parse_token(input.next().unwrap(), input, expression_stack)?.unwrap();

            Expression::Unary(token.clone(), Box::new(right)).into()
        }
        FALSE | TRUE | NUMBER(_) | NIL | STRING(_) => Expression::Literal(token.clone()).into(),
        IDENTIFIER(_) => handle_identifier(token, input)?,
        MINUS => {
            let left = expression_stack.pop();
            let value = match left {
                Some(left) => {
                    let right = parse_precedence(token, input, expression_stack)?.unwrap();
                    Expression::Binary(Box::new(left), token.clone(), Box::new(right))
                }
                None => {
                    let right =
                        parse_token(input.next().unwrap(), input, expression_stack)?.unwrap();
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
        | EQUAL(EqualType::EQUAL_EQUAL)
        | OR
        | AND => {
            let left = expression_stack.pop().ok_or_else(|| create_error(token))?;
            let right = parse_precedence(token, input, expression_stack)?.unwrap();

            Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into()
        }
        STAR => {
            let left = expression_stack.pop().ok_or_else(|| create_error(token))?;
            let right = parse_token(input.next().unwrap(), input, expression_stack)?.unwrap();
            Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into()
        }

        LEFT_PAREN => {
            let (inner, _) = parse_expression(input, token, &[RIGHT_PAREN], true)?;
            let r = Expression::Grouping(Box::new(inner)).into();
            r
        }
        RIGHT_PAREN => {
            return Err(format!(
                "Error at '{}': Expect ';' after value.",
                token.token_type.get_lexeme()
            ));
        }

        PRINT => {
            let mut arguments = Vec::new();
            loop {
                let (arg, _) = parse_expression(input, token, &[COMMA, SEMICOLON], false)?;
                arguments.push(arg);

                let next = input.next().unwrap();
                if next.token_type == SEMICOLON {
                    break;
                }
            }

            Expression::FunctionCall(token.clone(), arguments).into()
        }
        SEMICOLON => None,
        LEFT_BRACE => {
            let mut expressions = Vec::new();
            loop {
                let (expr, next) =
                    parse_expressions(input, token, &[SEMICOLON, RIGHT_BRACE], false)?;
                // dbg!(&expr);
                expressions.extend(expr);
                input.next().unwrap(); // consume the semicolon or right brace

                if next.token_type == RIGHT_BRACE {
                    break;
                }
            }
            // dbg!(&expressions);
            Expression::Scope(token.clone(), expressions).into()
        }

        VAR => {
            let name = input.next().unwrap().token_type.get_lexeme();

            let next_token = input.peek().unwrap();
            if matches!(next_token.token_type, EQUAL(EqualType::EQUAL)) {
                input.next().unwrap();
                let (expr, _) = parse_expression(input, token, &[SEMICOLON], false)?;
                Expression::Variable(name, Box::new(expr)).into()
            } else {
                Expression::Variable(
                    name,
                    Box::new(Expression::Literal(Token::new(
                        TokenType::NIL,
                        token.line_index,
                    )))
                    .into(),
                )
                .into()
            }
        }
        EQUAL(EqualType::EQUAL) => handle_assignment(expression_stack, token, input)?,
        EOF => None,
        WHILE => handle_while(expression_stack, token, input)?,
        FOR => handle_for(expression_stack, token, input)?,
        FUN => handle_fun(token, input)?,
        RETURN => {
            let (mut exprs, _) = parse_expressions(input, token, &[SEMICOLON], true)?;
            if exprs.is_empty() {
                return Ok(Some(Expression::Return(Box::new(Expression::nil()))));
            }
            if exprs.len() > 1 {
                return Err("Expected single expression after return".to_string());
            }
            let expr = exprs.remove(0);
            Expression::Return(Box::new(expr)).into()
        }

        IF => handle_conditionals(expression_stack, token, input)?,
        e => {
            panic!("Invalid token type {:?}", e);
        }
    };
    Ok(expr)
}
