use crate::{
    token::Token,
    token_type::{ParseOutput, PartialParseOutput, TokenType},
};

pub enum TokenizerResult {
    INVALID(Vec<String>),
    VALID(Vec<String>),
}

pub fn tokenize(i: usize, input: &str) -> TokenizerResult {
    let mut is_error = false;
    let mut result = Vec::new();
    let mut chars = input.chars();
    let mut input = String::new();

    loop {
		// println!("input: {:?}", input);
        if input.len() == 0 {
            let Some(c) = chars.next() else {
                break;
            };
            input.push(c);
        }
        let parse_output = TokenType::parse(&input);
        match parse_output {
            ParseOutput::Invalid(e) => {
                is_error = true;
                eprintln!("[line {}] Error: {}", i + 1, e);
                input.clear();
            }
            ParseOutput::Token(t) => {
                let token = Token::new(t);
                result.push(format!("{}", token.to_string()));
                input.clear();
            }
            ParseOutput::Partial(v) => {
                let next_char = chars.next();
                let token = match next_char {
                    None => {
                        let token = Token::new(v);
						input.clear();
                        token
                    }
                    Some(new_char) => {
                        let parse_output =
                            TokenType::parse_partial(&format!("{}{}", input, new_char), v);
                        let token = match parse_output {
                            PartialParseOutput::Mismatched(token) => {
                                let token = Token::new(token);
                                input = new_char.to_string();
                                token.into()
                            }
                            PartialParseOutput::Token(t) => {
                                let token = Token::new(t);
                                input.clear();
                                token.into()
                            }
                            PartialParseOutput::Partial(_) => {
                                todo!()
                            }
                        };
                        match token {
                            Some(t) => t,
                            None => todo!(),
                        }
                    }
                };
                result.push(format!("{}", token.to_string()));
            }
        }
    }

    if is_error {
        return TokenizerResult::INVALID(result);
    }
    TokenizerResult::VALID(result)
}
