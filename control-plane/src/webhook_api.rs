use crate::{
    error::ApiError, secrets::resolve_secret, state::AppState, webhooks::verify_sha256_signature,
};
use axum::{
    Json, Router,
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::post,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::Row;
use subtle::ConstantTimeEq;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new().route("/webhooks/{connection_id}", post(ingest))
}

#[derive(Serialize)]
struct WebhookResponse {
    status: &'static str,
    delivery_id: String,
}

async fn ingest(
    State(state): State<AppState>,
    Path(connection_id): Path<Uuid>,
    headers: HeaderMap,
    payload: Bytes,
) -> Result<(StatusCode, Json<WebhookResponse>), ApiError> {
    let connection = sqlx::query(
        "SELECT organization_id, provider, secret_reference
         FROM provider_connections
         WHERE id = $1",
    )
    .bind(connection_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    let organization_id: Uuid = connection.get("organization_id");
    let provider: String = connection.get("provider");
    let secret_reference: String = connection.get("secret_reference");
    let secret = resolve_secret(&secret_reference)?;

    let (delivery_id, event_type) = match provider.as_str() {
        "github" => authenticate_github(&headers, &payload, &secret)?,
        "azure_devops" => authenticate_azure(&headers, &payload, &secret)?,
        _ => return Err(ApiError::BadRequest("unsupported provider".into())),
    };

    let payload_hash = format!("{:x}", Sha256::digest(&payload));
    let repository_external_id = extract_repository_id(&provider, &payload);
    let mut tx = state.pool.begin().await?;

    let inserted = sqlx::query(
        "INSERT INTO webhook_deliveries
            (provider_connection_id, delivery_id, event_type, payload_sha256, status)
         VALUES ($1,$2,$3,$4,'received')
         ON CONFLICT (provider_connection_id, delivery_id) DO NOTHING",
    )
    .bind(connection_id)
    .bind(&delivery_id)
    .bind(&event_type)
    .bind(&payload_hash)
    .execute(&mut *tx)
    .await?;

    if inserted.rows_affected() == 0 {
        tx.commit().await?;
        return Ok((
            StatusCode::ACCEPTED,
            Json(WebhookResponse {
                status: "duplicate",
                delivery_id,
            }),
        ));
    }

    let dedupe_key = format!("{connection_id}:{delivery_id}");
    sqlx::query(
        "INSERT INTO background_jobs
            (organization_id, kind, dedupe_key, payload, status)
         VALUES ($1,'provider_webhook',$2,$3,'queued')
         ON CONFLICT DO NOTHING",
    )
    .bind(organization_id)
    .bind(dedupe_key)
    .bind(serde_json::json!({
        "connection_id": connection_id,
        "provider": provider,
        "delivery_id": delivery_id,
        "event_type": event_type,
        "payload_sha256": payload_hash,
        "repository_external_id": repository_external_id,
    }))
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(WebhookResponse {
            status: "queued",
            delivery_id,
        }),
    ))
}

fn extract_repository_id(provider: &str, payload: &[u8]) -> Option<String> {
    let json: serde_json::Value = serde_json::from_slice(payload).ok()?;
    match provider {
        "github" => json.pointer("/repository/id").and_then(|value| {
            value
                .as_u64()
                .map(|id| id.to_string())
                .or_else(|| value.as_str().map(str::to_owned))
        }),
        "azure_devops" => json
            .pointer("/resource/repository/id")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        _ => None,
    }
}

fn authenticate_github(
    headers: &HeaderMap,
    payload: &[u8],
    secret: &[u8],
) -> Result<(String, String), ApiError> {
    let signature = required_header(headers, "x-hub-signature-256")?;
    if !verify_sha256_signature(secret, payload, &signature) {
        return Err(ApiError::Unauthorized);
    }
    let delivery = required_header(headers, "x-github-delivery")?;
    let event = required_header(headers, "x-github-event")?;
    Ok((delivery, event))
}

fn authenticate_azure(
    headers: &HeaderMap,
    payload: &[u8],
    expected_credentials: &[u8],
) -> Result<(String, String), ApiError> {
    let authorization = required_header(headers, "authorization")?;
    let encoded = authorization
        .strip_prefix("Basic ")
        .ok_or(ApiError::Unauthorized)?;
    let decoded = STANDARD
        .decode(encoded)
        .map_err(|_| ApiError::Unauthorized)?;
    if !bool::from(decoded.ct_eq(expected_credentials)) {
        return Err(ApiError::Unauthorized);
    }

    let json: serde_json::Value =
        serde_json::from_slice(payload).map_err(|_| ApiError::BadRequest("invalid JSON".into()))?;
    let delivery = json
        .get("id")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError::BadRequest("Azure webhook event id is required".into()))?
        .to_owned();
    let event = json
        .get("eventType")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError::BadRequest("Azure webhook eventType is required".into()))?
        .to_owned();

    Ok((delivery, event))
}

fn required_header(headers: &HeaderMap, name: &'static str) -> Result<String, ApiError> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or(ApiError::Unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn azure_basic_auth_is_checked_constant_time_against_injected_secret() {
        let payload = br#"{"id":"event-1","eventType":"git.push"}"#;
        let credentials = b"gitgat:secret";
        let header = format!("Basic {}", STANDARD.encode(credentials));
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str(&header).expect("authorization header"),
        );

        let (delivery, event) =
            authenticate_azure(&headers, payload, credentials).expect("valid Azure hook");
        assert_eq!(delivery, "event-1");
        assert_eq!(event, "git.push");
        assert!(authenticate_azure(&headers, payload, b"wrong").is_err());
    }
}
