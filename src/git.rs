use crate::domain::{
    BranchInfo, ChangeKind, CommitInfo, FileChange, LfsLock, ReflogEntry, RepositorySnapshot,
    RepositoryStatus, StashInfo, WorktreeInfo,
};
use crate::process::ProcessRunner;
use crate::store::Catalog;
use anyhow::{Context, Result, bail};
use serde_json::Value;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum ConflictChoice { Ours, Theirs }

#[derive(Debug, Clone, Copy)]
pub enum ResetMode { Soft, Mixed, Hard }

#[derive(Debug, Clone)]
pub struct GitClient {
    runner: ProcessRunner,
    catalog: Catalog,
}

impl GitClient {
    pub fn new(catalog: Catalog) -> Self { Self { runner: ProcessRunner, catalog } }

    pub fn is_repository(&self, path: &Path) -> bool {
        self.runner.run_allow_failure("git", ["rev-parse", "--is-inside-work-tree"], Some(path))
            .map(|(ok, text)| ok && text.trim() == "true").unwrap_or(false)
    }

    pub fn repository_root(&self, path: &Path) -> Result<PathBuf> {
        let value = self.runner.run("git", ["rev-parse", "--show-toplevel"], Some(path))?;
        Ok(PathBuf::from(value.trim()))
    }

    pub fn clone_repository(&self, url: &str, destination: &Path) -> Result<PathBuf> {
        if url.trim().is_empty() { bail!("clone URL is required"); }
        if destination.exists() && destination.read_dir()?.next().is_some() { bail!("clone destination must be empty"); }
        let args = vec!["clone".to_owned(), "--".to_owned(), url.trim().to_owned(), destination.to_string_lossy().to_string()];
        self.runner.run("git", args, None)?;
        let root = self.repository_root(destination)?;
        self.catalog.touch_repository(&root)?;
        Ok(root)
    }

    pub fn snapshot(&self, repo: &Path, history_limit: usize) -> Result<RepositorySnapshot> {
        Ok(RepositorySnapshot {
            status: self.status(repo)?,
            changes: self.changes(repo)?,
            branches: self.branches(repo)?,
            commits: self.history(repo, history_limit)?,
            stashes: self.stashes(repo)?,
            worktrees: self.worktrees(repo)?,
            reflog: self.reflog(repo, 40)?,
            lfs_locks: self.lfs_locks(repo).unwrap_or_default(),
            conflicts: self.conflicts(repo)?,
        })
    }

    pub fn status(&self, repo: &Path) -> Result<RepositoryStatus> {
        let output = self.runner.run("git", ["status", "--porcelain=v2", "--branch", "--untracked-files=all"], Some(repo))?;
        let mut status = RepositoryStatus::default();
        for line in output.lines() {
            if let Some(value) = line.strip_prefix("# branch.head ") {
                status.detached = value == "(detached)";
                status.branch = value.to_owned();
            } else if let Some(value) = line.strip_prefix("# branch.upstream ") {
                status.upstream = Some(value.to_owned());
            } else if let Some(value) = line.strip_prefix("# branch.ab ") {
                for part in value.split_whitespace() {
                    if let Some(v) = part.strip_prefix('+') { status.ahead = v.parse().unwrap_or_default(); }
                    else if let Some(v) = part.strip_prefix('-') { status.behind = v.parse().unwrap_or_default(); }
                }
            }
        }
        Ok(status)
    }

    pub fn changes(&self, repo: &Path) -> Result<Vec<FileChange>> {
        let output = self.runner.run("git", ["-c", "core.quotepath=false", "status", "--porcelain=v2", "--untracked-files=all"], Some(repo))?;
        Ok(parse_porcelain_v2(&output))
    }

    pub fn stage(&self, repo: &Path, paths: &[String]) -> Result<()> {
        require_paths(paths)?;
        let mut args = vec!["add".to_owned(), "--".to_owned()];
        args.extend(paths.iter().cloned());
        self.runner.run("git", args, Some(repo))?;
        Ok(())
    }

    pub fn unstage(&self, repo: &Path, paths: &[String]) -> Result<()> {
        require_paths(paths)?;
        let mut args = vec!["restore".to_owned(), "--staged".to_owned(), "--".to_owned()];
        args.extend(paths.iter().cloned());
        self.runner.run("git", args, Some(repo))?;
        Ok(())
    }

