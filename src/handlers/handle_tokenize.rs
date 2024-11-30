use crate::{sub_tokens::SlashType, token_type::TokenType, tokenize::tokenize};
use std::process::exit;

pub fn handle_tokenize(input: String) {
    let tokens = tokenize(input.as_str());

    let mut has_error = false;
    for result in tokens.iter() {
        match result {
            Ok(token) => {
                if token.token_type == TokenType::SLASH(SlashType::COMMENT) {
                    continue;
                }
                println!("{}", token.to_string());
            }
            Err(e) => {
                eprintln!("{}", e.message);
                has_error = true;
            }
        }
    }
    if has_error {
        exit(65);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_multiple_identifiers() {
        let input = r#"
		nil 
		IF 
		WHILE
		false
		THIS
		return
		OR
		else
		class
		if
		VAR
		RETURN
		AND
		SUPER
		while
		print
		this
		CLASS
		fun
		PRINT
		super
		FOR
		true
		ELSE
		NIL
		for
		and
		var
		TRUE
		FALSE
		FUN
		or
		"#;

        let expected = vec![
            "NIL nil null",
            "IDENTIFIER IF null",
            "IDENTIFIER WHILE null",
            "FALSE false null",
            "IDENTIFIER THIS null",
            "RETURN return null",
            "IDENTIFIER OR null",
            "ELSE else null",
            "CLASS class null",
            "IF if null",
            "IDENTIFIER VAR null",
            "IDENTIFIER RETURN null",
            "IDENTIFIER AND null",
            "IDENTIFIER SUPER null",
            "WHILE while null",
            "PRINT print null",
            "THIS this null",
            "IDENTIFIER CLASS null",
            "FUN fun null",
            "IDENTIFIER PRINT null",
            "SUPER super null",
            "IDENTIFIER FOR null",
            "TRUE true null",
            "IDENTIFIER ELSE null",
            "IDENTIFIER NIL null",
            "FOR for null",
            "AND and null",
            "VAR var null",
            "IDENTIFIER TRUE null",
            "IDENTIFIER FALSE null",
            "IDENTIFIER FUN null",
            "OR or null",
            "EOF  null",
        ];
        let file_contents = String::from(input);
        let result = tokenize(file_contents.as_str());
        let result = result
            .iter()
            .map(|x| x.as_ref().unwrap().to_string())
            .collect::<Vec<String>>();
        assert_eq!(result, expected);
    }
    #[test]
    fn test_handle_tokenize() {
        let input = "\"world\" \"unterminated";
        let result = tokenize(input);
        dbg!(&result);

        assert_eq!(result.len(), 3);
        let first = result.get(0).unwrap().as_ref().unwrap();
        let err = result.get(1).unwrap().as_ref();
        let second = result.get(2).unwrap().as_ref().unwrap();
        assert!(matches!(first.token_type, TokenType::STRING(_)));
        assert!(err.is_err());
        assert!(matches!(second.token_type, TokenType::EOF));
    }

    #[test]
    fn test_handle_tokenize_with_error() {
        let input = "$\t@%";
        let result = tokenize(input);
        dbg!(&result);
    }
}
