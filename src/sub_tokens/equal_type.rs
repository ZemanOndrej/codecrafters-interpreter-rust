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

impl ToString for EqualType {
    fn to_string(&self) -> String {
        use EqualType::*;

        match self {
            EQUAL => "EQUAL",
            EQUAL_EQUAL => "EQUAL_EQUAL",
        }
        .into()
    }
}
