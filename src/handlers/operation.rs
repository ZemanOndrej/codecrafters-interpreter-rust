#[derive(Debug)]
pub enum Operation {
    Tokenize,
    Parse,
    Evaluate,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tokenize => write!(f, "tokenize"),
            Self::Parse => write!(f, "parse"),
            Self::Evaluate => write!(f, "evaluate"),
        }
    }
}

impl Operation {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "tokenize" => Some(Self::Tokenize),
            "parse" => Some(Self::Parse),
            "evaluate" => Some(Self::Evaluate),
            _ => None,
        }
    }
}

impl From<String> for Operation {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}