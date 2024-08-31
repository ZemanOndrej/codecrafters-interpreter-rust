use std::process::exit;
use crate::tokenize::{tokenize, TokenizerResult};

pub fn process_file_content(file_contents: String) {
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
