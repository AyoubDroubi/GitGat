use crate::domain::{
    ActivityInfo, ForgeSnapshot, NotificationInfo, PullRequestInfo, WorkflowRunInfo,
};
use crate::process::ProcessRunner;
use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeProviderKind {
    GitHub,
    AzureDevOps,
}

impl ForgeProviderKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::GitHub => "GitHub",
            Self::AzureDevOps => "Azure DevOps",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzureRepoIdentity {
    pub organization: String,
    pub organization_url: String,
    pub project: String,
    pub repository: String,
    pub ssh_remote: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ForgeClient {
    runner: ProcessRunner,
}

impl ForgeClient {
    pub fn provider(&self, repo: &Path) -> Result<ForgeProviderKind> {
        let url = self.origin(repo)?;
        detect_provider(&url).ok_or_else(|| {
            anyhow::anyhow!(
                "origin is not a supported forge remote; GitGat currently supports GitHub and Azure DevOps"
            )
        })
    }

    pub fn provider_label(&self, repo: &Path) -> Result<&'static str> {
        Ok(self.provider(repo)?.label())
    }

    pub fn auth_status(&self, repo: &Path) -> Result<(bool, Option<String>)> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.github_auth_status(),
            ForgeProviderKind::AzureDevOps => self.azure_auth_status(repo),
        }
    }

    pub fn login(&self, repo: &Path) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run(
                "gh",
                ["auth", "login", "--web", "--git-protocol", "https"],
                None,
            ),
            ForgeProviderKind::AzureDevOps => {
                self.require_azure_https(repo)?;
                self.runner.run("az", ["login"], None)
            }
        }
    }

    pub fn logout(&self, repo: &Path) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run("gh", ["auth", "logout"], None),
            ForgeProviderKind::AzureDevOps => self.runner.run("az", ["logout"], None),
        }
    }

    pub fn snapshot(&self, repo: &Path) -> Result<ForgeSnapshot> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.github_snapshot(repo),
            ForgeProviderKind::AzureDevOps => self.azure_snapshot(repo),
        }
    }

    pub fn pull_requests(&self, repo: &Path) -> Result<Vec<PullRequestInfo>> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.github_pull_requests(repo),
            ForgeProviderKind::AzureDevOps => self.azure_pull_requests(repo),
        }
    }

    pub fn create_pull_request(
        &self,
        repo: &Path,
        title: &str,
        body: &str,
        base: &str,
    ) -> Result<String> {
        if title.trim().is_empty() {
            bail!("pull request title is required");
        }
        if base.trim().is_empty() {
            bail!("pull request base branch is required");
        }
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run(
                "gh",
                [
                    "pr",
                    "create",
                    "--title",
                    title.trim(),
                    "--body",
                    body.trim(),
                    "--base",
                    base.trim(),
                ],
                Some(repo),
            ),
            ForgeProviderKind::AzureDevOps => {
                let identity = self.azure_identity(repo)?;
                let source = current_branch(&self.runner, repo)?;
                self.runner.run(
                    "az",
                    [
                        "repos",
                        "pr",
                        "create",
                        "--organization",
                        &identity.organization_url,
                        "--project",
                        &identity.project,
                        "--repository",
                        &identity.repository,
                        "--source-branch",
                        &source,
                        "--target-branch",
                        base.trim(),
                        "--title",
                        title.trim(),
                        "--description",
                        body.trim(),
                        "--output",
                        "json",
                    ],
                    Some(repo),
                )
            }
        }
    }

    pub fn edit_pull_request(
        &self,
        repo: &Path,
        number: u64,
        title: &str,
        body: &str,
    ) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run(
                "gh",
                [
                    "pr",
                    "edit",
                    &number.to_string(),
                    "--title",
                    title,
                    "--body",
                    body,
                ],
                Some(repo),
            ),
            ForgeProviderKind::AzureDevOps => {
                let identity = self.azure_identity(repo)?;
                self.runner.run(
                    "az",
                    [
                        "repos",
                        "pr",
                        "update",
                        "--id",
                        &number.to_string(),
                        "--organization",
                        &identity.organization_url,
                        "--title",
                        title,
                        "--description",
                        body,
                        "--output",
                        "json",
                    ],
                    Some(repo),
                )
            }
        }
    }

    pub fn comment_pull_request(&self, repo: &Path, number: u64, body: &str) -> Result<String> {
        if body.trim().is_empty() {
            bail!("comment body is required");
        }
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run(
                "gh",
                ["pr", "comment", &number.to_string(), "--body", body.trim()],
                Some(repo),
            ),
            ForgeProviderKind::AzureDevOps => self.azure_comment_pull_request(repo, number, body),
        }
    }

    pub fn review_pull_request(
        &self,
        repo: &Path,
        number: u64,
        approve: bool,
        body: &str,
    ) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => {
                let mode = if approve { "--approve" } else { "--comment" };
                let mut args = vec![
                    "pr".to_owned(),
                    "review".to_owned(),
                    number.to_string(),
                    mode.to_owned(),
                ];
                if !body.trim().is_empty() {
                    args.extend(["--body".to_owned(), body.trim().to_owned()]);
                }
                self.runner.run("gh", args, Some(repo))
            }
            ForgeProviderKind::AzureDevOps => {
                if !body.trim().is_empty() {
                    self.azure_comment_pull_request(repo, number, body)?;
                }
                if !approve {
                    return Ok("Azure DevOps review comment submitted.".to_owned());
                }
                let identity = self.azure_identity(repo)?;
                self.runner.run(
                    "az",
                    [
                        "repos",
                        "pr",
                        "set-vote",
                        "--id",
                        &number.to_string(),
                        "--vote",
                        "approve",
                        "--organization",
                        &identity.organization_url,
                        "--output",
                        "json",
                    ],
                    Some(repo),
                )
            }
        }
    }

    pub fn merge_pull_request(&self, repo: &Path, number: u64) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run(
                "gh",
                [
                    "pr",
                    "merge",
                    &number.to_string(),
                    "--merge",
                    "--delete-branch",
                ],
                Some(repo),
            ),
            ForgeProviderKind::AzureDevOps => {
                let identity = self.azure_identity(repo)?;
                self.runner.run(
                    "az",
                    [
                        "repos",
                        "pr",
                        "update",
                        "--id",
                        &number.to_string(),
                        "--organization",
                        &identity.organization_url,
                        "--status",
                        "completed",
                        "--delete-source-branch",
                        "true",
                        "--output",
                        "json",
                    ],
                    Some(repo),
                )
            }
        }
    }

    pub fn workflow_runs(&self, repo: &Path) -> Result<Vec<WorkflowRunInfo>> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.github_workflow_runs(repo),
            ForgeProviderKind::AzureDevOps => self.azure_workflow_runs(repo),
        }
    }

    pub fn rerun_workflow(&self, repo: &Path, id: u64) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self
                .runner
                .run("gh", ["run", "rerun", &id.to_string()], Some(repo)),
            ForgeProviderKind::AzureDevOps => {
                let identity = self.azure_identity(repo)?;
                let run = self.runner.run(
                    "az",
                    [
                        "pipelines",
                        "runs",
                        "show",
                        "--id",
                        &id.to_string(),
                        "--organization",
                        &identity.organization_url,
                        "--project",
                        &identity.project,
                        "--output",
                        "json",
                    ],
                    Some(repo),
                )?;
                let value: Value = serde_json::from_str(&run)?;
                let pipeline_id = value
                    .get("definition")
                    .and_then(|definition| definition.get("id"))
                    .and_then(Value::as_u64)
                    .ok_or_else(|| anyhow::anyhow!("Azure pipeline definition ID is missing"))?;
                let branch = text_at_any(&value, &[&["sourceBranch"], &["sourceBranchName"]]);
                let mut args = vec![
                    "pipelines".to_owned(),
                    "run".to_owned(),
                    "--id".to_owned(),
                    pipeline_id.to_string(),
                    "--organization".to_owned(),
                    identity.organization_url,
                    "--project".to_owned(),
                    identity.project,
                    "--output".to_owned(),
                    "json".to_owned(),
                ];
                if !branch.is_empty() {
                    args.extend([
                        "--branch".to_owned(),
                        strip_ref_prefix(&branch).to_owned(),
                    ]);
                }
                self.runner.run("az", args, Some(repo))
            }
        }
    }

    pub fn workflow_log(&self, repo: &Path, id: u64) -> Result<String> {
        match self.provider(repo)? {
            ForgeProviderKind::GitHub => self.runner.run(
                "gh",
                ["run", "view", &id.to_string(), "--log-failed"],
                Some(repo),
            ),
            ForgeProviderKind::AzureDevOps => self.azure_workflow_log(repo, id),
        }
    }

    fn github_auth_status(&self) -> Result<(bool, Option<String>)> {
        let (ok, _) = self
            .runner
            .run_allow_failure("gh", ["auth", "status"], None)?;
        if !ok {
            return Ok((false, None));
        }
        let (_, user) =
            self.runner
                .run_allow_failure("gh", ["api", "user", "--jq", ".login"], None)?;
        Ok((true, nonempty(user.trim())))
    }

    fn github_snapshot(&self, repo: &Path) -> Result<ForgeSnapshot> {
        let (authenticated, account) = self.github_auth_status()?;
        if !authenticated {
            return Ok(ForgeSnapshot {
                authenticated,
                account,
                ..Default::default()
            });
        }
        let slug = github_remote_slug(&self.runner, repo)?;
        Ok(ForgeSnapshot {
            authenticated,
            account,
            pull_requests: self.github_pull_requests(repo)?,
            workflow_runs: self.github_workflow_runs(repo)?,
            notifications: self.github_notifications()?,
            activity: self.github_activity(&slug)?,
        })
    }

    fn github_pull_requests(&self, repo: &Path) -> Result<Vec<PullRequestInfo>> {
        let output = self.runner.run(
            "gh",
            [
                "pr",
                "list",
                "--limit",
                "100",
                "--json",
                "number,title,state,author,headRefName,baseRefName,url,isDraft",
            ],
            Some(repo),
        )?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values
            .into_iter()
            .map(|value| PullRequestInfo {
                number: value["number"].as_u64().unwrap_or_default(),
                title: text(&value["title"]),
                state: text(&value["state"]),
                author: text(&value["author"]["login"]),
                head: text(&value["headRefName"]),
                base: text(&value["baseRefName"]),
                url: text(&value["url"]),
                draft: value["isDraft"].as_bool().unwrap_or(false),
            })
            .collect())
    }

    fn github_workflow_runs(&self, repo: &Path) -> Result<Vec<WorkflowRunInfo>> {
        let output = self.runner.run(
            "gh",
            [
                "run",
                "list",
                "--limit",
                "50",
                "--json",
                "databaseId,name,status,conclusion,headBranch,url,createdAt",
            ],
            Some(repo),
        )?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values
            .into_iter()
            .map(|value| WorkflowRunInfo {
                id: value["databaseId"].as_u64().unwrap_or_default(),
                name: text(&value["name"]),
                status: text(&value["status"]),
                conclusion: text(&value["conclusion"]),
                branch: text(&value["headBranch"]),
                url: text(&value["url"]),
                created_at: text(&value["createdAt"]),
            })
            .collect())
    }

    fn github_notifications(&self) -> Result<Vec<NotificationInfo>> {
        let output = self
            .runner
            .run("gh", ["api", "notifications", "--paginate"], None)?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values
            .into_iter()
            .map(|value| NotificationInfo {
                id: text(&value["id"]),
                reason: text(&value["reason"]),
                title: text(&value["subject"]["title"]),
                kind: text(&value["subject"]["type"]),
                updated_at: text(&value["updated_at"]),
            })
            .collect())
    }

    fn github_activity(&self, slug: &str) -> Result<Vec<ActivityInfo>> {
        let endpoint = format!("repos/{slug}/events?per_page=50");
        let output = self.runner.run("gh", ["api", &endpoint], None)?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values
            .into_iter()
            .map(|value| ActivityInfo {
                id: text(&value["id"]),
                event_type: text(&value["type"]),
                actor: text(&value["actor"]["login"]),
                created_at: text(&value["created_at"]),
            })
            .collect())
    }

    fn azure_auth_status(&self, repo: &Path) -> Result<(bool, Option<String>)> {
        self.require_azure_https(repo)?;
        let (ok, output) = self.runner.run_allow_failure(
            "az",
            ["account", "show", "--output", "json"],
            None,
        )?;
        if !ok {
            return Ok((false, None));
        }
        let value: Value = serde_json::from_str(&output).unwrap_or(Value::Null);
        let account = text_at_any(
            &value,
            &[
                &["user", "name"],
                &["user", "displayName"],
                &["name"],
            ],
        );
        Ok((true, nonempty(&account)))
    }

    fn azure_snapshot(&self, repo: &Path) -> Result<ForgeSnapshot> {
        let (authenticated, account) = self.azure_auth_status(repo)?;
        if !authenticated {
            return Ok(ForgeSnapshot {
                authenticated,
                account,
                ..Default::default()
            });
        }
        Ok(ForgeSnapshot {
            authenticated,
            account,
            pull_requests: self.azure_pull_requests(repo)?,
            workflow_runs: self.azure_workflow_runs(repo)?,
            // Azure DevOps does not expose a direct GitHub-style notification
            // feed through the CLI. Provider-specific dashboards can be added
            // without weakening the common PR/pipeline contract.
            notifications: Vec::new(),
            activity: Vec::new(),
        })
    }

    fn azure_pull_requests(&self, repo: &Path) -> Result<Vec<PullRequestInfo>> {
        let identity = self.azure_identity(repo)?;
        let output = self.runner.run(
            "az",
            [
                "repos",
                "pr",
                "list",
                "--organization",
                &identity.organization_url,
                "--project",
                &identity.project,
                "--repository",
                &identity.repository,
                "--status",
                "active",
                "--top",
                "100",
                "--output",
                "json",
            ],
            Some(repo),
        )?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values
            .into_iter()
            .map(|value| {
                let number = value["pullRequestId"].as_u64().unwrap_or_default();
                let url = text_at_any(
                    &value,
                    &[
                        &["_links", "web", "href"],
                        &["remoteUrl"],
                        &["url"],
                    ],
                );
                PullRequestInfo {
                    number,
                    title: text(&value["title"]),
                    state: text(&value["status"]),
                    author: text_at_any(
                        &value,
                        &[
                            &["createdBy", "displayName"],
                            &["createdBy", "uniqueName"],
                        ],
                    ),
                    head: strip_ref_prefix(&text(&value["sourceRefName"])).to_owned(),
                    base: strip_ref_prefix(&text(&value["targetRefName"])).to_owned(),
                    url,
                    draft: value["isDraft"].as_bool().unwrap_or(false),
                }
            })
            .collect())
    }

    fn azure_workflow_runs(&self, repo: &Path) -> Result<Vec<WorkflowRunInfo>> {
        let identity = self.azure_identity(repo)?;
        let output = self.runner.run(
            "az",
            [
                "pipelines",
                "runs",
                "list",
                "--organization",
                &identity.organization_url,
                "--project",
                &identity.project,
                "--top",
                "50",
                "--output",
                "json",
            ],
            Some(repo),
        )?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values
            .into_iter()
            .map(|value| WorkflowRunInfo {
                id: value["id"].as_u64().unwrap_or_default(),
                name: text_at_any(
                    &value,
                    &[
                        &["definition", "name"],
                        &["buildNumber"],
                        &["name"],
                    ],
                ),
                status: text(&value["status"]),
                conclusion: text_at_any(&value, &[&["result"], &["conclusion"]]),
                branch: strip_ref_prefix(&text(&value["sourceBranch"])).to_owned(),
                url: text_at_any(
                    &value,
                    &[
                        &["_links", "web", "href"],
                        &["url"],
                    ],
                ),
                created_at: text_at_any(
                    &value,
                    &[
                        &["queueTime"],
                        &["startTime"],
                        &["createdDate"],
                    ],
                ),
            })
            .collect())
    }

    fn azure_comment_pull_request(&self, repo: &Path, number: u64, body: &str) -> Result<String> {
        let identity = self.azure_identity(repo)?;
        let repository_id = self.azure_repository_id(repo, &identity)?;
        let payload = json!({
            "comments": [{
                "parentCommentId": 0,
                "content": body.trim(),
                "commentType": 1
            }],
            "status": 1
        });
        let input = temporary_json("gitgat-azure-pr-comment", &payload)?;
        let result = self.runner.run(
            "az",
            [
                "devops",
                "invoke",
                "--area",
                "git",
                "--resource",
                "pullRequestThreads",
                "--organization",
                &identity.organization_url,
                "--route-parameters",
                &format!("project={}", identity.project),
                &format!("repositoryId={repository_id}"),
                &format!("pullRequestId={number}"),
                "--http-method",
                "POST",
                "--api-version",
                "7.1",
                "--in-file",
                input.to_string_lossy().as_ref(),
                "--output",
                "json",
            ],
            Some(repo),
        );
        let _ = fs::remove_file(&input);
        result
    }

    fn azure_repository_id(&self, repo: &Path, identity: &AzureRepoIdentity) -> Result<String> {
        let output = self.runner.run(
            "az",
            [
                "repos",
                "show",
                "--organization",
                &identity.organization_url,
                "--project",
                &identity.project,
                "--repository",
                &identity.repository,
                "--output",
                "json",
            ],
            Some(repo),
        )?;
        let value: Value = serde_json::from_str(&output)?;
        let id = text(&value["id"]);
        if id.is_empty() {
            bail!("Azure repository ID is missing");
        }
        Ok(id)
    }

    fn azure_workflow_log(&self, repo: &Path, id: u64) -> Result<String> {
        let identity = self.azure_identity(repo)?;
        let output = self.runner.run(
            "az",
            [
                "devops",
                "invoke",
                "--area",
                "build",
                "--resource",
                "logs",
                "--organization",
                &identity.organization_url,
                "--route-parameters",
                &format!("project={}", identity.project),
                &format!("buildId={id}"),
                "--api-version",
                "7.1",
                "--output",
                "json",
            ],
            Some(repo),
        )?;
        let value: Value = serde_json::from_str(&output)?;
        let logs = if let Some(values) = value.as_array() {
            values.clone()
        } else {
            value
                .get("value")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        };
        if logs.is_empty() {
            return Ok("No Azure Pipeline logs were returned for this run.".to_owned());
        }

        let mut combined = String::new();
        for log in logs.into_iter().rev().take(8).rev() {
            let Some(log_id) = log.get("id").and_then(Value::as_u64) else {
                continue;
            };
            let out = temporary_path(&format!("gitgat-azure-log-{id}-{log_id}"), "txt");
            let response = self.runner.run(
                "az",
                [
                    "devops",
                    "invoke",
                    "--area",
                    "build",
                    "--resource",
                    "logs",
                    "--organization",
                    &identity.organization_url,
                    "--route-parameters",
                    &format!("project={}", identity.project),
                    &format!("buildId={id}"),
                    &format!("logId={log_id}"),
                    "--api-version",
                    "7.1",
                    "--accept-media-type",
                    "text/plain",
                    "--out-file",
                    out.to_string_lossy().as_ref(),
                ],
                Some(repo),
            );
            if response.is_ok()
                && let Ok(text) = fs::read_to_string(&out)
            {
                combined.push_str(&format!("\n=== Azure log {log_id} ===\n"));
                combined.push_str(&text);
            }
            let _ = fs::remove_file(out);
        }
        if combined.trim().is_empty() {
            Ok("Azure Pipeline logs are available, but no text log content could be decoded.".to_owned())
        } else {
            Ok(combined)
        }
    }

    fn azure_identity(&self, repo: &Path) -> Result<AzureRepoIdentity> {
        let url = self.origin(repo)?;
        let identity = parse_azure_remote(&url)
            .ok_or_else(|| anyhow::anyhow!("origin is not an Azure DevOps Git remote"))?;
        if identity.ssh_remote {
            bail!(
                "Azure DevOps Git LFS repositories must use an HTTPS origin in GitGat; change the origin from SSH to HTTPS"
            );
        }
        Ok(identity)
    }

    fn require_azure_https(&self, repo: &Path) -> Result<()> {
        self.azure_identity(repo).map(|_| ())
    }

    fn origin(&self, repo: &Path) -> Result<String> {
        Ok(self
            .runner
            .run("git", ["remote", "get-url", "origin"], Some(repo))?
            .trim()
            .to_owned())
    }
}

