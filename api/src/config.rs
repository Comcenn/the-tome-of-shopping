#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub store_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            addr: std::env::var("API_ADDR").unwrap_or("127.0.0.1:3000".into()),
            store_path: std::env::var("API_STORE_PATH").unwrap_or("./store.json".into()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:3000".into(),
            store_path: "./store.json".into(),
        }
    }
}
