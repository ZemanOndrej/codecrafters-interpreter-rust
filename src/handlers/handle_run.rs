use super::handle_evaluate;

pub fn handle_run(input: String) {
    handle_evaluate(input);
}

#[cfg(test)]
mod tests {
    use crate::handlers::{handle_evaluate, handle_run};
    use ntest::test_case;

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
    fn test_scope(input: &str) {
        let _ = handle_run(input.to_string());
    }
    #[test_case(
        r#"
		{
		  var baz = "outer baz";
		  {
		  	print baz;
		    baz = "modified baz";
		    print baz;
		  }
		  print baz;
		}
    	"#
    )]
    #[test_case(
        r#"
		{
			var quz = "before";
			{
				var quz = "after";
				print quz;
			}
			print quz;
		}
		"#
    )]
    #[test_case(
        r#"
		{
			var quz = 29;
			{
				var foo = 29;
				print foo;
			}
			print quz;
		}
    	"#
    )]
    #[test_case(
        r#"
		{
			var quz = "world";
			print quz;
		}
		"#
    )]
    #[test_case(
        r#"
    		var quz = "world";
    		print quz;
    	"#
    )]
    #[test_case(
        r#"
    	var a;
    	var b;
    	a = b = 89;
    	"#
    )]
    fn test_handle_run(input: &str) {
        let _ = handle_run(input.to_string());
    }

    #[test_case("28 - 84 * 67 - 54", "-5654")]
    fn test_handle_run(input: &str, expected: &str) {
        let result = handle_evaluate(input.to_string());
        let result = result.first().unwrap().as_str();
        assert_eq!(result, expected);
    }
    #[test_case(
        r#"
    	var a;
    	a = 2;
    	print a;"#
    )]
    #[test_case(
        r#"
    	var a;
    	var b = 2;
    	var a = b = 1;
    	print a;
    "#
    )]

    fn test_handle_run(input: &str) {
        let _ = handle_run(input.to_string());
    }
    #[test_case("var a;")]
    #[test_case(
        r#"
    	var b;
    	var a = b = "foo";
    	print a;"#
    )]
    fn test_handle_run(input: &str) {
        let _ = handle_run(input.to_string());
    }

    #[test_case(
        r#"
		var bar;
		print bar;
	"#
    )]
    #[test_case(
        r#"
    	var bar = 49;
    	var hello = 49;
    	print bar + hello;
    	var quz = 49;
    	print bar + hello + quz;
    "#
    )]
    #[test_case(
        r#"
    	var a = "foo";
    	print a;"#
    )]
    #[test_case("1+2*3;")]
    #[test_case(r#"1 >= -2 + 3 ;"#)]
    #[test_case(r#"1 >= 1 * 2 ;"#)]
    #[test_case(
        r#"
    (27 + 25 - 46) > (43 - 27) * 2;
    print !true;
    "quz" + "hello" + "baz" + "foo" == "quzhellobazfoo";
    print !true;
    "#
    )]
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
    // TODO
    // #[test_case(
    //     r#"
    // {
    // // This is a complex test case
    // str1 = "Test"
    // str2 = "Case"
    // num1 = 100
    // num2 = 200.00
    // result = (str1 == str2) != ((num1 + num2) >= 300)
    // }
    // "#
    // )]
    // fn test_all_handle_evaluate(input: &str) {
    //     let _ = handle_run(input.to_string());
    // }

    // fn test(input: &str, expected: &str) {
    //     let result = handle_evaluate(input.to_string());
    //     let expected = vec![expected.to_string()];
    //     dbg!(result.clone());
    //     dbg!(expected.clone());
    //     assert_eq!(result, expected);
    // }
}
