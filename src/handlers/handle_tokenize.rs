use crate::{
    token::Token,
    token_type::TokenType,
    tokenize::{tokenize, TokenizerResult},
};
use std::process::exit;

pub fn handle_tokenize(input: String) {
    let tokens_per_line = tokenize_multiline(input);

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

pub fn tokenize_multiline(file_contents: String) -> Vec<TokenizerResult> {
    let lines = file_contents.lines();
    let mut results: Vec<_> = lines
        .enumerate()
        .map(|(i, line)| tokenize(i, line))
        .collect();

    results.push(TokenizerResult::VALID(vec![Token::new(TokenType::EOF)]));

    results
}
