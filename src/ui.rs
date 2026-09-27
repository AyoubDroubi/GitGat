use crate::domain::{ForgeSnapshot, RepositorySnapshot};
use crate::forge::GitHubClient;
use crate::git::{ConflictChoice, GitClient, ResetMode};
use crate::store::Catalog;
use anyhow::Result;
use eframe::egui::{self, Key, RichText, ScrollArea, TextEdit};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Overview,
    Changes,
    Branches,
    History,
    PullRequests,
    Locks,
    Actions,
    MyWork,
    Workspaces,
    Advanced,
    Settings,
}

impl Tab {
    const ALL: [Tab; 11] = [
        Tab::Overview,
        Tab::Changes,
        Tab::Branches,
        Tab::History,
        Tab::PullRequests,
        Tab::Locks,
        Tab::Actions,
        Tab::MyWork,
        Tab::Workspaces,
        Tab::Advanced,
        Tab::Settings,
    ];
    fn label(self) -> &'static str {
        match self {
            Tab::Overview => "Overview",
            Tab::Changes => "Changes",
            Tab::Branches => "Branches",
            Tab::History => "History",
            Tab::PullRequests => "Pull Requests",
            Tab::Locks => "LFS Locks",
            Tab::Actions => "Actions",
            Tab::MyWork => "My Work",
            Tab::Workspaces => "Workspaces",
            Tab::Advanced => "Advanced",
            Tab::Settings => "Settings",
        }
    }
}

pub struct GitGatApp {
    catalog: Catalog,
    git: GitClient,
    forge: GitHubClient,
    tab: Tab,
    repository: Option<PathBuf>,
    snapshot: RepositorySnapshot,
    forge_snapshot: ForgeSnapshot,
    selected_paths: BTreeSet<String>,
    repository_input: String,
    clone_url: String,
    clone_destination: String,
    commit_message: String,
    branch_name: String,
    target_ref: String,
    confirmation: String,
    stash_message: String,
    stash_ref: String,
    worktree_path: String,
    worktree_branch: String,
    lfs_path: String,
    pr_number: String,
    pr_title: String,
    pr_body: String,
    pr_comment: String,
    run_id: String,
    workspace_name: String,
    diff_text: String,
    relocate_from: String,
    relocate_to: String,
    output_text: String,
    notice: String,
    history_limit: usize,
}

impl GitGatApp {
    pub fn new(catalog: Catalog) -> Self {
        Self {
            git: GitClient::new(catalog.clone()),
            catalog,
            forge: GitHubClient::default(),
            tab: Tab::Overview,
            repository: None,
            snapshot: RepositorySnapshot::default(),
            forge_snapshot: ForgeSnapshot::default(),
            selected_paths: BTreeSet::new(),
            repository_input: String::new(),
            clone_url: String::new(),
            clone_destination: String::new(),
            commit_message: String::new(),
            branch_name: String::new(),
            target_ref: String::from("HEAD~1"),
            confirmation: String::new(),
            stash_message: String::new(),
            stash_ref: String::from("stash@{0}"),
            worktree_path: String::new(),
            worktree_branch: String::new(),
            lfs_path: String::new(),
            pr_number: String::new(),
            pr_title: String::new(),
            pr_body: String::new(),
            pr_comment: String::new(),
            run_id: String::new(),
            workspace_name: String::new(),
            diff_text: String::new(),
            relocate_from: String::new(),
            relocate_to: String::new(),
            output_text: String::new(),
            notice: String::from("Open or clone a repository to begin."),
            history_limit: 100,
        }
    }

    fn current_repo(&self) -> Option<PathBuf> {
        self.repository.clone()
    }

    fn open_repository(&mut self, path: PathBuf) {
        let candidate = if path.is_file() {
            path.parent().map(Path::to_path_buf).unwrap_or(path)
        } else {
            path
        };
        match self.git.repository_root(&candidate) {
            Ok(root) => {
                if let Err(error) = self.catalog.touch_repository(&root) {
                    self.notice = error.to_string();
                    return;
                }
                self.repository_input = root.to_string_lossy().to_string();
                self.repository = Some(root);
                self.selected_paths.clear();
                self.refresh_git();
            }
            Err(error) => self.notice = format!("Not a Git repository: {error}"),
        }
    }

