mod auth;
mod dashboard;
mod health;
mod meta;
mod organizations;
mod teams;

use crate::state::AppState;
use axum::{Router, middleware, routing::get};

pub fn router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/api/v1/me", get(auth::me))
        .route(
            "/api/v1/organizations",
            get(organizations::list).post(organizations::create),
        )
        .route(
            "/api/v1/organizations/{organization_id}/access",
            get(organizations::access),
        )
        .route(
            "/api/v1/organizations/{organization_id}/dashboard/summary",
            get(dashboard::summary),
        )
        .route(
            "/api/v1/organizations/{organization_id}/members",
            get(organizations::members),
        )
        .route(
            "/api/v1/organizations/{organization_id}/members/{user_id}/role",
            axum::routing::patch(organizations::update_member_role),
        )
        .route(
            "/api/v1/organizations/{organization_id}/teams",
            get(teams::list).post(teams::create),
        )
        .route(
            "/api/v1/organizations/{organization_id}/teams/{team_id}/members",
            get(teams::members),
        )
        .route(
            "/api/v1/organizations/{organization_id}/teams/{team_id}/members/{user_id}",
            axum::routing::put(teams::upsert_member).delete(teams::remove_member),
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
