pub mod token;
pub mod token_type;

use std::process::exit;
use token::Token;

pub fn process_file_contents(file_contents: String) {
    let lines = file_contents.lines();
    let results: Vec<_> = lines
        .enumerate()
        .map(|(i, line)| {
            let result = tokenize(i, line);
            match &result {
                TokenizerResult::VALID(tokens) => print_output(tokens),
                TokenizerResult::INVALID(tokens) => print_output(tokens),
            };
            result
        })
        .collect();

    println!("EOF  null");

    if results.iter().any(|r| match r {
        TokenizerResult::INVALID(_) => true,
        _ => false,
    }) {
        exit(65)
    }
}

fn print_output(tokens: &[String]) {
    for token in tokens {
        println!("{}", token)
    }
}

pub enum TokenizerResult {
    INVALID(Vec<String>),
    VALID(Vec<String>),
}

pub fn tokenize(i: usize, input: &str) -> TokenizerResult {
    let mut is_error = false;
    let mut result = Vec::new();
    for c in input.chars() {
        let token = Token::new(c.to_string().as_str());
        match token {
            Ok(t) => {
                result.push(format!("{}", t.to_string()));
            }
            Err(e) => {
                is_error = true;
                eprintln!("[line {}] Error: {}", i + 1, e);
            }
        }
    }
    if is_error {
        return TokenizerResult::INVALID(result);
    }
    TokenizerResult::VALID(result)
}