    pub fn discard(&self, repo: &Path, paths: &[String]) -> Result<()> {
        require_paths(paths)?;
        let changes = self.changes(repo)?;
        for path in paths {
            let Some(change) = changes.iter().find(|change| &change.path == path) else { bail!("path is not a tracked change: {path}"); };
            if change.kind == ChangeKind::Untracked { bail!("GitGat never deletes untracked files through Discard: {path}"); }
        }
        let mut args = vec!["restore".to_owned(), "--worktree".to_owned(), "--source=HEAD".to_owned(), "--".to_owned()];
        args.extend(paths.iter().cloned());
        self.runner.run("git", args, Some(repo))?;
        Ok(())
    }

    pub fn commit(&self, repo: &Path, message: &str) -> Result<String> {
        if message.trim().is_empty() { bail!("commit message is required"); }
        self.runner.run("git", ["commit", "-m", message.trim()], Some(repo))
    }

    pub fn fetch(&self, repo: &Path) -> Result<String> { self.runner.run("git", ["fetch", "--prune"], Some(repo)) }
    pub fn pull_ff_only(&self, repo: &Path) -> Result<String> { self.runner.run("git", ["pull", "--ff-only"], Some(repo)) }
    pub fn push(&self, repo: &Path) -> Result<String> { self.runner.run("git", ["push"], Some(repo)) }

    pub fn branches(&self, repo: &Path) -> Result<Vec<BranchInfo>> {
        let output = self.runner.run("git", ["for-each-ref", "--format=%(refname)|%(refname:short)|%(HEAD)|%(upstream:short)|%(objectname:short)", "refs/heads", "refs/remotes"], Some(repo))?;
        Ok(output.lines().filter_map(|line| {
            let mut parts = line.split('|');
            let full_ref = parts.next()?;
            let name = parts.next()?.to_owned();
            let head = parts.next().unwrap_or_default();
            let upstream = parts.next().filter(|v| !v.is_empty()).map(str::to_owned);
            let commit = parts.next().filter(|v| !v.is_empty()).map(str::to_owned);
            Some(BranchInfo { remote: full_ref.starts_with("refs/remotes/"), name, current: head == "*", upstream, commit })
        }).collect())
    }

    pub fn create_branch(&self, repo: &Path, name: &str) -> Result<()> {
        validate_ref_name(name)?;
        self.runner.run("git", ["branch", name.trim()], Some(repo))?;
        Ok(())
    }

    pub fn checkout_branch(&self, repo: &Path, name: &str) -> Result<()> {
        self.require_clean(repo, "switch branches")?;
        validate_ref_name(name)?;
        self.runner.run("git", ["switch", name.trim()], Some(repo))?;
        Ok(())
    }

    pub fn delete_branch(&self, repo: &Path, name: &str) -> Result<()> {
        validate_ref_name(name)?;
        self.runner.run("git", ["branch", "-d", name.trim()], Some(repo))?;
        Ok(())
    }

    pub fn history(&self, repo: &Path, limit: usize) -> Result<Vec<CommitInfo>> {
        let arg = format!("-n{}", limit.clamp(1, 2000));
        let output = self.runner.run("git", ["log", "--date=iso-strict", &arg, "--pretty=format:%H%x1f%h%x1f%an%x1f%ad%x1f%s%x1f%D"], Some(repo))?;
        Ok(output.lines().filter_map(|line| {
            let parts = line.split('\x1f').collect::<Vec<_>>();
            (parts.len() >= 6).then(|| CommitInfo {
                sha: parts[0].to_owned(), short_sha: parts[1].to_owned(), author: parts[2].to_owned(),
                timestamp: parts[3].to_owned(), subject: parts[4].to_owned(), decorations: parts[5].to_owned(),
            })
        }).collect())
    }

    pub fn diff(&self, repo: &Path, path: &str, staged: bool) -> Result<String> {
        let mut args = vec!["diff".to_owned()];
        if staged { args.push("--staged".to_owned()); }
        args.push("--".to_owned());
        args.push(path.to_owned());
        self.runner.run("git", args, Some(repo))
    }

