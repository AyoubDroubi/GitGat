use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MetaResponse {
    service: &'static str,
    version: &'static str,
    api_version: &'static str,
    active_lock_authority: &'static str,
}

pub async fn get() -> Json<MetaResponse> {
    Json(MetaResponse {
        service: "gitgat-control-plane",
        version: env!("CARGO_PKG_VERSION"),
        api_version: "v1",
        active_lock_authority: "provider_git_lfs",
    })
}
