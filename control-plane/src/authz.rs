use crate::error::ApiError;
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    ViewRepositories,
    ManageEnrollment,
    ManageProtectedPatterns,
    ViewActiveLocks,
    RequestForceUnlock,
    ApproveForceUnlock,
    ManageExceptions,
    ViewAudit,
    ManageProviderConnections,
    ManageMembers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    OrganizationOwner,
    OrganizationAdmin,
    RepositoryAdmin,
    TeamLead,
    Developer,
    Auditor,
}

impl Role {
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "organization_owner" => Some(Self::OrganizationOwner),
            "organization_admin" => Some(Self::OrganizationAdmin),
            "repository_admin" => Some(Self::RepositoryAdmin),
            "team_lead" => Some(Self::TeamLead),
            "developer" => Some(Self::Developer),
            "auditor" => Some(Self::Auditor),
            _ => None,
        }
    }

    pub fn permissions(self) -> HashSet<Permission> {
        use Permission::*;
        match self {
            Self::OrganizationOwner | Self::OrganizationAdmin => [
                ViewRepositories,
                ManageEnrollment,
                ManageProtectedPatterns,
                ViewActiveLocks,
                RequestForceUnlock,
                ApproveForceUnlock,
                ManageExceptions,
                ViewAudit,
                ManageProviderConnections,
                ManageMembers,
            ]
            .into_iter()
            .collect(),
            Self::RepositoryAdmin => [
                ViewRepositories,
                ManageEnrollment,
                ManageProtectedPatterns,
                ViewActiveLocks,
                RequestForceUnlock,
                ApproveForceUnlock,
                ManageExceptions,
                ViewAudit,
            ]
            .into_iter()
            .collect(),
            Self::TeamLead => [
                ViewRepositories,
                ViewActiveLocks,
                RequestForceUnlock,
                ApproveForceUnlock,
                ViewAudit,
            ]
            .into_iter()
            .collect(),
            Self::Developer => [ViewRepositories, ViewActiveLocks, RequestForceUnlock]
                .into_iter()
                .collect(),
            Self::Auditor => [ViewRepositories, ViewActiveLocks, ViewAudit]
                .into_iter()
                .collect(),
        }
    }

    pub fn allows(self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
}

pub async fn require_org_permission(
    pool: &PgPool,
    subject: &str,
    organization_id: Uuid,
    permission: Permission,
) -> Result<Uuid, ApiError> {
    let row = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT u.id, m.role
         FROM users u
         JOIN memberships m ON m.user_id = u.id
         WHERE u.external_subject = $1 AND m.organization_id = $2",
    )
    .bind(subject)
    .bind(organization_id)
    .fetch_optional(pool)
    .await?;

    let Some((user_id, role_name)) = row else {
        return Err(ApiError::Forbidden);
    };
    let role = Role::from_db(&role_name).ok_or(ApiError::Forbidden)?;
    if !role.allows(permission) {
        return Err(ApiError::Forbidden);
    }
    Ok(user_id)
}

pub async fn repository_organization(
    pool: &PgPool,
    repository_id: Uuid,
) -> Result<Uuid, ApiError> {
    sqlx::query_scalar::<_, Uuid>(
        "SELECT organization_id FROM repository_registrations WHERE id = $1",
    )
    .bind(repository_id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)
}

pub async fn require_repository_permission(
    pool: &PgPool,
    subject: &str,
    repository_id: Uuid,
    permission: Permission,
) -> Result<Uuid, ApiError> {
    let organization_id = repository_organization(pool, repository_id).await?;

    if let Ok(user_id) = require_org_permission(pool, subject, organization_id, permission).await {
        return Ok(user_id);
    }

    let row = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT u.id, r.role
         FROM users u
         JOIN repository_role_assignments r ON r.user_id = u.id
         WHERE u.external_subject = $1 AND r.repository_id = $2",
    )
    .bind(subject)
    .bind(repository_id)
    .fetch_optional(pool)
    .await?;

    let Some((user_id, role_name)) = row else {
        return Err(ApiError::Forbidden);
    };
    let role = Role::from_db(&role_name).ok_or(ApiError::Forbidden)?;
    if !role.allows(permission) {
        return Err(ApiError::Forbidden);
    }
    Ok(user_id)
}

pub fn separation_of_duty_allows(requester: &str, approver: &str, enabled: bool) -> bool {
    !enabled || requester != approver
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn developer_is_least_privilege() {
        let role = Role::Developer;
        assert!(role.allows(Permission::ViewRepositories));
        assert!(role.allows(Permission::RequestForceUnlock));
        assert!(!role.allows(Permission::ApproveForceUnlock));
        assert!(!role.allows(Permission::ManageMembers));
    }

    #[test]
    fn auditor_is_read_only() {
        let role = Role::Auditor;
        assert!(role.allows(Permission::ViewAudit));
        assert!(!role.allows(Permission::ManageEnrollment));
        assert!(!role.allows(Permission::RequestForceUnlock));
    }

    #[test]
    fn separation_of_duty_blocks_self_approval() {
        assert!(!separation_of_duty_allows("user-a", "user-a", true));
        assert!(separation_of_duty_allows("user-a", "user-b", true));
        assert!(separation_of_duty_allows("user-a", "user-a", false));
    }
}
