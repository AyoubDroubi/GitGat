use crate::config::Config;
use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_jwt_auth::{Decoder, JwtDecoder, RemoteJwksDecoder};
use jsonwebtoken::{Algorithm, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub exp: usize,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub preferred_username: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthIdentity {
    pub subject: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
}

#[derive(Clone)]
pub enum AuthService {
    Oidc { decoder: Decoder<AuthClaims> },
    Development { identity: AuthIdentity },
    Unconfigured,
}

impl std::fmt::Debug for AuthService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oidc { .. } => formatter.write_str("AuthService::Oidc"),
            Self::Development { identity } => formatter
                .debug_struct("AuthService::Development")
                .field("subject", &identity.subject)
                .finish(),
            Self::Unconfigured => formatter.write_str("AuthService::Unconfigured"),
        }
    }
}

impl AuthService {
    pub async fn from_config(
        config: &Config,
    ) -> Result<(Self, Option<CancellationToken>), Box<dyn std::error::Error>> {
        if let Some(subject) = config.dev_auth_subject.as_deref() {
            if !config.bind.ip().is_loopback() {
                return Err("development authentication is allowed only on a loopback bind".into());
            }
            return Ok((
                Self::Development {
                    identity: AuthIdentity {
                        subject: subject.to_owned(),
                        email: config.dev_auth_email.clone(),
                        display_name: config.dev_auth_name.clone(),
                    },
                },
                None,
            ));
        }

        match (
            config.oidc_issuer.as_deref(),
            config.oidc_audience.as_deref(),
            config.oidc_jwks_url.as_deref(),
        ) {
            (Some(issuer), Some(audience), Some(jwks_url)) => {
                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_issuer(&[issuer]);
                validation.set_audience(&[audience]);
                validation.set_required_spec_claims(&["exp", "sub", "iss", "aud"]);

                let decoder = RemoteJwksDecoder::builder()
                    .jwks_url(jwks_url.to_owned())
                    .validation(validation)
                    .build()?;
                let decoder = Arc::new(decoder);
                let shutdown = decoder.initialize().await?;

                Ok((Self::Oidc { decoder }, Some(shutdown)))
            }
            (None, None, None) if config.bind.ip().is_loopback() => {
                Ok((Self::Unconfigured, None))
            }
            (None, None, None) => Err(
                "OIDC authentication is required when the Control Plane binds to a non-loopback address"
                    .into(),
            ),
            _ => Err(
                "GITGAT_OIDC_ISSUER, GITGAT_OIDC_AUDIENCE and GITGAT_OIDC_JWKS_URL must be configured together"
                    .into(),
            ),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Oidc { .. } => "oidc",
            Self::Development { .. } => "development",
            Self::Unconfigured => "unconfigured",
        }
    }
}

pub async fn require_auth(
    State(state): State<crate::state::AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let identity = match &state.auth {
        AuthService::Development { identity } => identity.clone(),
        AuthService::Unconfigured => {
            return auth_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "auth.not_configured",
                "Authentication is not configured.",
            );
        }
        AuthService::Oidc { decoder } => {
            let Some(token) = bearer_token(request.headers().get(header::AUTHORIZATION)) else {
                return auth_error(
                    StatusCode::UNAUTHORIZED,
                    "auth.required",
                    "A valid bearer token is required.",
                );
            };

            match decoder.decode(token).await {
                Ok(token_data) => AuthIdentity {
                    subject: token_data.claims.sub,
                    email: token_data.claims.email,
                    display_name: token_data
                        .claims
                        .name
                        .or(token_data.claims.preferred_username),
                },
                Err(error) => {
                    tracing::warn!(%error, "OIDC token validation failed");
                    return auth_error(
                        StatusCode::UNAUTHORIZED,
                        "auth.invalid_token",
                        "The bearer token is invalid or expired.",
                    );
                }
            }
        }
    };

    request.extensions_mut().insert(identity);
    next.run(request).await
}

fn bearer_token(value: Option<&axum::http::HeaderValue>) -> Option<&str> {
    let value = value?.to_str().ok()?;
    value.strip_prefix("Bearer ").filter(|token| !token.is_empty())
}

fn auth_error(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "error": {
                "code": code,
                "message": message
            }
        })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::bearer_token;
    use axum::http::HeaderValue;

    #[test]
    fn extracts_only_bearer_tokens() {
        let valid = HeaderValue::from_static("Bearer token-value");
        assert_eq!(bearer_token(Some(&valid)), Some("token-value"));

        let wrong = HeaderValue::from_static("Basic abc");
        assert_eq!(bearer_token(Some(&wrong)), None);
        assert_eq!(bearer_token(None), None);
    }
}
