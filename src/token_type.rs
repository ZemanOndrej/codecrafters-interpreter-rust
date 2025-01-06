#![allow(clippy::missing_panics_doc)]

use crate::sub_tokens::{BangType, EqualType, GreaterType, LessType, SlashType};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenType {
    // Single-character tokens.
    #[default]
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH(SlashType),
    // COMMENT,
    STAR,

    // One or two character tokens.
    BANG(BangType),
    // BANG_EQUAL,
    EQUAL(EqualType),
    // EQUAL_EQUAL,
    GREATER(GreaterType),
    LESS(LessType),
    // LESS_EQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(String),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseOutput {
    Token(TokenType),
    Invalid(String),
    Partial(TokenType),
}
#[derive(Debug, Clone, PartialEq)]
pub enum PartialParseOutput {
    Token(TokenType),
    Partial(TokenType),
    Mismatched(TokenType),
}

impl TokenType {
    pub fn is_bool_logic_operator(&self) -> bool {
        use TokenType::*;
        matches!(self, AND | OR)
    }

    pub fn is_ignored(&self) -> bool {
        use TokenType::*;
        matches!(self, SLASH(SlashType::COMMENT))
    }

    pub fn get_value(&self) -> String {
        use TokenType::*;
        let value = match self {
            STRING(v) => v.to_string(),
            // TRUE | FALSE | NIL => self.get_lexeme(),
            NUMBER(value) => {
                let value = if value.contains('.') {
                    value.trim_end_matches('0')
                } else {
                    value
                };
                let Ok(number) = value.parse::<f64>() else {
                    panic!("Invalid TokenType number with value {value}.")
                };

                if number.fract() == 0.0 {
                    format!("{number:.1}") // Format with one decimal place
                } else {
                    value.to_string() // Preserve all decimal places
                }
            }
            IDENTIFIER(_v) => {
                "null".to_string()
                // TODO dont print null for
                // if TokenType::is_keyword(v) {
                //     "null".to_string()
                // } else {
                //     v.to_string()
                // }
            }
            _ => "null".to_string(),
        };
        value
    }

    pub fn get_lexeme(&self) -> String {
        use TokenType::*;
        match self {
            LEFT_PAREN => "(".to_string(),
            RIGHT_PAREN => ")".to_string(),
            LEFT_BRACE => "{".to_string(),
            RIGHT_BRACE => "}".to_string(),
            COMMA => ",".to_string(),
            DOT => ".".to_string(),
            MINUS => "-".to_string(),
            PLUS => "+".to_string(),
            SEMICOLON => ";".to_string(),
            SLASH(n) => n.get_lexeme(),
            // COMMENT => "//".to_string(),
            STAR => "*".to_string(),
            BANG(n) => n.get_lexeme(),
            // BANG_EQUAL => "!=".to_string(),
            EQUAL(n) => n.get_lexeme(),
            // EQUAL_EQUAL => "==".to_string(),
            GREATER(n) => n.get_lexeme(),
            LESS(n) => n.get_lexeme(),
            // LESS_EQUAL => "<=".to_string(),
            STRING(v) => format!("\"{v}\""),
            NUMBER(v) | IDENTIFIER(v) => v.to_string(),
            AND => "and".to_string(),
            CLASS => "class".to_string(),
            ELSE => "else".to_string(),
            FALSE => "false".to_string(),
            FUN => "fun".to_string(),
            FOR => "for".to_string(),
            IF => "if".to_string(),
            NIL => "nil".to_string(),
            OR => "or".to_string(),
            PRINT => "print".to_string(),
            RETURN => "return".to_string(),
            SUPER => "super".to_string(),
            THIS => "this".to_string(),
            TRUE => "true".to_string(),
            VAR => "var".to_string(),
            WHILE => "while".to_string(),
            EOF => String::new(),
        }
    }

    pub fn get_precedence(&self) -> i32 {
        use TokenType::*;
        match self {
            STAR | SLASH(SlashType::SLASH) => 1,
            PLUS | MINUS => 2,
            GREATER(GreaterType::GREATER | GreaterType::GREATER_EQUAL)
            | LESS(LessType::LESS | LessType::LESS_EQUAL)
            | EQUAL(EqualType::EQUAL_EQUAL)
            | BANG(BangType::BANG_EQUAL) => 3,
            _ => 4,
        }
    }

