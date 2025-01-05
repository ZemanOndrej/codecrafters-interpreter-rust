use super::InputIter;
use crate::{
    evaluation::Expression,
    parser::{parse_expressions, ParseError},
    token::Token,
    token_type::TokenType,
};

pub fn handle_identifier(
    token: &Token,
    input: &mut InputIter,
) -> Result<Option<Expression>, ParseError> {
    if let Some(next) = input.peek() {
        use TokenType::*;

        if next.token_type == LEFT_PAREN {
            let left_paren = input.next().unwrap(); // consume the left paren

            let arguments = parse_args(input, left_paren)?;

            let function_call = Expression::FunctionCall(token.clone(), arguments);
            if let Some(next) = input.peek() {
                if next.token_type == LEFT_PAREN {
                    // handle function(args)(args)
                    let left_paren = input.next().unwrap(); // consume the left paren

                    let args = parse_args(input, left_paren)?;

                    let mut lambda_call =
                        Expression::FunctionCallLambda(Box::new(function_call), args);
                    loop {
                        if let Some(next) = input.peek() {
                            if next.token_type == LEFT_PAREN {
                                let left_paren = input.next().unwrap(); // consume the left paren
                                let args = parse_args(input, left_paren)?;
                                lambda_call =
                                    Expression::FunctionCallLambda(Box::new(lambda_call), args);
                                continue;
                            }
                        }
                        break;
                    }
                    return Ok(Some(lambda_call));
                }
            }
            return Ok(Some(function_call));
        }
    }
    Ok(Some(Expression::Literal(token.clone())))
}

fn parse_args(input: &mut InputIter, left_paren: &Token) -> Result<Vec<Expression>, ParseError> {
    use TokenType::*;

    let mut arguments = vec![];
    loop {
        let (exprs, end_tok) = parse_expressions(input, left_paren, &[COMMA, RIGHT_PAREN], true)?;
        let Some(expr) = exprs.into_iter().next() else {
            break;
        };

        arguments.push(expr);

        if end_tok.token_type == RIGHT_PAREN {
            break;
        }
    }
    Ok(arguments)
}