    fn refresh_git(&mut self) {
        let Some(repo) = self.current_repo() else {
            return;
        };
        match self.git.snapshot(&repo, self.history_limit) {
            Ok(snapshot) => {
                self.snapshot = snapshot;
                self.notice = "Repository refreshed.".to_owned();
            }
            Err(error) => self.notice = error.to_string(),
        }
    }

    fn refresh_forge(&mut self) {
        let Some(repo) = self.current_repo() else {
            self.notice = "Open a repository first.".to_owned();
            return;
        };
        match self.forge.snapshot(&repo) {
            Ok(snapshot) => {
                self.forge_snapshot = snapshot;
                self.notice = "GitHub data refreshed.".to_owned();
            }
            Err(error) => self.notice = error.to_string(),
        }
    }

    fn finish_git(&mut self, result: Result<String>) {
        match result {
            Ok(text) => {
                self.output_text = text;
                self.notice = "Operation completed.".to_owned();
                self.refresh_git();
            }
            Err(error) => self.notice = error.to_string(),
        }
    }

    fn finish_forge(&mut self, result: Result<String>) {
        match result {
            Ok(text) => {
                self.output_text = text;
                self.notice = "GitHub operation completed.".to_owned();
                self.refresh_forge();
            }
            Err(error) => self.notice = error.to_string(),
        }
    }

