use gitgat::git::{ConflictChoice, GitClient, ResetMode};
use gitgat::store::Catalog;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

// E2E tests use isolated repositories and never touch the developer's working tree.

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

fn init_repo(path: &Path) {
    std::fs::create_dir_all(path).unwrap();
    git(path, &["init", "--initial-branch=main"]);
    configure_identity(path);
}

fn configure_identity(repo: &Path) {
    git(
        repo,
        &["config", "user.email", "gitgat-e2e@example.invalid"],
    );
    git(repo, &["config", "user.name", "GitGat E2E"]);
}

fn write(path: &Path, content: &str) {
    std::fs::write(path, content).unwrap();
}

fn commit_all(repo: &Path, message: &str) -> String {
    git(repo, &["add", "."]);
    git(repo, &["commit", "-m", message]);
    git(repo, &["rev-parse", "HEAD"])
}

fn create_remote_fixture(root: &TempDir) -> (GitClient, Catalog, std::path::PathBuf) {
    let bare = root.path().join("remote.git");
    std::fs::create_dir_all(&bare).unwrap();
    git(&bare, &["init", "--bare", "--initial-branch=main"]);

    let seed = root.path().join("seed");
    init_repo(&seed);
    write(&seed.join("tracked.txt"), "seed\n");
    write(&seed.join("conflict.txt"), "base\n");
    write(&seed.join("abort.txt"), "base\n");
    commit_all(&seed, "initial");
    git(&seed, &["remote", "add", "origin", bare.to_str().unwrap()]);
    git(&seed, &["push", "-u", "origin", "main"]);

    let catalog = Catalog::open(root.path().join("catalog.db")).unwrap();
    (GitClient::new(catalog.clone()), catalog, bare)
}

fn clean_test_changes(repo: &Path) {
    git(repo, &["reset", "--hard", "HEAD"]);
    git(repo, &["clean", "-fd"]);
}

