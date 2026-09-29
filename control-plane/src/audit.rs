use sha2::{Digest, Sha256};

pub fn chained_event_hash(previous_hash: Option<&str>, canonical_event: &str) -> String {
    let mut hasher = Sha256::new();
    if let Some(previous) = previous_hash {
        hasher.update(previous.as_bytes());
    }
    hasher.update(b"\n");
    hasher.update(canonical_event.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_hash_changes_with_history_or_payload() {
        let a = chained_event_hash(None, "event-a");
        let b = chained_event_hash(Some(&a), "event-b");
        let altered = chained_event_hash(Some(&a), "event-c");
        assert_ne!(a, b);
        assert_ne!(b, altered);
    }
}
