use super::InputIter;
use crate::{
    evaluation::Expression, parser::parse_expressions, token::Token, token_type::TokenType,
};

pub fn handle_identifier(
    token: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, String> {
    if let Some(next) = input.peek() {
        use TokenType::*;

        if next.token_type == LEFT_PAREN {
            let left_paren = input.next().unwrap(); // consume the left paren

            let mut arguments = vec![];
            loop {
                let (exprs, end_tok) =
                    parse_expressions(input, left_paren, &[COMMA, RIGHT_PAREN], true)?;
                let Some(expr) = exprs.into_iter().next() else {
                    break;
                };

                arguments.push(expr);

                if end_tok.token_type == RIGHT_PAREN {
                    break;
                }
            }

            return Ok(Some(Expression::FunctionCall(token.clone(), arguments)));
        }
    }
    Ok(Some(Expression::Literal(token.clone())))
}
