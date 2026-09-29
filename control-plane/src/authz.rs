use std::collections::HashSet;

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
    pub fn permissions(self) -> HashSet<Permission> {
        use Permission::*;
        match self {
            Self::OrganizationOwner => [
                ViewRepositories, ManageEnrollment, ManageProtectedPatterns, ViewActiveLocks,
                RequestForceUnlock, ApproveForceUnlock, ManageExceptions, ViewAudit,
                ManageProviderConnections, ManageMembers,
            ].into_iter().collect(),
            Self::OrganizationAdmin => [
                ViewRepositories, ManageEnrollment, ManageProtectedPatterns, ViewActiveLocks,
                RequestForceUnlock, ApproveForceUnlock, ManageExceptions, ViewAudit,
                ManageProviderConnections, ManageMembers,
            ].into_iter().collect(),
            Self::RepositoryAdmin => [
                ViewRepositories, ManageEnrollment, ManageProtectedPatterns, ViewActiveLocks,
                RequestForceUnlock, ApproveForceUnlock, ManageExceptions, ViewAudit,
            ].into_iter().collect(),
            Self::TeamLead => [
                ViewRepositories, ViewActiveLocks, RequestForceUnlock, ApproveForceUnlock, ViewAudit,
            ].into_iter().collect(),
            Self::Developer => [
                ViewRepositories, ViewActiveLocks, RequestForceUnlock,
            ].into_iter().collect(),
            Self::Auditor => [ViewRepositories, ViewActiveLocks, ViewAudit].into_iter().collect(),
        }
    }

    pub fn allows(self, permission: Permission) -> bool {
        self.permissions().contains(&permission)
    }
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
