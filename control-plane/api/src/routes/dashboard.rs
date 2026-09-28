use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    organizations: i64,
    repositories: i64,
    provider_connections: i64,
    active_lock_observations: i64,
    stale_lock_observations: i64,
    pending_force_unlock_requests: i64,
    active_policy_exceptions: i64,
    audit_events: i64,
}

pub async fn summary(State(state): State<AppState>) -> Response {
    let query = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64)>(
        r#"
        SELECT
            (SELECT count(*) FROM organizations),
            (SELECT count(*) FROM repository_registrations WHERE enabled = true),
            (SELECT count(*) FROM provider_connections WHERE status <> 'revoked'),
            (SELECT count(*) FROM lock_observations WHERE verification_state = 'verified'),
            (SELECT count(*) FROM lock_observations WHERE verification_state = 'stale'),
            (SELECT count(*) FROM force_unlock_requests WHERE status = 'pending'),
            (SELECT count(*) FROM policy_exceptions WHERE revoked_at IS NULL AND expires_at > now()),
            (SELECT count(*) FROM audit_events)
        "#,
    )
    .fetch_one(&state.database)
    .await;

    match query {
        Ok((
            organizations,
            repositories,
            provider_connections,
            active_lock_observations,
            stale_lock_observations,
            pending_force_unlock_requests,
            active_policy_exceptions,
            audit_events,
        )) => Json(DashboardSummary {
            organizations,
            repositories,
            provider_connections,
            active_lock_observations,
            stale_lock_observations,
            pending_force_unlock_requests,
            active_policy_exceptions,
            audit_events,
        })
        .into_response(),
        Err(error) => {
            tracing::error!(%error, "failed to load dashboard summary");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "error": {
                        "code": "dashboard.unavailable",
                        "message": "Dashboard data is temporarily unavailable."
                    }
                })),
            )
                .into_response()
        }
    }
}
