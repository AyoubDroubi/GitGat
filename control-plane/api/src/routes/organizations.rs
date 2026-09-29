use crate::{
    auth::AuthIdentity,
    rbac::{Permission, Role, require_permission},
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct OrganizationSummary {
    id: Uuid,
    slug: String,
    name: String,
    role: String,
}

#[derive(Debug, Serialize)]
pub struct OrganizationMember {
    user_id: Uuid,
    subject: String,
    email: Option<String>,
    display_name: String,
    role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    slug: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleRequest {
    role: String,
    reason: String,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(identity): Extension<AuthIdentity>,
) -> Response {
    let values = sqlx::query_as::<_, (Uuid, String, String, String)>(
        r#"
        SELECT organization.id, organization.slug, organization.name, membership.role
        FROM organizations organization
        INNER JOIN organization_memberships membership
            ON membership.organization_id = organization.id
        INNER JOIN users ON users.id = membership.user_id
        WHERE users.subject = $1
          AND users.disabled_at IS NULL
        ORDER BY organization.name, organization.id
        "#,
    )
    .bind(&identity.subject)
    .fetch_all(&state.database)
    .await;

    match values {
        Ok(values) => Json(
            values
                .into_iter()
                .map(|(id, slug, name, role)| OrganizationSummary {
                    id,
                    slug,
                    name,
                    role,
                })
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(error) => database_error(error),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Extension(identity): Extension<AuthIdentity>,
    Json(request): Json<CreateOrganizationRequest>,
) -> Response {
    let slug = request.slug.trim().to_ascii_lowercase();
    let name = request.name.trim();
    if !valid_slug(&slug) || name.is_empty() {
        return validation_error(
            "organization.invalid",
            "Organization name and a valid lowercase slug are required.",
        );
    }

    let mut transaction = match state.database.begin().await {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    let display_name = identity
        .display_name
        .as_deref()
        .or(identity.email.as_deref())
        .unwrap_or(&identity.subject);

    let user_id = match sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO users (subject, email, display_name)
        VALUES ($1, $2, $3)
        ON CONFLICT (subject) DO UPDATE SET
            email = COALESCE(EXCLUDED.email, users.email),
            display_name = EXCLUDED.display_name,
            updated_at = now()
        RETURNING id
        "#,
    )
    .bind(&identity.subject)
    .bind(&identity.email)
    .bind(display_name)
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    let organization_id = match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO organizations (slug, name) VALUES ($1, $2) RETURNING id",
    )
    .bind(&slug)
    .bind(name)
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(value) => value,
        Err(error) if is_unique_violation(&error) => {
            return conflict(
                "organization.slug_exists",
                "That organization slug is already in use.",
            );
        }
        Err(error) => return database_error(error),
    };

    if let Err(error) = sqlx::query(
        "INSERT INTO organization_memberships (organization_id, user_id, role) VALUES ($1, $2, 'owner')",
    )
    .bind(organization_id)
    .bind(user_id)
    .execute(&mut *transaction)
    .await
    {
        return database_error(error);
    }

    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO audit_events (
            organization_id, actor_user_id, actor_key, action, target_type, target_id, metadata
        )
        VALUES ($1, $2, $3, 'organization.created', 'organization', $4, $5)
        "#,
    )
    .bind(organization_id)
    .bind(user_id)
    .bind(&identity.subject)
    .bind(organization_id.to_string())
    .bind(serde_json::json!({ "slug": slug, "name": name }))
    .execute(&mut *transaction)
    .await
    {
        return database_error(error);
    }

    if let Err(error) = transaction.commit().await {
        return database_error(error);
    }

    (
        StatusCode::CREATED,
        Json(OrganizationSummary {
            id: organization_id,
            slug,
            name: name.to_owned(),
            role: Role::Owner.as_str().to_owned(),
        }),
    )
        .into_response()
}

pub async fn access(
    State(state): State<AppState>,
    Path(organization_id): Path<Uuid>,
    Extension(identity): Extension<AuthIdentity>,
) -> Response {
    match require_permission(
        &state.database,
        &identity,
        organization_id,
        Permission::ViewOrganization,
    )
    .await
    {
        Ok(access) => Json(access).into_response(),
        Err(error) => access_error(error),
    }
}

pub async fn members(
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

    match sqlx::query_as::<_, (Uuid, String, Option<String>, String, String)>(
        r#"
        SELECT users.id, users.subject, users.email, users.display_name, membership.role
        FROM organization_memberships membership
        INNER JOIN users ON users.id = membership.user_id
        WHERE membership.organization_id = $1
          AND users.disabled_at IS NULL
        ORDER BY users.display_name, users.id
        "#,
    )
    .bind(organization_id)
    .fetch_all(&state.database)
    .await
    {
        Ok(values) => Json(
            values
                .into_iter()
                .map(
                    |(user_id, subject, email, display_name, role)| OrganizationMember {
                        user_id,
                        subject,
                        email,
                        display_name,
                        role,
                    },
                )
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(error) => database_error(error),
    }
}

pub async fn update_member_role(
    State(state): State<AppState>,
    Path((organization_id, user_id)): Path<(Uuid, Uuid)>,
    Extension(identity): Extension<AuthIdentity>,
    Json(request): Json<UpdateMemberRoleRequest>,
) -> Response {
    let access = match require_permission(
        &state.database,
        &identity,
        organization_id,
        Permission::ManageMembers,
    )
    .await
    {
        Ok(value) => value,
        Err(error) => return access_error(error),
    };

    let new_role = match Role::from_str(request.role.trim()) {
        Ok(value) => value,
        Err(()) => {
            return validation_error(
                "membership.invalid_role",
                "The requested organization role is not valid.",
            );
        }
    };
    let reason = request.reason.trim();
    if reason.is_empty() {
        return validation_error(
            "membership.reason_required",
            "A reason is required when changing an organization role.",
        );
    }

    let current_role = match sqlx::query_scalar::<_, String>(
        "SELECT role FROM organization_memberships WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_optional(&state.database)
    .await
    {
        Ok(Some(value)) => value,
        Ok(None) => return not_found("membership.not_found", "Organization member was not found."),
        Err(error) => return database_error(error),
    };

    if (current_role == Role::Owner.as_str() || new_role == Role::Owner)
        && access.role != Role::Owner
    {
        return forbidden(
            "membership.owner_role_requires_owner",
            "Only an organization owner can grant or remove the owner role.",
        );
    }

    if current_role == Role::Owner.as_str() && new_role != Role::Owner {
        let owner_count = match sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM organization_memberships WHERE organization_id = $1 AND role = 'owner'",
        )
        .bind(organization_id)
        .fetch_one(&state.database)
        .await
        {
            Ok(value) => value,
            Err(error) => return database_error(error),
        };
        if owner_count <= 1 {
            return conflict(
                "membership.last_owner",
                "The final organization owner cannot be demoted.",
            );
        }
    }

    let actor_user_id = match current_user_id(&state, &identity).await {
        Ok(value) => value,
        Err(error) => return identity_lookup_error(error),
    };

    let mut transaction = match state.database.begin().await {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    if let Err(error) = sqlx::query(
        "UPDATE organization_memberships SET role = $3, updated_at = now() WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(organization_id)
    .bind(user_id)
    .bind(new_role.as_str())
    .execute(&mut *transaction)
    .await
    {
        return database_error(error);
    }

    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO audit_events (
            organization_id, actor_user_id, actor_key, action, target_type, target_id, reason, metadata
        )
        VALUES ($1, $2, $3, 'organization.member_role_changed', 'user', $4, $5, $6)
        "#,
    )
    .bind(organization_id)
    .bind(actor_user_id)
    .bind(&identity.subject)
    .bind(user_id.to_string())
    .bind(reason)
    .bind(serde_json::json!({
        "before_role": current_role,
        "after_role": new_role.as_str()
    }))
    .execute(&mut *transaction)
    .await
    {
        return database_error(error);
    }

    if let Err(error) = transaction.commit().await {
        return database_error(error);
    }

    StatusCode::NO_CONTENT.into_response()
}

#[derive(Debug, thiserror::Error)]
pub enum IdentityLookupError {
    #[error("authenticated identity is not provisioned")]
    NotProvisioned,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub async fn current_user_id(
    state: &AppState,
    identity: &AuthIdentity,
) -> Result<Uuid, IdentityLookupError> {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM users WHERE subject = $1 AND disabled_at IS NULL",
    )
    .bind(&identity.subject)
    .fetch_optional(&state.database)
    .await
    .map_err(IdentityLookupError::Database)?
    .ok_or(IdentityLookupError::NotProvisioned)
}

pub fn identity_lookup_error(error: IdentityLookupError) -> Response {
    match error {
        IdentityLookupError::NotProvisioned => forbidden(
            "identity.not_provisioned",
            "The authenticated identity is not provisioned.",
        ),
        IdentityLookupError::Database(error) => database_error(error),
    }
}

pub fn access_error(error: crate::rbac::AccessError) -> Response {
    match error {
        crate::rbac::AccessError::NotMember | crate::rbac::AccessError::Forbidden => forbidden(
            "authorization.denied",
            "You do not have permission for this organization.",
        ),
        crate::rbac::AccessError::UnknownRole => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": {
                    "code": "authorization.invalid_role",
                    "message": "The membership role is not recognized."
                }
            })),
        )
            .into_response(),
        crate::rbac::AccessError::Database(error) => database_error(error),
    }
}

