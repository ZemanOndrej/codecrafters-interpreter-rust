use crate::{handle_tokenize::tokenize_multiline, parse::parse, tokenize::TokenizerResult};
use std::process::exit;

pub fn handle_parse(input: String) -> Vec<String> {
    let tokens_per_line = tokenize_multiline(input);

    if tokens_per_line.iter().any(|r| match r {
        TokenizerResult::INVALID(_) => true,
        _ => false,
    }) {
        exit(65)
    }

    let result: Vec<_> = tokens_per_line
        .into_iter()
        .map(|result| match result {
            TokenizerResult::VALID(tokens) => parse(tokens),
            _ => {
                panic!("Invalid tokens");
            }
        })
        .collect();

    for i in result.iter() {
        for expr in i.iter() {
            println!("{}", expr.to_string());
        }
    }
    return result
        .iter()
        .flat_map(|x| x.iter().map(|y| y.to_string()))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::test_case;

    #[test]
    fn test_handle_parse() {
        test("94 <= 104", "(<= 94.0 104.0)")
    }

    #[test_case("83 < 99 < 115", "(< (< 83.0 99.0) 115.0)")]
    #[test_case("(2+1)", "(group (+ 2.0 1.0))")]
    #[test_case("!(false)", "(! (group false))")]
    #[test_case("(!!(false))", "(group (! (! (group false))))")]
    #[test_case("1 + 2", "(+ 1.0 2.0)")]
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