pub fn detect_provider(url: &str) -> Option<ForgeProviderKind> {
    if parse_github_remote(url).is_some() {
        Some(ForgeProviderKind::GitHub)
    } else if parse_azure_remote(url).is_some() {
        Some(ForgeProviderKind::AzureDevOps)
    } else {
        None
    }
}

fn github_remote_slug(runner: &ProcessRunner, repo: &Path) -> Result<String> {
    let url = runner.run("git", ["remote", "get-url", "origin"], Some(repo))?;
    parse_github_remote(url.trim()).ok_or_else(|| anyhow::anyhow!("origin is not a GitHub remote"))
}

pub fn parse_github_remote(url: &str) -> Option<String> {
    let trimmed = url.trim().trim_end_matches(".git");
    if let Some(value) = trimmed.strip_prefix("https://github.com/") {
        return valid_slug(value);
    }
    if let Some(value) = trimmed.strip_prefix("http://github.com/") {
        return valid_slug(value);
    }
    if let Some(value) = trimmed.strip_prefix("git@github.com:") {
        return valid_slug(value);
    }
    if let Some(value) = trimmed.strip_prefix("ssh://git@github.com/") {
        return valid_slug(value);
    }
    None
}

pub fn parse_azure_remote(url: &str) -> Option<AzureRepoIdentity> {
    let trimmed = url.trim().trim_end_matches(".git");

    if let Some(value) = trimmed.strip_prefix("git@ssh.dev.azure.com:v3/") {
        let mut parts = value.split('/');
        let organization = parts.next()?.to_owned();
        let project = parts.next()?.to_owned();
        let repository = parts.next()?.to_owned();
        if organization.is_empty() || project.is_empty() || repository.is_empty() || parts.next().is_some() {
            return None;
        }
        return Some(AzureRepoIdentity {
            organization_url: format!("https://dev.azure.com/{organization}"),
            organization,
            project,
            repository,
            ssh_remote: true,
        });
    }

    let https = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))?;

    if let Some(rest) = https.split_once("dev.azure.com/").map(|(_, rest)| rest) {
        let mut parts = rest.split('/');
        let organization = strip_optional_user(parts.next()?);
        let mut organization = organization.to_owned();

        // Credential-style URLs may be user@dev.azure.com/org/...
        if organization.contains('@') {
            organization = parts.next()?.to_owned();
        }

        let project = parts.next()?.to_owned();
        if parts.next()? != "_git" {
            return None;
        }
        let repository = parts.next()?.to_owned();
        if organization.is_empty() || project.is_empty() || repository.is_empty() || parts.next().is_some() {
            return None;
        }
        return Some(AzureRepoIdentity {
            organization_url: format!("https://dev.azure.com/{organization}"),
            organization,
            project,
            repository,
            ssh_remote: false,
        });
    }

    let host_and_path = https;
    let (host, path) = host_and_path.split_once('/')?;
    if let Some(organization) = host.strip_suffix(".visualstudio.com") {
        let mut parts = path.split('/');
        let project = parts.next()?.to_owned();
        if parts.next()? != "_git" {
            return None;
        }
        let repository = parts.next()?.to_owned();
        if organization.is_empty() || project.is_empty() || repository.is_empty() || parts.next().is_some() {
            return None;
        }
        return Some(AzureRepoIdentity {
            organization_url: format!("https://dev.azure.com/{organization}"),
            organization: organization.to_owned(),
            project,
            repository,
            ssh_remote: false,
        });
    }

    None
}

