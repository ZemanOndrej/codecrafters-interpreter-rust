use super::handle_parse::parse;

pub fn handle_evaluate(input: String) -> Vec<String> {
    let parsed_input = parse(input);

    for line in &parsed_input {
        for e in line {
            dbg!(e);
        }
    }

    return parsed_input
        .iter()
        .flat_map(|x| x.iter().map(|y| y.evaluate()))
        .map(|x| x.value)
        .collect();
}

#[cfg(test)]
mod tests {

    use super::*;
    use ntest::test_case;

    #[test_case("\"17\" == 17 ", "false")]
    fn test_handle_evaluate(input: &str, expected: &str) {
        test(input, expected)
    }
    #[test_case("\"17\" == 17 ", "false")]
    #[test_case("17 == \"17\"", "false")]
    #[test_case("\"bar\" != \"foo\"", "true")]
    #[test_case("57 > -65", "true")]
    #[test_case("11 >= 11", "true")]
    #[test_case("\"bar\" + \"quz\"", "barquz")]
    #[test_case("20 + 74 - (-(14 - 33))", "75")]
    #[test_case("1- (-2)", "3")]
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
        dbg!(result.clone());
        dbg!(expected.clone());
        assert_eq!(result, expected);
    }
}
