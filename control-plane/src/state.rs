use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub auth_proxy_secret: Option<Arc<Vec<u8>>>,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool, auth_proxy_secret: Option<String>) -> Self {
        Self {
            pool,
            auth_proxy_secret: auth_proxy_secret.map(|value| Arc::new(value.into_bytes())),
        }
    }
}
