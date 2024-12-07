use super::InputIter;
use crate::{
    evaluate::Expression,
    parse::{create_error, parse_expression, parse_token},
    sub_tokens::EqualType,
    token::Token,
    token_type::TokenType,
};

pub fn handle_assignment(
    expression_stack: &mut Vec<Expression>,
    token: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, String> {
    use TokenType::*;

    // dbg!(&input, &expression_stack);
    let left = expression_stack.pop().ok_or_else(|| create_error(token))?;
    let mut right = parse_token(input.next().unwrap(), input, expression_stack)?.unwrap();
    let Some(next) = input.peek() else {
        return Ok(Some(Expression::Binary(
            Box::new(left),
            token.clone(),
            Box::new(right),
        )));
    };
    dbg!(next, &right);
    if next.token_type == EQUAL(EqualType::EQUAL) {
        return handle_next_assignment(input, right, expression_stack, left);
    } else if next.token_type != SEMICOLON {
        let next = input.next().unwrap();
        let (expr, _) = parse_expression(input, next, &[SEMICOLON], true)?;
        right = Expression::Binary(Box::new(right), next.clone(), Box::new(expr))
    }
    Ok(Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into())
}

fn handle_next_assignment(
    input: &mut InputIter,
    prev_right: Expression,
    expression_stack: &mut Vec<Expression>,
    prev_left: Expression,
) -> Result<Option<Expression>, String> {
    let token = input.next().unwrap(); // consume the equal token
    expression_stack.push(prev_right);

    let new_right = handle_assignment(expression_stack, token, input)?.unwrap();

    return Ok(Some(Expression::Binary(
        Box::new(prev_left),
        token.clone(),
        Box::new(new_right),
    )));
}
