#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum BangType {
    BANG,
    BANG_EQUAL,
}

impl BangType {
     pub fn get_lexeme(&self) -> String {
        use BangType::*;
        match self {
            BANG => "!",
            BANG_EQUAL => "!=",
        }
        .into()
    }
}

impl std::fmt::Display for BangType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BangType::*;

        let s = match self {
            BANG => "BANG",
            BANG_EQUAL => "BANG_EQUAL",
        };
        write!(f, "{s}")
    }
}