#[test]
fn end_to_end_local_git_and_catalog_flow() {
    let root = tempfile::tempdir().unwrap();
    let (client, catalog, bare) = create_remote_fixture(&root);

    let repo = root.path().join("primary");
    let cloned = client
        .clone_repository(bare.to_str().unwrap(), &repo)
        .expect("clone through GitGat");
    configure_identity(&repo);

    assert_eq!(cloned, client.repository_root(&repo).unwrap());
    assert!(client.is_repository(&repo));
    assert_eq!(client.status(&repo).unwrap().branch, "main");

    catalog.set_favorite(&repo, true).unwrap();
    let workspace_id = catalog.create_workspace("E2E Workspace").unwrap();
    catalog.add_to_workspace(workspace_id, &repo).unwrap();
    let repositories = catalog.repositories().unwrap();
    assert!(
        repositories
            .iter()
            .any(|item| item.path == cloned && item.favorite)
    );
    let workspaces = catalog.workspaces().unwrap();
    assert!(
        workspaces
            .iter()
            .any(|workspace| workspace.name == "E2E Workspace"
                && workspace.repositories.contains(&cloned))
    );

    // Changes / diff / stage / unstage / guarded discard.
    write(&repo.join("tracked.txt"), "edited\n");
    write(&repo.join("new file.txt"), "untracked\n");
    let changes = client.changes(&repo).unwrap();
    assert_eq!(changes.len(), 2);
    assert!(
        client
            .diff(&repo, "tracked.txt", false)
            .unwrap()
            .contains("edited")
    );

    client.stage(&repo, &["tracked.txt".to_owned()]).unwrap();
    assert!(
        client
            .diff(&repo, "tracked.txt", true)
            .unwrap()
            .contains("edited")
    );
    client.unstage(&repo, &["tracked.txt".to_owned()]).unwrap();
    assert!(
        client
            .discard(&repo, &["new file.txt".to_owned()])
            .unwrap_err()
            .to_string()
            .contains("never deletes untracked")
    );
    client.discard(&repo, &["tracked.txt".to_owned()]).unwrap();
    assert_eq!(
        std::fs::read_to_string(repo.join("tracked.txt")).unwrap(),
        "seed\n"
    );
    std::fs::remove_file(repo.join("new file.txt")).unwrap();

    // Commit and normal push.
    write(&repo.join("tracked.txt"), "primary commit\n");
    client.stage(&repo, &["tracked.txt".to_owned()]).unwrap();
    client.commit(&repo, "primary commit").unwrap();
    client.push(&repo).unwrap();

    // A second clone pushes a remote change; the first clone fetches and FF-only pulls it.
    let peer = root.path().join("peer");
    client
        .clone_repository(bare.to_str().unwrap(), &peer)
        .expect("peer clone");
    configure_identity(&peer);
    write(&peer.join("peer.txt"), "from peer\n");
    client.stage(&peer, &["peer.txt".to_owned()]).unwrap();
    client.commit(&peer, "peer commit").unwrap();
    client.push(&peer).unwrap();

    client.fetch(&repo).unwrap();
    assert!(client.status(&repo).unwrap().behind >= 1);
    client.pull_ff_only(&repo).unwrap();
    assert_eq!(
        std::fs::read_to_string(repo.join("peer.txt")).unwrap(),
        "from peer\n"
    );

    // Branch creation, dirty-worktree guard, switch and safe delete.
    client.create_branch(&repo, "feature/e2e").unwrap();
    write(&repo.join("tracked.txt"), "dirty guard\n");
    let switch_error = client.checkout_branch(&repo, "feature/e2e").unwrap_err();
    assert!(switch_error.to_string().contains("clean working tree"));
    client.discard(&repo, &["tracked.txt".to_owned()]).unwrap();

    client.checkout_branch(&repo, "feature/e2e").unwrap();
    write(&repo.join("feature.txt"), "feature\n");
    client.stage(&repo, &["feature.txt".to_owned()]).unwrap();
    client.commit(&repo, "feature commit").unwrap();
    client.checkout_branch(&repo, "main").unwrap();
    git(
        &repo,
        &["merge", "--no-ff", "feature/e2e", "-m", "merge feature"],
    );
    client.delete_branch(&repo, "feature/e2e").unwrap();

    let branches = client.branches(&repo).unwrap();
    assert!(
        branches
            .iter()
            .any(|branch| branch.name == "main" && !branch.remote)
    );
    assert!(
        branches
            .iter()
            .any(|branch| branch.remote && branch.name.ends_with("origin/main"))
    );
    assert!(!client.history(&repo, 20).unwrap().is_empty());

    // Stash push/apply/pop/drop, including untracked content.
    write(&repo.join("tracked.txt"), "stashed\n");
    write(&repo.join("stash-untracked.txt"), "stash me\n");
    client.stash_push(&repo, "e2e stash", true).unwrap();
    let stash = client.stashes(&repo).unwrap().into_iter().next().unwrap();
    client.stash_apply(&repo, &stash.reference).unwrap();
    assert_eq!(
        std::fs::read_to_string(repo.join("tracked.txt")).unwrap(),
        "stashed\n"
    );
    clean_test_changes(&repo);
    client.stash_pop(&repo, &stash.reference).unwrap();
    assert_eq!(
        std::fs::read_to_string(repo.join("tracked.txt")).unwrap(),
        "stashed\n"
    );
    clean_test_changes(&repo);

    write(&repo.join("tracked.txt"), "drop me\n");
    client.stash_push(&repo, "drop stash", false).unwrap();
    let drop_ref = client.stashes(&repo).unwrap()[0].reference.clone();
    client.stash_drop(&repo, &drop_ref).unwrap();

    // Conflict detection, marker guard and "ours" resolution.
    client.create_branch(&repo, "conflict-side").unwrap();
    client.checkout_branch(&repo, "conflict-side").unwrap();
    write(&repo.join("conflict.txt"), "side\n");
    client.stage(&repo, &["conflict.txt".to_owned()]).unwrap();
    client.commit(&repo, "side conflict").unwrap();

    client.checkout_branch(&repo, "main").unwrap();
    write(&repo.join("conflict.txt"), "main\n");
    client.stage(&repo, &["conflict.txt".to_owned()]).unwrap();
    client.commit(&repo, "main conflict").unwrap();

    let merge = command(&repo, &["merge", "conflict-side"]);
    assert!(!merge.status.success());
    assert_eq!(client.conflicts(&repo).unwrap(), vec!["conflict.txt"]);
    assert!(
        client
            .mark_conflict_resolved(&repo, "conflict.txt")
            .is_err()
    );
    client
        .resolve_conflict(&repo, "conflict.txt", ConflictChoice::Ours)
        .unwrap();
    assert!(client.conflicts(&repo).unwrap().is_empty());
    git(&repo, &["commit", "-m", "resolve conflict with ours"]);
    client.delete_branch(&repo, "conflict-side").unwrap();

    // A second conflict proves "theirs" resolution.
    client.create_branch(&repo, "theirs-side").unwrap();
    client.checkout_branch(&repo, "theirs-side").unwrap();
    write(&repo.join("conflict.txt"), "theirs\n");
    client.stage(&repo, &["conflict.txt".to_owned()]).unwrap();
    client.commit(&repo, "theirs change").unwrap();
    client.checkout_branch(&repo, "main").unwrap();
    write(&repo.join("conflict.txt"), "ours again\n");
    client.stage(&repo, &["conflict.txt".to_owned()]).unwrap();
    client.commit(&repo, "ours change").unwrap();
    assert!(!command(&repo, &["merge", "theirs-side"]).status.success());
    client
        .resolve_conflict(&repo, "conflict.txt", ConflictChoice::Theirs)
        .unwrap();
    git(&repo, &["commit", "-m", "resolve conflict with theirs"]);
    assert_eq!(
        std::fs::read_to_string(repo.join("conflict.txt")).unwrap(),
        "theirs\n"
    );
    client.delete_branch(&repo, "theirs-side").unwrap();

    // Abort an active merge conflict.
    client.create_branch(&repo, "abort-side").unwrap();
    client.checkout_branch(&repo, "abort-side").unwrap();
    write(&repo.join("abort.txt"), "side abort\n");
    client.stage(&repo, &["abort.txt".to_owned()]).unwrap();
    client.commit(&repo, "abort side").unwrap();
    client.checkout_branch(&repo, "main").unwrap();
    write(&repo.join("abort.txt"), "main abort\n");
    client.stage(&repo, &["abort.txt".to_owned()]).unwrap();
    client.commit(&repo, "abort main").unwrap();
    assert!(!command(&repo, &["merge", "abort-side"]).status.success());
    client.abort_current_operation(&repo).unwrap();
    assert!(client.conflicts(&repo).unwrap().is_empty());
    assert_eq!(
        std::fs::read_to_string(repo.join("abort.txt")).unwrap(),
        "main abort\n"
    );

    // Rebase a clean feature branch onto a newer main.
    client.create_branch(&repo, "rebase-e2e").unwrap();
    client.checkout_branch(&repo, "rebase-e2e").unwrap();
    write(&repo.join("rebase-feature.txt"), "feature\n");
    client
        .stage(&repo, &["rebase-feature.txt".to_owned()])
        .unwrap();
    client.commit(&repo, "rebase feature").unwrap();
    client.checkout_branch(&repo, "main").unwrap();
    write(&repo.join("main-only.txt"), "main moved\n");
    client.stage(&repo, &["main-only.txt".to_owned()]).unwrap();
    client.commit(&repo, "advance main").unwrap();
    client.checkout_branch(&repo, "rebase-e2e").unwrap();
    client.rebase(&repo, "main").unwrap();
    assert!(repo.join("main-only.txt").exists());
    assert!(repo.join("rebase-feature.txt").exists());
    client.checkout_branch(&repo, "main").unwrap();

    // Cherry-pick from another branch.
    client.create_branch(&repo, "cherry-source").unwrap();
    client.checkout_branch(&repo, "cherry-source").unwrap();
    write(&repo.join("cherry.txt"), "pick me\n");
    client.stage(&repo, &["cherry.txt".to_owned()]).unwrap();
    client.commit(&repo, "cherry source").unwrap();
    let cherry_sha = git(&repo, &["rev-parse", "HEAD"]);
    client.checkout_branch(&repo, "main").unwrap();
    client.cherry_pick(&repo, &cherry_sha).unwrap();
    assert_eq!(
        std::fs::read_to_string(repo.join("cherry.txt")).unwrap(),
        "pick me\n"
    );

    // Soft and mixed reset keep worktree content while changing index state.
    let before_reset = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("tracked.txt"), "reset content\n");
    client.stage(&repo, &["tracked.txt".to_owned()]).unwrap();
    client.commit(&repo, "reset candidate").unwrap();
    client.reset(&repo, &before_reset, ResetMode::Soft).unwrap();
    assert!(
        client
            .changes(&repo)
            .unwrap()
            .iter()
            .any(|change| { change.path == "tracked.txt" && change.staged })
    );
    client
        .reset(&repo, &before_reset, ResetMode::Mixed)
        .unwrap();
    assert!(
        client
            .changes(&repo)
            .unwrap()
            .iter()
            .any(|change| { change.path == "tracked.txt" && !change.staged })
    );
    client.discard(&repo, &["tracked.txt".to_owned()]).unwrap();

    // Hard reset moves a clean tree backwards; recovery branch restores the discarded commit.
    let hard_base = git(&repo, &["rev-parse", "HEAD"]);
    write(&repo.join("tracked.txt"), "hard reset candidate\n");
    client.stage(&repo, &["tracked.txt".to_owned()]).unwrap();
    client.commit(&repo, "hard reset candidate").unwrap();
    let discarded_sha = git(&repo, &["rev-parse", "HEAD"]);
    client.reset(&repo, &hard_base, ResetMode::Hard).unwrap();
    assert_ne!(git(&repo, &["rev-parse", "HEAD"]), discarded_sha);
    client
        .create_recovery_branch(&repo, "recovery/e2e", &discarded_sha)
        .unwrap();
    assert_eq!(git(&repo, &["rev-parse", "recovery/e2e"]), discarded_sha);

    // Worktree lifecycle.
    client.create_branch(&repo, "worktree-e2e").unwrap();
    let worktree_path = root.path().join("secondary-worktree");
    client
        .add_worktree(&repo, &worktree_path, "worktree-e2e")
        .unwrap();
    let expected_worktree = worktree_path.canonicalize().unwrap();
    assert!(
        client
            .worktrees(&repo)
            .unwrap()
            .iter()
            .filter_map(|item| item.path.canonicalize().ok())
            .any(|path| path == expected_worktree)
    );
    client.remove_worktree(&repo, &worktree_path).unwrap();
    assert!(!worktree_path.exists());
    client.delete_branch(&repo, "worktree-e2e").unwrap();

    // Reflog, snapshots and risky-operation journal are populated.
    assert!(!client.reflog(&repo, 100).unwrap().is_empty());
    let snapshot = client.snapshot(&repo, 100).unwrap();
    assert_eq!(snapshot.status.branch, "main");
    assert!(!snapshot.commits.is_empty());
    let journal = catalog.recent_journal(&repo, 100).unwrap();
    for expected in [
        "stash-pop",
        "stash-drop",
        "rebase",
        "cherry-pick",
        "reset",
        "recovery-branch",
    ] {
        assert!(
            journal.iter().any(|line| line.contains(expected)),
            "journal should contain {expected}: {journal:?}"
        );
    }

    // Catalog relocation persists across repository and workspace records.
    let movable = root.path().join("movable");
    init_repo(&movable);
    write(&movable.join("x.txt"), "x\n");
    commit_all(&movable, "movable");
    catalog.touch_repository(&movable).unwrap();
    let move_workspace = catalog.create_workspace("Relocation E2E").unwrap();
    catalog.add_to_workspace(move_workspace, &movable).unwrap();
    let moved = root.path().join("moved");
    std::fs::rename(&movable, &moved).unwrap();
    catalog.relocate_repository(&movable, &moved).unwrap();
    let moved = moved.canonicalize().unwrap();
    assert!(
        catalog
            .repositories()
            .unwrap()
            .iter()
            .any(|item| item.path == moved)
    );
    assert!(
        catalog
            .workspaces()
            .unwrap()
            .iter()
            .any(|workspace| workspace.name == "Relocation E2E"
                && workspace.repositories.contains(&moved))
    );
}

