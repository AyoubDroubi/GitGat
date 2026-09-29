use std::env;

pub struct Config {
    pub database_url: String,
    pub bind: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            bind: env::var("GITGAT_BIND").unwrap_or_else(|_| "127.0.0.1:8080".into()),
        })
    }
}
