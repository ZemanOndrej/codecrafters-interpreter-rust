#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum EqualType {
    EQUAL,
    EQUAL_EQUAL,
}

impl EqualType {
    pub fn get_lexeme(&self) -> String {
        use EqualType::*;
        match self {
            EQUAL => "=",
            EQUAL_EQUAL => "==",
        }
        .into()
    }
}

impl std::fmt::Display for EqualType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EqualType::*;

        let s = match self {
            EQUAL => "EQUAL",
            EQUAL_EQUAL => "EQUAL_EQUAL",
        };
        write!(f, "{}", s)
    }
}
