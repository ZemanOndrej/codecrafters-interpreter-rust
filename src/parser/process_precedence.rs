use crate::{evaluation::Expression, token::Token, token_type::TokenType};

use super::{parse_token, InputIter, ParseError};

pub fn parse_precedence(
    starting_operator: &Token,
    input: &mut InputIter,
    stack: &mut Vec<Expression>,
) -> Result<Option<Expression>, ParseError> {
    let left = input.peek().unwrap();

    // dbg!(left);
    // dbg!(starting_operator);
    // dbg!(&stack);
    // todo implement better operator precedence
    if !matches!(
        &left,
        Token {
            token_type: TokenType::NUMBER(_)
                | TokenType::LEFT_PAREN
                | TokenType::MINUS
                | TokenType::IDENTIFIER(_)
                | TokenType::EOF,
            ..
        }
    ) {
        return parse_token(input.next().unwrap(), input, stack);
    }

    Ok(get_right_most_token_with_lowest_precedence(starting_operator, input, stack).into())
}

fn get_right_most_token_with_lowest_precedence(
    starting_operator: &Token,
    input: &mut InputIter,
    stack: &mut Vec<Expression>,
) -> Expression {
    // dbg!(starting_operator);
    // dbg!(&input);
    let start_operator_precedence = starting_operator.token_type.get_precedence();
    let first = input.next().unwrap().clone();
    let mut expressions = vec![parse_token(&first, input, stack).unwrap().unwrap()]; // this is problem if it is grouping
    let mut operators = vec![];
    while let Some(next) = input.peek() {
        let next_precedence = next.token_type.get_precedence();
        if next_precedence < start_operator_precedence {
            let next_operator = input.next().unwrap().clone();
            let next_input = input.next().unwrap().clone();
            let next_input = parse_token(&next_input, input, stack).unwrap().unwrap();
            // let next_input = parse_token(&next_input, input, stack).unwrap().unwrap();
            // dbg!(&next_operator);

            operators.push(next_operator);
            expressions.push(next_input);
        } else {
            break;
        }
    }

    if operators.len() == 0 {
        return expressions.pop().unwrap();
    }

    // dbg!(&expressions);
    // dbg!(&operators);
    let mut expressions = expressions.into_iter().rev().collect::<Vec<_>>();
    let mut operators = operators.into_iter().rev().collect::<Vec<_>>();
    let mut left: Option<Expression> = None;

    while expressions.len() > 0 && operators.len() > 0 {
        left = if let Some(right) = left {
            let operator = operators.pop().unwrap();
            let left = expressions.pop().unwrap();
            Some(Expression::Binary(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        } else {
            let left = expressions.pop().unwrap();
            let operator = operators.pop().unwrap();
            let right = expressions.pop().unwrap();
            Some(Expression::Binary(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        };
    }

    left.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // test get right most token with lowest precedence
    #[test]
    fn test_get_right_most_token_with_lowest_precedence_2_times_3() {
        // "1 + 2 * 3";

        let input = vec![
            Token {
                token_type: TokenType::NUMBER("2.0".into()),
                ..Default::default()
            },
            Token {
                token_type: TokenType::STAR,
                ..Default::default()
            },
            Token {
                token_type: TokenType::NUMBER("3.0".into()),
                ..Default::default()
            },
        ];
        let mut input = input.iter().peekable();
        let left = Token {
            token_type: TokenType::PLUS,
            ..Default::default()
        };

        let result = get_right_most_token_with_lowest_precedence(&left, &mut input, &mut vec![]);
        let result = result
            .evaluate(&mut Default::default())
            .unwrap()
            .assert_value()
            .unwrap();
        assert_eq!(result.to_string(), "6");
    }

    #[test]
    fn test_get_right_most_token_with_lowest_precedence() {
        // "1 + 2 * 3 * 4";

        let input = vec![
            Token {
                token_type: TokenType::NUMBER("2.0".into()),
                ..Default::default()
            },
            Token {
                token_type: TokenType::STAR,
                ..Default::default()
            },
            Token {
                token_type: TokenType::NUMBER("3.0".into()),
                ..Default::default()
            },
            Token {
                token_type: TokenType::STAR,
                ..Default::default()
            },
            Token {
                token_type: TokenType::NUMBER("4.0".into()),
                ..Default::default()
            },
        ];
        let mut input = input.iter().peekable();
        let left = Token {
            token_type: TokenType::PLUS,
            ..Default::default()
        };

        let result = get_right_most_token_with_lowest_precedence(&left, &mut input, &mut vec![]);
        let result = result
            .evaluate(&mut Default::default())
            .unwrap()
            .assert_value()
            .unwrap();
        assert_eq!(result.to_string(), "24");
    }
}
