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
        }.into()
    }
}

impl ToString for GreaterType {
	fn to_string(&self) -> String {
        use GreaterType::*;

		match self {
			GREATER => "GREATER",
			GREATER_EQUAL => "GREATER_EQUAL",
			
		}.into()

	}
}