fn strip_optional_user(value: &str) -> &str {
    value.rsplit('@').next().unwrap_or(value)
}

fn current_branch(runner: &ProcessRunner, repo: &Path) -> Result<String> {
    let branch = runner
        .run("git", ["branch", "--show-current"], Some(repo))?
        .trim()
        .to_owned();
    if branch.is_empty() {
        bail!("creating a pull request requires a named branch");
    }
    Ok(branch)
}

fn valid_slug(value: &str) -> Option<String> {
    let mut parts = value.split('/');
    let owner = parts.next()?;
    let repository = parts.next()?;
    if owner.is_empty() || repository.is_empty() || parts.next().is_some() {
        return None;
    }
    Some(format!("{owner}/{repository}"))
}

fn strip_ref_prefix(value: &str) -> &str {
    value.strip_prefix("refs/heads/").unwrap_or(value)
}

fn text(value: &Value) -> String {
    value.as_str().unwrap_or_default().to_owned()
}

fn text_at_any(value: &Value, paths: &[&[&str]]) -> String {
    for path in paths {
        let mut current = value;
        let mut found = true;
        for part in *path {
            let Some(next) = current.get(*part) else {
                found = false;
                break;
            };
            current = next;
        }
        if found {
            let value = text(current);
            if !value.is_empty() {
                return value;
            }
        }
    }
    String::new()
}

