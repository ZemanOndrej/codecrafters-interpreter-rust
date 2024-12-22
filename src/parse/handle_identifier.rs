use super::InputIter;
use crate::{evaluate::Expression, parse::parse_expressions, token::Token, token_type::TokenType};

pub fn handle_identifier(
    token: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, String> {
    if let Some(next) = input.peek() {
        use TokenType::*;

        if next.token_type == LEFT_PAREN {
            let left_paren = input.next().unwrap(); // consume the left paren
            let (arguments, _) = parse_expressions(input, left_paren, &[RIGHT_PAREN], true)?;

            return Ok(Some(Expression::FunctionCall(token.clone(), arguments)));
        }
    }
    Ok(Some(Expression::Literal(token.clone())))
}
