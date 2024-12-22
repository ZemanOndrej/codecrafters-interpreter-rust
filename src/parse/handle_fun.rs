use super::{parse_expressions, parse_token, InputIter};
use crate::{evaluate::Expression, token::Token, token_type::TokenType};

pub fn handle_fun(_: &Token, input: &mut InputIter) -> Result<Option<Expression>, String> {
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
        let Some(expr) = exprs.first() else {
            break;
        };
        match expr {
            Expression::Literal(token) => {
                args.push(token.token_type.get_lexeme());
            }
            _ => {
                return Err("Expected identifier".to_string());
            }
        }
        if let TokenType::RIGHT_PAREN = end_tok.token_type {
            break;
        }
    }
    let token = input.next().unwrap();

    let expr = parse_token(token, input, &mut Default::default())?.unwrap();

    Ok(Some(Expression::FunctionDeclaration(
        fn_name.clone(),
        args,
        Box::new(expr),
    )))
}
