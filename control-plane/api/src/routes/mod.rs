mod auth;
mod dashboard;
mod health;
mod meta;
mod organizations;

use crate::state::AppState;
use axum::{
    Router,
    middleware,
    routing::get,
};

pub fn router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/api/v1/me", get(auth::me))
        .route("/api/v1/organizations", get(organizations::list))
        .route(
            "/api/v1/organizations/{organization_id}/access",
            get(organizations::access),
        )
        .route(
            "/api/v1/organizations/{organization_id}/dashboard/summary",
            get(dashboard::summary),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::require_auth,
        ));

    Router::new()
        .route("/health/live", get(health::live))
        .route("/health/ready", get(health::ready))
        .route("/api/v1/meta", get(meta::get))
        .merge(protected)
        .with_state(state)
}
