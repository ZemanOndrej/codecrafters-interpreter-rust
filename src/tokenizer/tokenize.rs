use super::{
    handle_identifier, handle_keyword, handle_number_literal, handle_string_literal, TokenError,
};
use crate::{
    sub_tokens::SlashType,
    token::Token,
    token_type::{ParseOutput, PartialParseOutput, TokenType},
};
use std::{iter::Peekable, str::Chars};


pub fn tokenize(input: &str) -> Vec<Result<Token, TokenError>> {
    let mut line_index = 0;
    let mut results = Vec::new();
    let mut chars = input.chars().peekable();

    loop {
        let result = process_chars(&mut chars, &mut line_index);
        if matches!(&result, Ok(token) if token.token_type == TokenType::EOF) {
            results.push(result);
            break;
        }
        results.push(result);
    }
    results
}

fn process_chars(
    chars: &mut Peekable<Chars<'_>>,
    line_index: &mut usize,
) -> Result<Token, TokenError> {
    let mut state = String::new();

    if state.is_empty() {
        let Some(c) = find_first_non_whitespace(chars, line_index) else {
            return Ok(Token::new(TokenType::EOF, *line_index));
        };
        state.push(c);
    }
    let parse_output = TokenType::parse(&state);
    match parse_output {
        ParseOutput::Invalid(e) => {
            eprintln!("[line {}] Error: {}", *line_index + 1, e);
            Err(TokenError {
                line_index: *line_index,
                message: e,
            })
        }
        ParseOutput::Token(t) => {
            let token = Token::new(t, *line_index);
            Ok(token)
        }
        ParseOutput::Partial(v) => {
            let token = match v {
                TokenType::IDENTIFIER(v) => handle_identifier(v, chars, &mut state, *line_index),
                TokenType::STRING(v) => handle_string_literal(v, chars, &mut state, *line_index),
                TokenType::NUMBER(v) => {
                    handle_number_literal(v.as_str(), chars, &mut state, *line_index)
                }
                _ => {
                    let next_char = chars.peek();

                    let result = match next_char {
                        None => {
                            let token = Token::new(v, *line_index);
                            state.clear();
                            token
                        }
                        Some(new_char) => {
                            let parse_output =
                                TokenType::parse_partial(&format!("{state}{new_char}"), v);
                            let token = match parse_output {
                                PartialParseOutput::Mismatched(token) => {
                                    let token = Token::new(token, *line_index);

                                    token.into()
                                }
                                PartialParseOutput::Token(t) => {
                                    let token = Token::new(t, *line_index);
                                    chars.next();
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
                    Ok(result)
                }
            };
            match token {
                Err(e) => {
                    eprintln!("[line {}] Error: {}", *line_index + 1, e);
                    go_to_next_line(chars, line_index);
                    Err(TokenError {
                        line_index: *line_index,
                        message: e,
                    })
                }
                Ok(token) => {
                    let token = match token.token_type {
                        TokenType::SLASH(SlashType::COMMENT) => {
                            go_to_next_line(chars, line_index);
                            token
                        }
                        TokenType::IDENTIFIER(_) => handle_keyword(token, *line_index),
                        _ => token,
                    };

                    Ok(token)
                }
            }
        }
    }

    // (is_error, result)
}
fn go_to_next_line(chars: &mut Peekable<Chars<'_>>, line_index: &mut usize) {
    for c in chars.by_ref() {
        if c == '\n' {
            *line_index += 1;
            break;
        }
    }
}

fn find_first_non_whitespace(
    chars: &mut Peekable<Chars<'_>>,
    line_count: &mut usize,
) -> Option<char> {
    for c in chars.by_ref() {
        if c == '\n' {
            *line_count += 1;
        }

        if !c.is_whitespace() {
            return Some(c);
        }
    }
    None
}
