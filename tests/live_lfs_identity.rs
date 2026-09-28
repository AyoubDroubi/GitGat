use gitgat::domain::LfsLockOwnership;
use gitgat::git::GitClient;
use gitgat::store::Catalog;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn command(cwd: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("git command should start")
}

fn git(cwd: &Path, args: &[&str]) -> String {
    let output = command(cwd, args);
    assert!(
        output.status.success(),
        "git {:?} failed:\nstdout: {}\nstderr: {}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn make_writable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path).unwrap().permissions();
        permissions.set_mode(permissions.mode() | 0o200);
        std::fs::set_permissions(path, permissions).unwrap();
    }

    #[cfg(windows)]
    {
        let status = Command::new("attrib")
            .arg("-R")
            .arg(path)
            .status()
            .expect("attrib should start");
        assert!(
            status.success(),
            "attrib -R should make the fixture writable"
        );
    }
}

fn live_fixture() -> (tempfile::TempDir, GitClient, PathBuf, String) {
    let repo = PathBuf::from(
        std::env::var("GITGAT_LIVE_REPO")
            .expect("GITGAT_LIVE_REPO must point to the teammate clone"),
    );
    let path = std::env::var("GITGAT_LIVE_LOCK_PATH").expect("GITGAT_LIVE_LOCK_PATH must be set");
    assert!(repo.join(&path).exists(), "protected file should exist");
    let root = tempfile::tempdir().unwrap();
    let catalog = Catalog::open(root.path().join("catalog.db")).unwrap();
    (root, GitClient::new(catalog), repo, path)
}

#[test]
#[ignore = "requires two distinct live provider identities; run only from the dedicated workflow"]
fn teammate_lock_blocks_stage_commit_and_push() {
    let (_root, client, repo, path) = live_fixture();

    assert!(client.is_lfs_lockable(&repo, &path).unwrap());

    let locks = client.lfs_verified_locks(&repo).unwrap();
    let lock = locks
        .iter()
        .find(|lock| lock.path == path)
        .expect("remote lock should be visible");
    assert_eq!(lock.ownership, LfsLockOwnership::Theirs);
    assert!(
        std::fs::metadata(repo.join(&path))
            .unwrap()
            .permissions()
            .readonly(),
        "teammate copy should be read-only while another identity owns the lock"
    );

    // Simulate a user deliberately bypassing the read-only bit outside GitGat.
    let file = repo.join(&path);
    make_writable(&file);
    std::fs::write(&file, b"teammate attempted edit\n").unwrap();

    let stage_error = client
        .stage(&repo, std::slice::from_ref(&path))
        .unwrap_err()
        .to_string();
    assert!(
        stage_error.contains("locked by"),
        "unexpected Stage error: {stage_error}"
    );

    // Simulate bypassing GitGat Stage with raw Git. Commit must still be blocked.
    git(&repo, &["add", "--", &path]);
    let commit_error = client
        .commit(&repo, "attempted protected commit")
        .unwrap_err()
        .to_string();
    assert!(
        commit_error.contains("locked by"),
        "unexpected Commit error: {commit_error}"
    );

    // Simulate bypassing both GitGat Stage and Commit. Strict LFS push verification
    // must still stop the conflicting update.
    git(&repo, &["commit", "-m", "raw bypass attempt"]);
    let push_error = client.push(&repo).unwrap_err().to_string();
    let push_error_lower = push_error.to_lowercase();
    assert!(
        push_error_lower.contains("lock")
            || push_error_lower.contains("locked")
            || push_error_lower.contains("verify"),
        "unexpected Push error: {push_error}"
    );
}

#[test]
#[ignore = "requires a live provider identity that owns the protected-file lock"]
fn owned_lock_allows_normal_gitgat_flow() {
    let (_root, client, repo, path) = live_fixture();

    let locks = client.lfs_verified_locks(&repo).unwrap();
    let lock = locks
        .iter()
        .find(|lock| lock.path == path)
        .expect("owned remote lock should be visible");
    assert_eq!(lock.ownership, LfsLockOwnership::Ours);
    assert!(
        !std::fs::metadata(repo.join(&path))
            .unwrap()
            .permissions()
            .readonly(),
        "owner copy should be writable"
    );

    std::fs::write(repo.join(&path), b"owned edit accepted\n").unwrap();
    client.stage(&repo, std::slice::from_ref(&path)).unwrap();
    client.commit(&repo, "test(e2e): owned lock edit").unwrap();
    client.push(&repo).unwrap();
}
