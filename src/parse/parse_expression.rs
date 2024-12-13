use crate::{evaluate::Expression, sub_tokens::SlashType, token::Token, token_type::TokenType};

use super::{parse_token::parse_token, InputIter};

pub fn parse_expression_with_stack(
    input: &mut InputIter,
    token: &Token,
    end_tokens: &[TokenType],
    remove_end_token: bool,
    stack: Vec<Expression>,
) -> Result<(Expression, Token), String> {
    let (mut stack, next) =
        parse_expression_internal(input, token, end_tokens, remove_end_token, stack)?;
    let inner = stack
        .pop()
        .ok_or(generate_error_message(token, end_tokens))?;
    Ok((inner, next.unwrap()))
}
pub fn parse_expression(
    input: &mut InputIter,
    token: &Token,
    end_tokens: &[TokenType],
    remove_end_token: bool,
) -> Result<(Expression, Token), String> {
    let (mut stack, next) = parse_expression_internal(
        input,
        token,
        end_tokens,
        remove_end_token,
        Default::default(),
    )?;
    let inner = stack
        .pop()
        .ok_or(generate_error_message(token, end_tokens))?;
    Ok((inner, next.unwrap()))
}
pub fn parse_expressions(
    input: &mut InputIter,
    token: &Token,
    end_tokens: &[TokenType],
    remove_end_token: bool,
) -> Result<(Vec<Expression>, Token), String> {
    let (stack, next) = parse_expression_internal(
        input,
        token,
        end_tokens,
        remove_end_token,
        Default::default(),
    )?;

    Ok((stack, next.unwrap()))
}

fn parse_expression_internal(
    input: &mut InputIter,
    token: &Token,
    end_tokens: &[TokenType],
    remove_end_token: bool,
    mut stack: Vec<Expression>,
) -> Result<(Vec<Expression>, Option<Token>), String> {
    let mut next: Option<&Token>;
    loop {
        next = input.peek().map(|v| &**v);
        let Some(next) = next else {
            // dbg!(&next);
            return Err(generate_error_message(token, end_tokens));
        };
        if end_tokens.contains(&next.token_type) {
            if remove_end_token {
                input.next();
            }
            break;
        }

        input.next().unwrap();
        if next.token_type == TokenType::SLASH(SlashType::COMMENT) {
            continue;
        }
        let value = parse_token(next, input, &mut stack)?
            .ok_or(generate_error_message(token, end_tokens))?;
        stack.push(value);
    }
    Ok((stack, next.cloned()))
}

fn generate_error_message(token: &Token, end_tokens: &[TokenType]) -> String {
    format!(
        "Error at '{}': Expect {}",
        token.token_type.get_lexeme(),
        end_tokens
            .iter()
            .map(|x| format!("'{}'", x.get_lexeme()))
            .collect::<Vec<String>>()
            .join(", ")
    )
}
