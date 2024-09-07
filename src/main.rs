use codecrafters_interpreter::operation::Operation;
use codecrafters_interpreter::{handle_parse, handle_tokenize};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = Operation::from_str(args[1].as_str()).unwrap();
    let filename = &args[2];

    use Operation::*;
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");
    match command {
        Parse => handle_parse(filename),
        Tokenize => handle_tokenize(filename),
        _ => {
            eprintln!("Unknown command: {}", command);
            return;
        }
    }
}
