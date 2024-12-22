use super::InputIter;
use crate::{evaluate::Expression, parse::parse_expressions, token::Token, token_type::TokenType};

pub fn handle_fun(_: &Token, input: &mut InputIter) -> Result<Option<Expression>, String> {
    let fn_name = input.next().unwrap();
    let left_paren = input.next().unwrap();
    let (arguments, _) = parse_expressions(input, left_paren, &[TokenType::RIGHT_PAREN], true)?;

    Ok(Some(Expression::FunctionDeclaration(
        fn_name.clone(),
        arguments,
    )))
}
