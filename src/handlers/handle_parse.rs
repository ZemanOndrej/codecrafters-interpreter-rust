use crate::{
    evaluation::Expression,
    parser::{parse_tokens, ParseError},
    token::Token,
    tokenizer::{tokenize, TokenError},
};
use std::process::exit;


pub fn handle_parse(input: &str) -> Vec<String> {
    let result = parse(input);
    result
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

pub fn parse(input: &str) -> Vec<Expression> {
    let result = parse_internal(input);

    match result {
        Ok(expr) => expr,
        Err(ParseError::Default(e)) => {
            dbg!(&e);
            eprintln!("{e}");
            exit(65);
        }
        Err(ParseError::Syntax(e)) => {
            dbg!(&e);
            eprintln!("{e}");
            exit(70);
        }
    }
}

pub(super) fn parse_internal(input: &str) -> Result<Vec<Expression>, ParseError> {
    let tokens = tokenize(input);

    let result: Result<Vec<Token>, TokenError> = tokens.into_iter().collect();

    let Ok(tokens) = result else {
        exit(65);
    };

    parse_tokens(tokens)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ntest::test_case;

    #[test_case("85();")]
    #[test_case("(true == true)();")]
    pub fn test_handle_evaluate_error(input: &str) {
        let result = parse_internal(input);
        assert!(result.is_err());
        assert!(matches!(result, Err(ParseError::Syntax(_))));
    }
    #[test_case("(95 +)")]
    #[test_case("print;")]
    pub fn test_handle_evaluate_error(input: &str) {
        let result = parse_internal(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_assignment() {
        let input = r"
		var a;
		var b=1;
		a = b + 89;
		";
        let expected = vec![
            "variable declaration \"a\":nil",
            "variable declaration \"b\":1.0",
            "(= null (+ null 89.0))",
        ];

        let result = handle_parse(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_multiple_variable_assignment_expression() {
        let input = r"
		var a;
		var b=1;
		a = b + 89 + 1;
		";
        let expected = vec![
            "variable declaration \"a\":nil",
            "variable declaration \"b\":1.0",
            "(= null (+ (+ null 89.0) 1.0))",
        ];

        let result = handle_parse(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_multiple_variable_assignment() {
        let input = r"
		var a;
		var b;
		a = b = 89;
		";
        let expected = vec![
            "variable declaration \"a\":nil",
            "variable declaration \"b\":nil",
            "(= null (= null 89.0))",
        ];

        let result = handle_parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_variable_assignment_with_expression() {
        let input = r"
		var c = 1;
		var a;
		var b;
		a = b = 89 + c;
		";
        let expected = vec![
            "variable declaration \"c\":1.0",
            "variable declaration \"a\":nil",
            "variable declaration \"b\":nil",
            "(= null (= null (+ 89.0 null)))",
        ];

        let result = handle_parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_variable_assignment_with_simple_expression() {
        let input = r"
		var c = 1;
		var a;
		var b;
		a = b = c + 2;
		";
        let expected = vec![
            "variable declaration \"c\":1.0",
            "variable declaration \"a\":nil",
            "variable declaration \"b\":nil",
            "(= null (= null (+ null 2.0)))",
        ];

        let result = handle_parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple() {
        let input = r"
		var baz = 1;
		print baz = 2;

		";
        let expected = vec![
            "variable declaration \"baz\":1.0",
            "function PRINT print null:(= null 2.0)",
        ];

        let result = handle_parse(input);
        assert_eq!(result, expected);
    }

    #[test_case("!(false)", "(! (group false))")]
    fn test_handle_parse(input: &str, expected: &str) {
        test(input, expected);
    }

    #[test_case("(14 - 33)", "(group (- 14.0 33.0))")]
    #[test_case("\"hello\" != \"world\"", "(!= hello world)")]
    #[test_case("14 - 33", "(- 14.0 33.0)")]
    #[test_case("-(-2)", "(- (group (- 2.0)))")]
    #[test_case("(-2)", "(group (- 2.0))")]
    #[test_case("94 <= 104", "(<= 94.0 104.0)")]
    #[test_case("83 < 99 < 115", "(< (< 83.0 99.0) 115.0)")]
    #[test_case("(2+1)", "(group (+ 2.0 1.0))")]
    #[test_case("!(false)", "(! (group false))")]
    #[test_case("(!!(false))", "(group (! (! (group false))))")]
    #[test_case("50 == 45", "(== 50.0 45.0)")]
    #[test_case("(\"foo\")", "(group foo)")]
    #[test_case("1 + 2", "(+ 1.0 2.0)")]
    fn test_handle_parse_all(input: &str, expected: &str) {
        test(input, expected);
    }

    fn test(input: &str, expected: &str) {
        let result = handle_parse(input);
        assert_eq!(result, vec![expected]);
    }
}