pub fn database_error(error: sqlx::Error) -> Response {
    tracing::error!(%error, "organization query failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": {
                "code": "database.unavailable",
                "message": "Organization data is temporarily unavailable."
            }
        })),
    )
        .into_response()
}

pub fn validation_error(code: &'static str, message: &'static str) -> Response {
    error_response(StatusCode::BAD_REQUEST, code, message)
}

pub fn conflict(code: &'static str, message: &'static str) -> Response {
    error_response(StatusCode::CONFLICT, code, message)
}

pub fn forbidden(code: &'static str, message: &'static str) -> Response {
    error_response(StatusCode::FORBIDDEN, code, message)
}

pub fn not_found(code: &'static str, message: &'static str) -> Response {
    error_response(StatusCode::NOT_FOUND, code, message)
}

fn error_response(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "error": {
                "code": code,
                "message": message
            }
        })),
    )
        .into_response()
}

fn is_unique_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|value| value.code())
        .is_some_and(|code| code == "23505")
}

fn valid_slug(value: &str) -> bool {
    let length = value.len();
    (2..=63).contains(&length)
        && value.bytes().enumerate().all(|(index, byte)| match byte {
            b'a'..=b'z' | b'0'..=b'9' => true,
            b'-' => index > 0 && index + 1 < length,
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use super::valid_slug;

    #[test]
    fn validates_organization_slugs() {
        assert!(valid_slug("gitgat"));
        assert!(valid_slug("gitgat-team-01"));
        assert!(!valid_slug("-gitgat"));
        assert!(!valid_slug("gitgat-"));
        assert!(!valid_slug("A"));
        assert!(!valid_slug("with space"));
    }
}
