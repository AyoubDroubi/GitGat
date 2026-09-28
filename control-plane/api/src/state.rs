use crate::auth::AuthService;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: PgPool,
    pub auth: AuthService,
}

impl AppState {
    pub fn new(database: PgPool, auth: AuthService) -> Self {
        Self { database, auth }
    }
}
