use crate::{
    domain::{ManagedPolicy, ManagedRepository},
    store::Catalog,
};
use anyhow::{Context, Result, bail};
use reqwest::blocking::Client;
use std::path::Path;
use std::time::Duration;

pub const POLICY_CACHE_MAX_AGE_SECONDS: i64 = 15 * 60;

#[derive(Debug, Clone)]
pub struct ControlPlaneClient {
    base_url: String,
    bearer_token: Option<String>,
    client: Client,
}

impl ControlPlaneClient {
    pub fn new(base_url: impl Into<String>, bearer_token: Option<String>) -> Result<Self> {
        let base_url = base_url.into().trim_end_matches('/').to_owned();
        if base_url.is_empty() {
            bail!("Control Plane URL is required");
        }
        Ok(Self {
            base_url,
            bearer_token,
            client: Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .context("failed to create Control Plane HTTP client")?,
        })
    }

    pub fn fetch_policy(&self, repository_id: &str) -> Result<ManagedPolicy> {
        let repository_id = repository_id.trim();
        if repository_id.is_empty() {
            bail!("Control Plane repository ID is required");
        }
        let url = format!(
            "{}/api/v1/repositories/{repository_id}/policy",
            self.base_url
        );
        let mut request = self.client.get(url);
        if let Some(token) = self
            .bearer_token
            .as_deref()
            .filter(|value| !value.is_empty())
        {
            request = request.bearer_auth(token);
        }
        request
            .send()
            .context("Control Plane policy request failed")?
            .error_for_status()
            .context("Control Plane rejected policy request")?
            .json::<ManagedPolicy>()
            .context("Control Plane returned an invalid policy payload")
    }

    pub fn refresh_policy(
        &self,
        catalog: &Catalog,
        repository_path: &Path,
        managed: &ManagedRepository,
    ) -> Result<ManagedPolicy> {
        let policy = self.fetch_policy(&managed.control_plane_repository_id)?;
        catalog.update_managed_policy(repository_path, &policy)?;
        Ok(policy)
    }
}

#[cfg(test)]
mod tests {
    use super::POLICY_CACHE_MAX_AGE_SECONDS;
    use crate::domain::{ManagedPolicy, ManagedRepository};

    #[test]
    fn managed_policy_cache_has_bounded_freshness() {
        let managed = ManagedRepository {
            control_plane_url: "https://control.example".into(),
            control_plane_repository_id: "repo-1".into(),
            policy: Some(ManagedPolicy {
                version: 3,
                protected_patterns: vec!["Assets/**/*.psd".into()],
                excluded_patterns: Vec::new(),
                required_lock: true,
                max_lock_age_minutes: Some(120),
                force_unlock_approval_required: true,
            }),
            policy_fetched_unix: Some(1000),
        };

        assert!(managed.policy_is_fresh_at(
            1000 + POLICY_CACHE_MAX_AGE_SECONDS,
            POLICY_CACHE_MAX_AGE_SECONDS
        ));
        assert!(!managed.policy_is_fresh_at(
            1001 + POLICY_CACHE_MAX_AGE_SECONDS,
            POLICY_CACHE_MAX_AGE_SECONDS
        ));
    }
}
