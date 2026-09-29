#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            bind: std::env::var("GITGAT_BIND")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_owned()),
        })
    }
}
