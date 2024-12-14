use crate::{evaluate::Expression, token::Token};
use super::{parse_token, InputIter};

pub fn handle_while(
    expression_stack: &mut Vec<Expression>,
    _: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, String> {
    let Some(next) = input.next() else {
        return Err("Expected condition".to_string());
    };
    let condition = parse_token(next, input, expression_stack)?.unwrap();
    let Some(next) = input.next() else {
        return Err("Expected then statement".to_string());
    };
    let loop_body = parse_token(next, input, expression_stack)?.unwrap();
    Ok(Some(Expression::While {
        condition: Box::new(condition),
        then: Box::new(loop_body),
    }))
}
