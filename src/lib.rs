use token_type::Token;

pub mod token_type;

pub fn process_file_contents(file_contents: String) {
    let lines = file_contents.lines();
    for line in lines {
        let result = tokenize(line);
        print!("{}", result);
    }
    println!("EOF  null");
}

pub fn tokenize(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        let token = Token::new(c.to_string().as_str());

        let token = format!("{}\n", token.to_string());
        result.push_str(token.as_str());
    }

    result
}
