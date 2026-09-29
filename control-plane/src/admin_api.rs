use crate::{
    audit::{self, AuditEventInput},
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
            "/organizations/{organization_id}/repositories",
            get(list_repositories).post(enroll_repository),
        )
        .route(
            "/organizations/{organization_id}/provider-connections",
            post(add_provider_connection),
        )
        .route(
            "/repositories/{repository_id}/policy",
            get(get_policy).put(put_policy),
        )
        .route("/repositories/{repository_id}/locks", get(list_locks))
        .route(
            "/repositories/{repository_id}/force-unlock-requests",
            post(request_force_unlock),
        )
        .route(
            "/organizations/{organization_id}/audit",
            get(list_audit_events),
        )
}

#[derive(Deserialize)]
struct EnrollRepository {
    provider: String,
    provider_repository_id: String,
    display_name: String,
    policy_mode: String,
}

#[derive(Serialize)]
struct IdResponse {
    id: Uuid,
}

async fn enroll_repository(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
    Json(input): Json<EnrollRepository>,
) -> Result<Json<IdResponse>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ManageEnrollment,
    )
    .await?;

    if !matches!(input.provider.as_str(), "github" | "azure_devops") {
        return Err(ApiError::BadRequest("unsupported provider".into()));
    }
    if !matches!(input.policy_mode.as_str(), "observe" | "warn" | "enforce") {
        return Err(ApiError::BadRequest("invalid policy mode".into()));
    }

    let mut tx = state.pool.begin().await?;
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO repository_registrations
            (organization_id, provider, provider_repository_id, display_name, policy_mode)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id",
    )
    .bind(organization_id)
    .bind(&input.provider)
    .bind(&input.provider_repository_id)
    .bind(&input.display_name)
    .bind(&input.policy_mode)
    .fetch_one(&mut *tx)
    .await?;

    let target_id = id.to_string();
    audit::append_event(
        &mut tx,
        AuditEventInput {
            organization_id,
            actor_id: &identity.subject,
            actor_type: "user",
            action: "repository.enrolled",
            target_type: "repository",
            target_id: Some(&target_id),
            repository_id: Some(id),
            path: None,
            reason: None,
            outcome: "created",
            correlation_id: None,
            evidence_reference: Some(&input.provider_repository_id),
            payload: serde_json::json!({
                "provider": &input.provider,
                "display_name": &input.display_name,
                "policy_mode": &input.policy_mode,
            }),
        },
    )
    .await?;
    tx.commit().await?;

    Ok(Json(IdResponse { id }))
}

#[derive(Serialize)]
struct RepositoryView {
    id: Uuid,
    provider: String,
    provider_repository_id: String,
    display_name: String,
    policy_mode: String,
    provider_health: String,
}

async fn list_repositories(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
) -> Result<Json<Vec<RepositoryView>>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ViewRepositories,
    )
    .await?;

    let rows = sqlx::query(
        "SELECT id, provider, provider_repository_id, display_name, policy_mode, provider_health
         FROM repository_registrations
         WHERE organization_id = $1
         ORDER BY display_name",
    )
    .bind(organization_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|row| RepositoryView {
                id: row.get("id"),
                provider: row.get("provider"),
                provider_repository_id: row.get("provider_repository_id"),
                display_name: row.get("display_name"),
                policy_mode: row.get("policy_mode"),
                provider_health: row.get("provider_health"),
            })
            .collect(),
    ))
}

#[derive(Deserialize)]
struct AddProviderConnection {
    provider: String,
    external_installation_id: String,
    secret_reference: String,
}

async fn add_provider_connection(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
    Json(input): Json<AddProviderConnection>,
) -> Result<Json<IdResponse>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ManageProviderConnections,
    )
    .await?;

    if !matches!(input.provider.as_str(), "github" | "azure_devops") {
        return Err(ApiError::BadRequest("unsupported provider".into()));
    }
    if input.secret_reference.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "secret_reference must point to external secret storage".into(),
        ));
    }

    let mut tx = state.pool.begin().await?;
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO provider_connections
            (organization_id, provider, external_installation_id, secret_reference)
         VALUES ($1, $2, $3, $4)
         RETURNING id",
    )
    .bind(organization_id)
    .bind(&input.provider)
    .bind(&input.external_installation_id)
    .bind(&input.secret_reference)
    .fetch_one(&mut *tx)
    .await?;

    let target_id = id.to_string();
    audit::append_event(
        &mut tx,
        AuditEventInput {
            organization_id,
            actor_id: &identity.subject,
            actor_type: "user",
            action: "provider_connection.created",
            target_type: "provider_connection",
            target_id: Some(&target_id),
            repository_id: None,
            path: None,
            reason: None,
            outcome: "created",
            correlation_id: None,
            evidence_reference: Some(&input.external_installation_id),
            payload: serde_json::json!({
                "provider": &input.provider,
                "secret_reference_present": true,
            }),
        },
    )
    .await?;
    tx.commit().await?;

    Ok(Json(IdResponse { id }))
}

