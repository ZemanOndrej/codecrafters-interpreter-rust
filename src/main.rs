use codecrafters_interpreter::operation::Operation;
use codecrafters_interpreter::{handle_parse, handle_tokenize};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let Some(command) = Operation::from_str(args[1].as_str()) else {
        eprintln!("Unknown command: {}", args[1]);
        return;
    };
    let filename = &args[2];

    use Operation::*;
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command {
        Parse => {
            handle_parse(file_contents);
            ()
        }
        Tokenize => handle_tokenize(file_contents),
    }
}
