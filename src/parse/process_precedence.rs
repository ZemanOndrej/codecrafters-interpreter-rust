use crate::{evaluate::Expression, token::Token, token_type::TokenType};

use super::{parse_token, InputIter};

pub fn parse_precedence(
    token_type: TokenType,
    left: Expression,
    input: &mut InputIter,
    stack: &mut Vec<Expression>,
) -> Expression {
    // todo implement multi operator precedence
    if !matches!(
        &left,
        Expression::Literal(Token {
            token_type: TokenType::NUMBER(_),
            ..
        })
    ) {
        return left;
    }
    let Some(next) = input.peek() else {
        return left;
    };
    let next_precedence = next.token_type.get_precedence();
    let current_precedence = token_type.get_precedence();

    if next_precedence > current_precedence {
        let next = input.next().unwrap();
        let right = parse_token(input.next().unwrap(), input, stack)
            .unwrap()
            .unwrap();
        let r = Expression::Binary(Box::new(left), next.clone(), Box::new(right)).into();
        r
    } else {
        left
    }
}
