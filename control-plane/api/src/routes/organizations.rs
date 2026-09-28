use crate::{
    auth::AuthIdentity,
    rbac::{Permission, require_permission},
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
pub struct OrganizationSummary {
    id: Uuid,
    slug: String,
    name: String,
    role: String,
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

pub fn access_error(error: crate::rbac::AccessError) -> Response {
    match error {
        crate::rbac::AccessError::NotMember | crate::rbac::AccessError::Forbidden => (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": {
                    "code": "authorization.denied",
                    "message": "You do not have permission for this organization."
                }
            })),
        )
            .into_response(),
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

fn database_error(error: sqlx::Error) -> Response {
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