    fn selected(&self) -> Vec<String> {
        self.selected_paths.iter().cloned().collect()
    }
    fn parse_pr(&self) -> Result<u64> {
        Ok(self.pr_number.trim().parse::<u64>()?)
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let refresh = ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::R));
        let commit = ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Enter));
        let fetch = ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(Key::F));
        let push = ctx.input(|i| i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(Key::P));
        if refresh {
            self.refresh_git();
        }
        if let Some(repo) = self.current_repo() {
            if commit && !self.commit_message.trim().is_empty() {
                let message = self.commit_message.clone();
                let result = self.git.commit(&repo, &message);
                self.finish_git(result);
            } else if fetch {
                let result = self.git.fetch(&repo);
                self.finish_git(result);
            } else if push {
                let result = self.git.push(&repo);
                self.finish_git(result);
            }
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("GitGat");
            ui.separator();
            if let Some(repo) = &self.repository {
                ui.monospace(repo.to_string_lossy());
                ui.separator();
                ui.label(format!(
                    "{}  ↑{} ↓{}",
                    self.snapshot.status.branch,
                    self.snapshot.status.ahead,
                    self.snapshot.status.behind
                ));
            } else {
                ui.weak("No repository open");
            }
            ui.separator();
            if ui.button("Refresh").clicked() {
                self.refresh_git();
            }
            if let Some(repo) = self.current_repo() {
                if ui.button("Fetch").clicked() {
                    let r = self.git.fetch(&repo);
                    self.finish_git(r);
                }
                if ui.button("Pull FF-only").clicked() {
                    let r = self.git.pull_ff_only(&repo);
                    self.finish_git(r);
                }
                if ui.button("Push").clicked() {
                    let r = self.git.push(&repo);
                    self.finish_git(r);
                }
            }
        });
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        for tab in Tab::ALL {
            if ui.selectable_label(self.tab == tab, tab.label()).clicked() {
                self.tab = tab;
            }
        }
        ui.add_space(12.0);
        ui.separator();
        ui.small("Shortcuts");
        ui.weak("Ctrl+R Refresh");
        ui.weak("Ctrl+Enter Commit");
        ui.weak("Ctrl+Shift+F Fetch");
        ui.weak("Ctrl+Shift+P Push");
    }

    fn overview(&mut self, ui: &mut egui::Ui) {
        ui.heading("Repository");
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.repository_input).hint_text("Repository folder"));
            if ui.button("Open").clicked() {
                self.open_repository(PathBuf::from(self.repository_input.trim()));
            }
        });
        ui.weak("You can also drag a repository folder onto GitGat.");
        ui.add_space(14.0);
        ui.heading("Clone");
        ui.add(
            TextEdit::singleline(&mut self.clone_url)
                .hint_text("https://github.com/owner/repo.git"),
        );
        ui.horizontal(|ui| {
            ui.add(
                TextEdit::singleline(&mut self.clone_destination).hint_text("Destination folder"),
            );
            if ui.button("Clone").clicked() {
                let destination = PathBuf::from(self.clone_destination.trim());
                match self
                    .git
                    .clone_repository(self.clone_url.trim(), &destination)
                {
                    Ok(root) => self.open_repository(root),
                    Err(error) => self.notice = error.to_string(),
                }
            }
        });
        ui.add_space(18.0);
        ui.heading("Recent repositories");
        match self.catalog.repositories() {
            Ok(repositories) => {
                for repository in repositories {
                    ui.horizontal(|ui| {
                        let mut favorite = repository.favorite;
                        if ui.checkbox(&mut favorite, "").changed()
                            && let Err(error) =
                                self.catalog.set_favorite(&repository.path, favorite)
                        {
                            self.notice = error.to_string();
                        }
                        if ui.button(&repository.name).clicked() {
                            if repository.path.exists() {
                                self.open_repository(repository.path.clone());
                            } else {
                                self.relocate_from = repository.path.to_string_lossy().to_string();
                                self.relocate_to.clear();
                                self.tab = Tab::Settings;
                                self.notice = format!(
                                    "{} no longer exists. Choose its new location below.",
                                    repository.path.display()
                                );
                            }
                        }
                        ui.weak(repository.path.to_string_lossy());
                    });
                }
            }
            Err(error) => {
                ui.label(error.to_string());
            }
        }
        if self.repository.is_some() {
            ui.add_space(18.0);
            ui.heading("At a glance");
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("{} changed files", self.snapshot.changes.len()));
                ui.separator();
                ui.label(format!("{} branches", self.snapshot.branches.len()));
                ui.separator();
                ui.label(format!("{} commits loaded", self.snapshot.commits.len()));
                ui.separator();
                ui.label(format!("{} conflicts", self.snapshot.conflicts.len()));
                ui.separator();
                ui.label(format!("{} LFS locks", self.snapshot.lfs_locks.len()));
            });
        }
    }

    fn changes(&mut self, ui: &mut egui::Ui) {
        ui.heading("Changes");
        let Some(repo) = self.current_repo() else {
            ui.label("Open a repository first.");
            return;
        };
        ui.horizontal(|ui| {
            if ui.button("Stage selected").clicked() {
                let p = self.selected();
                let r = self
                    .git
                    .stage(&repo, &p)
                    .map(|_| "Staged selected files.".to_owned());
                self.finish_git(r);
            }
            if ui.button("Unstage selected").clicked() {
                let p = self.selected();
                let r = self
                    .git
                    .unstage(&repo, &p)
                    .map(|_| "Unstaged selected files.".to_owned());
                self.finish_git(r);
            }
            if ui.button("Discard tracked").clicked() {
                let p = self.selected();
                let r = self
                    .git
                    .discard(&repo, &p)
                    .map(|_| "Discarded tracked changes.".to_owned());
                self.finish_git(r);
            }
            if ui.button("Select all").clicked() {
                self.selected_paths = self
                    .snapshot
                    .changes
                    .iter()
                    .map(|v| v.path.clone())
                    .collect();
            }
            if ui.button("Clear").clicked() {
                self.selected_paths.clear();
            }
        });
        ui.add_space(8.0);
        ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
            for change in self.snapshot.changes.clone() {
                ui.horizontal(|ui| {
                    let mut checked = self.selected_paths.contains(&change.path);
                    if ui.checkbox(&mut checked, "").changed() {
                        if checked {
                            self.selected_paths.insert(change.path.clone());
                        } else {
                            self.selected_paths.remove(&change.path);
                        }
                    }
                    ui.monospace(format!("{}{}", change.index_status, change.worktree_status));
                    if ui.selectable_label(false, &change.path).clicked() {
                        match self.git.diff(&repo, &change.path, change.staged) {
                            Ok(diff) => self.diff_text = diff,
                            Err(error) => self.notice = error.to_string(),
                        }
                    }
                    ui.weak(format!("{:?}", change.kind));
                });
            }
        });
        ui.separator();
        ui.heading("Commit");
        ui.add(
            TextEdit::multiline(&mut self.commit_message)
                .desired_rows(3)
                .hint_text("Commit message"),
        );
        if ui.button("Commit staged changes").clicked() {
            let message = self.commit_message.clone();
            let result = self.git.commit(&repo, &message);
            if result.is_ok() {
                self.commit_message.clear();
            }
            self.finish_git(result);
        }
        if !self.diff_text.is_empty() {
            ui.separator();
            ui.heading("Diff");
            ScrollArea::both().max_height(360.0).show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut self.diff_text)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });
        }
    }

    fn branches(&mut self, ui: &mut egui::Ui) {
        ui.heading("Branches");
        let Some(repo) = self.current_repo() else {
            ui.label("Open a repository first.");
            return;
        };
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.branch_name).hint_text("feature/my-change"));
            if ui.button("Create").clicked() {
                let n = self.branch_name.clone();
                let r = self
                    .git
                    .create_branch(&repo, &n)
                    .map(|_| format!("Created {n}"));
                self.finish_git(r);
            }
        });
        ui.separator();
        for branch in self.snapshot.branches.clone() {
            ui.horizontal(|ui| {
                if branch.current {
                    ui.label(RichText::new("●").strong());
                } else {
                    ui.label(" ");
                }
                ui.monospace(&branch.name);
                if !branch.current && !branch.remote && ui.button("Switch").clicked() {
                    let r = self
                        .git
                        .checkout_branch(&repo, &branch.name)
                        .map(|_| format!("Switched to {}", branch.name));
                    self.finish_git(r);
                }
                if !branch.current && !branch.remote && ui.button("Delete safely").clicked() {
                    let r = self
                        .git
                        .delete_branch(&repo, &branch.name)
                        .map(|_| format!("Deleted {}", branch.name));
                    self.finish_git(r);
                }
                if let Some(upstream) = branch.upstream {
                    ui.weak(upstream);
                }
            });
        }
    }

    fn history(&mut self, ui: &mut egui::Ui) {
        ui.heading("History");
        ui.horizontal(|ui| {
            ui.label(format!("Showing up to {}", self.history_limit));
            if ui.button("Load 100 more").clicked() {
                self.history_limit = (self.history_limit + 100).min(2000);
                self.refresh_git();
            }
        });
        ScrollArea::vertical().show(ui, |ui| {
            for commit in self.snapshot.commits.clone() {
                ui.horizontal_wrapped(|ui| {
                    if ui.selectable_label(false, &commit.short_sha).clicked() {
                        self.target_ref = commit.sha.clone();
                    }
                    ui.label(&commit.subject);
                    ui.weak(&commit.author);
                    if !commit.decorations.is_empty() {
                        ui.monospace(commit.decorations);
                    }
                });
            }
        });
    }

    fn pull_requests(&mut self, ui: &mut egui::Ui) {
        ui.heading("Pull Requests");
        let Some(repo) = self.current_repo() else {
            ui.label("Open a repository first.");
            return;
        };
        ui.horizontal(|ui| {
            if ui.button("Refresh GitHub").clicked() {
                self.refresh_forge();
            }
            ui.label(if self.forge_snapshot.authenticated {
                format!(
                    "Connected as {}",
                    self.forge_snapshot.account.clone().unwrap_or_default()
                )
            } else {
                "GitHub not connected".to_owned()
            });
        });
        for pr in self.forge_snapshot.pull_requests.clone() {
            ui.horizontal_wrapped(|ui| {
                if ui.button(format!("#{}", pr.number)).clicked() {
                    self.pr_number = pr.number.to_string();
                    self.pr_title = pr.title.clone();
                }
                ui.label(&pr.title);
                ui.weak(format!("{} → {} by {}", pr.head, pr.base, pr.author));
                if pr.draft {
                    ui.weak("draft");
                }
            });
        }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("PR #");
            ui.add(TextEdit::singleline(&mut self.pr_number).desired_width(80.0));
            ui.add(TextEdit::singleline(&mut self.pr_title).hint_text("Title"));
        });
        ui.add(
            TextEdit::multiline(&mut self.pr_body)
                .desired_rows(4)
                .hint_text("Body"),
        );
        ui.horizontal(|ui| {
            if ui.button("Update PR").clicked()
                && let Ok(n) = self.parse_pr()
            {
                let t = self.pr_title.clone();
                let b = self.pr_body.clone();
                let r = self.forge.edit_pull_request(&repo, n, &t, &b);
                self.finish_forge(r);
            }
            if ui.button("Approve").clicked()
                && let Ok(n) = self.parse_pr()
            {
                let b = self.pr_comment.clone();
                let r = self.forge.review_pull_request(&repo, n, true, &b);
                self.finish_forge(r);
            }
            if ui.button("Merge").clicked()
                && let Ok(n) = self.parse_pr()
            {
                let r = self.forge.merge_pull_request(&repo, n);
                self.finish_forge(r);
            }
        });
        ui.add(
            TextEdit::multiline(&mut self.pr_comment)
                .desired_rows(3)
                .hint_text("Review/comment"),
        );
        if ui.button("Comment").clicked()
            && let Ok(n) = self.parse_pr()
        {
            let b = self.pr_comment.clone();
            let r = self.forge.comment_pull_request(&repo, n, &b);
            self.finish_forge(r);
        }
    }

    fn locks(&mut self, ui: &mut egui::Ui) {
        ui.heading("Git LFS Locks");
        let Some(repo) = self.current_repo() else {
            ui.label("Open a repository first.");
            return;
        };
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.lfs_path).hint_text("Assets/file.psd"));
            if ui.button("Lock").clicked() {
                let p = self.lfs_path.clone();
                let r = self.git.lfs_lock(&repo, &p);
                self.finish_git(r);
            }
            if ui.button("Unlock").clicked() {
                let p = self.lfs_path.clone();
                let r = self.git.lfs_unlock(&repo, &p);
                self.finish_git(r);
            }
        });
        for lock in self.snapshot.lfs_locks.clone() {
            ui.horizontal(|ui| {
                ui.monospace(lock.path);
                ui.weak(format!("{} · {}", lock.owner, lock.locked_at));
            });
        }
        ui.weak("Force unlock is intentionally not exposed by default.");
    }

    fn actions(&mut self, ui: &mut egui::Ui) {
        ui.heading("GitHub Actions");
        let Some(repo) = self.current_repo() else {
            ui.label("Open a repository first.");
            return;
        };
        ui.horizontal(|ui| {
            if ui.button("Refresh GitHub").clicked() {
                self.refresh_forge();
            }
            ui.add(
                TextEdit::singleline(&mut self.run_id)
                    .desired_width(120.0)
                    .hint_text("Run ID"),
            );
            if ui.button("Rerun").clicked()
                && let Ok(id) = self.run_id.trim().parse::<u64>()
            {
                let r = self.forge.rerun_workflow(&repo, id);
                self.finish_forge(r);
            }
            if ui.button("Failed log").clicked()
                && let Ok(id) = self.run_id.trim().parse::<u64>()
            {
                match self.forge.workflow_log(&repo, id) {
                    Ok(log) => self.output_text = log,
                    Err(error) => self.notice = error.to_string(),
                }
            }
        });
        for run in self.forge_snapshot.workflow_runs.clone() {
            ui.horizontal_wrapped(|ui| {
                if ui.button(run.id.to_string()).clicked() {
                    self.run_id = run.id.to_string();
                }
                ui.label(run.name);
                ui.monospace(format!("{} / {}", run.status, run.conclusion));
                ui.weak(run.branch);
            });
        }
    }

    fn my_work(&mut self, ui: &mut egui::Ui) {
        ui.heading("My Work");
        ui.label(format!("Current branch: {}", self.snapshot.status.branch));
        ui.label(format!("Local changes: {}", self.snapshot.changes.len()));
        ui.label(format!(
            "Open PRs loaded: {}",
            self.forge_snapshot.pull_requests.len()
        ));
        ui.label(format!(
            "Notifications loaded: {}",
            self.forge_snapshot.notifications.len()
        ));
        ui.separator();
        ui.heading("Notifications");
        for notification in self
            .forge_snapshot
            .notifications
            .clone()
            .into_iter()
            .take(30)
        {
            ui.horizontal_wrapped(|ui| {
                ui.label(notification.title);
                ui.weak(format!("{} · {}", notification.kind, notification.reason));
            });
        }
        ui.separator();
        ui.heading("Team activity");
        for activity in self.forge_snapshot.activity.clone().into_iter().take(30) {
            ui.horizontal(|ui| {
                ui.monospace(activity.event_type);
                ui.label(activity.actor);
                ui.weak(activity.created_at);
            });
        }
    }

    fn workspaces(&mut self, ui: &mut egui::Ui) {
        ui.heading("Workspaces");
        ui.horizontal(|ui| {
            ui.add(TextEdit::singleline(&mut self.workspace_name).hint_text("Workspace name"));
            if ui.button("Create").clicked() {
                match self.catalog.create_workspace(&self.workspace_name) {
                    Ok(id) => {
                        if let Some(repo) = self.current_repo()
                            && let Err(error) = self.catalog.add_to_workspace(id, &repo)
                        {
                            self.notice = error.to_string();
                        }
                        self.notice = "Workspace saved.".to_owned();
                    }
                    Err(error) => self.notice = error.to_string(),
                }
            }
        });
        match self.catalog.workspaces() {
            Ok(workspaces) => {
                for workspace in workspaces {
                    ui.collapsing(workspace.name, |ui| {
                        if let Some(repo) = self.current_repo()
                            && ui.button("Add current repository").clicked()
                            && let Err(error) = self.catalog.add_to_workspace(workspace.id, &repo)
                        {
                            self.notice = error.to_string();
                        }
                        for path in workspace.repositories {
                            if ui.button(path.to_string_lossy()).clicked() {
                                self.open_repository(path);
                            }
                        }
                    });
                }
            }
            Err(error) => {
                ui.label(error.to_string());
            }
        }
    }

    fn advanced(&mut self, ui: &mut egui::Ui) {
        ui.heading("Advanced Git");
        let Some(repo) = self.current_repo() else {
            ui.label("Open a repository first.");
            return;
        };

        ui.collapsing("Conflicts", |ui| {
            for path in self.snapshot.conflicts.clone() {
                ui.horizontal(|ui| {
                    ui.monospace(&path);
                    if ui.button("Ours").clicked() {
                        let r = self
                            .git
                            .resolve_conflict(&repo, &path, ConflictChoice::Ours)
                            .map(|_| "Resolved with ours.".to_owned());
                        self.finish_git(r);
                    }
                    if ui.button("Theirs").clicked() {
                        let r = self
                            .git
                            .resolve_conflict(&repo, &path, ConflictChoice::Theirs)
                            .map(|_| "Resolved with theirs.".to_owned());
                        self.finish_git(r);
                    }
                    if ui.button("Mark resolved").clicked() {
                        let r = self
                            .git
                            .mark_conflict_resolved(&repo, &path)
                            .map(|_| "Marked resolved.".to_owned());
                        self.finish_git(r);
                    }
                });
            }
            if ui.button("Abort active operation").clicked() {
                let r = self.git.abort_current_operation(&repo);
                self.finish_git(r);
            }
        });

        ui.collapsing("Stash", |ui| {
            ui.add(TextEdit::singleline(&mut self.stash_message).hint_text("Stash message"));
            if ui.button("Stash tracked + untracked").clicked() {
                let m = self.stash_message.clone();
                let r = self.git.stash_push(&repo, &m, true);
                self.finish_git(r);
            }
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.stash_ref).desired_width(140.0));
                if ui.button("Apply").clicked() {
                    let s = self.stash_ref.clone();
                    let r = self.git.stash_apply(&repo, &s);
                    self.finish_git(r);
                }
            });
            for stash in self.snapshot.stashes.clone() {
                ui.horizontal(|ui| {
                    if ui.button(&stash.reference).clicked() {
                        self.stash_ref = stash.reference.clone();
                    }
                    ui.label(stash.subject);
                });
            }
            ui.label("Destructive stash actions require exact confirmation:");
            ui.horizontal(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.confirmation)
                        .hint_text("POP stash@{0} or DROP stash@{0}"),
                );
                let pop = format!("POP {}", self.stash_ref);
                let drop = format!("DROP {}", self.stash_ref);
                if ui
                    .add_enabled(self.confirmation == pop, egui::Button::new("Pop"))
                    .clicked()
                {
                    let s = self.stash_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.stash_pop(&repo, &s);
                    self.finish_git(r);
                }
                if ui
                    .add_enabled(self.confirmation == drop, egui::Button::new("Drop"))
                    .clicked()
                {
                    let s = self.stash_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.stash_drop(&repo, &s);
                    self.finish_git(r);
                }
            });
        });

        ui.collapsing("History-changing operations", |ui| {
            ui.add(TextEdit::singleline(&mut self.target_ref).hint_text("Target ref / SHA"));
            ui.add(TextEdit::singleline(&mut self.confirmation).hint_text("Typed confirmation"));
            let rebase = format!("REBASE {}", self.target_ref);
            let cherry = format!("CHERRY-PICK {}", self.target_ref);
            let soft = format!("RESET SOFT {}", self.target_ref);
            let mixed = format!("RESET MIXED {}", self.target_ref);
            let hard = format!("RESET HARD {}", self.target_ref);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(self.confirmation == rebase, egui::Button::new("Rebase"))
                    .clicked()
                {
                    let t = self.target_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.rebase(&repo, &t);
                    self.finish_git(r);
                }
                if ui
                    .add_enabled(
                        self.confirmation == cherry,
                        egui::Button::new("Cherry-pick"),
                    )
                    .clicked()
                {
                    let t = self.target_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.cherry_pick(&repo, &t);
                    self.finish_git(r);
                }
                if ui
                    .add_enabled(self.confirmation == soft, egui::Button::new("Reset soft"))
                    .clicked()
                {
                    let t = self.target_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.reset(&repo, &t, ResetMode::Soft);
                    self.finish_git(r);
                }
                if ui
                    .add_enabled(self.confirmation == mixed, egui::Button::new("Reset mixed"))
                    .clicked()
                {
                    let t = self.target_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.reset(&repo, &t, ResetMode::Mixed);
                    self.finish_git(r);
                }
                if ui
                    .add_enabled(self.confirmation == hard, egui::Button::new("Reset hard"))
                    .clicked()
                {
                    let t = self.target_ref.clone();
                    self.confirmation.clear();
                    let r = self.git.reset(&repo, &t, ResetMode::Hard);
                    self.finish_git(r);
                }
            });
            ui.weak("Hard reset is additionally blocked while the working tree is dirty.");
        });

        ui.collapsing("Worktrees", |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    TextEdit::singleline(&mut self.worktree_path).hint_text("New worktree path"),
                );
                ui.add(
                    TextEdit::singleline(&mut self.worktree_branch).hint_text("Existing branch"),
                );
                if ui.button("Create").clicked() {
                    let p = PathBuf::from(self.worktree_path.trim());
                    let b = self.worktree_branch.clone();
                    let r = self
                        .git
                        .add_worktree(&repo, &p, &b)
                        .map(|_| "Worktree created.".to_owned());
                    self.finish_git(r);
                }
            });
            for worktree in self.snapshot.worktrees.clone() {
                ui.horizontal(|ui| {
                    if ui.button(worktree.path.to_string_lossy()).clicked() {
                        self.open_repository(worktree.path.clone());
                    }
                    ui.weak(worktree.branch.unwrap_or_else(|| "detached".to_owned()));
                    if worktree.path != repo && ui.button("Remove safely").clicked() {
                        let r = self
                            .git
                            .remove_worktree(&repo, &worktree.path)
                            .map(|_| "Worktree removed.".to_owned());
                        self.finish_git(r);
                    }
                });
            }
        });

        ui.collapsing("Reflog recovery", |ui| {
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.branch_name).hint_text("recovery/name"));
                ui.add(TextEdit::singleline(&mut self.target_ref).hint_text("HEAD@{1}"));
                if ui.button("Create recovery branch").clicked() {
                    let n = self.branch_name.clone();
                    let t = self.target_ref.clone();
                    let r = self
                        .git
                        .create_recovery_branch(&repo, &n, &t)
                        .map(|_| "Recovery branch created.".to_owned());
                    self.finish_git(r);
                }
            });
            for item in self.snapshot.reflog.clone() {
                ui.horizontal_wrapped(|ui| {
                    if ui.button(&item.selector).clicked() {
                        self.target_ref = item.selector.clone();
                    }
                    ui.label(item.subject);
                    ui.weak(item.sha.chars().take(8).collect::<String>());
                });
            }
        });

        ui.collapsing("Operation journal", |ui| {
            match self.catalog.recent_journal(&repo, 50) {
                Ok(items) => {
                    for item in items {
                        ui.monospace(item);
                    }
                }
                Err(error) => {
                    ui.label(error.to_string());
                }
            }
        });
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("GitHub credentials are owned by GitHub CLI / the OS credential store. GitGat does not store GitHub tokens.");
        ui.horizontal(|ui| {
            if ui.button("Check GitHub").clicked() {
                self.refresh_forge();
            }
            if ui.button("Connect GitHub").clicked() {
                let r = self.forge.login();
                self.finish_forge(r);
            }
            if ui.button("Disconnect GitHub").clicked() {
                match self.forge.logout() {
                    Ok(text) => {
                        self.output_text = text;
                        self.forge_snapshot = ForgeSnapshot::default();
                        self.notice = "GitHub disconnected.".to_owned();
                    }
                    Err(error) => self.notice = error.to_string(),
                }
            }
        });
        ui.separator();
        ui.heading("Relocate moved repository");
        ui.weak("Use this when a repository in Recent repositories was moved or renamed outside GitGat.");
        ui.add(TextEdit::singleline(&mut self.relocate_from).hint_text("Old stored path"));
        ui.add(TextEdit::singleline(&mut self.relocate_to).hint_text("New repository path"));
        if ui.button("Relocate catalog entry").clicked() {
            let old = PathBuf::from(self.relocate_from.trim());
            let new_input = PathBuf::from(self.relocate_to.trim());
            match self.git.repository_root(&new_input) {
                Ok(new_root) => match self.catalog.relocate_repository(&old, &new_root) {
                    Ok(()) => {
                        if self.repository.as_ref() == Some(&old) {
                            self.repository = Some(new_root.clone());
                            self.repository_input = new_root.to_string_lossy().to_string();
                            self.refresh_git();
                        }
                        self.relocate_from.clear();
                        self.relocate_to.clear();
                        self.notice = "Repository catalog entry relocated.".to_owned();
                    }
                    Err(error) => self.notice = error.to_string(),
                },
                Err(error) => self.notice = format!("New path is not a Git repository: {error}"),
            }
        }

        ui.separator();
        ui.heading("Safety defaults");
        ui.label("• Pull uses --ff-only.");
        ui.label("• No force push action.");
        ui.label("• Branch deletion uses git branch -d.");
        ui.label("• Discard refuses untracked files.");
        ui.label("• Branch switch, rebase and cherry-pick require a clean worktree.");
        ui.label("• Hard reset requires a clean worktree plus typed confirmation.");
        ui.label("• Risky history operations are journaled.");
        ui.label("• Process output redacts GitHub tokens and URL credentials.");
    }
}

