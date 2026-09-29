use crate::error::ApiError;

pub fn resolve_secret(reference: &str) -> Result<Vec<u8>, ApiError> {
    let Some(variable) = reference.strip_prefix("env:") else {
        return Err(ApiError::BadRequest(
            "unsupported secret reference; inject secrets with env:<NAME> or a deployment secret adapter"
                .into(),
        ));
    };
    if variable.is_empty()
        || !variable
            .chars()
            .all(|value| value.is_ascii_uppercase() || value.is_ascii_digit() || value == '_')
    {
        return Err(ApiError::BadRequest("invalid secret environment reference".into()));
    }
    std::env::var(variable)
        .map(|value| value.into_bytes())
        .map_err(|_| ApiError::Unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_secret_store_references() {
        assert!(resolve_secret("plaintext:secret").is_err());
        assert!(resolve_secret("env:bad-name").is_err());
    }
}
