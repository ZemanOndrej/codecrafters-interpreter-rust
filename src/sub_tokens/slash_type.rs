
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

impl ToString for SlashType {
    fn to_string(&self) -> String {
        use SlashType::*;

        match self {
            SLASH => "SLASH",
            COMMENT => "COMMENT",
        }
        .into()
    }
}
