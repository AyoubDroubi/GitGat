pub mod admin_api;
pub mod audit;
pub mod auth;
pub mod authz;
pub mod config;
pub mod error;
pub mod force_unlock;
pub mod governance;
pub mod operations_api;
pub mod privileged_api;
pub mod routes;
pub mod secrets;
pub mod stale;
pub mod state;
pub mod webhook_api;
pub mod webhooks;
pub mod worker;

use axum::{
    Router,
    http::{HeaderName, HeaderValue},
};
use state::AppState;
use tower_http::{
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(routes::router())
        .with_state(state)
        .layer(RequestBodyLimitLayer::new(5 * 1024 * 1024))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static("default-src 'self'; style-src 'self' 'unsafe-inline'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
}
