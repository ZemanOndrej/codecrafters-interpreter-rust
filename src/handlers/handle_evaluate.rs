use super::handle_parse::parse;
use crate::evaluation::{Context, EvaluatedExpression};
use std::process::exit;


pub fn handle_evaluate(input: &str) -> Vec<String> {
    handle_evaluate_internal(input)
        .map(|x| match x {
            Ok(x) => x.to_string(),
            Err(e) => {
                dbg!(&e);
                eprintln!("{e}");
                exit(70);
            }
        })
        .collect()
}

pub(super) fn handle_evaluate_internal(
    input: &str,
) -> impl Iterator<Item = Result<EvaluatedExpression, String>> {
    let parsed_input = parse(input);

    let mut context = Context::new_root();

    parsed_input
        .into_iter()
        .map(move |x| x.evaluate(&mut context)?.assert_value())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::evaluation::EvaluatedExpression;

    use super::*;
    use ntest::test_case;

    #[test_case("43 * 9 / 5 + 32;", "109.4")]
    #[test_case("1-(-2)", "3")]
    #[test_case("3+(2) * 2", "7")]
    fn test_handle_evaluate(input: &str, expected: &str) {
        test(input, expected);
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
        test(input, expected);
    }

    fn test(input: &str, expected: &str) {
        let handle_evaluate_internal = handle_evaluate_internal(input)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let result = handle_evaluate_internal.first().unwrap();
        let expected = expected.to_string();
        assert_eq!(result.to_string(), expected);
    }
    #[test_case(" \"foo\" + false")]
    fn test_handle_evaluate_error(input: &str) {
        test_error(input);
    }

    #[test]
    fn test_function_with_return() {
        let input = r"
    	fun foo() { return 10; }
    	print foo();
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_higher_order_function() {
        let input = r#"
    	var globalGreeting = "Hello";

    	fun makeGreeter() {
    		fun greet(name) {
    			print globalGreeting + " " + name;
    		}
    		return greet;
    	}

    	var sayHello = makeGreeter();
    	sayHello("Bob");
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();

        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_nested_higher_order_function() {
        let input = r#"
    	fun returnArg(arg) {
    	  return arg;
    	}

    	fun returnFunCallWithArg(func, arg) {
    	  return returnArg(func)(arg);
    	}

    	fun printArg(arg) {
    	  print arg;
    	}

    	returnFunCallWithArg(printArg, "quz");
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();

        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_filter_higher_order_function() {
        let input = r#"
    	// This program creates a function that returns another function
		// and uses it to filter a list of numbers
		fun makeFilter(min) {
		  fun filter(n) {
		    if (n < min) {
		      return false;
		    }
		    return true;
		  }
		  return filter;
		}

		// This function applies a function to a list of numbers
		fun applyToNumbers(f, count) {
		  var n = 0;
		  while (n < count) {
		    if (f(n)) {
		      print n;
		    }
		    n = n + 1;
		  }
		}

		var greaterThanX = makeFilter(38);
		var greaterThanY = makeFilter(93);

		print "Numbers >= 38:";
		applyToNumbers(greaterThanX, 38 + 6);

		print "Numbers >= 93:";
		applyToNumbers(greaterThanY, 93 + 6);
		
	
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();

        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_global_scope_change() {
        let input = r"
    	var count = 2;

		fun tick() {
			count = count - 1;
			print count;
		}
		tick();
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();

        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_global_scope() {
        let input = r#"
    	var count = 3;

		fun tick() {
			if (count > 0) {
				print count;
				count = count - 1;
				return false;
			}
			print "Blast off!";
			return true;
		}

		while (!tick()) {}
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();

        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_return_function_with_closure() {
        let input = r"
    	fun outer(i) {
			var x = 1;
			fun inner(){
				return x+i;
			
			}
			x = 2;
    		return inner;
    	}
    	print outer(1)();
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_recursive_function_with_return() {
        let input = r"
    	// This program computes the 35th Fibonacci number
    	fun mult(x, n) {

    	  if (n == 0) {
    	  	return 1;
    	  }

    	  return mult(x, n - 1) * x;
    	}

    	print mult(5, 2);
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_function_with_no_return() {
        let input = r#"
		// This program uses a return statement inside a while loop to return "ok" if the condition is false
		fun f() {
		while (!false) return "ok";
		}

		print f();
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_function_with_return_nil() {
        let input = r#"
		 // This program relies on the return statement returning nil by default
		fun f() {
			return;
			print "bad";
		}

		print f();
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_fibonacii_function_with_return() {
        let input = r"
    	// This program computes the 35th Fibonacci number
    	fun fib(n) {
    	  if (n < 2) return n;
    	  return fib(n - 2) + fib(n - 1);
    	}

    	var start = clock();
    	print fib(10) == 55;
    	print (clock() - start) < 5; // 5 seconds
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_custom_function_with_args_print() {
        let input = r"
		// This function takes three arguments and prints their sum
		fun f3(a, b, c) { print a + b + c; }
		f3(36, 36, 36);
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_custom_function_with_arg_print() {
        let input = r"
		// This is a simple function that takes one argument and prints it
		fun f1(a) { print a; }
		f1(43);
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_custom_function_print() {
        let input = r"
    	// This program should print <fn foo>
		fun foo() {}
		print foo;
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_custom_function() {
        let input = r"
    	fun bar() { print 10; }
		bar();
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
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

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_function_clock() {
        let input = r"
    	print clock() + 48;
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_comment() {
        let input = r#"
    	// this is a comment
		print "hello world!";
    	"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    // TODO move this elsewhere because parsing is failing
    // #[test]
    // fn test_invalid_for_condition() {
    //     let input = r#"
    // 	 for (var a = 1; {}; a = a + 1) {}
    // 	"#;

    //     let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
    //     assert!(res.is_err());
    // }
    // #[test]
    // fn test_invalid_for_inicialization() {
    //     let input = r#"
    // 	 for ({}; a < 2; a = a + 1) {}
    // 	"#;

    //     let _ = handle_evaluate(input);
    // }
    // #[test]
    // fn test_invalid_for_increment() {
    //     let input = r#"
    // 	for (var a = 1; a < 2; {}) {}
    // 	"#;
    //     let _ = handle_evaluate(input);
    // }
    #[test]
    fn test_while_operator() {
        let input = r"
    	var foo = 0;
    	while (foo < 3) print foo = foo + 1;
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test]
    fn test_for_operator() {
        let input = r"
    	for (var foo = 0; foo < 3;) print foo = foo + 1;
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_for_inc_operator() {
        let input = r"
    	for (var foo = 0; foo < 3; foo = foo + 1) print foo;
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_for_no_declaration() {
        let input = r"
    	var baz = 0;
		for (; baz < 2; baz = baz + 1) print baz;

		for (var hello = 0; hello < 2;) {
		  print hello;
		  hello = hello + 1;
		}
    	";

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
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

        let _ = r"
			false
			1
			false
			true
			24
		";
        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
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

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }

    #[test]
    fn test_if_else_operator() {
        let input = r#"
		// This program uses a return statement inside an if statement to return "ok" if the condition is false
		fun f() {
		  if (true) return "no"; else return "ok";
		}

		print f();
		"#;

        let res: Result<Vec<_>, _> = handle_evaluate_internal(input).collect();
        res.unwrap_or_else(|e| {
            dbg!(e);
            panic!("Error");
        });
    }
    #[test_case(" \"foo\" + false")]
    #[test_case(" false / false")]
    #[test_case(" \"bar\" / 47")]
    #[test_case("14 * \"bar\"")]
    #[test_case(r"print a;")]
    #[test_case(
        r"
    	var quz;
    	quz = 1;
    	print quz;
    	print quz = 2;
    	print quz;
    "
    )]
    #[test_case(
        r"
    	var world = 21;
    	var result = (world + bar) / foo;
    	print result;
    "
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
    #[test_case(
        r"
		fun f(a, b) {}
		f(1); // expect runtime error: Expected 2 arguments but got 1.
	"
    )]
    fn test_all_handle_evaluate_error(input: &str) {
        test_error(input);
    }

    fn test_error(input: &str) {
        let res = parse(input);
        let res: Result<Vec<EvaluatedExpression>, String> = res
            .iter()
            .map(|x| x.evaluate(&mut Rc::default())?.assert_value())
            .collect();
        assert!(res.is_err());
    }
}
