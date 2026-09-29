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
pub struct TeamSummary {
    id: Uuid,
    slug: String,
    name: String,
    members: i64,
}

#[derive(Debug, Serialize)]
pub struct TeamMember {
    user_id: Uuid,
    subject: String,
    email: Option<String>,
    display_name: String,
    role: String,
}

pub async fn list(
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

    match sqlx::query_as::<_, (Uuid, String, String, i64)>(
        r#"
        SELECT team.id, team.slug, team.name, count(membership.id)
        FROM teams team
        LEFT JOIN team_memberships membership ON membership.team_id = team.id
        WHERE team.organization_id = $1
        GROUP BY team.id, team.slug, team.name
        ORDER BY team.name, team.id
        "#,
    )
    .bind(organization_id)
    .fetch_all(&state.database)
    .await
    {
        Ok(values) => Json(
            values
                .into_iter()
                .map(|(id, slug, name, members)| TeamSummary {
                    id,
                    slug,
                    name,
                    members,
                })
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(error) => database_error(error),
    }
}

pub async fn members(
    State(state): State<AppState>,
    Path((organization_id, team_id)): Path<(Uuid, Uuid)>,
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
        FROM team_memberships membership
        INNER JOIN teams team ON team.id = membership.team_id
        INNER JOIN users ON users.id = membership.user_id
        WHERE team.organization_id = $1
          AND team.id = $2
          AND users.disabled_at IS NULL
        ORDER BY users.display_name, users.id
        "#,
    )
    .bind(organization_id)
    .bind(team_id)
    .fetch_all(&state.database)
    .await
    {
        Ok(values) => Json(
            values
                .into_iter()
                .map(|(user_id, subject, email, display_name, role)| TeamMember {
                    user_id,
                    subject,
                    email,
                    display_name,
                    role,
                })
                .collect::<Vec<_>>(),
        )
        .into_response(),
        Err(error) => database_error(error),
    }
}

fn database_error(error: sqlx::Error) -> Response {
    tracing::error!(%error, "team query failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(serde_json::json!({
            "error": {
                "code": "database.unavailable",
                "message": "Team data is temporarily unavailable."
            }
        })),
    )
        .into_response()
}
