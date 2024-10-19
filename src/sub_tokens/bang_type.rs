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

impl ToString for BangType {
    fn to_string(&self) -> String {
        use BangType::*;

        match self {
            BANG => "BANG",
            BANG_EQUAL => "BANG_EQUAL",
        }
        .into()
    }
}
