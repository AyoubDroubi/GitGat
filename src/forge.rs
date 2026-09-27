use crate::domain::{ActivityInfo, ForgeSnapshot, NotificationInfo, PullRequestInfo, WorkflowRunInfo};
use crate::process::ProcessRunner;
use anyhow::{Result, bail};
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct GitHubClient { runner: ProcessRunner }

impl GitHubClient {
    pub fn auth_status(&self) -> Result<(bool, Option<String>)> {
        let (ok, _) = self.runner.run_allow_failure("gh", ["auth", "status"], None)?;
        if !ok { return Ok((false, None)); }
        let (_, user) = self.runner.run_allow_failure("gh", ["api", "user", "--jq", ".login"], None)?;
        Ok((true, nonempty(user.trim())))
    }

    pub fn login(&self) -> Result<String> { self.runner.run("gh", ["auth", "login", "--web", "--git-protocol", "https"], None) }
    pub fn logout(&self) -> Result<String> { self.runner.run("gh", ["auth", "logout"], None) }

    pub fn snapshot(&self, repo: &Path) -> Result<ForgeSnapshot> {
        let (authenticated, account) = self.auth_status()?;
        if !authenticated { return Ok(ForgeSnapshot { authenticated, account, ..Default::default() }); }
        let slug = remote_slug(&self.runner, repo)?;
        Ok(ForgeSnapshot {
            authenticated, account,
            pull_requests: self.pull_requests(repo)?,
            workflow_runs: self.workflow_runs(repo)?,
            notifications: self.notifications()?,
            activity: self.activity(&slug)?,
        })
    }

    pub fn pull_requests(&self, repo: &Path) -> Result<Vec<PullRequestInfo>> {
        let output = self.runner.run("gh", ["pr","list","--limit","100","--json","number,title,state,author,headRefName,baseRefName,url,isDraft"], Some(repo))?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values.into_iter().map(|value| PullRequestInfo {
            number: value["number"].as_u64().unwrap_or_default(), title: text(&value["title"]),
            state: text(&value["state"]), author: text(&value["author"]["login"]), head: text(&value["headRefName"]),
            base: text(&value["baseRefName"]), url: text(&value["url"]), draft: value["isDraft"].as_bool().unwrap_or(false),
        }).collect())
    }

    pub fn edit_pull_request(&self, repo: &Path, number: u64, title: &str, body: &str) -> Result<String> {
        self.runner.run("gh", ["pr","edit",&number.to_string(),"--title",title,"--body",body], Some(repo))
    }

    pub fn comment_pull_request(&self, repo: &Path, number: u64, body: &str) -> Result<String> {
        if body.trim().is_empty() { bail!("comment body is required"); }
        self.runner.run("gh", ["pr","comment",&number.to_string(),"--body",body.trim()], Some(repo))
    }

    pub fn review_pull_request(&self, repo: &Path, number: u64, approve: bool, body: &str) -> Result<String> {
        let mode = if approve { "--approve" } else { "--comment" };
        let mut args = vec!["pr".to_owned(),"review".to_owned(),number.to_string(),mode.to_owned()];
        if !body.trim().is_empty() { args.extend(["--body".to_owned(), body.trim().to_owned()]); }
        self.runner.run("gh", args, Some(repo))
    }

    pub fn merge_pull_request(&self, repo: &Path, number: u64) -> Result<String> {
        self.runner.run("gh", ["pr","merge",&number.to_string(),"--merge","--delete-branch"], Some(repo))
    }

    pub fn workflow_runs(&self, repo: &Path) -> Result<Vec<WorkflowRunInfo>> {
        let output = self.runner.run("gh", ["run","list","--limit","50","--json","databaseId,name,status,conclusion,headBranch,url,createdAt"], Some(repo))?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values.into_iter().map(|value| WorkflowRunInfo {
            id: value["databaseId"].as_u64().unwrap_or_default(), name: text(&value["name"]),
            status: text(&value["status"]), conclusion: text(&value["conclusion"]), branch: text(&value["headBranch"]),
            url: text(&value["url"]), created_at: text(&value["createdAt"]),
        }).collect())
    }

    pub fn rerun_workflow(&self, repo: &Path, id: u64) -> Result<String> {
        self.runner.run("gh", ["run","rerun",&id.to_string()], Some(repo))
    }

    pub fn workflow_log(&self, repo: &Path, id: u64) -> Result<String> {
        self.runner.run("gh", ["run","view",&id.to_string(),"--log-failed"], Some(repo))
    }

    pub fn notifications(&self) -> Result<Vec<NotificationInfo>> {
        let output = self.runner.run("gh", ["api","notifications","--paginate"], None)?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values.into_iter().map(|value| NotificationInfo {
            id: text(&value["id"]), reason: text(&value["reason"]), title: text(&value["subject"]["title"]),
            kind: text(&value["subject"]["type"]), updated_at: text(&value["updated_at"]),
        }).collect())
    }

    pub fn activity(&self, slug: &str) -> Result<Vec<ActivityInfo>> {
        let endpoint = format!("repos/{slug}/events?per_page=50");
        let output = self.runner.run("gh", ["api",&endpoint], None)?;
        let values: Vec<Value> = serde_json::from_str(&output)?;
        Ok(values.into_iter().map(|value| ActivityInfo {
            id: text(&value["id"]), event_type: text(&value["type"]), actor: text(&value["actor"]["login"]),
            created_at: text(&value["created_at"]),
        }).collect())
    }
}

fn remote_slug(runner: &ProcessRunner, repo: &Path) -> Result<String> {
    let url = runner.run("git", ["remote","get-url","origin"], Some(repo))?;
    parse_github_remote(url.trim()).ok_or_else(|| anyhow::anyhow!("origin is not a GitHub remote"))
}

pub fn parse_github_remote(url: &str) -> Option<String> {
    let trimmed = url.trim().trim_end_matches(".git");
    if let Some(value) = trimmed.strip_prefix("https://github.com/") { return valid_slug(value); }
    if let Some(value) = trimmed.strip_prefix("http://github.com/") { return valid_slug(value); }
    if let Some(value) = trimmed.strip_prefix("git@github.com:") { return valid_slug(value); }
    if let Some(value) = trimmed.strip_prefix("ssh://git@github.com/") { return valid_slug(value); }
    None
}

fn valid_slug(value: &str) -> Option<String> {
    let mut parts = value.split('/');
    let owner = parts.next()?;
    let repository = parts.next()?;
    if owner.is_empty() || repository.is_empty() || parts.next().is_some() { return None; }
    Some(format!("{owner}/{repository}"))
}
fn text(value: &Value) -> String { value.as_str().unwrap_or_default().to_owned() }
fn nonempty(value: &str) -> Option<String> { (!value.is_empty()).then(|| value.to_owned()) }

#[cfg(test)]
mod tests {
    use super::parse_github_remote;
    #[test]
    fn maps_common_github_remote_formats() {
        for (remote, expected) in [
            ("https://github.com/AyoubDroubi/GitGat.git", "AyoubDroubi/GitGat"),
            ("git@github.com:AyoubDroubi/GitGat.git", "AyoubDroubi/GitGat"),
            ("ssh://git@github.com/AyoubDroubi/GitGat", "AyoubDroubi/GitGat"),
        ] { assert_eq!(parse_github_remote(remote).as_deref(), Some(expected)); }
    }
    #[test]
    fn rejects_non_github_or_malformed_remotes() {
        assert_eq!(parse_github_remote("https://gitlab.com/a/b.git"), None);
        assert_eq!(parse_github_remote("https://github.com/only-owner"), None);
    }
}
