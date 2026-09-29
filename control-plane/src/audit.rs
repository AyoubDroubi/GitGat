use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, Row};
use uuid::Uuid;

pub fn chained_event_hash(previous_hash: Option<&str>, canonical_event: &str) -> String {
    let mut hasher = Sha256::new();
    if let Some(previous) = previous_hash {
        hasher.update(previous.as_bytes());
    }
    hasher.update(b"\n");
    hasher.update(canonical_event.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub struct AuditEventInput<'a> {
    pub organization_id: Uuid,
    pub actor_id: &'a str,
    pub actor_type: &'a str,
    pub action: &'a str,
    pub target_type: &'a str,
    pub target_id: Option<&'a str>,
    pub repository_id: Option<Uuid>,
    pub path: Option<&'a str>,
    pub reason: Option<&'a str>,
    pub outcome: &'a str,
    pub correlation_id: Option<&'a str>,
    pub evidence_reference: Option<&'a str>,
    pub payload: Value,
}

pub async fn append_event(
    connection: &mut PgConnection,
    event: AuditEventInput<'_>,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1))")
        .bind(event.organization_id.to_string())
        .execute(&mut *connection)
        .await?;

    let previous_hash = sqlx::query(
        "SELECT event_hash
         FROM audit_events
         WHERE organization_id = $1 AND event_hash IS NOT NULL
         ORDER BY occurred_at DESC, id DESC
         LIMIT 1",
    )
    .bind(event.organization_id)
    .fetch_optional(&mut *connection)
    .await?
    .and_then(|row| row.try_get::<String, _>("event_hash").ok());

    let canonical = serde_json::json!({
        "organization_id": event.organization_id,
        "actor_id": event.actor_id,
        "actor_type": event.actor_type,
        "action": event.action,
        "target_type": event.target_type,
        "target_id": event.target_id,
        "repository_id": event.repository_id,
        "path": event.path,
        "reason": event.reason,
        "outcome": event.outcome,
        "correlation_id": event.correlation_id,
        "evidence_reference": event.evidence_reference,
        "payload": event.payload,
    })
    .to_string();
    let event_hash = chained_event_hash(previous_hash.as_deref(), &canonical);

    sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO audit_events
            (organization_id, actor_id, actor_type, action, target_type, target_id,
             correlation_id, payload, repository_id, path, reason, outcome,
             evidence_reference, previous_hash, event_hash)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)
         RETURNING id",
    )
    .bind(event.organization_id)
    .bind(event.actor_id)
    .bind(event.actor_type)
    .bind(event.action)
    .bind(event.target_type)
    .bind(event.target_id)
    .bind(event.correlation_id)
    .bind(event.payload)
    .bind(event.repository_id)
    .bind(event.path)
    .bind(event.reason)
    .bind(event.outcome)
    .bind(event.evidence_reference)
    .bind(previous_hash)
    .bind(event_hash)
    .fetch_one(&mut *connection)
    .await
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
