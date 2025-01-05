use super::{parse_expressions, parse_token, InputIter};
use crate::{evaluation::Expression, parser::ParseError, token::Token, token_type::TokenType};

pub fn handle_fun(_: &Token, input: &mut InputIter) -> Result<Option<Expression>, ParseError> {
    let fn_name = input.next().unwrap();
    let left_paren = input.next().unwrap();

    let mut args = vec![];
    loop {
        let (exprs, end_tok) = parse_expressions(
            input,
            left_paren,
            &[TokenType::COMMA, TokenType::RIGHT_PAREN],
            true,
        )?;
        if exprs.len() > 1 {
            let second = exprs.get(1).unwrap();
            if let Expression::Literal(token) = second {
                return Err(format!(
                    "Error at '{}': Expect ')' after parameters.",
                    token.token_type.get_lexeme(),
                )
                .into());
            } else {
                return Err("Expected identifier".into());
            }
        }
        let Some(expr) = exprs.first() else {
            break;
        };
        match expr {
            Expression::Literal(token) => {
                args.push(token.token_type.get_lexeme());
            }
            _ => {
                return Err("Expected identifier".into());
            }
        }
        if let TokenType::RIGHT_PAREN = end_tok.token_type {
            break;
        }
    }
    let token = input.next().unwrap();

    let body = parse_token(token, input, &mut Default::default())?.unwrap();

    match body {
        Expression::Scope(_, _) => {}
        _ => {
            return Err(format!(
                "Error at '{}': Expect '{{' before function body.",
                token.token_type.get_lexeme()
            )
            .into())
        }
    }

    Ok(Some(Expression::FunctionDeclaration {
        name: fn_name.clone(),
        args,
        body: Box::new(body),
    }))
}
