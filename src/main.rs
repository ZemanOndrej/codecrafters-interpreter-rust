#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]
use codecrafters_interpreter::handlers::{
    handle_evaluate, handle_parse, handle_run, handle_tokenize, Operation,
};
use std::{env, fs, str::FromStr};

fn main() {
    use Operation::*;
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let Ok(command) = Operation::from_str(args[1].as_str()) else {
        eprintln!("Unknown command: {}", args[1]);
        return;
    };
    let filename = &args[2];

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {filename}");
        String::new()
    });
    match command {
        Parse => {
            let result = handle_parse(file_contents);
            for expr in &result {
                println!("{expr}");
            }
        }
        Tokenize => handle_tokenize(file_contents),
        Evaluate => {
            let result = handle_evaluate(file_contents);
            for expr in &result {
                println!("{expr}");
            }
        }
        Run => {
            handle_run(file_contents);
        }
    }
}
