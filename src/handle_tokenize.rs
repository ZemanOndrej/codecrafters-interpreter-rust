use crate::{
    token::Token,
    token_type::TokenType,
    tokenize::{tokenize, TokenizerResult},
};
use std::{fs, process::exit};

pub fn handle_tokenize(filename: &String) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let tokens_per_line = tokenize_file(file_contents);

    for result in tokens_per_line.iter() {
        match result {
            TokenizerResult::VALID(tokens) => print_output(tokens),
            TokenizerResult::INVALID(tokens) => print_output(tokens),
        }
    }

    if tokens_per_line.iter().any(|r| match r {
        TokenizerResult::INVALID(_) => true,
        _ => false,
    }) {
        exit(65)
    }
}

fn print_output(tokens: &[Token]) {
    for token in tokens {
        println!("{}", token.to_string())
    }
}

pub fn tokenize_file(file_contents: String) -> Vec<TokenizerResult> {
    let lines = file_contents.lines();
    let mut results: Vec<_> = lines
        .enumerate()
        .map(|(i, line)| tokenize(i, line))
        .collect();

    results.push(TokenizerResult::VALID(vec![Token::new(TokenType::EOF)]));

    results
}
