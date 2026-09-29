use serde_json::Value;
use sqlx::{PgPool, Row};
use tokio::time::{Duration, sleep};
use uuid::Uuid;

const MAX_ATTEMPTS: i32 = 5;

fn should_dead_letter(attempts: i32) -> bool {
    attempts >= MAX_ATTEMPTS
}

pub async fn run(pool: PgPool) {
    loop {
        match process_one(&pool).await {
            Ok(true) => {}
            Ok(false) => sleep(Duration::from_secs(2)).await,
            Err(error) => {
                tracing::error!(error = %error, "background worker iteration failed");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn process_one(pool: &PgPool) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let job = sqlx::query(
        "SELECT id, organization_id, kind, payload, attempts
         FROM background_jobs
         WHERE status = 'queued'
           AND next_attempt_at <= now()
           AND kind = 'provider_webhook'
         ORDER BY created_at
         FOR UPDATE SKIP LOCKED
         LIMIT 1",
    )
    .fetch_optional(&mut *tx)
    .await?;

    let Some(job) = job else {
        tx.commit().await?;
        return Ok(false);
    };

    let id: Uuid = job.get("id");
    let organization_id: Option<Uuid> = job.get("organization_id");
    let payload: Value = job.get("payload");
    let attempts: i32 = job.get("attempts");

    sqlx::query(
        "UPDATE background_jobs
         SET status = 'running', attempts = attempts + 1, updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    if let Err(error) = process_webhook(pool, organization_id, &payload).await {
        fail_job(pool, id, attempts + 1, &error.to_string()).await?;
    } else {
        sqlx::query(
            "UPDATE background_jobs
             SET status = 'succeeded', last_error = NULL, updated_at = now()
             WHERE id = $1",
        )
        .bind(id)
        .execute(pool)
        .await?;
    }

    Ok(true)
}

async fn process_webhook(
    pool: &PgPool,
    organization_id: Option<Uuid>,
    payload: &Value,
) -> Result<(), sqlx::Error> {
    let connection_id = payload
        .get("connection_id")
        .and_then(Value::as_str)
        .and_then(|value| Uuid::parse_str(value).ok());
    let delivery_id = payload.get("delivery_id").and_then(Value::as_str);
    let provider = payload.get("provider").and_then(Value::as_str);
    let repository_external_id = payload
        .get("repository_external_id")
        .and_then(Value::as_str);

    if let (Some(connection_id), Some(delivery_id)) = (connection_id, delivery_id) {
        sqlx::query(
            "UPDATE webhook_deliveries
             SET status = 'processed', processed_at = now()
             WHERE provider_connection_id = $1 AND delivery_id = $2",
        )
        .bind(connection_id)
        .bind(delivery_id)
        .execute(pool)
        .await?;
    }

    if let (Some(organization_id), Some(provider), Some(repository_external_id)) =
        (organization_id, provider, repository_external_id)
    {
        if let Some(repository_id) = sqlx::query_scalar::<_, Uuid>(
            "SELECT id
             FROM repository_registrations
             WHERE organization_id = $1
               AND provider = $2
               AND provider_repository_id = $3",
        )
        .bind(organization_id)
        .bind(provider)
        .bind(repository_external_id)
        .fetch_optional(pool)
        .await?
        {
            let dedupe_key = format!("reconcile:{repository_id}");
            sqlx::query(
                "INSERT INTO background_jobs
                    (organization_id, kind, dedupe_key, payload, status)
                 VALUES ($1,'reconcile_repository',$2,$3,'queued')
                 ON CONFLICT DO NOTHING",
            )
            .bind(organization_id)
            .bind(dedupe_key)
            .bind(serde_json::json!({
                "repository_id": repository_id,
                "trigger": "webhook",
            }))
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

async fn fail_job(pool: &PgPool, id: Uuid, attempts: i32, error: &str) -> Result<(), sqlx::Error> {
    if should_dead_letter(attempts) {
        sqlx::query(
            "UPDATE background_jobs
             SET status = 'dead_letter', last_error = $2, updated_at = now()
             WHERE id = $1",
        )
        .bind(id)
        .bind(error)
        .execute(pool)
        .await?;
    } else {
        let backoff_seconds = i64::from(30 * 2_i32.pow((attempts - 1).max(0) as u32)).min(3600);
        sqlx::query(
            "UPDATE background_jobs
             SET status = 'queued',
                 last_error = $2,
                 next_attempt_at = now() + make_interval(secs => $3::double precision),
                 updated_at = now()
             WHERE id = $1",
        )
        .bind(id)
        .bind(error)
        .bind(backoff_seconds as f64)
        .execute(pool)
        .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::should_dead_letter;

    #[test]
    fn retry_policy_dead_letters_after_five_attempts() {
        assert!(!should_dead_letter(4));
        assert!(should_dead_letter(5));
    }
}
