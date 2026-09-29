use crate::{
    auth::AuthenticatedIdentity,
    authz::{self, Permission},
    error::ApiError,
    governance::normalize_pattern,
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/force-unlock-requests/{request_id}/approve",
            post(approve_force_unlock),
        )
        .route(
            "/organizations/{organization_id}/exceptions",
            get(list_exceptions).post(create_exception),
        )
        .route(
            "/organizations/{organization_id}/stale-locks",
            get(list_stale_locks),
        )
}

#[derive(Deserialize)]
struct ApprovalInput {
    reason: Option<String>,
}

#[derive(Serialize)]
struct StatusResponse {
    status: &'static str,
}

async fn approve_force_unlock(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(request_id): Path<Uuid>,
    Json(input): Json<ApprovalInput>,
) -> Result<Json<StatusResponse>, ApiError> {
    let preliminary = sqlx::query("SELECT repository_id FROM force_unlock_requests WHERE id = $1")
        .bind(request_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or(ApiError::NotFound)?;
    let repository_id: Uuid = preliminary.get("repository_id");

    let approver = authz::require_repository_permission(
        &state.pool,
        &identity.subject,
        repository_id,
        Permission::ApproveForceUnlock,
    )
    .await?;

    let mut tx = state.pool.begin().await?;
    let row = sqlx::query(
        "SELECT requester_user_id, status
         FROM force_unlock_requests
         WHERE id = $1
         FOR UPDATE",
    )
    .bind(request_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ApiError::NotFound)?;

    let requester: Uuid = row.get("requester_user_id");
    let status: String = row.get("status");
    if requester == approver {
        return Err(ApiError::Forbidden);
    }
    if status != "requested" {
        return Err(ApiError::Conflict(
            "force-unlock request is no longer pending".into(),
        ));
    }

    sqlx::query(
        "INSERT INTO force_unlock_approvals
            (request_id, approver_user_id, decision, reason)
         VALUES ($1, $2, 'approve', $3)",
    )
    .bind(request_id)
    .bind(approver)
    .bind(input.reason)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE force_unlock_requests
         SET status = 'approved', version = version + 1, updated_at = now()
         WHERE id = $1",
    )
    .bind(request_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(Json(StatusResponse { status: "approved" }))
}

#[derive(Deserialize)]
struct ExceptionInput {
    repository_id: Option<Uuid>,
    subject_external_id: Option<String>,
    path_pattern: String,
    reason: String,
    starts_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    linked_reference: Option<String>,
}

#[derive(Serialize)]
struct ExceptionView {
    id: Uuid,
    repository_id: Option<Uuid>,
    subject_external_id: Option<String>,
    path_pattern: String,
    reason: String,
    starts_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

async fn create_exception(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
    Json(input): Json<ExceptionInput>,
) -> Result<Json<ExceptionView>, ApiError> {
    let approver = authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ManageExceptions,
    )
    .await?;

    let Some(path_pattern) = normalize_pattern(&input.path_pattern) else {
        return Err(ApiError::BadRequest("invalid path pattern".into()));
    };
    if input.reason.trim().is_empty() || input.expires_at <= input.starts_at {
        return Err(ApiError::BadRequest(
            "reason and a valid exception window are required".into(),
        ));
    }

    if let Some(repository_id) = input.repository_id {
        let repository_org = authz::repository_organization(&state.pool, repository_id).await?;
        if repository_org != organization_id {
            return Err(ApiError::Forbidden);
        }
    }

    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO policy_exceptions
            (organization_id, repository_id, subject_external_id, path_pattern, reason,
             approved_by_user_id, starts_at, expires_at, linked_reference)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         RETURNING id",
    )
    .bind(organization_id)
    .bind(input.repository_id)
    .bind(&input.subject_external_id)
    .bind(&path_pattern)
    .bind(&input.reason)
    .bind(approver)
    .bind(input.starts_at)
    .bind(input.expires_at)
    .bind(input.linked_reference)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ExceptionView {
        id,
        repository_id: input.repository_id,
        subject_external_id: input.subject_external_id,
        path_pattern,
        reason: input.reason,
        starts_at: input.starts_at,
        expires_at: input.expires_at,
    }))
}

async fn list_exceptions(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
) -> Result<Json<Vec<ExceptionView>>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ViewRepositories,
    )
    .await?;

    let rows = sqlx::query(
        "SELECT id, repository_id, subject_external_id, path_pattern, reason,
                starts_at, expires_at
         FROM policy_exceptions
         WHERE organization_id = $1 AND expires_at > now()
         ORDER BY expires_at",
    )
    .bind(organization_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|row| ExceptionView {
                id: row.get("id"),
                repository_id: row.get("repository_id"),
                subject_external_id: row.get("subject_external_id"),
                path_pattern: row.get("path_pattern"),
                reason: row.get("reason"),
                starts_at: row.get("starts_at"),
                expires_at: row.get("expires_at"),
            })
            .collect(),
    ))
}

#[derive(Serialize)]
struct StaleLockView {
    repository_id: Uuid,
    path: String,
    owner_external_id: String,
    locked_at: DateTime<Utc>,
    verified_at: DateTime<Utc>,
    age_minutes: i64,
}

async fn list_stale_locks(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
) -> Result<Json<Vec<StaleLockView>>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ViewActiveLocks,
    )
    .await?;

    let rows = sqlx::query(
        "SELECT lo.repository_id, lo.path, lo.owner_external_id, lo.locked_at, lo.verified_at,
                FLOOR(EXTRACT(EPOCH FROM (now() - lo.locked_at)) / 60)::bigint AS age_minutes
         FROM lock_observations lo
         JOIN repository_registrations r ON r.id = lo.repository_id
         JOIN lock_policies lp ON lp.repository_id = lo.repository_id
         WHERE r.organization_id = $1
           AND lo.expires_at > now()
           AND lo.locked_at IS NOT NULL
           AND lp.max_lock_age_minutes IS NOT NULL
           AND lo.locked_at <= now() - make_interval(mins => lp.max_lock_age_minutes::int)
         ORDER BY lo.locked_at",
    )
    .bind(organization_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|row| StaleLockView {
                repository_id: row.get("repository_id"),
                path: row.get("path"),
                owner_external_id: row.get("owner_external_id"),
                locked_at: row.get("locked_at"),
                verified_at: row.get("verified_at"),
                age_minutes: row.get("age_minutes"),
            })
            .collect(),
    ))
}
