use crate::{admin_api, error::ApiError, operations_api, privileged_api, state::AppState};
use axum::{Json, Router, extract::State, response::Html, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: &'static str,
    database: &'static str,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .route("/admin", get(admin))
        .route("/health", get(health))
        .route("/ready", get(ready))
        .nest(
            "/api/v1",
            Router::new()
                .route("/governance/summary", get(summary))
                .merge(admin_api::router())
                .merge(privileged_api::router())
                .merge(operations_api::router()),
        )
}

async fn root() -> &'static str {
    "GitGat Control Plane"
}

async fn admin() -> Html<&'static str> {
    Html(include_str!("../admin/index.html"))
}

async fn health() -> Json<Health> {
    Json(Health {
        status: "ok",
        database: "unchecked",
    })
}

async fn ready(State(state): State<AppState>) -> Result<Json<Health>, ApiError> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(Health {
        status: "ok",
        database: "ok",
    }))
}

#[derive(Serialize)]
struct GovernanceSummary {
    organizations: i64,
    repositories: i64,
    policies: i64,
}

async fn summary(State(state): State<AppState>) -> Result<Json<GovernanceSummary>, ApiError> {
    let organizations = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM organizations")
        .fetch_one(&state.pool)
        .await?;
    let repositories =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM repository_registrations")
            .fetch_one(&state.pool)
            .await?;
    let policies = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM lock_policies")
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(GovernanceSummary {
        organizations,
        repositories,
        policies,
    }))
}
