use super::handle_parse::parse;
use crate::evaluate::EvaluatedExpression;
use std::{collections::HashMap, process::exit};

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub variables: HashMap<String, EvaluatedExpression>,
}

pub fn handle_evaluate(input: String) -> Vec<String> {
    let result = handle_evaluate_internal(input)
        .map(|x| match x {
            Ok(x) => x.value,
            Err(e) => {
                dbg!(&e);
                eprintln!("{}", e);
                exit(70);
            }
        })
        .collect();

    return result;
}

fn handle_evaluate_internal(
    input: String,
) -> impl Iterator<Item = Result<EvaluatedExpression, String>> {
    let parsed_input = parse(input);

    let mut context = Context::default();
    let result = parsed_input
        .into_iter()
        .map(move |x| x.evaluate(&mut context));

    return result;
}

#[cfg(test)]
mod tests {
    use crate::evaluate::EvaluatedExpression;

    use super::*;
    use ntest::test_case;

    #[test_case("1-(-2)", "3")]
    #[test_case("3+(2) * 2", "7")]
    fn test_handle_evaluate(input: &str, expected: &str) {
        test(input, expected)
    }

    #[test_case("1+(2) * 3", "7")]
    #[test_case("1+(2) * 2*2", "9")]
    #[test_case("1- (-2)", "3")]
    #[test_case("3+(2) * 2", "7")]
    #[test_case("11 >= 11", "true")]
    #[test_case("(21 * 2 + 57 * 2) / (2)", "78")]
    #[test_case("\"17\" == 17 ", "false")]
    #[test_case("17 == \"17\"", "false")]
    #[test_case("\"bar\" != \"foo\"", "true")]
    #[test_case("57 > -65", "true")]
    #[test_case("\"bar\" + \"quz\"", "barquz")]
    #[test_case("20 + 74 - (-(14 - 33))", "75")]
    #[test_case("(18 * 3 / (3 * 6))", "3")]
    #[test_case("!\"test\"", "false")]
    #[test_case("!(73.40)", "false")]
    #[test_case("-(73)", "-73")]
    #[test_case("(\"hello world!\")", "hello world!")]
    #[test_case("10.4000", "10.4")]
    #[test_case("50", "50")]
    #[test_case("\"hello world!\"", "hello world!")]
    #[test_case("true", "true")]
    fn test_all_handle_evaluate(input: &str, expected: &str) {
        test(input, expected)
    }

    fn test(input: &str, expected: &str) {
        let result = handle_evaluate(input.to_string());
        let expected = vec![expected.to_string()];
        // dbg!(result.clone());
        // dbg!(expected.clone());
        assert_eq!(result, expected);
    }
    #[test_case(" \"foo\" + false")]
    fn test_handle_evaluate_error(input: &str) {
        test_error(input)
    }

    #[test_case(" \"foo\" + false")]
    #[test_case(" false / false")]
    #[test_case(" \"bar\" / 47")]
    #[test_case("14 * \"bar\"")]
    #[test_case(r#"print a;"#)]
    #[test_case(
        r#"
    	var quz;
    	quz = 1;
    	print quz;
    	print quz = 2;
    	print quz;
    "#
    )]
    #[test_case(
        r#"
    	var world = 21;
    	var result = (world + bar) / foo;
    	print result;
    "#
    )]

    fn test_all_handle_evaluate_error(input: &str) {
        test_error(input)
    }

    fn test_error(input: &str) {
        let res = parse(input.to_string());
        let res: Result<Vec<EvaluatedExpression>, String> = res
            .iter()
            .map(|x| x.evaluate(&mut Default::default()))
            .collect();
        assert!(res.is_err());
    }
}
