use crate::{evaluate::Expression, token::Token};

use super::parse_token;

pub fn parse_tokens(results: Vec<Token>) -> Result<Vec<Expression>, String> {
    let mut stack = Vec::new();

    let mut iterator = results.iter().peekable();
    while let Some(token) = iterator.next() {
        let expr = parse_token(token, &mut iterator, &mut stack)?;

        let Some(expr) = expr else {
            continue;
        };
        stack.push(expr);
    }

    Ok(stack)
}

pub fn create_error(token: &Token) -> String {
    format!(
        "Error at '{}': Expect expression.",
        token.token_type.get_lexeme()
    )
}

#[cfg(test)]
mod test {
    use crate::{parse::parse_tokens, tokenize::tokenize};

    #[test]
    fn test_invalid_braces() {
        let input = "(foo";
        let tokens: Result<Vec<_>, _> = tokenize(input).into_iter().collect();
        let result = parse_tokens(tokens.unwrap());
        let Err(result) = result else {
            panic!("Expected error, got {:?}", result);
        };
        assert_eq!(result, "Error at '(': Expect expression.".to_string());
    }
}
