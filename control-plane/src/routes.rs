use crate::{error::ApiError, state::AppState};
use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: &'static str,
    database: &'static str,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .nest(
            "/api/v1",
            Router::new().route("/governance/summary", get(summary)),
        )
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
    let repositories = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM repository_registrations")
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
