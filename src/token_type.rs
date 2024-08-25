use anyhow::bail;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER,

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

impl TokenType {
    pub fn get_value(&self) -> String {
        use TokenType::*;
        let value = match self {
            STRING(v) => v.to_string(),
            _ => "null".to_string(),
        };
        value
    }
    pub fn get_lexeme(&self) -> String {
        use TokenType::*;
        match self {
            LEFT_PAREN => "(",
            RIGHT_PAREN => ")",
            LEFT_BRACE => "{",
            RIGHT_BRACE => "}",
            COMMA => ",",
            DOT => ".",
            MINUS => "-",
            PLUS => "+",
            SEMICOLON => ";",
            SLASH => "/",
            STAR => "*",
            BANG => "!",
            BANG_EQUAL => "!=",
            EQUAL => "=",
            EQUAL_EQUAL => "==",
            GREATER => ">",
            GREATER_EQUAL => ">=",
            LESS => "<",
            LESS_EQUAL => "<=",
            STRING(_) => "STRING",
            NUMBER => "NUMBER",
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
            IDENTIFIER(v) => v,
        }
        .into()
    }
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        use TokenType::*;
        let res = match input {
            "(" => LEFT_PAREN,
            ")" => RIGHT_PAREN,
            "{" => LEFT_BRACE,
            "}" => RIGHT_BRACE,
            "," => COMMA,
            "." => DOT,
            "-" => MINUS,
            "+" => PLUS,
            ";" => SEMICOLON,
            "/" => SLASH,
            "*" => STAR,
            "!" => BANG,
            "!=" => BANG_EQUAL,
            "=" => EQUAL,
            "==" => EQUAL_EQUAL,
            ">" => GREATER,
            ">=" => GREATER_EQUAL,
            "<" => LESS,
            "<=" => LESS_EQUAL,
            "STRING" => STRING(input[1..input.len() - 1].into()),
            "NUMBER" => NUMBER,
            "AND" => AND,
            "CLASS" => CLASS,
            "ELSE" => ELSE,
            "FALSE" => FALSE,
            "FUN" => FUN,
            "FOR" => FOR,
            "IF" => IF,
            "NIL" => NIL,
            "OR" => OR,
            "PRINT" => PRINT,
            "RETURN" => RETURN,
            "SUPER" => SUPER,
            "THIS" => THIS,
            "TRUE" => TRUE,
            "VAR" => VAR,
            "WHILE" => WHILE,
            "EOF" => EOF,
            "$" | "#" | "@" | "%" => bail!("Unexpected character: {}", input),
            _ => IDENTIFIER(input.into()),
        };
        Ok(res)
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        use TokenType::*;
        match self {
            LEFT_PAREN => "LEFT_PAREN",
            RIGHT_PAREN => "RIGHT_PAREN",
            LEFT_BRACE => "LEFT_BRACE",
            RIGHT_BRACE => "RIGHT_BRACE",
            COMMA => "COMMA",
            DOT => "DOT",
            MINUS => "MINUS",
            PLUS => "PLUS",
            SEMICOLON => "SEMICOLON",
            SLASH => "SLASH",
            STAR => "STAR",
            BANG => "BANG",
            BANG_EQUAL => "BANG_EQUAL",
            EQUAL => "EQUAL",
            EQUAL_EQUAL => "EQUAL_EQUAL",
            GREATER => "GREATER",
            GREATER_EQUAL => "GREATER_EQUAL",
            LESS => "LESS",
            LESS_EQUAL => "LESS_EQUAL",
            IDENTIFIER(_) => "IDENTIFIER",
            STRING(_) => "STRING",
            NUMBER => "NUMBER",
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
        }
        .into()
    }
}
