use super::handle_evaluate;

pub fn handle_run(input: String) {
    handle_evaluate(input);
}

#[cfg(test)]
mod tests {
    use crate::handlers::{handle_evaluate, handle_run};
    use ntest::test_case;

    #[test_case("print;")]
    fn test_handle_run_error(input: &str) {
        let _ = handle_run(input.to_string());
    }
    // #[test_case("1+2*3", "7")]
    #[test_case("28 - 84 * 67 - 54", "-5654")]
    fn test_handle_run(input: &str, expected: &str) {
        let result = handle_evaluate(input.to_string());
        let result = result.first().unwrap().as_str();
        assert_eq!(result, expected);
    }

    #[test_case(
        r#"
{
// This is a complex test case
str1 = "Test"
str2 = "Case"
num1 = 100
num2 = 200.00
result = (str1 == str2) != ((num1 + num2) >= 300)
}
"#
    )]
    fn test_handle_run(input: &str) {
        let _ = handle_run(input.to_string());
        // let expected = vec![expected.to_string()];
        // dbg!(result.clone());
        // dbg!(expected.clone());
        // assert_eq!(result, expected);
    }

    #[test_case(
        r#"
print false != true;

print "29
59
37
";

print "There should be an empty line above this.";

print "(" + "" + ")";

print "non-ascii: ॐ";

"#
    )]
    #[test_case(
        r#"
print "quz" + "world" + "foo";
print 22 - 56;
print "foo" == "bar";
"#
    )]
    fn test_all_handle_evaluate(input: &str) {
        let _ = handle_run(input.to_string());
        // let expected = vec![expected.to_string()];
        // dbg!(result.clone());
        // dbg!(expected.clone());
        // assert_eq!(result, expected);
    }
    #[test_case("print \"Hello, World!\";")]
    #[test_case("print \"world\" + \"foo\" + \"baz\";")]
    #[test_case("print false;")]
    fn test_all_handle_evaluate(input: &str) {
        let _ = handle_run(input.to_string());
        // let expected = vec![expected.to_string()];
        // dbg!(result.clone());
        // dbg!(expected.clone());
        // assert_eq!(result, expected);
    }

    // fn test(input: &str, expected: &str) {
    //     let result = handle_evaluate(input.to_string());
    //     let expected = vec![expected.to_string()];
    //     dbg!(result.clone());
    //     dbg!(expected.clone());
    //     assert_eq!(result, expected);
    // }
}
