#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockIdentity {
    pub provider_lock_id: String,
    pub path: String,
    pub owner: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForceUnlockStatus {
    Requested,
    Approved,
    Rejected,
    Executed,
    Cancelled,
}

pub fn live_lock_matches_expected(expected: &LockIdentity, live: &LockIdentity) -> bool {
    expected == live
}

pub fn approval_allowed(requester: &str, approver: &str, separation_of_duty: bool) -> bool {
    !separation_of_duty || requester != approver
}

pub fn exception_is_active(starts_at_epoch: i64, expires_at_epoch: i64, now_epoch: i64) -> bool {
    starts_at_epoch <= now_epoch && now_epoch < expires_at_epoch
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lock(owner: &str) -> LockIdentity {
        LockIdentity {
            provider_lock_id: "lock-1".into(),
            path: "assets/model.psd".into(),
            owner: owner.into(),
        }
    }

    #[test]
    fn force_unlock_revalidation_fails_if_owner_changes() {
        assert!(live_lock_matches_expected(&lock("a"), &lock("a")));
        assert!(!live_lock_matches_expected(&lock("a"), &lock("b")));
    }

    #[test]
    fn separation_of_duty_blocks_requester() {
        assert!(!approval_allowed("a", "a", true));
        assert!(approval_allowed("a", "b", true));
    }

    #[test]
    fn exceptions_expire_exactly_at_end() {
        assert!(exception_is_active(100, 200, 199));
        assert!(!exception_is_active(100, 200, 200));
    }
}
