use crate::{
    evaluate::Expression,
    sub_tokens::{BangType, EqualType, GreaterType, LessType, SlashType},
    token::Token,
    token_type::TokenType,
};

pub fn parse_tokens(results: Vec<Token>) -> Result<Vec<Expression>, String> {
    let mut stack = Vec::new();

    let mut iterator = results.iter();
    while let Some(token) = iterator.next() {
        let expr = parse_token(token, &mut iterator, &mut stack)?;

        let Some(expr) = expr else {
            continue;
        };
        stack.push(expr);
    }

    Ok(stack)
}

fn parse_token(
    token: &Token,
    input: &mut std::slice::Iter<'_, Token>,
    stack: &mut Vec<Expression>,
) -> Result<Option<Expression>, String> {
    use TokenType::*;
    // dbg!(&token);
    let expr = match &token.token_type {
        BANG(BangType::BANG) => {
            let right = parse_token(input.next().unwrap(), input, stack)?.unwrap();

            Expression::Unary(token.clone(), Box::new(right)).into()
        }
        FALSE | TRUE | NUMBER(_) | NIL | STRING(_) => Expression::Literal(token.clone()).into(),
        MINUS => {
            let right = parse_token(input.next().unwrap(), input, stack)?.unwrap();

            let left = stack.pop();
            let value = match left {
                Some(left) => Expression::Binary(Box::new(left), token.clone(), Box::new(right)),
                None => Expression::Unary(token.clone(), Box::new(right)),
            };
            value.into()
        }

        PLUS
        | STAR
        | SLASH(SlashType::SLASH)
        | LESS(LessType::LESS)
        | LESS(LessType::LESS_EQUAL)
        | GREATER(GreaterType::GREATER)
        | BANG(BangType::BANG_EQUAL)
        | GREATER(GreaterType::GREATER_EQUAL)
        | EQUAL(EqualType::EQUAL)
        | EQUAL(EqualType::EQUAL_EQUAL) => {
            let left = stack.pop().ok_or_else(|| create_error(token))?;
            let right = parse_token(input.next().unwrap(), input, stack)?.unwrap();
            Expression::Binary(Box::new(left), token.clone(), Box::new(right)).into()
        }

        LEFT_PAREN => {
            let (inner, _) = parse_expression(input, token, &[RIGHT_PAREN])?;
            let r = Expression::Grouping(Box::new(inner)).into();
            r
        }
        RIGHT_PAREN => {
            let right = stack.pop().ok_or_else(|| create_error(token))?;

            stack.pop().unwrap();
            Expression::Grouping(right.into()).into()
        }
        IDENTIFIER(_) => {
            let value = Expression::Literal(token.clone()).into();
            value
        }
        PRINT => {
            let mut arguments = Vec::new();
            loop {
                let (arg, next) = parse_expression(input, token, &[COMMA, SEMICOLON])?;
                arguments.push(arg);

                if next.token_type == SEMICOLON {
                    break;
                }
            }

            Expression::Function(token.clone(), arguments).into()
        }
        SEMICOLON => None,
        EOF => None,

        _ => {
            panic!("Invalid token type");
        }
    };
    Ok(expr)
}

fn parse_expression(
    input: &mut std::slice::Iter<'_, Token>,
    token: &Token,
    end_tokens: &[TokenType],
) -> Result<(Expression, Token), String> {
    let mut stack = Vec::new();
    let mut next;
    loop {
        next = input.next();
        let Some(next) = next else {
            return Err(format!(
                "Error at '{}': Expect closing bracket",
                token.token_type.get_lexeme()
            ));
        };
        if end_tokens.contains(&next.token_type) {
            break;
        }
        let value = parse_token(next, input, &mut stack)?.unwrap();
        stack.push(value);
    }
    let inner = stack.pop().unwrap();
    Ok((inner, next.unwrap().clone()))
}

pub fn create_error(token: &Token) -> String {
    format!(
        "Error at '{}': Expect expression.",
        token.token_type.get_lexeme()
    )
}
