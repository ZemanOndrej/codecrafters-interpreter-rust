use super::InputIter;
use crate::{
    evaluation::Expression,
    parser::{parse_token, ParseError},
    token::Token,
    token_type::TokenType,
};

pub fn handle_conditionals(
    expression_stack: &mut Vec<Expression>,
    _: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, ParseError> {
    let Some(next) = input.next() else {
        return Err("Expected condition".into());
    };
    let condition = parse_token(next, input, expression_stack)?.unwrap();
    let Some(next) = input.next() else {
        return Err("Expected then statement".into());
    };

    let then = parse_token(next, input, expression_stack)?.unwrap();
    let next = input.peek();
    if matches!(next, Some(token) if token.token_type == TokenType::ELSE) {
        input.next(); // consume the else token
        let Some(next) = input.next() else {
            return Err("Expected else statement".into());
        };
        let else_ = parse_token(next, input, expression_stack)?.unwrap();
        Ok(Some(Expression::IfElse {
            condition: Box::new(condition),
            then: Box::new(then),
            else_expr: Some(Box::new(else_)),
        }))
    } else {
        Ok(Some(Expression::IfElse {
            condition: Box::new(condition),
            then: Box::new(then),
            else_expr: None,
        }))
    }
}
