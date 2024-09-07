use crate::{handle_tokenize::tokenize_file, parse::parse, tokenize::TokenizerResult};
use std::{fs, process::exit};

pub fn handle_parse(filename: &String) {
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    let tokens_per_line = tokenize_file(file_contents);

    if tokens_per_line.iter().any(|r| match r {
        TokenizerResult::INVALID(_) => true,
        _ => false,
    }) {
        exit(65)
    }
    for result in tokens_per_line {
        match result {
            TokenizerResult::VALID(tokens) => parse(tokens),
            _ => {
				panic!("Invalid tokens");
			}
        }
    }
}
