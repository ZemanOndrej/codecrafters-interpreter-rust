use super::InputIter;
use crate::{
    evaluation::Expression,
    parser::{create_error, parse_expression_with_stack, parse_token, ParseError},
    sub_tokens::EqualType,
    token::Token,
    token_type::TokenType,
};

pub fn handle_assignment(
    expression_stack: &mut Vec<Expression>,
    token: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, ParseError> {
    use TokenType::*;

    let left = expression_stack.pop().ok_or_else(|| create_error(token))?;
    let right_token = input.next().unwrap();
    let mut right = parse_token(right_token, input, expression_stack)?.unwrap();
    let Some(next) = input.peek() else {
        return Ok(Some(Expression::Binary(
            Box::new(left),
            right_token.clone(),
            Box::new(right),
        )));
    };
    // dbg!(&right, right_token, next);
    if next.token_type == EQUAL(EqualType::EQUAL) {
        return handle_next_assignment(input, right, expression_stack, left);
    } else if next.token_type != SEMICOLON && next.token_type != RIGHT_PAREN {
        (right, _) = parse_expression_with_stack(
            input,
            right_token,
            &[SEMICOLON, RIGHT_PAREN],
            false,
            vec![right],
        )?;
    }
    Ok(Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into())
}

fn handle_next_assignment(
    input: &mut InputIter,
    prev_right: Expression,
    expression_stack: &mut Vec<Expression>,
    prev_left: Expression,
) -> Result<Option<Expression>, ParseError> {
    let token = input.next().unwrap(); // consume the equal token
    expression_stack.push(prev_right);

    let new_right = handle_assignment(expression_stack, token, input)?.unwrap();

    Ok(Some(Expression::Binary(
        Box::new(prev_left),
        token.clone(),
        Box::new(new_right),
    )))
}