#[test]
fn ff_only_pull_rejects_divergent_history() {
    let root = tempfile::tempdir().unwrap();
    let (client, _catalog, bare) = create_remote_fixture(&root);

    let left = root.path().join("left");
    let right = root.path().join("right");
    client
        .clone_repository(bare.to_str().unwrap(), &left)
        .unwrap();
    client
        .clone_repository(bare.to_str().unwrap(), &right)
        .unwrap();
    configure_identity(&left);
    configure_identity(&right);

    write(&left.join("left.txt"), "left\n");
    client.stage(&left, &["left.txt".to_owned()]).unwrap();
    client.commit(&left, "left commit").unwrap();

    write(&right.join("right.txt"), "right\n");
    client.stage(&right, &["right.txt".to_owned()]).unwrap();
    client.commit(&right, "right commit").unwrap();
    client.push(&right).unwrap();

    client.fetch(&left).unwrap();
    let error = client.pull_ff_only(&left).unwrap_err();
    let text = error.to_string().to_lowercase();
    assert!(
        text.contains("fast-forward") || text.contains("diverg") || text.contains("not possible"),
        "unexpected pull failure: {text}"
    );
    assert_eq!(
        std::fs::read_to_string(left.join("left.txt")).unwrap(),
        "left\n"
    );
}