    pub fn stashes(&self, repo: &Path) -> Result<Vec<StashInfo>> {
        let output = self.runner.run("git", ["stash", "list", "--format=%gd%x1f%s"], Some(repo))?;
        Ok(output.lines().filter_map(|line| {
            let (reference, subject) = line.split_once('\x1f')?;
            Some(StashInfo { reference: reference.to_owned(), subject: subject.to_owned() })
        }).collect())
    }

    pub fn stash_push(&self, repo: &Path, message: &str, include_untracked: bool) -> Result<String> {
        let mut args = vec!["stash".to_owned(), "push".to_owned()];
        if include_untracked { args.push("--include-untracked".to_owned()); }
        if !message.trim().is_empty() { args.extend(["-m".to_owned(), message.trim().to_owned()]); }
        self.runner.run("git", args, Some(repo))
    }

    pub fn stash_apply(&self, repo: &Path, reference: &str) -> Result<String> {
        validate_stash(reference)?;
        self.runner.run("git", ["stash", "apply", reference], Some(repo))
    }

    pub fn stash_pop(&self, repo: &Path, reference: &str) -> Result<String> {
        validate_stash(reference)?;
        self.journal(repo, "stash-pop", reference)?;
        self.runner.run("git", ["stash", "pop", reference], Some(repo))
    }

    pub fn stash_drop(&self, repo: &Path, reference: &str) -> Result<String> {
        validate_stash(reference)?;
        self.journal(repo, "stash-drop", reference)?;
        self.runner.run("git", ["stash", "drop", reference], Some(repo))
    }

    pub fn conflicts(&self, repo: &Path) -> Result<Vec<String>> {
        let output = self.runner.run("git", ["diff", "--name-only", "--diff-filter=U"], Some(repo))?;
        Ok(output.lines().filter(|v| !v.is_empty()).map(str::to_owned).collect())
    }

    pub fn resolve_conflict(&self, repo: &Path, path: &str, choice: ConflictChoice) -> Result<()> {
        let side = match choice { ConflictChoice::Ours => "--ours", ConflictChoice::Theirs => "--theirs" };
        self.runner.run("git", ["checkout", side, "--", path], Some(repo))?;
        self.runner.run("git", ["add", "--", path], Some(repo))?;
        Ok(())
    }

    pub fn mark_conflict_resolved(&self, repo: &Path, path: &str) -> Result<()> {
        let full = repo.join(path);
        let metadata = std::fs::metadata(&full)?;
        if metadata.len() > 5 * 1024 * 1024 { bail!("files over 5 MB must be resolved with Ours/Theirs or explicitly staged"); }
        let text = std::fs::read_to_string(&full).with_context(|| format!("{} is not UTF-8 text", full.display()))?;
        if contains_conflict_markers(&text) { bail!("conflict markers still exist in {path}"); }
        self.runner.run("git", ["add", "--", path], Some(repo))?;
        Ok(())
    }

    pub fn rebase(&self, repo: &Path, target: &str) -> Result<String> {
        self.require_clean(repo, "rebase")?;
        self.verify_commit(repo, target)?;
        let old_head = self.head(repo)?;
        self.journal(repo, "rebase", &format!("{old_head} -> {target}"))?;
        self.runner.run("git", ["rebase", target], Some(repo))
    }

    pub fn cherry_pick(&self, repo: &Path, target: &str) -> Result<String> {
        self.require_clean(repo, "cherry-pick")?;
        self.verify_commit(repo, target)?;
        let old_head = self.head(repo)?;
        self.journal(repo, "cherry-pick", &format!("{old_head} + {target}"))?;
        self.runner.run("git", ["cherry-pick", target], Some(repo))
    }

    pub fn reset(&self, repo: &Path, target: &str, mode: ResetMode) -> Result<String> {
        self.verify_commit(repo, target)?;
        if matches!(mode, ResetMode::Hard) { self.require_clean(repo, "hard reset")?; }
        let old_head = self.head(repo)?;
        let mode_arg = match mode { ResetMode::Soft => "--soft", ResetMode::Mixed => "--mixed", ResetMode::Hard => "--hard" };
        self.journal(repo, "reset", &format!("{mode_arg} {old_head} -> {target}"))?;
        self.runner.run("git", ["reset", mode_arg, target], Some(repo))
    }

