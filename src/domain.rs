use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryStatus {
    pub branch: String,
    pub upstream: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub detached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeKind {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Conflicted,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileChange {
    pub path: String,
    pub index_status: char,
    pub worktree_status: char,
    pub kind: ChangeKind,
    pub staged: bool,
    pub conflicted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BranchInfo {
    pub name: String,
    pub current: bool,
    pub remote: bool,
    pub upstream: Option<String>,
    pub commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommitInfo {
    pub sha: String,
    pub short_sha: String,
    pub author: String,
    pub timestamp: String,
    pub subject: String,
    pub decorations: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StashInfo {
    pub reference: String,
    pub subject: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorktreeInfo {
    pub path: PathBuf,
    pub head: String,
    pub branch: Option<String>,
    pub bare: bool,
    pub detached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReflogEntry {
    pub selector: String,
    pub sha: String,
    pub subject: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LfsLockOwnership {
    Ours,
    Theirs,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LfsLock {
    pub id: String,
    pub path: String,
    pub owner: String,
    pub locked_at: String,
    pub ownership: LfsLockOwnership,
}

#[derive(Debug, Clone, Default)]
pub struct RepositorySnapshot {
    pub status: RepositoryStatus,
    pub changes: Vec<FileChange>,
    pub branches: Vec<BranchInfo>,
    pub commits: Vec<CommitInfo>,
    pub stashes: Vec<StashInfo>,
    pub worktrees: Vec<WorktreeInfo>,
    pub reflog: Vec<ReflogEntry>,
    pub lfs_locks: Vec<LfsLock>,
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogRepository {
    pub path: PathBuf,
    pub name: String,
    pub favorite: bool,
    pub last_opened_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub repositories: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PullRequestInfo {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: String,
    pub head: String,
    pub base: String,
    pub url: String,
    pub draft: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowRunInfo {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: String,
    pub branch: String,
    pub url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationInfo {
    pub id: String,
    pub reason: String,
    pub title: String,
    pub kind: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivityInfo {
    pub id: String,
    pub event_type: String,
    pub actor: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Default)]
pub struct ForgeSnapshot {
    pub authenticated: bool,
    pub account: Option<String>,
    pub pull_requests: Vec<PullRequestInfo>,
    pub workflow_runs: Vec<WorkflowRunInfo>,
    pub notifications: Vec<NotificationInfo>,
    pub activity: Vec<ActivityInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManagedPolicy {
    pub version: i64,
    pub protected_patterns: Vec<String>,
    pub excluded_patterns: Vec<String>,
    pub required_lock: bool,
    pub max_lock_age_minutes: Option<i64>,
    pub force_unlock_approval_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManagedRepository {
    pub control_plane_url: String,
    pub control_plane_repository_id: String,
    pub policy: Option<ManagedPolicy>,
    pub policy_fetched_unix: Option<i64>,
}

impl ManagedRepository {
    pub fn policy_is_fresh_at(&self, now_unix: i64, max_age_seconds: i64) -> bool {
        self.policy.is_some()
            && self
                .policy_fetched_unix
                .is_some_and(|fetched| now_unix >= fetched && now_unix - fetched <= max_age_seconds)
    }
}
