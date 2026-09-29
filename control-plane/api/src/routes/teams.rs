use crate::{
    auth::AuthIdentity,
    rbac::{Permission, require_permission},
    routes::organizations::{
        access_error, conflict, current_user_id, database_error, not_found, validation_error,
    },
    state::AppState,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    slug: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertTeamMemberRequest {
    role: String,
    reason: String,
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

pub async fn create(
    State(state): State<AppState>,
    Path(organization_id): Path<Uuid>,
    Extension(identity): Extension<AuthIdentity>,
    Json(request): Json<CreateTeamRequest>,
) -> Response {
    if let Err(error) = require_permission(
        &state.database,
        &identity,
        organization_id,
        Permission::ManageMembers,
    )
    .await
    {
        return access_error(error);
    }

    let slug = request.slug.trim().to_ascii_lowercase();
    let name = request.name.trim();
    if !valid_slug(&slug) || name.is_empty() {
        return validation_error(
            "team.invalid",
            "Team name and a valid lowercase slug are required.",
        );
    }

    let actor_user_id = match current_user_id(&state, &identity).await {
        Ok(value) => value,
        Err(response) => return response,
    };

    let mut transaction = match state.database.begin().await {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    let team_id = match sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO teams (organization_id, slug, name, created_by_user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(organization_id)
    .bind(&slug)
    .bind(name)
    .bind(actor_user_id)
    .fetch_one(&mut *transaction)
    .await
    {
        Ok(value) => value,
        Err(error) if is_unique_violation(&error) => {
            return conflict("team.slug_exists", "That team slug is already in use.");
        }
        Err(error) => return database_error(error),
    };

    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO audit_events (
            organization_id, actor_user_id, actor_key, action, target_type, target_id, metadata
        )
        VALUES ($1, $2, $3, 'team.created', 'team', $4, $5)
        "#,
    )
    .bind(organization_id)
    .bind(actor_user_id)
    .bind(&identity.subject)
    .bind(team_id.to_string())
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
        Json(TeamSummary {
            id: team_id,
            slug,
            name: name.to_owned(),
            members: 0,
        }),
    )
        .into_response()
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

pub async fn upsert_member(
    State(state): State<AppState>,
    Path((organization_id, team_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
    Extension(identity): Extension<AuthIdentity>,
    Json(request): Json<UpsertTeamMemberRequest>,
) -> Response {
    if let Err(error) = require_permission(
        &state.database,
        &identity,
        organization_id,
        Permission::ManageMembers,
    )
    .await
    {
        return access_error(error);
    }

    let role = request.role.trim();
    if !matches!(role, "lead" | "member") {
        return validation_error(
            "team_membership.invalid_role",
            "Team role must be lead or member.",
        );
    }
    let reason = request.reason.trim();
    if reason.is_empty() {
        return validation_error(
            "team_membership.reason_required",
            "A reason is required when changing team membership.",
        );
    }

    if !team_exists(&state, organization_id, team_id).await {
        return not_found("team.not_found", "Team was not found.");
    }
    if !organization_member_exists(&state, organization_id, user_id).await {
        return validation_error(
            "team_membership.user_not_in_organization",
            "A team member must already belong to the organization.",
        );
    }

    let actor_user_id = match current_user_id(&state, &identity).await {
        Ok(value) => value,
        Err(response) => return response,
    };

    let mut transaction = match state.database.begin().await {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    let before = match sqlx::query_scalar::<_, String>(
        "SELECT role FROM team_memberships WHERE team_id = $1 AND user_id = $2",
    )
    .bind(team_id)
    .bind(user_id)
    .fetch_optional(&mut *transaction)
    .await
    {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO team_memberships (team_id, user_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (team_id, user_id) DO UPDATE SET
            role = EXCLUDED.role,
            updated_at = now()
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(role)
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
        VALUES ($1, $2, $3, 'team.member_upserted', 'user', $4, $5, $6)
        "#,
    )
    .bind(organization_id)
    .bind(actor_user_id)
    .bind(&identity.subject)
    .bind(user_id.to_string())
    .bind(reason)
    .bind(serde_json::json!({
        "team_id": team_id,
        "before_role": before,
        "after_role": role
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

pub async fn remove_member(
    State(state): State<AppState>,
    Path((organization_id, team_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
    Extension(identity): Extension<AuthIdentity>,
) -> Response {
    if let Err(error) = require_permission(
        &state.database,
        &identity,
        organization_id,
        Permission::ManageMembers,
    )
    .await
    {
        return access_error(error);
    }

    let actor_user_id = match current_user_id(&state, &identity).await {
        Ok(value) => value,
        Err(response) => return response,
    };

    let mut transaction = match state.database.begin().await {
        Ok(value) => value,
        Err(error) => return database_error(error),
    };

    let removed_role = match sqlx::query_scalar::<_, String>(
        r#"
        DELETE FROM team_memberships membership
        USING teams team
        WHERE membership.team_id = team.id
          AND team.organization_id = $1
          AND team.id = $2
          AND membership.user_id = $3
        RETURNING membership.role
        "#,
    )
    .bind(organization_id)
    .bind(team_id)
    .bind(user_id)
    .fetch_optional(&mut *transaction)
    .await
    {
        Ok(Some(value)) => value,
        Ok(None) => {
            return not_found(
                "team_membership.not_found",
                "Team membership was not found.",
            );
        }
        Err(error) => return database_error(error),
    };

    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO audit_events (
            organization_id, actor_user_id, actor_key, action, target_type, target_id, metadata
        )
        VALUES ($1, $2, $3, 'team.member_removed', 'user', $4, $5)
        "#,
    )
    .bind(organization_id)
    .bind(actor_user_id)
    .bind(&identity.subject)
    .bind(user_id.to_string())
    .bind(serde_json::json!({
        "team_id": team_id,
        "removed_role": removed_role
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

async fn team_exists(state: &AppState, organization_id: Uuid, team_id: Uuid) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM teams WHERE organization_id = $1 AND id = $2)",
    )
    .bind(organization_id)
    .bind(team_id)
    .fetch_one(&state.database)
    .await
    .unwrap_or(false)
}

async fn organization_member_exists(
    state: &AppState,
    organization_id: Uuid,
    user_id: Uuid,
) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM organization_memberships WHERE organization_id = $1 AND user_id = $2)",
    )
    .bind(organization_id)
    .bind(user_id)
    .fetch_one(&state.database)
    .await
    .unwrap_or(false)
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
    fn validates_team_slugs() {
        assert!(valid_slug("platform"));
        assert!(valid_slug("game-assets"));
        assert!(!valid_slug("-platform"));
        assert!(!valid_slug("platform-"));
        assert!(!valid_slug("X"));
    }
}