    pub fn parse(input: &str) -> ParseOutput {
        use ParseOutput::*;
        use TokenType::*;

        if input.parse::<f64>().is_ok() {
            return Partial(NUMBER(input.to_string()));
        }

        match input {
            " " | "\r" | "\t" => Token(TokenType::EOF),
            "$" | "#" | "@" | "%" => ParseOutput::Invalid(format!("Unexpected character: {input}")),
            "(" => Token(LEFT_PAREN),
            ")" => Token(RIGHT_PAREN),
            "{" => Token(LEFT_BRACE),
            "}" => Token(RIGHT_BRACE),
            "," => Token(COMMA),
            "." => Token(DOT),
            "-" => Token(MINUS),
            "+" => Token(PLUS),
            ";" => Token(SEMICOLON),
            "/" => Partial(SLASH(SlashType::SLASH)),
            "*" => Token(STAR),
            "!" => Partial(BANG(BangType::BANG)),
            // "!=" => Token(BANG(BangType::BANG_EQUAL)),
            "=" => Partial(EQUAL(EqualType::EQUAL)),
            // "==" => Token(EQUAL(EqualType::EQUAL_EQUAL)),
            ">" => Partial(GREATER(GreaterType::GREATER)),
            // ">=" => Token(GREATER(GreaterType::GREATER_EQUAL)),
            "<" => Partial(LESS(LessType::LESS)),
            "\"" => Partial(STRING(String::new())),
            // "<=" => Token(LESS(LessType::LESS_EQUAL)),
            // "STRING" => STRING(input[1..input.len() - 1].into()),
            // "NUMBER" => NUMBER,
            // "AND" => AND,
            // "CLASS" => CLASS,
            // "ELSE" => ELSE,
            // "FALSE" => FALSE,
            // "FUN" => FUN,
            // "FOR" => FOR,
            // "IF" => IF,
            // "NIL" => NIL,
            // "OR" => OR,
            // "PRINT" => PRINT,
            // "RETURN" => RETURN,
            // "SUPER" => SUPER,
            // "THIS" => THIS,
            // "TRUE" => TRUE,
            // "VAR" => VAR,
            // "WHILE" => WHILE,
            _ => Partial(IDENTIFIER(input.into())),
        }
    }

    pub fn parse_partial(input: &str, partial: TokenType) -> PartialParseOutput {
        use PartialParseOutput::*;
        use TokenType::*;
        match input {
            "!=" => Token(BANG(BangType::BANG_EQUAL)),
            "==" => Token(EQUAL(EqualType::EQUAL_EQUAL)),
            ">=" => Token(GREATER(GreaterType::GREATER_EQUAL)),
            "<=" => Token(LESS(LessType::LESS_EQUAL)),
            "//" => Token(SLASH(SlashType::COMMENT)),

            _ => Mismatched(partial),
        }
    }

    pub fn is_keyword(input: &str) -> bool {
        let lowercase = input.to_lowercase();
        matches!(
            lowercase.as_str(),
            "and"
                | "class"
                | "else"
                | "false"
                | "fun"
                | "for"
                | "if"
                | "nil"
                | "or"
                | "print"
                | "return"
                | "super"
                | "this"
                | "true"
                | "var"
                | "while"
        )
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        let s = match self {
            LEFT_PAREN => "LEFT_PAREN",
            RIGHT_PAREN => "RIGHT_PAREN",
            LEFT_BRACE => "LEFT_BRACE",
            RIGHT_BRACE => "RIGHT_BRACE",
            COMMA => "COMMA",
            DOT => "DOT",
            MINUS => "MINUS",
            PLUS => "PLUS",
            SEMICOLON => "SEMICOLON",
            SLASH(n) => return write!(f, "{n}"),
            STAR => "STAR",
            BANG(n) => return write!(f, "{n}"),
            EQUAL(n) => return write!(f, "{n}"),
            GREATER(n) => return write!(f, "{n}"),
            LESS(n) => return write!(f, "{n}"),
            IDENTIFIER(_) => "IDENTIFIER",
            STRING(_) => "STRING",
            NUMBER(_) => "NUMBER",
            AND => "AND",
            CLASS => "CLASS",
            ELSE => "ELSE",
            FALSE => "FALSE",
            FUN => "FUN",
            FOR => "FOR",
            IF => "IF",
            NIL => "NIL",
            OR => "OR",
            PRINT => "PRINT",
            RETURN => "RETURN",
            SUPER => "SUPER",
            THIS => "THIS",
            TRUE => "TRUE",
            VAR => "VAR",
            WHILE => "WHILE",
            EOF => "EOF",
        };
        write!(f, "{s}")
    }
}
