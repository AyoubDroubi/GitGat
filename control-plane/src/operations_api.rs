use crate::{
    auth::AuthenticatedIdentity,
    authz::{self, Permission},
    error::ApiError,
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, put},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/organizations/{organization_id}/enforcement",
            get(list_enforcement),
        )
        .route(
            "/repositories/{repository_id}/enforcement/observation",
            put(record_enforcement_observation),
        )
        .route(
            "/organizations/{organization_id}/operations",
            get(operations_summary),
        )
}

#[derive(Deserialize)]
struct EnforcementObservationInput {
    provider: String,
    required_check_configured: bool,
    commit_binding_verified: bool,
    observed_source_sha: Option<String>,
    observed_target_branch: Option<String>,
    details: serde_json::Value,
}

#[derive(Serialize)]
struct EnforcementView {
    repository_id: Uuid,
    display_name: String,
    provider: String,
    desired_mode: String,
    required_check_configured: bool,
    commit_binding_verified: bool,
    observed_source_sha: Option<String>,
    observed_target_branch: Option<String>,
    observed_at: Option<DateTime<Utc>>,
    healthy: bool,
}

async fn list_enforcement(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
) -> Result<Json<Vec<EnforcementView>>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ViewRepositories,
    )
    .await?;

    let rows = sqlx::query(
        "SELECT r.id, r.display_name, r.provider, r.policy_mode,
                o.required_check_configured, o.commit_binding_verified,
                o.observed_source_sha, o.observed_target_branch, o.observed_at
         FROM repository_registrations r
         LEFT JOIN provider_policy_observations o ON o.repository_id = r.id
         WHERE r.organization_id = $1
         ORDER BY r.display_name",
    )
    .bind(organization_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|row| {
                let desired_mode: String = row.get("policy_mode");
                let required_check_configured =
                    row.try_get("required_check_configured").unwrap_or(false);
                let commit_binding_verified =
                    row.try_get("commit_binding_verified").unwrap_or(false);
                let observed_at = row.try_get("observed_at").ok();
                let healthy = desired_mode != "enforce"
                    || (required_check_configured
                        && commit_binding_verified
                        && observed_at.is_some());
                EnforcementView {
                    repository_id: row.get("id"),
                    display_name: row.get("display_name"),
                    provider: row.get("provider"),
                    desired_mode,
                    required_check_configured,
                    commit_binding_verified,
                    observed_source_sha: row.try_get("observed_source_sha").ok(),
                    observed_target_branch: row.try_get("observed_target_branch").ok(),
                    observed_at,
                    healthy,
                }
            })
            .collect(),
    ))
}

async fn record_enforcement_observation(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<EnforcementObservationInput>,
) -> Result<Json<EnforcementView>, ApiError> {
    authz::require_repository_permission(
        &state.pool,
        &identity.subject,
        repository_id,
        Permission::ManageEnrollment,
    )
    .await?;

    if !matches!(input.provider.as_str(), "github" | "azure_devops") {
        return Err(ApiError::BadRequest("unsupported provider".into()));
    }

    let registered_provider = sqlx::query_scalar::<_, String>(
        "SELECT provider FROM repository_registrations WHERE id = $1",
    )
    .bind(repository_id)
    .fetch_one(&state.pool)
    .await?;
    if registered_provider != input.provider {
        return Err(ApiError::Conflict(
            "observation provider does not match enrolled repository".into(),
        ));
    }

    sqlx::query(
        "INSERT INTO provider_policy_observations
            (repository_id, provider, required_check_configured, commit_binding_verified,
             observed_source_sha, observed_target_branch, details, observed_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,now())
         ON CONFLICT (repository_id) DO UPDATE SET
            provider = EXCLUDED.provider,
            required_check_configured = EXCLUDED.required_check_configured,
            commit_binding_verified = EXCLUDED.commit_binding_verified,
            observed_source_sha = EXCLUDED.observed_source_sha,
            observed_target_branch = EXCLUDED.observed_target_branch,
            details = EXCLUDED.details,
            observed_at = now()",
    )
    .bind(repository_id)
    .bind(&input.provider)
    .bind(input.required_check_configured)
    .bind(input.commit_binding_verified)
    .bind(&input.observed_source_sha)
    .bind(&input.observed_target_branch)
    .bind(&input.details)
    .execute(&state.pool)
    .await?;

    let row = sqlx::query(
        "SELECT r.id, r.display_name, r.provider, r.policy_mode,
                o.required_check_configured, o.commit_binding_verified,
                o.observed_source_sha, o.observed_target_branch, o.observed_at
         FROM repository_registrations r
         JOIN provider_policy_observations o ON o.repository_id = r.id
         WHERE r.id = $1",
    )
    .bind(repository_id)
    .fetch_one(&state.pool)
    .await?;

    let desired_mode: String = row.get("policy_mode");
    let required_check_configured: bool = row.get("required_check_configured");
    let commit_binding_verified: bool = row.get("commit_binding_verified");
    let observed_at: DateTime<Utc> = row.get("observed_at");
    Ok(Json(EnforcementView {
        repository_id,
        display_name: row.get("display_name"),
        provider: row.get("provider"),
        desired_mode: desired_mode.clone(),
        required_check_configured,
        commit_binding_verified,
        observed_source_sha: row.get("observed_source_sha"),
        observed_target_branch: row.get("observed_target_branch"),
        observed_at: Some(observed_at),
        healthy: desired_mode != "enforce"
            || (required_check_configured && commit_binding_verified),
    }))
}

#[derive(Serialize)]
struct OperationsSummary {
    repositories: i64,
    degraded_repositories: i64,
    failed_webhooks: i64,
    dead_letter_jobs: i64,
    queued_jobs: i64,
    stale_lock_candidates: i64,
}

async fn operations_summary(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
) -> Result<Json<OperationsSummary>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ViewAudit,
    )
    .await?;

    let repositories = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM repository_registrations WHERE organization_id = $1",
    )
    .bind(organization_id)
    .fetch_one(&state.pool)
    .await?;

    let degraded_repositories = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM repository_registrations r
         LEFT JOIN provider_policy_observations o ON o.repository_id = r.id
         WHERE r.organization_id = $1
           AND r.policy_mode = 'enforce'
           AND (
             o.repository_id IS NULL
             OR NOT o.required_check_configured
             OR NOT o.commit_binding_verified
           )",
    )
    .bind(organization_id)
    .fetch_one(&state.pool)
    .await?;

    let failed_webhooks = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM webhook_deliveries w
         JOIN provider_connections p ON p.id = w.provider_connection_id
         WHERE p.organization_id = $1 AND w.status IN ('failed','dead_letter')",
    )
    .bind(organization_id)
    .fetch_one(&state.pool)
    .await?;

    let dead_letter_jobs = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM background_jobs
         WHERE organization_id = $1 AND status = 'dead_letter'",
    )
    .bind(organization_id)
    .fetch_one(&state.pool)
    .await?;

    let queued_jobs = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM background_jobs
         WHERE organization_id = $1 AND status IN ('queued','running')",
    )
    .bind(organization_id)
    .fetch_one(&state.pool)
    .await?;

    let stale_lock_candidates = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
         FROM stale_lock_alerts a
         JOIN lock_observations lo ON lo.id = a.lock_observation_id
         JOIN repository_registrations r ON r.id = lo.repository_id
         WHERE r.organization_id = $1 AND a.state != 'resolved'",
    )
    .bind(organization_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(OperationsSummary {
        repositories,
        degraded_repositories,
        failed_webhooks,
        dead_letter_jobs,
        queued_jobs,
        stale_lock_candidates,
    }))
}
