use crate::sub_tokens::*;

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
    pub fn is_ignored(&self) -> bool {
        use TokenType::*;
        match self {
            SLASH(SlashType::COMMENT) => true,
            _ => false,
        }
    }

    pub fn get_value(&self) -> String {
        use TokenType::*;
        let value = match self {
            STRING(v) => v.to_string(),
            // TRUE | FALSE | NIL => self.get_lexeme(),
            NUMBER(value) => {
                let value = if value.contains('.') {
                    value.trim_end_matches("0")
                } else {
                    value
                };
                let Ok(number) = value.parse::<f64>() else {
                    panic!("Invalid TokenType number with value {}.", value)
                };

                if number.fract() == 0.0 {
                    format!("{:.1}", number) // Format with one decimal place
                } else {
                    value.to_string() // Preserve all decimal places
                }
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
            STRING(v) => format!("\"{}\"", v),
            NUMBER(v) => v.to_string(),
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
            EOF => "".to_string(),
            IDENTIFIER(v) => v.to_string(),
        }
    }
    pub fn get_precedence(&self) -> i32 {
        use TokenType::*;
        match self {
            STAR | SLASH(SlashType::SLASH) => 1,
            PLUS | MINUS => 2,
            GREATER(GreaterType::GREATER)
            | GREATER(GreaterType::GREATER_EQUAL)
            | LESS(LessType::LESS)
            | LESS(LessType::LESS_EQUAL)
            | EQUAL(EqualType::EQUAL_EQUAL)
            | BANG(BangType::BANG_EQUAL) => 3,
            _ => 4,
        }
    }

    pub fn parse(input: &str) -> ParseOutput {
        use ParseOutput::*;
        use TokenType::*;

        if let Ok(_) = input.parse::<f64>() {
            return Partial(NUMBER(input.to_string()));
        }

        match input {
            " " | "\r" | "\t" => return Token(TokenType::EOF),
            "$" | "#" | "@" | "%" => {
                return ParseOutput::Invalid(format!("Unexpected character: {}", input))
            }
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
            "\"" => Partial(STRING("".into())),
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
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        use TokenType::*;
        match self {
            LEFT_PAREN => "LEFT_PAREN".to_string(),
            RIGHT_PAREN => "RIGHT_PAREN".to_string(),
            LEFT_BRACE => "LEFT_BRACE".to_string(),
            RIGHT_BRACE => "RIGHT_BRACE".to_string(),
            COMMA => "COMMA".to_string(),
            DOT => "DOT".to_string(),
            MINUS => "MINUS".to_string(),
            PLUS => "PLUS".to_string(),
            SEMICOLON => "SEMICOLON".to_string(),
            SLASH(n) => n.to_string(),
            STAR => "STAR".to_string(),
            BANG(n) => n.to_string(),
            // BANG_EQUAL => "BANG_EQUAL".to_string(),
            EQUAL(n) => n.to_string(),
            // EQUAL_EQUAL => "EQUAL_EQUAL".to_string(),
            GREATER(n) => n.to_string(),
            LESS(n) => n.to_string(),
            // LESS_EQUAL => "LESS_EQUAL".to_string(),
            IDENTIFIER(_) => "IDENTIFIER".to_string(),
            STRING(_) => "STRING".to_string(),
            NUMBER(_) => "NUMBER".to_string(),
            AND => "AND".to_string(),
            CLASS => "CLASS".to_string(),
            ELSE => "ELSE".to_string(),
            FALSE => "FALSE".to_string(),
            FUN => "FUN".to_string(),
            FOR => "FOR".to_string(),
            IF => "IF".to_string(),
            NIL => "NIL".to_string(),
            OR => "OR".to_string(),
            PRINT => "PRINT".to_string(),
            RETURN => "RETURN".to_string(),
            SUPER => "SUPER".to_string(),
            THIS => "THIS".to_string(),
            TRUE => "TRUE".to_string(),
            VAR => "VAR".to_string(),
            WHILE => "WHILE".to_string(),
            EOF => "EOF".to_string(),
        }
    }
}
