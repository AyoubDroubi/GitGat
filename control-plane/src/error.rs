use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
    #[error("resource not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    code: &'a str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code) = match self {
            Self::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal.database"),
            Self::NotFound => (StatusCode::NOT_FOUND, "common.not_found"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "auth.forbidden"),
        };
        (
            status,
            Json(ErrorBody {
                code,
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
