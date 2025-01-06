#[cfg(test)]
mod tests {
    use crate::handlers::handle_parse::parse_internal;
    use ntest::test_case;

    #[test_case("clock(;", "Error at ';': Expect expression.")]
    #[test_case("print clock(;", "Error at ';': Expect expression.")]
    #[test_case("print clock)));", "Error at ')': Expect ';' after value.")]
    #[test_case(
        r"
    	fun f() 74;
    	print f();
    ",
        "Error at '74': Expect '{' before function body."
    )]
    #[test_case(
        r"
    	fun foo(a, b c, d, e, f) {}
    	foo();
    ",
        "Error at 'c': Expect ')' after parameters."
    )]
    fn test_handle_evaluate_error(input: &str, expected_err: &str) {
        test_handle_parse_error(input, expected_err);
    }

    pub fn test_handle_parse_error(input: &str, expected_err: &str) {
        let result = parse_internal(input);
        assert!(result.is_err());
        let err = result.unwrap_err();

        assert_eq!(err, expected_err.into());
    }
}
