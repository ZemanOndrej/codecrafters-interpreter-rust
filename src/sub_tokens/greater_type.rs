#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum GreaterType {
    GREATER,
    GREATER_EQUAL,
}

impl GreaterType {
     pub fn get_lexeme(&self) -> String {
        use GreaterType::*;
        match self {
            GREATER => ">",
            GREATER_EQUAL => ">=",
        }
        .into()
    }
}

impl std::fmt::Display for GreaterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GreaterType::*;

        let s = match self {
            GREATER => "GREATER",
            GREATER_EQUAL => "GREATER_EQUAL",
        };
        write!(f, "{s}")
    }
}
