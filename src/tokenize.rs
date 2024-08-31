use std::str::Chars;

use crate::{
    sub_tokens::SlashType,
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
        if input.len() == 0 {
            let Some(c) = find_first_non_whitespace(&mut chars) else {
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
                if token.token_type == TokenType::SLASH(SlashType::COMMENT) {
                    chars = "".chars();
                }
                if !token.token_type.is_ignored() {
                    result.push(format!("{}", token.to_string()));
                }
            }
        }
    }

    if is_error {
        return TokenizerResult::INVALID(result);
    }
    TokenizerResult::VALID(result)
}

fn find_first_non_whitespace(chars: &mut Chars<'_>) -> Option<char> {
    while let Some(c) = chars.next() {
        if !c.is_whitespace() {
            return Some(c);
        }
    }
    None
}
