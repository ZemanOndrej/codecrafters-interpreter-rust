use crate::{
    evaluate::Expression,
    parse::parse_tokens,
    token::Token,
    tokenize::{tokenize, TokenError},
};
use std::process::exit;

pub fn handle_parse(input: String) -> Vec<String> {
    let result = parse(input);
    return result.iter().map(|x| x.to_string()).collect();
}

pub fn parse(input: String) -> Vec<Expression> {
    let result = parse_internal(input);
    let result = match result {
        Ok(expr) => expr,
        Err(e) => {
            dbg!(&e);
            eprintln!("{}", e);
            exit(65);
        }
    };

    result
}

pub(super) fn parse_internal(input: String) -> Result<Vec<Expression>, String> {
    let tokens = tokenize(input.as_str());

    let result: Result<Vec<Token>, TokenError> = tokens.into_iter().collect();

    let Ok(tokens) = result else {
        exit(65);
    };

    let result = parse_tokens(tokens);
    result
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use ntest::test_case;

    #[test_case("(95 +)")]
    #[test_case("print;")]
    pub fn test_handle_evaluate_error(input: &str) {
        let file_contents = String::from(input);
        let result = parse_internal(file_contents);
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_assignment() {
        let input = r#"
		var a;
		var b=1;
		a = b + 89;
		"#;
        let expected = vec![
            "variable declaration \"a\":nil",
            "variable declaration \"b\":1.0",
            "(= null (+ null 89.0))",
        ];
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_multiple_variable_assignment_expression() {
        let input = r#"
		var a;
		var b=1;
		a = b + 89 + 1;
		"#;
        let expected = vec![
            "variable declaration \"a\":nil",
            "variable declaration \"b\":1.0",
            "(= null (+ (+ null 89.0) 1.0))",
        ];
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_multiple_variable_assignment() {
        let input = r#"
		var a;
		var b;
		a = b = 89;
		"#;
        let expected = vec![
            "variable declaration \"a\":nil",
            "variable declaration \"b\":nil",
            "(= null (= null 89.0))",
        ];
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_variable_assignment_with_expression() {
        let input = r#"
		var c = 1;
		var a;
		var b;
		a = b = 89 + c;
		"#;
        let expected = vec![
            "variable declaration \"c\":1.0",
            "variable declaration \"a\":nil",
            "variable declaration \"b\":nil",
            "(= null (= null (+ 89.0 null)))",
        ];
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_variable_assignment_with_simple_expression() {
        let input = r#"
		var c = 1;
		var a;
		var b;
		a = b = c + 2;
		"#;
        let expected = vec![
            "variable declaration \"c\":1.0",
            "variable declaration \"a\":nil",
            "variable declaration \"b\":nil",
            "(= null (= null (+ null 2.0)))",
        ];
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple() {
        let input = r#"
		var baz = 1;
		print baz = 2;

		"#;
        let expected = vec![
            "variable declaration \"baz\":1.0",
            "function PRINT print null:(= null 2.0)",
        ];
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, expected);
    }

    #[test_case("!(false)", "(! (group false))")]
    fn test_handle_parse(input: &str, expected: &str) {
        test(input, expected)
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
        test(input, expected)
    }

    fn test(input: &str, expected: &str) {
        let file_contents = String::from(input);
        let result = handle_parse(file_contents);
        assert_eq!(result, vec![expected]);
    }
}
