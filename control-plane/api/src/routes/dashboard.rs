use crate::{
    auth::AuthIdentity,
    rbac::{Permission, require_permission},
    routes::organizations::access_error,
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use uuid::Uuid;

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

pub async fn summary(
    State(state): State<AppState>,
    Path(organization_id): Path<Uuid>,
    Extension(identity): Extension<AuthIdentity>,
) -> Response {
    if let Err(error) = require_permission(
        &state.database,
        &identity,
        organization_id,
        Permission::ViewOrganization,
    )
    .await
    {
        return access_error(error);
    }

    let query = sqlx::query_as::<_, (i64, i64, i64, i64, i64, i64, i64, i64)>(
        r#"
        SELECT
            1::bigint,
            (
                SELECT count(*)
                FROM repository_registrations
                WHERE organization_id = $1 AND enabled = true
            ),
            (
                SELECT count(*)
                FROM provider_connections
                WHERE organization_id = $1 AND status <> 'revoked'
            ),
            (
                SELECT count(*)
                FROM lock_observations observation
                INNER JOIN repository_registrations repository
                    ON repository.id = observation.repository_id
                WHERE repository.organization_id = $1
                  AND observation.verification_state = 'verified'
            ),
            (
                SELECT count(*)
                FROM lock_observations observation
                INNER JOIN repository_registrations repository
                    ON repository.id = observation.repository_id
                WHERE repository.organization_id = $1
                  AND observation.verification_state = 'stale'
            ),
            (
                SELECT count(*)
                FROM force_unlock_requests request
                INNER JOIN repository_registrations repository
                    ON repository.id = request.repository_id
                WHERE repository.organization_id = $1
                  AND request.status = 'pending'
            ),
            (
                SELECT count(*)
                FROM policy_exceptions exception
                INNER JOIN repository_registrations repository
                    ON repository.id = exception.repository_id
                WHERE repository.organization_id = $1
                  AND exception.revoked_at IS NULL
                  AND exception.expires_at > now()
            ),
            (
                SELECT count(*)
                FROM audit_events
                WHERE organization_id = $1
            )
        "#,
    )
    .bind(organization_id)
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
            tracing::error!(%error, %organization_id, "failed to load dashboard summary");
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
