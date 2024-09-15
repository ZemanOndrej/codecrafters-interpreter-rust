use super::handle_parse::parse;

pub fn handle_evaluate(input: String) {
    let parsed_input = parse(input);

    for line in parsed_input {
        for e in line {
            println!("{}", e.to_string());
        }
    }
}
