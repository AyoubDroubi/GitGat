use crate::{error::ApiError, state::AppState};
use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, request::Parts},
};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;
const MAX_AUTH_AGE_SECONDS: i64 = 300;

#[derive(Debug, Clone)]
pub struct AuthenticatedIdentity {
    pub subject: String,
}

pub fn auth_signature(secret: &[u8], subject: &str, timestamp: i64) -> String {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts arbitrary key lengths");
    mac.update(subject.as_bytes());
    mac.update(b"\n");
    mac.update(timestamp.to_string().as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn verify_headers(headers: &HeaderMap, secret: &[u8], now: i64) -> Result<String, ApiError> {
    let subject = headers
        .get("x-gitgat-subject")
        .and_then(|value| value.to_str().ok())
        .filter(|value| !value.trim().is_empty())
        .ok_or(ApiError::Unauthorized)?;

    let timestamp = headers
        .get("x-gitgat-auth-timestamp")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or(ApiError::Unauthorized)?;

    if (now - timestamp).abs() > MAX_AUTH_AGE_SECONDS {
        return Err(ApiError::Unauthorized);
    }

    let provided = headers
        .get("x-gitgat-auth-signature")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| hex::decode(value).ok())
        .ok_or(ApiError::Unauthorized)?;

    let mut verifier = HmacSha256::new_from_slice(secret).map_err(|_| ApiError::Unauthorized)?;
    verifier.update(subject.as_bytes());
    verifier.update(b"\n");
    verifier.update(timestamp.to_string().as_bytes());
    verifier
        .verify_slice(&provided)
        .map_err(|_| ApiError::Unauthorized)?;

    Ok(subject.to_owned())
}

impl FromRequestParts<AppState> for AuthenticatedIdentity {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let secret = state
            .auth_proxy_secret
            .as_deref()
            .ok_or(ApiError::Unauthorized)?;
        let subject = verify_headers(&parts.headers, secret, Utc::now().timestamp())?;
        Ok(Self { subject })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn signed_identity_is_verified_and_replay_window_is_enforced() {
        let secret = b"test-secret";
        let now = 1_800_000_000;
        let subject = "oidc|alice";
        let signature = auth_signature(secret, subject, now);

        let mut headers = HeaderMap::new();
        headers.insert("x-gitgat-subject", HeaderValue::from_static("oidc|alice"));
        headers.insert(
            "x-gitgat-auth-timestamp",
            HeaderValue::from_str(&now.to_string()).expect("timestamp header"),
        );
        headers.insert(
            "x-gitgat-auth-signature",
            HeaderValue::from_str(&signature).expect("signature header"),
        );

        assert_eq!(
            verify_headers(&headers, secret, now).expect("valid auth"),
            subject
        );
        assert!(verify_headers(&headers, secret, now + 301).is_err());
    }
}
