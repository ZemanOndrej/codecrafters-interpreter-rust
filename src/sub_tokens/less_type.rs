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

impl std::fmt::Display for LessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LessType::*;

        let s = match self {
            LESS => "LESS",
            LESS_EQUAL => "LESS_EQUAL",
        };

        write!(f, "{s}")
    }
}