    pub fn abort_current_operation(&self, repo: &Path) -> Result<String> {
        for args in [["rebase", "--abort"], ["merge", "--abort"], ["cherry-pick", "--abort"], ["revert", "--abort"]] {
            let (ok, text) = self.runner.run_allow_failure("git", args, Some(repo))?;
            if ok { return Ok(text); }
        }
        bail!("no abortable Git operation is active")
    }

    pub fn reflog(&self, repo: &Path, limit: usize) -> Result<Vec<ReflogEntry>> {
        let arg = format!("-n{}", limit.clamp(1, 500));
        let output = self.runner.run("git", ["reflog", &arg, "--format=%gD%x1f%H%x1f%gs"], Some(repo))?;
        Ok(output.lines().filter_map(|line| {
            let parts = line.split('\x1f').collect::<Vec<_>>();
            (parts.len() == 3).then(|| ReflogEntry { selector: parts[0].to_owned(), sha: parts[1].to_owned(), subject: parts[2].to_owned() })
        }).collect())
    }

    pub fn create_recovery_branch(&self, repo: &Path, name: &str, reference: &str) -> Result<()> {
        validate_ref_name(name)?;
        self.verify_commit(repo, reference)?;
        self.runner.run("git", ["branch", name.trim(), reference], Some(repo))?;
        self.journal(repo, "recovery-branch", &format!("{} <- {reference}", name.trim()))?;
        Ok(())
    }

    pub fn worktrees(&self, repo: &Path) -> Result<Vec<WorktreeInfo>> {
        let output = self.runner.run("git", ["worktree", "list", "--porcelain"], Some(repo))?;
        Ok(parse_worktrees(&output))
    }

    pub fn add_worktree(&self, repo: &Path, path: &Path, branch: &str) -> Result<()> {
        validate_ref_name(branch)?;
        let path_text = path.to_string_lossy().to_string();
        self.runner.run("git", ["worktree", "add", &path_text, branch.trim()], Some(repo))?;
        Ok(())
    }

    pub fn remove_worktree(&self, repo: &Path, path: &Path) -> Result<()> {
        let path_text = path.to_string_lossy().to_string();
        self.runner.run("git", ["worktree", "remove", &path_text], Some(repo))?;
        Ok(())
    }

    pub fn lfs_locks(&self, repo: &Path) -> Result<Vec<LfsLock>> {
        let (ok, output) = self.runner.run_allow_failure("git", ["lfs", "locks", "--json"], Some(repo))?;
        if !ok { return Ok(Vec::new()); }
        let value: Value = serde_json::from_str(&output)?;
        let locks = value.get("locks").and_then(Value::as_array).cloned().unwrap_or_default();
        Ok(locks.into_iter().map(|lock| LfsLock {
            id: string_at(&lock, &["id"]), path: string_at(&lock, &["path"]),
            owner: string_at(&lock, &["owner", "name"]), locked_at: string_at(&lock, &["locked_at"]),
        }).collect())
    }

    pub fn lfs_lock(&self, repo: &Path, path: &str) -> Result<String> {
        if path.trim().is_empty() { bail!("LFS path is required"); }
        self.runner.run("git", ["lfs", "lock", "--", path], Some(repo))
    }

    pub fn lfs_unlock(&self, repo: &Path, path: &str) -> Result<String> {
        if path.trim().is_empty() { bail!("LFS path is required"); }
        self.runner.run("git", ["lfs", "unlock", "--", path], Some(repo))
    }

    fn require_clean(&self, repo: &Path, operation: &str) -> Result<()> {
        if !self.changes(repo)?.is_empty() { bail!("{operation} requires a clean working tree"); }
        Ok(())
    }

    fn verify_commit(&self, repo: &Path, reference: &str) -> Result<()> {
        if reference.trim().is_empty() { bail!("commit/ref is required"); }
        let spec = format!("{}^{{commit}}", reference.trim());
        self.runner.run("git", ["rev-parse", "--verify", &spec], Some(repo))?;
        Ok(())
    }

    fn head(&self, repo: &Path) -> Result<String> {
        Ok(self.runner.run("git", ["rev-parse", "HEAD"], Some(repo))?.trim().to_owned())
    }

