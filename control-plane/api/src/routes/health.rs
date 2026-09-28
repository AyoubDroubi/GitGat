use crate::state::AppState;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

pub async fn live() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "gitgat-control-plane",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn ready(State(state): State<AppState>) -> Response {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.database)
        .await
    {
        Ok(1) => Json(HealthResponse {
            status: "ready",
            service: "gitgat-control-plane",
            version: env!("CARGO_PKG_VERSION"),
        })
        .into_response(),
        Ok(_) | Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HealthResponse {
                status: "not_ready",
                service: "gitgat-control-plane",
                version: env!("CARGO_PKG_VERSION"),
            }),
        )
            .into_response(),
    }
}
