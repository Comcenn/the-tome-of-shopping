use std::fmt;

#[derive(Debug)]
pub struct TomeError {
    details: String,
}

impl TomeError {
    pub fn new(details: impl Into<String>) -> Self {
        Self {
            details: details.into(),
        }
    }
}

impl fmt::Display for TomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TomeError: {}", self.details)
    }
}

impl std::error::Error for TomeError {}
