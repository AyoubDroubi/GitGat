use crate::auth::AuthIdentity;
use axum::{Extension, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    subject: String,
    email: Option<String>,
    display_name: Option<String>,
}

pub async fn me(Extension(identity): Extension<AuthIdentity>) -> Json<MeResponse> {
    Json(MeResponse {
        subject: identity.subject,
        email: identity.email,
        display_name: identity.display_name,
    })
}