    fn journal(&self, repo: &Path, operation: &str, details: &str) -> Result<()> {
        self.catalog.journal(repo, operation, details)
    }
}

fn validate_ref_name(name: &str) -> Result<()> {
    let name = name.trim();
    if name.is_empty() { bail!("branch name is required"); }
    let runner = ProcessRunner;
    let (ok, output) = runner.run_allow_failure("git", ["check-ref-format", "--branch", name], None)?;
    if !ok { bail!("invalid branch name: {}", output.trim()); }
    Ok(())
}

fn validate_stash(reference: &str) -> Result<()> {
    if !reference.starts_with("stash@{") || !reference.ends_with('}') { bail!("invalid stash reference"); }
    Ok(())
}

fn require_paths(paths: &[String]) -> Result<()> {
    if paths.is_empty() { bail!("select at least one file"); }
    Ok(())
}

fn string_at(value: &Value, path: &[&str]) -> String {
    let mut current = value;
    for part in path {
        let Some(next) = current.get(*part) else { return String::new(); };
        current = next;
    }
    current.as_str().unwrap_or_default().to_owned()
}

pub fn contains_conflict_markers(text: &str) -> bool {
    text.lines().any(|line| line.starts_with("<<<<<<< ") || line == "=======" || line.starts_with(">>>>>>> "))
}

pub fn parse_porcelain_v2(output: &str) -> Vec<FileChange> {
    let mut result = Vec::new();
    for line in output.lines() {
        if line.starts_with("# ") || line.is_empty() { continue; }
        if let Some(path) = line.strip_prefix("? ") {
            result.push(FileChange { path: path.to_owned(), index_status: '?', worktree_status: '?', kind: ChangeKind::Untracked, staged: false, conflicted: false });
            continue;
        }
        let tag = line.as_bytes()[0] as char;
        let (xy, path, conflicted) = match tag {
            '1' => (line.split_whitespace().nth(1).unwrap_or(".."), line.splitn(9, ' ').nth(8).unwrap_or_default(), false),
            '2' => (line.split_whitespace().nth(1).unwrap_or(".."), line.splitn(10, ' ').nth(9).and_then(|v| v.split('\t').next()).unwrap_or_default(), false),
            'u' => (line.split_whitespace().nth(1).unwrap_or("UU"), line.splitn(11, ' ').nth(10).unwrap_or_default(), true),
            _ => continue,
        };
        let mut chars = xy.chars();
        let x = chars.next().unwrap_or('.');
        let y = chars.next().unwrap_or('.');
        let kind = if conflicted { ChangeKind::Conflicted }
            else if x == 'D' || y == 'D' { ChangeKind::Deleted }
            else if x == 'R' || y == 'R' { ChangeKind::Renamed }
            else if x == 'C' || y == 'C' { ChangeKind::Copied }
            else if x == 'A' || y == 'A' { ChangeKind::Added }
            else if x == 'M' || y == 'M' { ChangeKind::Modified }
            else { ChangeKind::Unknown };
        result.push(FileChange { path: path.to_owned(), index_status: x, worktree_status: y, kind, staged: x != '.', conflicted });
    }
    result
}

