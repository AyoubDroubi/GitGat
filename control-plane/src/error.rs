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
    #[error("authentication required")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("invalid request: {0}")]
    BadRequest(String),
    #[error("conflict: {0}")]
    Conflict(String),
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
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "auth.unauthorized"),
            Self::Forbidden => (StatusCode::FORBIDDEN, "auth.forbidden"),
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, "common.bad_request"),
            Self::Conflict(_) => (StatusCode::CONFLICT, "common.conflict"),
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
