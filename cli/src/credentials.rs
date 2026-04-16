use serde::{Deserialize, Serialize};
use shared::user::UserId;
use std::io::{self, Write};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub user: UserId,
    pub password: String,
}

impl Credentials {
    pub fn load() -> anyhow::Result<Option<Self>> {
        let path = Self::path();

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)?;
        let creds: Credentials = toml::from_str(&content)?;
        Ok(Some(creds))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::path();
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, toml::to_string(self)?)?;
        Ok(())
    }

    fn path() -> PathBuf {
        dirs::home_dir()
            .unwrap()
            .join(".shopping-cli")
            .join("credentials.toml")
    }
}

pub fn prompt_for_credentials() -> anyhow::Result<Credentials> {
    print!("Username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    let password = rpassword::prompt_password("Password: ")?;

    Ok(Credentials {
        user: username.into(),
        password,
    })
}

pub fn extract_user(input: &str) -> Option<String> {
    let mut parts = input.split_whitespace();

    // Skip the "login" command
    let _ = parts.next();

    while let Some(part) = parts.next() {
        // Case 1: login alice
        if !part.starts_with("--") {
            return Some(part.to_string());
        }

        // Case 2: --user=alice
        if let Some(rest) = part.strip_prefix("--user=") {
            return Some(rest.to_string());
        }

        // Case 3: --user alice
        if part == "--user" {
            if let Some(next) = parts.next() {
                return Some(next.to_string());
            }
        }
    }

    None
}
