use super::handle_tokenize::tokenize_multiline;
use crate::{evaluate::Expression, parse::parse_tokens, token::Token, tokenize::TokenizerResult};
use std::process::exit;

pub fn handle_parse(input: String) -> Vec<String> {
    let result = parse(input);
    return result
        .iter()
        .flat_map(|x| x.iter().map(|y| y.to_string()))
        .collect();
}

pub fn parse(input: String) -> Vec<Vec<Expression>> {
    let tokens_per_line = tokenize_multiline(input);

    let result: Result<Vec<Vec<Token>>, ()> = tokens_per_line
        .into_iter()
        .map(|v| match v {
            TokenizerResult::VALID(tokens) => Ok(tokens),
            TokenizerResult::INVALID(_) => Err(()),
        })
        .collect();

    let Ok(tokens) = result else {
        exit(65);
    };

    let result: Vec<_> = tokens
        .into_iter()
        .enumerate()
        .map(|(line_i, result)| {
            let result = parse_tokens(result);
            match result {
                Ok(expr) => expr,
                Err(e) => {
                    eprintln!("[line {}] {}", line_i + 1, e);
                    exit(65);
                }
            }
        })
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::test_case;

    #[test_case("(14 - 33)", "(group (- 14.0 33.0))")]
    fn test_handle_evaluate(input: &str, expected: &str) {
        test(input, expected)
    }
    #[test_case("14 - 33", "(- 14.0 33.0)")]
    #[test_case("-(-2)", "(- (group (- 2.0)))")]
    #[test_case("(-2)", "(group (- 2.0))")]
    #[test_case("\"hello\" != \"world\"", "(!= hello world)")]
    #[test_case("94 <= 104", "(<= 94.0 104.0)")]
    #[test_case("83 < 99 < 115", "(< (< 83.0 99.0) 115.0)")]
    #[test_case("(2+1)", "(group (+ 2.0 1.0))")]
    #[test_case("!(false)", "(! (group false))")]
    #[test_case("(!!(false))", "(group (! (! (group false))))")]
    #[test_case("1 + 2", "(+ 1.0 2.0)")]
    #[test_case("50 == 45", "(== 50.0 45.0)")]
    #[test_case("(\"foo\")", "(group foo)")]
    fn test_handle_parse(input: &str, expected: &str) {
        test(input, expected)
    }

    fn test(input: &str, expected: &str) {
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, vec![expected]);
    }
}