fn nonempty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

fn temporary_json(prefix: &str, value: &Value) -> Result<PathBuf> {
    let path = temporary_path(prefix, "json");
    let data = serde_json::to_vec(value)?;
    fs::write(&path, data)
        .with_context(|| format!("failed to create temporary Azure DevOps request {}", path.display()))?;
    Ok(path)
}

fn temporary_path(prefix: &str, extension: &str) -> PathBuf {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or_default();
    std::env::temp_dir().join(format!(
        "{prefix}-{}-{epoch}.{extension}",
        std::process::id()
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        ForgeProviderKind, detect_provider, parse_azure_remote, parse_github_remote,
    };

    #[test]
    fn maps_common_github_remote_formats() {
        for (remote, expected) in [
            (
                "https://github.com/AyoubDroubi/GitGat.git",
                "AyoubDroubi/GitGat",
            ),
            (
                "git@github.com:AyoubDroubi/GitGat.git",
                "AyoubDroubi/GitGat",
            ),
            (
                "ssh://git@github.com/AyoubDroubi/GitGat",
                "AyoubDroubi/GitGat",
            ),
        ] {
            assert_eq!(parse_github_remote(remote).as_deref(), Some(expected));
            assert_eq!(detect_provider(remote), Some(ForgeProviderKind::GitHub));
        }
    }

    #[test]
    fn maps_azure_devops_remote_formats() {
        let https = parse_azure_remote(
            "https://dev.azure.com/InfiniteTek/MEP/_git/Collection",
        )
        .unwrap();
        assert_eq!(https.organization, "InfiniteTek");
        assert_eq!(https.project, "MEP");
        assert_eq!(https.repository, "Collection");
        assert!(!https.ssh_remote);
        assert_eq!(
            detect_provider("https://dev.azure.com/InfiniteTek/MEP/_git/Collection"),
            Some(ForgeProviderKind::AzureDevOps)
        );

        let credential = parse_azure_remote(
            "https://user@dev.azure.com/InfiniteTek/MEP/_git/Collection",
        )
        .unwrap();
        assert_eq!(credential.organization, "InfiniteTek");

        let ssh = parse_azure_remote(
            "git@ssh.dev.azure.com:v3/InfiniteTek/MEP/Collection",
        )
        .unwrap();
        assert!(ssh.ssh_remote);

        let legacy = parse_azure_remote(
            "https://InfiniteTek.visualstudio.com/MEP/_git/Collection",
        )
        .unwrap();
        assert_eq!(legacy.organization_url, "https://dev.azure.com/InfiniteTek");
    }

    #[test]
    fn rejects_unsupported_or_malformed_remotes() {
        assert_eq!(parse_github_remote("https://gitlab.com/a/b.git"), None);
        assert_eq!(parse_github_remote("https://github.com/only-owner"), None);
        assert_eq!(parse_azure_remote("https://dev.azure.com/org/project/nope/repo"), None);
        assert_eq!(detect_provider("https://gitlab.com/a/b.git"), None);
    }
}
