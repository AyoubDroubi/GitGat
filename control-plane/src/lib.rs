pub mod audit;
pub mod authz;
pub mod config;
pub mod error;
pub mod governance;
pub mod force_unlock;
pub mod stale;
pub mod webhooks;
pub mod routes;
pub mod state;

use axum::Router;
use state::AppState;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(routes::router())
        .with_state(state)
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
}
