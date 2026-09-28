mod dashboard;
mod health;
mod meta;

use crate::state::AppState;
use axum::{Router, routing::get};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .route("/api/v1/meta", get(meta::get))
        .route("/api/v1/dashboard/summary", get(dashboard::summary))
        .with_state(state)
}
