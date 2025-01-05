use super::{parse_token, InputIter};
use crate::{
    evaluation::Expression,
    parser::{parse_expression, ParseError},
    token::Token,
    token_type::TokenType,
};

pub fn handle_for(
    expression_stack: &mut Vec<Expression>,
    for_token: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, ParseError> {
    let Some(mut next) = input.next() else {
        return Err("Expected condition".into());
    };
    let next_token = input.peek().unwrap();

    let declaration = if next_token.token_type == TokenType::SEMICOLON {
        next = input.next().unwrap(); // consume the semicolon
        Expression::Literal(Token::new(TokenType::NIL, for_token.line_index))
    } else {
        let (declaration, _) = parse_expression(input, next, &[TokenType::SEMICOLON], true)?;
        declaration
    };
    let (condition, _) = parse_expression(input, next, &[TokenType::SEMICOLON], true)?;

    if let Some(next) = input.peek() {
        if next.token_type == TokenType::RIGHT_PAREN {
            input.next().unwrap(); // consume the right paren
            let next = input.next().unwrap(); // consume the left brace or scope

            let expression = Expression::create_for(
                declaration,
                condition,
                Expression::Literal(Token::new(TokenType::NIL, for_token.line_index)),
                parse_token(next, input, expression_stack)?.unwrap(),
            )?;
            return Ok(Some(expression));
        }
    }

    let (increment, _) = parse_expression(input, next, &[TokenType::RIGHT_PAREN], true)?;

    let token = input.next().unwrap(); // statement or scope
    let result = Expression::create_for(
        declaration,
        condition,
        increment,
        parse_token(token, input, expression_stack)?.unwrap(),
    )?;

    return Ok(Some(result));
}
