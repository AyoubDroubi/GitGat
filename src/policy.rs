use crate::domain::LfsLock;
use crate::git::parse_lfs_locks_json;
use crate::process::ProcessRunner;
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LockPolicyStatus {
    Pass,
    NotApplicable,
    FailLockMissing,
    FailLockOwnedByOther,
    FailPolicyChanged,
    FailPolicyUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockPolicyViolation {
    pub path: String,
    pub code: LockPolicyStatus,
    pub owner: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockPolicyReport {
    pub status: LockPolicyStatus,
    pub actor: String,
    pub repository: String,
    pub base: String,
    pub base_sha: String,
    pub head_sha: String,
    pub protected_paths: Vec<String>,
    pub violations: Vec<LockPolicyViolation>,
}

impl LockPolicyReport {
    pub fn allows_merge(&self) -> bool {
        matches!(
            self.status,
            LockPolicyStatus::Pass | LockPolicyStatus::NotApplicable
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct LockPolicyChecker {
    runner: ProcessRunner,
}

impl LockPolicyChecker {
    pub fn evaluate(&self, repo: &Path, base: &str, actor: &str) -> Result<LockPolicyReport> {
        let base = base.trim();
        let actor = actor.trim();
        if base.is_empty() {
            bail!("lock-policy base ref is required");
        }
        if actor.is_empty() {
            bail!("lock-policy actor is required");
        }

        let base_sha = self
            .runner
            .run(
                "git",
                ["rev-parse", "--verify", &format!("{base}^{{commit}}")],
                Some(repo),
            )?
            .trim()
            .to_owned();
        let head_sha = self
            .runner
            .run(
                "git",
                ["rev-parse", "--verify", "HEAD^{commit}"],
                Some(repo),
            )?
            .trim()
            .to_owned();
        let repository = self
            .runner
            .run("git", ["remote", "get-url", "origin"], Some(repo))
            .map(|value| value.trim().to_owned())
            .unwrap_or_default();

        let changed = self.changed_paths(repo, base)?;
        let policy_files = changed
            .iter()
            .filter(|path| is_attributes_file(path))
            .cloned()
            .collect::<Vec<_>>();
        if !policy_files.is_empty() {
            return Ok(LockPolicyReport {
                status: LockPolicyStatus::FailPolicyChanged,
                actor: actor.to_owned(),
                repository,
                base: base.to_owned(),
                base_sha,
                head_sha,
                protected_paths: Vec::new(),
                violations: policy_files
                    .into_iter()
                    .map(|path| LockPolicyViolation {
                        path,
                        code: LockPolicyStatus::FailPolicyChanged,
                        owner: None,
                        message: "Lock policy file changed. Protected-pattern changes require the dedicated administrator policy workflow.".to_owned(),
                    })
                    .collect(),
            });
        }

        let mut protected = Vec::new();
        for path in changed {
            if self.is_lockable_at(repo, base, &path)?
                || self.is_lockable_at(repo, "HEAD", &path)?
            {
                protected.push(path);
            }
        }

        if protected.is_empty() {
            let mut report = evaluate_lock_policy(base, actor, &protected, &[]);
            report.repository = repository;
            report.base_sha = base_sha;
            report.head_sha = head_sha;
            return Ok(report);
        }

        let (ok, output) =
            self.runner
                .run_allow_failure("git", ["lfs", "locks", "--json"], Some(repo))?;
        if !ok {
            return Ok(LockPolicyReport {
                status: LockPolicyStatus::FailPolicyUnavailable,
                actor: actor.to_owned(),
                repository,
                base: base.to_owned(),
                base_sha,
                head_sha,
                protected_paths: protected.clone(),
                violations: protected
                    .into_iter()
                    .map(|path| LockPolicyViolation {
                        path,
                        code: LockPolicyStatus::FailPolicyUnavailable,
                        owner: None,
                        message: "Git LFS lock service could not be queried.".to_owned(),
                    })
                    .collect(),
            });
        }

        let locks = parse_lfs_locks_json(&output)?;
        let mut report = evaluate_lock_policy(base, actor, &protected, &locks);
        report.repository = repository;
        report.base_sha = base_sha;
        report.head_sha = head_sha;
        Ok(report)
    }

    fn changed_paths(&self, repo: &Path, base: &str) -> Result<Vec<String>> {
        let range = format!("{base}...HEAD");
        let output = self.runner.run(
            "git",
            [
                "diff",
                "--name-only",
                "-z",
                "--diff-filter=ACDMRTUXB",
                &range,
            ],
            Some(repo),
        )?;
        Ok(output
            .split('\0')
            .filter(|path| !path.is_empty())
            .map(|path| path.replace('\\', "/"))
            .collect())
    }

    fn is_lockable_at(&self, repo: &Path, source: &str, path: &str) -> Result<bool> {
        let source_arg = format!("--source={source}");
        let output = self.runner.run(
            "git",
            ["check-attr", &source_arg, "lockable", "--", path],
            Some(repo),
        )?;
        let value = output
            .rsplit_once(':')
            .map(|(_, value)| value.trim())
            .unwrap_or_default();
        Ok(matches!(value, "set" | "true" | "yes" | "on"))
    }
}

fn is_attributes_file(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    normalized == ".gitattributes" || normalized.ends_with("/.gitattributes")
}

pub fn evaluate_lock_policy(
    base: &str,
    actor: &str,
    protected_paths: &[String],
    locks: &[LfsLock],
) -> LockPolicyReport {
    if protected_paths.is_empty() {
        return LockPolicyReport {
            status: LockPolicyStatus::NotApplicable,
            actor: actor.to_owned(),
            repository: String::new(),
            base: base.to_owned(),
            base_sha: String::new(),
            head_sha: String::new(),
            protected_paths: Vec::new(),
            violations: Vec::new(),
        };
    }

    let mut violations = Vec::new();
    for path in protected_paths {
        let normalized = path.replace('\\', "/");
        match locks
            .iter()
            .find(|lock| lock.path.replace('\\', "/") == normalized)
        {
            Some(lock) if identity_matches(&lock.owner, actor) => {}
            Some(lock) => violations.push(LockPolicyViolation {
                path: path.clone(),
                code: LockPolicyStatus::FailLockOwnedByOther,
                owner: Some(lock.owner.clone()),
                message: format!(
                    "{path} is protected and currently locked by {}, not {actor}.",
                    lock.owner
                ),
            }),
            None => violations.push(LockPolicyViolation {
                path: path.clone(),
                code: LockPolicyStatus::FailLockMissing,
                owner: None,
                message: format!("{path} is protected but has no active Git LFS lock."),
            }),
        }
    }

    let status = if violations
        .iter()
        .any(|item| item.code == LockPolicyStatus::FailLockOwnedByOther)
    {
        LockPolicyStatus::FailLockOwnedByOther
    } else if violations
        .iter()
        .any(|item| item.code == LockPolicyStatus::FailLockMissing)
    {
        LockPolicyStatus::FailLockMissing
    } else {
        LockPolicyStatus::Pass
    };

    LockPolicyReport {
        status,
        actor: actor.to_owned(),
        repository: String::new(),
        base: base.to_owned(),
        base_sha: String::new(),
        head_sha: String::new(),
        protected_paths: protected_paths.to_vec(),
        violations,
    }
}

fn identity_matches(owner: &str, actor: &str) -> bool {
    owner.trim().eq_ignore_ascii_case(actor.trim())
}

#[cfg(test)]
mod tests {
    use super::{LockPolicyStatus, evaluate_lock_policy};
    use crate::domain::{LfsLock, LfsLockOwnership};

    fn lock(path: &str, owner: &str) -> LfsLock {
        LfsLock {
            id: format!("{owner}-{path}"),
            path: path.to_owned(),
            owner: owner.to_owned(),
            locked_at: String::new(),
            ownership: LfsLockOwnership::Unknown,
        }
    }

    #[test]
    fn not_applicable_when_no_protected_files_changed() {
        let report = evaluate_lock_policy("origin/main", "Ayoub", &[], &[]);
        assert_eq!(report.status, LockPolicyStatus::NotApplicable);
        assert!(report.allows_merge());
    }

    #[test]
    fn passes_when_actor_owns_every_protected_lock() {
        let paths = vec!["Assets/a.psd".to_owned(), "Assets/b.blend".to_owned()];
        let locks = vec![
            lock("Assets/a.psd", "Ayoub"),
            lock("Assets/b.blend", "ayoub"),
        ];
        let report = evaluate_lock_policy("origin/main", "AYOUB", &paths, &locks);
        assert_eq!(report.status, LockPolicyStatus::Pass);
        assert!(report.violations.is_empty());
        assert!(report.allows_merge());
    }

    #[test]
    fn recognizes_attribute_policy_files() {
        assert!(super::is_attributes_file(".gitattributes"));
        assert!(super::is_attributes_file("Assets/.gitattributes"));
        assert!(!super::is_attributes_file("docs/gitattributes.md"));
    }

    #[test]
    fn fails_when_a_required_lock_is_missing() {
        let paths = vec!["Assets/a.psd".to_owned()];
        let report = evaluate_lock_policy("origin/main", "Ayoub", &paths, &[]);
        assert_eq!(report.status, LockPolicyStatus::FailLockMissing);
        assert_eq!(report.violations.len(), 1);
        assert!(!report.allows_merge());
    }

    #[test]
    fn fails_when_teammate_owns_a_required_lock() {
        let paths = vec!["Assets/a.psd".to_owned()];
        let report = evaluate_lock_policy(
            "origin/main",
            "Ayoub",
            &paths,
            &[lock("Assets/a.psd", "Saad")],
        );
        assert_eq!(report.status, LockPolicyStatus::FailLockOwnedByOther);
        assert_eq!(report.violations[0].owner.as_deref(), Some("Saad"));
        assert!(!report.allows_merge());
    }
}