pub fn parse_worktrees(output: &str) -> Vec<WorktreeInfo> {
    let mut result = Vec::new();
    let mut current: Option<WorktreeInfo> = None;
    for line in output.lines().chain(std::iter::once("")) {
        if let Some(path) = line.strip_prefix("worktree ") {
            if let Some(previous) = current.take() { result.push(previous); }
            current = Some(WorktreeInfo { path: PathBuf::from(path), head: String::new(), branch: None, bare: false, detached: false });
        } else if let Some(value) = current.as_mut() {
            if let Some(head) = line.strip_prefix("HEAD ") { value.head = head.to_owned(); }
            else if let Some(branch) = line.strip_prefix("branch refs/heads/") { value.branch = Some(branch.to_owned()); }
            else if line == "bare" { value.bare = true; }
            else if line == "detached" { value.detached = true; }
            else if line.is_empty() {
                if let Some(previous) = current.take() { result.push(previous); }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{GitClient, contains_conflict_markers, parse_porcelain_v2};
    use crate::domain::ChangeKind;
    use crate::store::Catalog;
    use std::process::Command;
    use tempfile::tempdir;

    fn git(repo: &std::path::Path, args: &[&str]) {
        let status = Command::new("git").args(args).current_dir(repo).status().unwrap();
        assert!(status.success());
    }

    fn client() -> (tempfile::TempDir, GitClient) {
        let root = tempdir().unwrap();
        let catalog = Catalog::open(root.path().join("catalog.db")).unwrap();
        (root, GitClient::new(catalog))
    }

    #[test]
    fn parses_porcelain_entries() {
        let values = parse_porcelain_v2("1 .M N... 100644 100644 100644 abc abc src/main.rs\n? notes.txt\nu UU N... 100644 100644 100644 100644 a b c conflict.txt\n");
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].path, "src/main.rs");
        assert_eq!(values[1].kind, ChangeKind::Untracked);
        assert!(values[2].conflicted);
    }

    #[test]
    fn detects_conflict_markers() {
        assert!(contains_conflict_markers("<<<<<<< HEAD\na\n=======\nb\n>>>>>>> main"));
        assert!(!contains_conflict_markers("normal text"));
    }

    #[test]
    fn daily_git_flow_and_untracked_discard_guard() {
        let (root, client) = client();
        let repo = root.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init"]);
        git(&repo, &["config", "user.email", "test@example.com"]);
        git(&repo, &["config", "user.name", "GitGat Test"]);
        std::fs::write(repo.join("a.txt"), "one").unwrap();
        git(&repo, &["add", "a.txt"]);
        git(&repo, &["commit", "-m", "initial"]);
        std::fs::write(repo.join("a.txt"), "two").unwrap();
        std::fs::write(repo.join("new.txt"), "new").unwrap();
        assert_eq!(client.changes(&repo).unwrap().len(), 2);
        client.stage(&repo, &["a.txt".to_owned()]).unwrap();
        client.unstage(&repo, &["a.txt".to_owned()]).unwrap();
        assert!(client.discard(&repo, &["new.txt".to_owned()]).is_err());
        client.discard(&repo, &["a.txt".to_owned()]).unwrap();
        assert_eq!(std::fs::read_to_string(repo.join("a.txt")).unwrap(), "one");
    }

    #[test]
    fn creates_and_switches_branches_safely() {
        let (root, client) = client();
        let repo = root.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init"]);
        git(&repo, &["config", "user.email", "test@example.com"]);
        git(&repo, &["config", "user.name", "GitGat Test"]);
        std::fs::write(repo.join("a.txt"), "one").unwrap();
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-m", "initial"]);
        client.create_branch(&repo, "feature/test").unwrap();
        client.checkout_branch(&repo, "feature/test").unwrap();
        assert_eq!(client.status(&repo).unwrap().branch, "feature/test");
    }

    #[test]
    fn blocks_branch_switch_when_worktree_is_dirty() {
        let (root, client) = client();
        let repo = root.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init"]);
        git(&repo, &["config", "user.email", "test@example.com"]);
        git(&repo, &["config", "user.name", "GitGat Test"]);
        std::fs::write(repo.join("a.txt"), "one").unwrap();
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-m", "initial"]);
        client.create_branch(&repo, "other").unwrap();
        std::fs::write(repo.join("a.txt"), "dirty").unwrap();

        let error = client.checkout_branch(&repo, "other").unwrap_err();
        assert!(error.to_string().contains("clean working tree"));
        assert_ne!(client.status(&repo).unwrap().branch, "other");
    }

    #[test]
    fn blocks_hard_reset_when_worktree_is_dirty() {
        let (root, client) = client();
        let repo = root.path().join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init"]);
        git(&repo, &["config", "user.email", "test@example.com"]);
        git(&repo, &["config", "user.name", "GitGat Test"]);
        std::fs::write(repo.join("a.txt"), "one").unwrap();
        git(&repo, &["add", "."]);
        git(&repo, &["commit", "-m", "initial"]);
        std::fs::write(repo.join("a.txt"), "dirty").unwrap();

        let error = client.reset(&repo, "HEAD", super::ResetMode::Hard).unwrap_err();
        assert!(error.to_string().contains("clean working tree"));
        assert_eq!(std::fs::read_to_string(repo.join("a.txt")).unwrap(), "dirty");
    }
}
