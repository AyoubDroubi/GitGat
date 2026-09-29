use crate::auth::AuthIdentity;
use serde::Serialize;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Admin,
    RepositoryAdmin,
    TeamLead,
    Developer,
    Auditor,
}

impl FromStr for Role {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "repository_admin" => Ok(Self::RepositoryAdmin),
            "team_lead" => Ok(Self::TeamLead),
            "developer" => Ok(Self::Developer),
            "auditor" => Ok(Self::Auditor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    ViewOrganization,
    ManageOrganization,
    ViewRepositories,
    ManageRepositories,
    ViewLocks,
    RequestForceUnlock,
    ApproveForceUnlock,
    ManagePolicies,
    ManageExceptions,
    ViewAudit,
    ManageProviderConnections,
    ManageMembers,
}

impl Role {
    pub fn permissions(self) -> &'static [Permission] {
        use Permission::*;

        match self {
            Self::Owner => &[
                ViewOrganization,
                ManageOrganization,
                ViewRepositories,
                ManageRepositories,
                ViewLocks,
                RequestForceUnlock,
                ApproveForceUnlock,
                ManagePolicies,
                ManageExceptions,
                ViewAudit,
                ManageProviderConnections,
                ManageMembers,
            ],
            Self::Admin => &[
                ViewOrganization,
                ManageOrganization,
                ViewRepositories,
                ManageRepositories,
                ViewLocks,
                RequestForceUnlock,
                ApproveForceUnlock,
                ManagePolicies,
                ManageExceptions,
                ViewAudit,
                ManageProviderConnections,
                ManageMembers,
            ],
            Self::RepositoryAdmin => &[
                ViewOrganization,
                ViewRepositories,
                ManageRepositories,
                ViewLocks,
                RequestForceUnlock,
                ApproveForceUnlock,
                ManagePolicies,
                ManageExceptions,
                ViewAudit,
                ManageProviderConnections,
            ],
            Self::TeamLead => &[
                ViewOrganization,
                ViewRepositories,
                ViewLocks,
                RequestForceUnlock,
                ManageExceptions,
                ViewAudit,
            ],
            Self::Developer => &[
                ViewOrganization,
                ViewRepositories,
                ViewLocks,
                RequestForceUnlock,
            ],
            Self::Auditor => &[ViewOrganization, ViewRepositories, ViewLocks, ViewAudit],
        }
    }

    pub fn allows(self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationAccess {
    pub organization_id: Uuid,
    pub role: Role,
    pub permissions: Vec<Permission>,
}

pub async fn require_permission(
    database: &PgPool,
    identity: &AuthIdentity,
    organization_id: Uuid,
    permission: Permission,
) -> Result<OrganizationAccess, AccessError> {
    let role = sqlx::query_scalar::<_, String>(
        r#"
        SELECT membership.role
        FROM organization_memberships membership
        INNER JOIN users ON users.id = membership.user_id
        WHERE membership.organization_id = $1
          AND users.subject = $2
          AND users.disabled_at IS NULL
        "#,
    )
    .bind(organization_id)
    .bind(&identity.subject)
    .fetch_optional(database)
    .await
    .map_err(AccessError::Database)?
    .ok_or(AccessError::NotMember)?;

    let role = Role::from_str(&role).map_err(|_| AccessError::UnknownRole)?;
    if !role.allows(permission) {
        return Err(AccessError::Forbidden);
    }

    Ok(OrganizationAccess {
        organization_id,
        role,
        permissions: role.permissions().to_vec(),
    })
}

#[derive(Debug, thiserror::Error)]
pub enum AccessError {
    #[error("not a member of the organization")]
    NotMember,
    #[error("permission denied")]
    Forbidden,
    #[error("membership has an unknown role")]
    UnknownRole,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[cfg(test)]
mod tests {
    use super::{Permission, Role};

    #[test]
    fn developer_cannot_approve_force_unlock_or_manage_policy() {
        assert!(Role::Developer.allows(Permission::ViewLocks));
        assert!(Role::Developer.allows(Permission::RequestForceUnlock));
        assert!(!Role::Developer.allows(Permission::ApproveForceUnlock));
        assert!(!Role::Developer.allows(Permission::ManagePolicies));
    }

    #[test]
    fn auditor_is_read_only() {
        assert!(Role::Auditor.allows(Permission::ViewAudit));
        assert!(Role::Auditor.allows(Permission::ViewLocks));
        assert!(!Role::Auditor.allows(Permission::ManageRepositories));
        assert!(!Role::Auditor.allows(Permission::RequestForceUnlock));
    }

    #[test]
    fn admins_have_full_governance_permissions() {
        for role in [Role::Owner, Role::Admin] {
            assert!(role.allows(Permission::ManageOrganization));
            assert!(role.allows(Permission::ManagePolicies));
            assert!(role.allows(Permission::ApproveForceUnlock));
            assert!(role.allows(Permission::ManageMembers));
        }
    }
}
