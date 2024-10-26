pub struct TokenError {
    pub line_index: usize,
    pub message: String,
}

impl std::fmt::Debug for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error: {}", self.line_index + 1, self.message)
    }
}
