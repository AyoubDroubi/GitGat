use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub bind: SocketAddr,
    pub oidc_issuer: Option<String>,
    pub oidc_audience: Option<String>,
    pub oidc_jwks_url: Option<String>,
    pub dev_auth_subject: Option<String>,
    pub dev_auth_email: Option<String>,
    pub dev_auth_name: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("GITGAT_DATABASE_URL").map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "GITGAT_DATABASE_URL is required",
            )
        })?;
        let bind = std::env::var("GITGAT_BIND")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_owned())
            .parse()?;
        let database_max_connections = std::env::var("GITGAT_DATABASE_MAX_CONNECTIONS")
            .ok()
            .map(|value| value.parse())
            .transpose()?
            .unwrap_or(10);

        let oidc_issuer = optional_env("GITGAT_OIDC_ISSUER");
        let oidc_audience = optional_env("GITGAT_OIDC_AUDIENCE");
        let oidc_jwks_url = optional_env("GITGAT_OIDC_JWKS_URL");
        let dev_auth_subject = optional_env("GITGAT_DEV_AUTH_SUBJECT");
        let dev_auth_email = optional_env("GITGAT_DEV_AUTH_EMAIL");
        let dev_auth_name = optional_env("GITGAT_DEV_AUTH_NAME");

        if dev_auth_subject.is_some() && !bind.ip().is_loopback() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "development authentication is allowed only on a loopback bind",
            )
            .into());
        }

        Ok(Self {
            database_url,
            database_max_connections,
            bind,
            oidc_issuer,
            oidc_audience,
            oidc_jwks_url,
            dev_auth_subject,
            dev_auth_email,
            dev_auth_name,
        })
    }
}


fn optional_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
