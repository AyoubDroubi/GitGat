#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StaleState {
    Fresh,
    Warning,
    Escalated,
}

pub fn classify_lock_age(
    age_minutes: i64,
    warning_after_minutes: i64,
    escalate_after_minutes: i64,
) -> StaleState {
    if age_minutes >= escalate_after_minutes {
        StaleState::Escalated
    } else if age_minutes >= warning_after_minutes {
        StaleState::Warning
    } else {
        StaleState::Fresh
    }
}

pub fn automatic_force_unlock_allowed() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_flow_never_auto_unlocks() {
        assert_eq!(classify_lock_age(30, 60, 120), StaleState::Fresh);
        assert_eq!(classify_lock_age(60, 60, 120), StaleState::Warning);
        assert_eq!(classify_lock_age(120, 60, 120), StaleState::Escalated);
        assert!(!automatic_force_unlock_allowed());
    }
}
