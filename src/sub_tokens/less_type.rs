#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum LessType {
    LESS,
    LESS_EQUAL,
}

impl LessType {
    pub fn get_lexeme(&self) -> String {
        use LessType::*;
        match self {
            LESS => "<",
            LESS_EQUAL => "<=",
        }
        .into()
    }
}

impl ToString for LessType {
    fn to_string(&self) -> String {
        use LessType::*;

        match self {
            LESS => "LESS",
            LESS_EQUAL => "LESS_EQUAL",
        }
        .into()
    }
}
