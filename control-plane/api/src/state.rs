use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: PgPool,
}

impl AppState {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}
