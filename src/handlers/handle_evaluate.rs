use super::handle_parse::parse;
use crate::evaluate::{Context, EvaluatedExpression};
use std::process::exit;

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

pub(super) fn handle_evaluate_internal(
    input: String,
) -> impl Iterator<Item = Result<EvaluatedExpression, String>> {
    let parsed_input = parse(input);

    let mut context = Context::new_root();
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

    #[test_case("43 * 9 / 5 + 32;", "109.4")]
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

    #[test]
    fn test_custom_function_with_args_print() {
        let input = r#"
		// This function takes three arguments and prints their sum
		fun f3(a, b, c) { print a + b + c; }
		f3(36, 36, 36);
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }
    #[test]
    fn test_custom_function_with_arg_print() {
        let input = r#"
		// This is a simple function that takes one argument and prints it
		fun f1(a) { print a; }
		f1(43);
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }
    #[test]
    fn test_custom_function_print() {
        let input = r#"
    	// This program should print <fn foo>
		fun foo() {}
		print foo;
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }
    #[test]
    fn test_custom_function() {
        let input = r#"
    	fun bar() { print 10; }
		bar();
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }
    #[test]
    fn test_complex_custom_function() {
        let input = r#"
    	fun cumulative_sum() {
			var n = 10;  // Fixed value
			var total = 0;
			var i = 1;
			while (i <= n) {
				total = total + i;
				i = i + 1;
			}
			print "The cumulative sum from 1 to 10 is: ";
			print total;
		}

		cumulative_sum();
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }
    #[test]
    fn test_function_clock() {
        let input = r#"
    	print clock() + 48;
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }

    #[test]
    fn test_comment() {
        let input = r#"
    	// this is a comment
		print "hello world!";
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }

    // TODO move this elsewhere because parsing is failing
    // #[test]
    // fn test_invalid_for_condition() {
    //     let input = r#"
    // 	 for (var a = 1; {}; a = a + 1) {}
    // 	"#;

    //     let res: Result<Vec<_>, _> = handle_evaluate_internal(input.to_string()).collect();
    //     assert!(res.is_err());
    // }
    // #[test]
    // fn test_invalid_for_inicialization() {
    //     let input = r#"
    // 	 for ({}; a < 2; a = a + 1) {}
    // 	"#;

    //     let _ = handle_evaluate(input.to_string());
    // }
    // #[test]
    // fn test_invalid_for_increment() {
    //     let input = r#"
    // 	for (var a = 1; a < 2; {}) {}
    // 	"#;
    //     let _ = handle_evaluate(input.to_string());
    // }
    #[test]
    fn test_while_operator() {
        let input = r#"
    	var foo = 0;
    	while (foo < 3) print foo = foo + 1;
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }
    #[test]
    fn test_for_operator() {
        let input = r#"
    	for (var foo = 0; foo < 3;) print foo = foo + 1;
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }

    #[test]
    fn test_for_inc_operator() {
        let input = r#"
    	for (var foo = 0; foo < 3; foo = foo + 1) print foo;
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }

    #[test]
    fn test_for_no_declaration() {
        let input = r#"
    	var baz = 0;
		for (; baz < 2; baz = baz + 1) print baz;

		for (var hello = 0; hello < 2;) {
		  print hello;
		  hello = hello + 1;
		}
    	"#;

        let file_contents = String::from(input);
        r#"
    	"#;
        let _ = handle_evaluate(file_contents);
    }

    #[test]
    fn test_and_operator() {
        let input = r#"
		print false and 1;
		print true and 1;
		print 24 and "hello" and false;
			
		print 24 and true;
		print 24 and "hello" and 24;
		"#;

        let file_contents = String::from(input);
        r#"
			false
			1
			false
			true
			24
		"#;
        let _ = handle_evaluate(file_contents);
    }

    #[test]
    fn test_or_operator() {
        let input = r#"
        if (false or "ok") print "foo";

        if (false or false) print "world";
        if (true or "world") print "world";

        if (12 or "bar") print "bar";
        if ("bar" or "bar") print "bar";
		if (nil or "ok") print "foo";
		"#;

        let file_contents = String::from(input);
        let _ = handle_evaluate(file_contents);
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
    #[test_case(
        r#"
    	{
    	  var baz = "outer baz";
    	  var bar = "outer bar";
    	  {
    		baz = "modified baz";
    		var bar = "inner bar";
    		print baz;
    		print bar;
    	  }
    	  print baz;
    	  print bar;
    	}
    	print bar;
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
