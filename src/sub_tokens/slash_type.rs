#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum SlashType {
    SLASH,
    COMMENT,
}

impl SlashType {
     pub fn get_lexeme(&self) -> String {
        use SlashType::*;
        match self {
            SLASH => "/",
            COMMENT => "//",
        }
        .into()
    }
}

impl std::fmt::Display for SlashType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SlashType::*;

        let s = match self {
            SLASH => "SLASH",
            COMMENT => "COMMENT",
        };

        write!(f, "{s}")
    }
}
