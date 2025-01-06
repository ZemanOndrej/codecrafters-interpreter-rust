use super::{parse_token, ParseError};
use crate::{evaluation::Expression, token::Token};

pub fn parse_tokens(results: Vec<Token>) -> Result<Vec<Expression>, ParseError> {
    let mut parsed_expressions = Vec::new();

    let mut iterator = results.iter().peekable();
    while let Some(token) = iterator.next() {
        let expression = parse_token(token, &mut iterator, &mut parsed_expressions)?;

        let Some(expr) = expression else {
            continue;
        };
        parsed_expressions.push(expr);
    }

    Ok(parsed_expressions)
}

 pub fn create_error(token: &Token) -> String {
    format!(
        "Error at '{}': Expect expression.",
        token.token_type.get_lexeme()
    )
}

#[cfg(test)]
mod test {
    use crate::{parser::parse_tokens, tokenizer::tokenize};

    #[test]
    fn test_invalid_braces() {
        let input = "(foo";
        let tokens: Result<Vec<_>, _> = tokenize(input).into_iter().collect();
        let result = parse_tokens(tokens.unwrap());
        let Err(result) = result else {
            panic!("Expected error, got {result:?}");
        };
        assert_eq!(result, "Error at '(': Expect ')'".into());
    }
}
