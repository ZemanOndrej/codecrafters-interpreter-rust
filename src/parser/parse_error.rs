use core::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Default(String),
    Syntax(String),
}
impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::Default(value)
    }
}
impl From<&str> for ParseError {
    fn from(value: &str) -> Self {
        ParseError::Default(value.to_string())
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Default(value) => write!(f, "{}", value),
            ParseError::Syntax(value) => write!(f, "{}", value),
        }
    }
}