#[derive(Deserialize)]
struct PolicyInput {
    protected_patterns: Vec<String>,
    excluded_patterns: Vec<String>,
    required_lock: bool,
    max_lock_age_minutes: Option<i64>,
    force_unlock_approval_required: bool,
}

#[derive(Serialize)]
struct PolicyView {
    version: i64,
    protected_patterns: serde_json::Value,
    excluded_patterns: serde_json::Value,
    required_lock: bool,
    max_lock_age_minutes: Option<i64>,
    force_unlock_approval_required: bool,
}

async fn get_policy(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(repository_id): Path<Uuid>,
) -> Result<Json<PolicyView>, ApiError> {
    authz::require_repository_permission(
        &state.pool,
        &identity.subject,
        repository_id,
        Permission::ViewRepositories,
    )
    .await?;

    let row = sqlx::query(
        "SELECT version, protected_patterns, excluded_patterns, required_lock,
                max_lock_age_minutes, force_unlock_approval_required
         FROM lock_policies WHERE repository_id = $1",
    )
    .bind(repository_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(Json(PolicyView {
        version,
        protected_patterns: row.get("protected_patterns"),
        excluded_patterns: row.get("excluded_patterns"),
        required_lock: row.get("required_lock"),
        max_lock_age_minutes: row.get("max_lock_age_minutes"),
        force_unlock_approval_required: row.get("force_unlock_approval_required"),
    }))
}

async fn put_policy(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<PolicyInput>,
) -> Result<Json<PolicyView>, ApiError> {
    authz::require_repository_permission(
        &state.pool,
        &identity.subject,
        repository_id,
        Permission::ManageProtectedPatterns,
    )
    .await?;

    let protected: Option<Vec<_>> = input
        .protected_patterns
        .iter()
        .map(|value| normalize_pattern(value))
        .collect();
    let excluded: Option<Vec<_>> = input
        .excluded_patterns
        .iter()
        .map(|value| normalize_pattern(value))
        .collect();
    let (protected, excluded) = match (protected, excluded) {
        (Some(protected), Some(excluded)) => (protected, excluded),
        _ => {
            return Err(ApiError::BadRequest(
                "invalid repository-relative pattern".into(),
            ));
        }
    };

    let organization_id = authz::repository_organization(&state.pool, repository_id).await?;
    let mut tx = state.pool.begin().await?;
    let row = sqlx::query(
        "INSERT INTO lock_policies
            (repository_id, protected_patterns, excluded_patterns, required_lock,
             max_lock_age_minutes, force_unlock_approval_required)
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (repository_id) DO UPDATE SET
            protected_patterns = EXCLUDED.protected_patterns,
            excluded_patterns = EXCLUDED.excluded_patterns,
            required_lock = EXCLUDED.required_lock,
            max_lock_age_minutes = EXCLUDED.max_lock_age_minutes,
            force_unlock_approval_required = EXCLUDED.force_unlock_approval_required,
            version = lock_policies.version + 1,
            updated_at = now()
         RETURNING version, protected_patterns, excluded_patterns, required_lock,
                   max_lock_age_minutes, force_unlock_approval_required",
    )
    .bind(repository_id)
    .bind(serde_json::json!(protected))
    .bind(serde_json::json!(excluded))
    .bind(input.required_lock)
    .bind(input.max_lock_age_minutes)
    .bind(input.force_unlock_approval_required)
    .fetch_one(&mut *tx)
    .await?;

    let version: i64 = row.get("version");
    let target_id = repository_id.to_string();
    audit::append_event(
        &mut tx,
        AuditEventInput {
            organization_id,
            actor_id: &identity.subject,
            actor_type: "user",
            action: "lock_policy.updated",
            target_type: "repository",
            target_id: Some(&target_id),
            repository_id: Some(repository_id),
            path: None,
            reason: None,
            outcome: "updated",
            correlation_id: None,
            evidence_reference: None,
            payload: serde_json::json!({
                "version": version,
                "protected_patterns": &protected,
                "excluded_patterns": &excluded,
                "required_lock": input.required_lock,
                "max_lock_age_minutes": input.max_lock_age_minutes,
                "force_unlock_approval_required": input.force_unlock_approval_required,
            }),
        },
    )
    .await?;
    tx.commit().await?;

    Ok(Json(PolicyView {
        version: row.get("version"),
        protected_patterns: row.get("protected_patterns"),
        excluded_patterns: row.get("excluded_patterns"),
        required_lock: row.get("required_lock"),
        max_lock_age_minutes: row.get("max_lock_age_minutes"),
        force_unlock_approval_required: row.get("force_unlock_approval_required"),
    }))
}

#[derive(Serialize)]
struct LockView {
    provider_lock_id: String,
    path: String,
    owner_external_id: String,
    locked_at: Option<DateTime<Utc>>,
    verified_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    authoritative: bool,
}

async fn list_locks(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(repository_id): Path<Uuid>,
) -> Result<Json<Vec<LockView>>, ApiError> {
    authz::require_repository_permission(
        &state.pool,
        &identity.subject,
        repository_id,
        Permission::ViewActiveLocks,
    )
    .await?;

    let rows = sqlx::query(
        "SELECT provider_lock_id, path, owner_external_id, locked_at, verified_at, expires_at
         FROM lock_observations
         WHERE repository_id = $1
         ORDER BY path",
    )
    .bind(repository_id)
    .fetch_all(&state.pool)
    .await?;

    let now = Utc::now();
    Ok(Json(
        rows.into_iter()
            .map(|row| {
                let expires_at: DateTime<Utc> = row.get("expires_at");
                LockView {
                    provider_lock_id: row.get("provider_lock_id"),
                    path: row.get("path"),
                    owner_external_id: row.get("owner_external_id"),
                    locked_at: row.get("locked_at"),
                    verified_at: row.get("verified_at"),
                    expires_at,
                    authoritative: now < expires_at,
                }
            })
            .collect(),
    ))
}

#[derive(Deserialize)]
struct ForceUnlockInput {
    path: String,
    provider_lock_id: String,
    current_owner: String,
    reason: String,
    linked_reference: Option<String>,
    verified_at: DateTime<Utc>,
}

async fn request_force_unlock(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<ForceUnlockInput>,
) -> Result<Json<IdResponse>, ApiError> {
    let requester = authz::require_repository_permission(
        &state.pool,
        &identity.subject,
        repository_id,
        Permission::RequestForceUnlock,
    )
    .await?;
    let organization_id = authz::repository_organization(&state.pool, repository_id).await?;

    if input.reason.trim().is_empty() {
        return Err(ApiError::BadRequest("reason is required".into()));
    }

    let mut tx = state.pool.begin().await?;
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO force_unlock_requests
            (organization_id, repository_id, path, provider_lock_id, current_owner,
             requester_user_id, reason, linked_reference, verified_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         RETURNING id",
    )
    .bind(organization_id)
    .bind(repository_id)
    .bind(&input.path)
    .bind(&input.provider_lock_id)
    .bind(&input.current_owner)
    .bind(requester)
    .bind(&input.reason)
    .bind(&input.linked_reference)
    .bind(input.verified_at)
    .fetch_one(&mut *tx)
    .await?;

    let target_id = id.to_string();
    audit::append_event(
        &mut tx,
        AuditEventInput {
            organization_id,
            actor_id: &identity.subject,
            actor_type: "user",
            action: "force_unlock.requested",
            target_type: "force_unlock_request",
            target_id: Some(&target_id),
            repository_id: Some(repository_id),
            path: Some(&input.path),
            reason: Some(&input.reason),
            outcome: "requested",
            correlation_id: None,
            evidence_reference: Some(&input.provider_lock_id),
            payload: serde_json::json!({
                "current_owner": &input.current_owner,
                "linked_reference": &input.linked_reference,
                "verified_at": input.verified_at,
            }),
        },
    )
    .await?;
    tx.commit().await?;

    Ok(Json(IdResponse { id }))
}

#[derive(Serialize)]
struct AuditView {
    action: String,
    target_type: String,
    target_id: Option<String>,
    outcome: String,
    occurred_at: DateTime<Utc>,
}

async fn list_audit_events(
    State(state): State<AppState>,
    identity: AuthenticatedIdentity,
    Path(organization_id): Path<Uuid>,
) -> Result<Json<Vec<AuditView>>, ApiError> {
    authz::require_org_permission(
        &state.pool,
        &identity.subject,
        organization_id,
        Permission::ViewAudit,
    )
    .await?;

    let rows = sqlx::query(
        "SELECT action, target_type, target_id, outcome, occurred_at
         FROM audit_events
         WHERE organization_id = $1
         ORDER BY occurred_at DESC
         LIMIT 200",
    )
    .bind(organization_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        rows.into_iter()
            .map(|row| AuditView {
                action: row.get("action"),
                target_type: row.get("target_type"),
                target_id: row.get("target_id"),
                outcome: row.get("outcome"),
                occurred_at: row.get("occurred_at"),
            })
            .collect(),
    ))
}
