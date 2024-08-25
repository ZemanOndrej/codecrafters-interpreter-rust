use token_type::Token;

pub mod token_type;

pub fn process_file_contents(file_contents: String) {
    let lines = file_contents.lines();
    for (i, line) in lines.enumerate() {
        let result = tokenize(i, line);
        print!("{}", result);
    }
    println!("EOF  null");
}

pub fn tokenize(i: usize, input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        let token = Token::new(c.to_string().as_str());
        match token {
            Ok(t) => {
                let token = format!("{}\n", t.to_string());
                result.push_str(token.as_str());
            }
            Err(e) => {
                println!("[line {}] Error: {}", i, e);
                continue;
            }
        }
    }

    result
}
