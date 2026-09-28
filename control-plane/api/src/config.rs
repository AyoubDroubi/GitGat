use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub bind: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("GITGAT_DATABASE_URL")
            .map_err(|_| "GITGAT_DATABASE_URL is required")?;
        let bind = std::env::var("GITGAT_BIND")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
            .parse()?;
        let database_max_connections = std::env::var("GITGAT_DATABASE_MAX_CONNECTIONS")
            .ok()
            .map(|value| value.parse())
            .transpose()?
            .unwrap_or(10);

        Ok(Self {
            database_url,
            database_max_connections,
            bind,
        })
    }
}
