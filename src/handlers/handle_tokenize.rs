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
