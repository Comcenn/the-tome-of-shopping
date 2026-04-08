use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct UserId(pub String);

impl From<String> for UserId {
    fn from(s: String) -> Self {
        UserId(s)
    }
}

impl From<&str> for UserId {
    fn from(s: &str) -> Self {
        UserId(s.to_string())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct UserContext {
    pub user: UserId,
    pub password: String,
}

impl UserContext {
    pub fn new(user: UserId, password: String) -> Self {
        Self { user, password }
    }
}
