use std::str::FromStr;

#[derive(Debug)]
pub enum Operation {
    Tokenize,
    Parse,
    Evaluate,
    Run,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tokenize => write!(f, "tokenize"),
            Self::Parse => write!(f, "parse"),
            Self::Evaluate => write!(f, "evaluate"),
            Self::Run => write!(f, "run"),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matched = match s.to_lowercase().as_str() {
            "tokenize" => Some(Self::Tokenize),
            "parse" => Some(Self::Parse),
            "evaluate" => Some(Self::Evaluate),
            "run" => Some(Self::Run),
            _ => None,
        };
        match matched {
            Some(op) => Ok(op),
            None => Err(()),
        }
    }
}

impl From<String> for Operation {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}
