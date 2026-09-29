#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyMode {
    Observe,
    Warn,
    Enforce,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockObservation {
    pub verified_at_epoch: i64,
    pub expires_at_epoch: i64,
}

impl LockObservation {
    pub fn is_fresh_at(&self, now_epoch: i64) -> bool {
        self.verified_at_epoch <= now_epoch && now_epoch < self.expires_at_epoch
    }
}

pub fn normalize_pattern(value: &str) -> Option<String> {
    let value = value.trim().replace('\\', "/");
    if value.is_empty() || value.starts_with('/') || value.contains("../") {
        return None;
    }
    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_observations_are_never_fresh() {
        let observation = LockObservation {
            verified_at_epoch: 100,
            expires_at_epoch: 200,
        };
        assert!(observation.is_fresh_at(150));
        assert!(!observation.is_fresh_at(200));
        assert!(!observation.is_fresh_at(99));
    }

    #[test]
    fn patterns_are_repository_relative_and_normalized() {
        assert_eq!(normalize_pattern(r"assets\**\*.psd"), Some("assets/**/*.psd".into()));
        assert_eq!(normalize_pattern("../secret"), None);
        assert_eq!(normalize_pattern("/absolute"), None);
    }
}