impl eframe::App for GitGatApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.set_visuals(egui::Visuals::dark());
        self.handle_shortcuts(&ctx);

        let dropped = ctx.input(|i| {
            i.raw
                .dropped_files
                .first()
                .map(|file| file.path().to_path_buf())
        });
        if let Some(path) = dropped {
            self.open_repository(path);
        }

        egui::Panel::top("toolbar").show(ui, |ui| self.toolbar(ui));
        egui::Panel::bottom("status").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(&self.notice).strong());
                if !self.output_text.trim().is_empty() {
                    ui.separator();
                    ui.weak(self.output_text.lines().next().unwrap_or_default());
                }
            });
        });
        egui::Panel::left("nav")
            .resizable(false)
            .default_size(170.0)
            .show(ui, |ui| self.sidebar(ui));
        egui::CentralPanel::default().show(ui, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match self.tab {
                    Tab::Overview => self.overview(ui),
                    Tab::Changes => self.changes(ui),
                    Tab::Branches => self.branches(ui),
                    Tab::History => self.history(ui),
                    Tab::PullRequests => self.pull_requests(ui),
                    Tab::Locks => self.locks(ui),
                    Tab::Actions => self.actions(ui),
                    Tab::MyWork => self.my_work(ui),
                    Tab::Workspaces => self.workspaces(ui),
                    Tab::Advanced => self.advanced(ui),
                    Tab::Settings => self.settings(ui),
                });
        });
    }
}
