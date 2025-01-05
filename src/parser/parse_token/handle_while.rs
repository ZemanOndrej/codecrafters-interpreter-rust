use super::{parse_token, InputIter};
use crate::{evaluation::Expression, parser::ParseError, token::Token};

pub fn handle_while(
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
    let loop_body = parse_token(next, input, expression_stack)?.unwrap();
    Ok(Some(Expression::While {
        condition: Box::new(condition),
        then: Box::new(loop_body),
    }))
}
