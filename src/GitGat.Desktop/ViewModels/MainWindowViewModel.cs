using System.Collections.ObjectModel;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using GitGat.Application.Forge;
using GitGat.Application.Git;
using GitGat.Application.Operations;
using GitGat.Application.Repositories;
using GitGat.Application.Workspaces;
using GitGat.Domain.Repositories;
using GitGat.Domain.Workspaces;

namespace GitGat.Desktop.ViewModels;

public partial class MainWindowViewModel : ObservableObject
{
    private readonly IGitClient _git;
    private readonly IGitLfsClient _lfs;
    private readonly IForgeClient _forge;
    private readonly IRepositoryCatalog _repositories;
    private readonly IWorkspaceCatalog _workspaces;
    private readonly IOperationJournal _journal;

    [ObservableProperty] private string _gitVersion = "Checking Git…";
    [ObservableProperty] private string _gitHubStatus = "Checking GitHub…";
    [ObservableProperty] private string _operationStatus = "Ready";
    [ObservableProperty] private bool _isBusy;
    [ObservableProperty] private string _repositorySearch = string.Empty;
    [ObservableProperty] private RepositoryIdentity? _activeRepository;
    [ObservableProperty] private RepositoryStatus? _currentRepositoryStatus;
    [ObservableProperty] private GitChange? _selectedChange;
    [ObservableProperty] private GitBranch? _selectedBranch;
    [ObservableProperty] private GitCommit? _selectedCommit;
    [ObservableProperty] private GitLfsLock? _selectedLock;
    [ObservableProperty] private PullRequestSummary? _selectedPullRequest;
    [ObservableProperty] private WorkflowRunSummary? _selectedWorkflowRun;
    [ObservableProperty] private GitStash? _selectedStash;
    [ObservableProperty] private GitWorktree? _selectedWorktree;
    [ObservableProperty] private GitReflogEntry? _selectedReflogEntry;
    [ObservableProperty] private string? _selectedConflict;
    [ObservableProperty] private WorkspaceIdentity? _selectedWorkspace;
    [ObservableProperty] private string _diffText = "Select a changed file to inspect its diff.";
    [ObservableProperty] private string _commitDetailText = string.Empty;
    [ObservableProperty] private string _pullRequestDetailText = string.Empty;
    [ObservableProperty] private string _workflowLogText = string.Empty;
    [ObservableProperty] private string _commitMessage = string.Empty;
    [ObservableProperty] private string _newBranchName = string.Empty;
    [ObservableProperty] private string _cloneUrl = string.Empty;
    [ObservableProperty] private string _cloneDestination = string.Empty;
    [ObservableProperty] private string _lfsPattern = "*.uasset";
    [ObservableProperty] private string _pullRequestComment = string.Empty;
    [ObservableProperty] private string _pullRequestTitle = string.Empty;
    [ObservableProperty] private string _pullRequestBody = string.Empty;
    [ObservableProperty] private string _pullRequestHead = string.Empty;
    [ObservableProperty] private string _pullRequestBase = "main";
    [ObservableProperty] private string _workspaceName = string.Empty;
    [ObservableProperty] private string _discardConfirmation = string.Empty;
    [ObservableProperty] private string _advancedConfirmation = string.Empty;
    [ObservableProperty] private string _worktreePath = string.Empty;
    [ObservableProperty] private string _worktreeBranch = string.Empty;
    [ObservableProperty] private string _advancedTarget = string.Empty;
    [ObservableProperty] private string _recoveryBranchName = string.Empty;
    [ObservableProperty] private string _statusSummary = "No repository selected.";

    public MainWindowViewModel(
        IGitClient git,
        IGitLfsClient lfs,
        IForgeClient forge,
        IRepositoryCatalog repositories,
        IWorkspaceCatalog workspaces,
        IOperationJournal journal)
    {
        _git = git;
        _lfs = lfs;
        _forge = forge;
        _repositories = repositories;
        _workspaces = workspaces;
        _journal = journal;
    }

    public ObservableCollection<RepositoryIdentity> Repositories { get; } = [];
    public ObservableCollection<GitChange> Changes { get; } = [];
    public ObservableCollection<GitBranch> Branches { get; } = [];
    public ObservableCollection<GitCommit> History { get; } = [];
    public ObservableCollection<GitLfsLock> Locks { get; } = [];
    public ObservableCollection<PullRequestSummary> PullRequests { get; } = [];
    public ObservableCollection<string> ChangedPullRequestFiles { get; } = [];
    public ObservableCollection<WorkflowRunSummary> WorkflowRuns { get; } = [];
    public ObservableCollection<ForgeNotification> Notifications { get; } = [];
    public ObservableCollection<TeamActivityItem> Activity { get; } = [];
    public ObservableCollection<string> MyWorkItems { get; } = [];
    public ObservableCollection<string> Conflicts { get; } = [];
    public ObservableCollection<GitStash> Stashes { get; } = [];
    public ObservableCollection<GitWorktree> Worktrees { get; } = [];
    public ObservableCollection<GitReflogEntry> Reflog { get; } = [];
    public ObservableCollection<OperationRecord> OperationJournal { get; } = [];
    public ObservableCollection<WorkspaceIdentity> Workspaces { get; } = [];

    public async Task InitializeAsync()
    {
        await RunAsync("Initializing GitGat…", async () =>
        {
            GitVersion = await _git.GetVersionAsync();
            await LoadRepositoriesAsync();
            await LoadWorkspacesAsync();
            await RefreshGitHubAsync();
            await LoadJournalAsync();
        });
    }

    public Task OpenRepositoryAsync(string path) =>
        RunAsync("Opening repository…", async () =>
        {
            var fullPath = Path.GetFullPath(path);
            if (!await _git.IsRepositoryAsync(fullPath))
            {
                throw new InvalidOperationException("The selected folder is not a Git working repository.");
            }

            var remote = await _git.GetRemoteUrlAsync(fullPath);
            var existing = Repositories.FirstOrDefault(item =>
                string.Equals(item.LocalPath, fullPath, StringComparison.OrdinalIgnoreCase));

            var repository = existing is null
                ? RepositoryIdentity.Create(new DirectoryInfo(fullPath).Name, fullPath, remote)
                : existing with
                {
                    RemoteUrl = remote,
                    LastOpenedUtc = DateTimeOffset.UtcNow,
                    Exists = true
                };

            await _repositories.AddOrUpdateAsync(repository);
            await LoadRepositoriesAsync();
            ActiveRepository = Repositories.FirstOrDefault(item => item.Id == repository.Id)
                ?? Repositories.First(item => string.Equals(item.LocalPath, fullPath, StringComparison.OrdinalIgnoreCase));
            await LoadActiveRepositoryAsync();
        });

    public void SetCloneDestination(string path)
    {
        CloneDestination = path;
    }

    public void SetWorktreePath(string path)
    {
        WorktreePath = path;
    }

    public Task UpdateActiveRepositoryPathAsync(string path) =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Updating repository location…", async () =>
            {
                var fullPath = Path.GetFullPath(path);
                if (!await _git.IsRepositoryAsync(fullPath))
                {
                    throw new InvalidOperationException("The selected folder is not a Git repository.");
                }

                var repositoryId = ActiveRepository.Id;
                await _repositories.UpdatePathAsync(repositoryId, fullPath);
                await LoadRepositoriesAsync();
                ActiveRepository = Repositories.FirstOrDefault(item => item.Id == repositoryId);
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task SearchRepositoriesAsync() =>
        RunAsync("Searching repositories…", () => LoadRepositoriesAsync(RepositorySearch));

    [RelayCommand]
    private Task OpenRecentAsync(RepositoryIdentity? repository) =>
        repository is null
            ? Task.CompletedTask
            : OpenRepositoryAsync(repository.LocalPath);

    [RelayCommand]
    private Task RemoveRepositoryAsync(RepositoryIdentity? repository) =>
        repository is null
            ? Task.CompletedTask
            : RunAsync("Removing repository from catalog…", async () =>
            {
                await _repositories.RemoveAsync(repository.Id);
                if (ActiveRepository?.Id == repository.Id)
                {
                    ActiveRepository = null;
                    ClearRepositoryState();
                }

                await LoadRepositoriesAsync();
            });

    [RelayCommand]
    private Task ToggleFavoriteAsync(RepositoryIdentity? repository) =>
        repository is null
            ? Task.CompletedTask
            : RunAsync("Updating favorite…", async () =>
            {
                await _repositories.SetFavoriteAsync(repository.Id, !repository.IsFavorite);
                await LoadRepositoriesAsync();
            });

    [RelayCommand]
    private Task CloneRepositoryAsync() =>
        RunAsync("Cloning repository…", async () =>
        {
            if (string.IsNullOrWhiteSpace(CloneUrl) || string.IsNullOrWhiteSpace(CloneDestination))
            {
                throw new InvalidOperationException("Clone URL and destination are required.");
            }

            await _git.CloneAsync(CloneUrl.Trim(), CloneDestination.Trim());
            await OpenRepositoryCoreAsync(CloneDestination.Trim());
        });

    [RelayCommand]
    private Task RefreshRepositoryAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Refreshing repository…", LoadActiveRepositoryAsync);

    [RelayCommand]
    private Task ShowDiffAsync() =>
        ActiveRepository is null || SelectedChange is null
            ? Task.CompletedTask
            : RunAsync("Loading diff…", async () =>
            {
                DiffText = await _git.GetDiffAsync(
                    ActiveRepository.LocalPath,
                    SelectedChange.Path,
                    SelectedChange.IsStaged && !SelectedChange.IsUnstaged);
            });

    [RelayCommand]
    private Task StageSelectedAsync() =>
        MutateSelectedChangeAsync(
            "Staging change…",
            (repository, path) => _git.StageAsync(repository, [path]));

    [RelayCommand]
    private Task UnstageSelectedAsync() =>
        MutateSelectedChangeAsync(
            "Unstaging change…",
            (repository, path) => _git.UnstageAsync(repository, [path]));

    [RelayCommand]
    private Task DiscardSelectedAsync() =>
        ActiveRepository is null || SelectedChange is null
            ? Task.CompletedTask
            : RunAsync("Discarding tracked change…", async () =>
            {
                if (!string.Equals(DiscardConfirmation.Trim(), SelectedChange.Path, StringComparison.Ordinal))
                {
                    throw new InvalidOperationException("Type the exact file path in the confirmation box before discarding.");
                }

                await _git.DiscardTrackedAsync(ActiveRepository.LocalPath, SelectedChange.Path);
                DiscardConfirmation = string.Empty;
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task CommitAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Creating commit…", async () =>
            {
                await _git.CommitAsync(ActiveRepository.LocalPath, CommitMessage);
                CommitMessage = string.Empty;
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task FetchAsync() => RunGitMutationAsync("Fetching…", path => _git.FetchAsync(path));

    [RelayCommand]
    private Task PullAsync() => RunGitMutationAsync("Pulling (fast-forward only)…", path => _git.PullAsync(path));

    [RelayCommand]
    private Task PushAsync() => RunGitMutationAsync("Pushing…", path => _git.PushAsync(path));

    [RelayCommand]
    private Task CheckoutBranchAsync() =>
        ActiveRepository is null || SelectedBranch is null
            ? Task.CompletedTask
            : RunAsync("Switching branch…", async () =>
            {
                if (SelectedBranch.IsRemote)
                {
                    throw new InvalidOperationException("Create a local tracking branch before switching to a remote branch.");
                }

                await _git.CheckoutBranchAsync(ActiveRepository.LocalPath, SelectedBranch.Name);
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task CreateBranchAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Creating branch…", async () =>
            {
                await _git.CreateBranchAsync(ActiveRepository.LocalPath, NewBranchName.Trim(), true);
                NewBranchName = string.Empty;
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task DeleteBranchAsync() =>
        ActiveRepository is null || SelectedBranch is null
            ? Task.CompletedTask
            : RunAsync("Deleting merged branch…", async () =>
            {
                await _git.DeleteBranchAsync(ActiveRepository.LocalPath, SelectedBranch.Name);
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task LoadMoreHistoryAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Loading more history…", async () =>
            {
                var next = await _git.GetHistoryAsync(ActiveRepository.LocalPath, History.Count, 100);
                foreach (var commit in next)
                {
                    History.Add(commit);
                }
            });

    [RelayCommand]
    private Task ShowCommitAsync() =>
        ActiveRepository is null || SelectedCommit is null
            ? Task.CompletedTask
            : RunAsync("Loading commit…", async () =>
            {
                var detail = await _git.GetCommitDetailAsync(ActiveRepository.LocalPath, SelectedCommit.Sha);
                CommitDetailText = string.Join(
                    Environment.NewLine,
                    $"{detail.Commit.ShortSha} — {detail.Commit.Subject}",
                    $"{detail.Commit.Author} · {detail.Commit.AuthoredAt:g}",
                    string.Empty,
                    detail.Body,
                    string.Empty,
                    detail.Statistics);
            });

    [RelayCommand]
    private Task LockSelectedAsync() =>
        ActiveRepository is null || SelectedChange is null
            ? Task.CompletedTask
            : RunAsync("Locking file…", async () =>
            {
                await _lfs.LockAsync(ActiveRepository.LocalPath, SelectedChange.Path);
                await LoadLocksAsync();
            });

    [RelayCommand]
    private Task UnlockSelectedAsync() =>
        ActiveRepository is null || SelectedLock is null
            ? Task.CompletedTask
            : RunAsync("Unlocking file…", async () =>
            {
                await _lfs.UnlockAsync(ActiveRepository.LocalPath, SelectedLock.Path, false);
                await LoadLocksAsync();
            });

    [RelayCommand]
    private Task ConfigureLockablePatternAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Configuring lockable pattern…", async () =>
            {
                await _lfs.ConfigureLockablePatternAsync(ActiveRepository.LocalPath, LfsPattern);
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task RefreshGitHubAsync() =>
        RunAsync("Refreshing GitHub…", RefreshGitHubCoreAsync);

    [RelayCommand]
    private Task ConnectGitHubAsync() =>
        RunAsync("Opening GitHub login…", async () =>
        {
            await _forge.StartInteractiveLoginAsync();
            GitHubStatus = "GitHub login opened in a terminal. Complete it, then press Refresh.";
        });

    [RelayCommand]
    private Task DisconnectGitHubAsync() =>
        RunAsync("Opening GitHub logout…", async () =>
        {
            await _forge.StartInteractiveLogoutAsync();
            GitHubStatus = "GitHub logout opened in a terminal. Complete it, then press Refresh.";
        });

    [RelayCommand]
    private Task OpenPullRequestAsync(PullRequestSummary? pullRequest) =>
        ActiveRepository is null || pullRequest is null
            ? Task.CompletedTask
            : RunAsync("Loading pull request…", async () =>
            {
                var forgeRepository = RequireForgeRepository();
                var detail = await _forge.GetPullRequestAsync(forgeRepository, pullRequest.Number);
                SelectedPullRequest = pullRequest;
                Replace(ChangedPullRequestFiles, detail.ChangedFiles);
                PullRequestTitle = detail.Summary.Title;
                PullRequestBody = detail.Body;
                PullRequestHead = detail.Summary.HeadBranch;
                PullRequestBase = detail.Summary.BaseBranch;
                PullRequestDetailText =
                    $"#{detail.Summary.Number} {detail.Summary.Title}{Environment.NewLine}" +
                    $"{detail.Summary.Author} · {detail.Summary.HeadBranch} → {detail.Summary.BaseBranch}{Environment.NewLine}" +
                    $"Mergeable: {detail.Mergeable} / {detail.MergeStateStatus}{Environment.NewLine}{Environment.NewLine}" +
                    detail.Body;
            });

    [RelayCommand]
    private Task CommentPullRequestAsync() =>
        SelectedPullRequest is null
            ? Task.CompletedTask
            : RunAsync("Posting comment…", async () =>
            {
                await _forge.CommentOnPullRequestAsync(
                    RequireForgeRepository(),
                    SelectedPullRequest.Number,
                    PullRequestComment);
                PullRequestComment = string.Empty;
                await LoadForgeForActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task ApprovePullRequestAsync() =>
        ReviewPullRequestAsync(PullRequestReviewAction.Approve);

    [RelayCommand]
    private Task RequestChangesAsync() =>
        ReviewPullRequestAsync(PullRequestReviewAction.RequestChanges);

    [RelayCommand]
    private Task EditPullRequestAsync() =>
        SelectedPullRequest is null
            ? Task.CompletedTask
            : RunAsync("Updating pull request…", async () =>
            {
                await _forge.EditPullRequestAsync(
                    RequireForgeRepository(),
                    SelectedPullRequest.Number,
                    PullRequestTitle,
                    PullRequestBody);
                await LoadForgeForActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task MergePullRequestAsync() =>
        SelectedPullRequest is null
            ? Task.CompletedTask
            : RunAsync("Merging pull request…", async () =>
            {
                await _forge.MergePullRequestAsync(RequireForgeRepository(), SelectedPullRequest.Number);
                await LoadForgeForActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task CreatePullRequestAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Creating pull request…", async () =>
            {
                var url = await _forge.CreatePullRequestAsync(
                    RequireForgeRepository(),
                    PullRequestHead,
                    PullRequestBase,
                    PullRequestTitle,
                    PullRequestBody);
                OperationStatus = $"Pull request created: {url}";
                PullRequestTitle = string.Empty;
                PullRequestBody = string.Empty;
                await LoadForgeForActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task ShowFailedWorkflowLogAsync() =>
        SelectedWorkflowRun is null
            ? Task.CompletedTask
            : RunAsync("Loading failed workflow log…", async () =>
            {
                WorkflowLogText = await _forge.GetFailedWorkflowLogAsync(
                    RequireForgeRepository(),
                    SelectedWorkflowRun.Id);
            });

    [RelayCommand]
    private Task RerunFailedWorkflowAsync() =>
        SelectedWorkflowRun is null
            ? Task.CompletedTask
            : RunAsync("Re-running failed workflow jobs…", async () =>
            {
                await _forge.RerunWorkflowAsync(RequireForgeRepository(), SelectedWorkflowRun.Id);
                await LoadForgeForActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task ResolveConflictOursAsync() =>
        ResolveConflictAsync(GitConflictResolution.KeepOurs);

    [RelayCommand]
    private Task ResolveConflictTheirsAsync() =>
        ResolveConflictAsync(GitConflictResolution.KeepTheirs);

    [RelayCommand]
    private Task MarkConflictResolvedAsync() =>
        ActiveRepository is null || string.IsNullOrWhiteSpace(SelectedConflict)
            ? Task.CompletedTask
            : RunAsync("Marking conflict resolved…", async () =>
            {
                await _git.MarkConflictResolvedAsync(
                    ActiveRepository.LocalPath,
                    SelectedConflict);
                await LoadActiveRepositoryAsync();
            });

    [RelayCommand]
    private Task RebaseAsync() =>
        RunAdvancedHistoryChangeAsync(
            "Rebasing…",
            $"REBASE {AdvancedTarget.Trim()}",
            path => _git.RebaseAsync(path, AdvancedTarget.Trim()));

    [RelayCommand]
    private Task CherryPickAsync() =>
        RunAdvancedHistoryChangeAsync(
            "Cherry-picking…",
            $"CHERRY-PICK {AdvancedTarget.Trim()}",
            path => _git.CherryPickAsync(path, AdvancedTarget.Trim()));

    [RelayCommand]
    private Task ResetSoftAsync() =>
        RunResetAsync(GitResetMode.Soft);

    [RelayCommand]
    private Task ResetMixedAsync() =>
        RunResetAsync(GitResetMode.Mixed);

    [RelayCommand]
    private Task ResetHardAsync() =>
        RunResetAsync(GitResetMode.Hard);

    [RelayCommand]
    private Task StashAllAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Stashing work…", async () =>
            {
                await _git.StashPushAsync(ActiveRepository.LocalPath, "GitGat stash", true);
                await LoadActiveRepositoryAsync();
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task ApplyStashAsync() =>
        ActiveRepository is null || SelectedStash is null
            ? Task.CompletedTask
            : RunAsync("Applying stash…", async () =>
            {
                await _git.StashApplyAsync(ActiveRepository.LocalPath, SelectedStash.Reference, false);
                await LoadActiveRepositoryAsync();
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task PopStashAsync() =>
        ActiveRepository is null || SelectedStash is null
            ? Task.CompletedTask
            : RunAsync("Popping stash…", async () =>
            {
                if (!string.Equals(AdvancedConfirmation.Trim(), SelectedStash.Reference, StringComparison.Ordinal))
                {
                    throw new InvalidOperationException(
                        "Type the exact stash reference to confirm pop.");
                }

                await _git.StashApplyAsync(
                    ActiveRepository.LocalPath,
                    SelectedStash.Reference,
                    true);
                AdvancedConfirmation = string.Empty;
                await LoadActiveRepositoryAsync();
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task DropStashAsync() =>
        ActiveRepository is null || SelectedStash is null
            ? Task.CompletedTask
            : RunAsync("Dropping stash…", async () =>
            {
                if (!string.Equals(AdvancedConfirmation.Trim(), SelectedStash.Reference, StringComparison.Ordinal))
                {
                    throw new InvalidOperationException(
                        "Type the exact stash reference to confirm drop.");
                }

                await _git.StashDropAsync(
                    ActiveRepository.LocalPath,
                    SelectedStash.Reference);
                AdvancedConfirmation = string.Empty;
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task AbortOperationAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Aborting Git operation…", async () =>
            {
                if (!string.Equals(AdvancedConfirmation.Trim(), "ABORT", StringComparison.Ordinal))
                {
                    throw new InvalidOperationException("Type ABORT to confirm aborting the current Git operation.");
                }

                await _git.AbortCurrentOperationAsync(ActiveRepository.LocalPath);
                AdvancedConfirmation = string.Empty;
                await LoadActiveRepositoryAsync();
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task CreateWorktreeAsync() =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Creating worktree…", async () =>
            {
                await _git.CreateWorktreeAsync(
                    ActiveRepository.LocalPath,
                    WorktreePath.Trim(),
                    WorktreeBranch.Trim());
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task OpenWorktreeAsync() =>
        SelectedWorktree is null
            ? Task.CompletedTask
            : OpenRepositoryAsync(SelectedWorktree.Path);

    [RelayCommand]
    private Task RemoveWorktreeAsync() =>
        ActiveRepository is null || SelectedWorktree is null
            ? Task.CompletedTask
            : RunAsync("Removing worktree…", async () =>
            {
                if (!string.Equals(AdvancedConfirmation.Trim(), SelectedWorktree.Path, StringComparison.Ordinal))
                {
                    throw new InvalidOperationException("Type the exact worktree path to confirm removal.");
                }

                await _git.RemoveWorktreeAsync(ActiveRepository.LocalPath, SelectedWorktree.Path);
                AdvancedConfirmation = string.Empty;
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task CreateRecoveryBranchAsync() =>
        ActiveRepository is null || SelectedReflogEntry is null
            ? Task.CompletedTask
            : RunAsync("Creating recovery branch…", async () =>
            {
                if (string.IsNullOrWhiteSpace(RecoveryBranchName))
                {
                    throw new InvalidOperationException("Recovery branch name is required.");
                }

                await _git.CreateRecoveryBranchAsync(
                    ActiveRepository.LocalPath,
                    SelectedReflogEntry.Sha,
                    RecoveryBranchName.Trim());

                RecoveryBranchName = string.Empty;
                await LoadActiveRepositoryAsync();
                await LoadAdvancedAsync();
            });

    [RelayCommand]
    private Task CreateWorkspaceAsync() =>
        RunAsync("Creating workspace…", async () =>
        {
            if (string.IsNullOrWhiteSpace(WorkspaceName))
            {
                throw new InvalidOperationException("Workspace name is required.");
            }

            var members = ActiveRepository is null ? Array.Empty<Guid>() : [ActiveRepository.Id];
            await _workspaces.SaveAsync(
                new WorkspaceIdentity(Guid.NewGuid(), WorkspaceName.Trim(), members, DateTimeOffset.UtcNow));
            WorkspaceName = string.Empty;
            await LoadWorkspacesAsync();
        });

    [RelayCommand]
    private Task AddRepositoryToWorkspaceAsync() =>
        SelectedWorkspace is null || ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Adding repository to workspace…", async () =>
            {
                var ids = SelectedWorkspace.RepositoryIds
                    .Append(ActiveRepository.Id)
                    .Distinct()
                    .ToArray();
                SelectedWorkspace = await _workspaces.SaveAsync(
                    SelectedWorkspace with { RepositoryIds = ids });
                await LoadWorkspacesAsync();
            });

    [RelayCommand]
    private Task RemoveRepositoryFromWorkspaceAsync() =>
        SelectedWorkspace is null || ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync("Removing repository from workspace…", async () =>
            {
                var ids = SelectedWorkspace.RepositoryIds
                    .Where(id => id != ActiveRepository.Id)
                    .ToArray();
                SelectedWorkspace = await _workspaces.SaveAsync(
                    SelectedWorkspace with { RepositoryIds = ids });
                await LoadWorkspacesAsync();
            });

    [RelayCommand]
    private Task DeleteWorkspaceAsync() =>
        SelectedWorkspace is null
            ? Task.CompletedTask
            : RunAsync("Deleting workspace…", async () =>
            {
                await _workspaces.DeleteAsync(SelectedWorkspace.Id);
                SelectedWorkspace = null;
                await LoadWorkspacesAsync();
            });

    [RelayCommand]
    private Task RefreshMyWorkAsync() =>
        RunAsync("Refreshing My Work…", LoadMyWorkAsync);

    private async Task OpenRepositoryCoreAsync(string path)
    {
        var fullPath = Path.GetFullPath(path);
        var remote = await _git.GetRemoteUrlAsync(fullPath);
        var repository = RepositoryIdentity.Create(new DirectoryInfo(fullPath).Name, fullPath, remote);
        await _repositories.AddOrUpdateAsync(repository);
        await LoadRepositoriesAsync();
        ActiveRepository = Repositories.First(item =>
            string.Equals(item.LocalPath, fullPath, StringComparison.OrdinalIgnoreCase));
        await LoadActiveRepositoryAsync();
    }

    private async Task LoadRepositoriesAsync(string? search = null)
    {
        Replace(Repositories, await _repositories.GetRecentAsync(search));
    }

    private async Task LoadWorkspacesAsync()
    {
        var selectedId = SelectedWorkspace?.Id;
        Replace(Workspaces, await _workspaces.GetAllAsync());
        SelectedWorkspace = selectedId is null
            ? SelectedWorkspace
            : Workspaces.FirstOrDefault(item => item.Id == selectedId);
    }

    private async Task LoadActiveRepositoryAsync()
    {
        if (ActiveRepository is null)
        {
            ClearRepositoryState();
            return;
        }

        if (!Directory.Exists(ActiveRepository.LocalPath))
        {
            StatusSummary = "Repository path is missing. Remove it from the catalog or open its new location.";
            ClearRepositoryCollections();
            return;
        }

        CurrentRepositoryStatus = await _git.GetStatusAsync(ActiveRepository.LocalPath);
        StatusSummary =
            $"{CurrentRepositoryStatus.Branch} · {CurrentRepositoryStatus.ChangedFiles} changed · " +
            $"{CurrentRepositoryStatus.UntrackedFiles} untracked · ↑{CurrentRepositoryStatus.Ahead} ↓{CurrentRepositoryStatus.Behind}";

        Replace(Changes, await _git.GetChangesAsync(ActiveRepository.LocalPath));
        Replace(Branches, await _git.GetBranchesAsync(ActiveRepository.LocalPath));
        Replace(History, await _git.GetHistoryAsync(ActiveRepository.LocalPath));
        Replace(Conflicts, await _git.GetConflictsAsync(ActiveRepository.LocalPath));
        await LoadLocksAsync();
        await LoadAdvancedAsync();
        await LoadForgeForActiveRepositoryAsync();
    }

    private async Task LoadLocksAsync()
    {
        if (ActiveRepository is null)
        {
            Locks.Clear();
            return;
        }

        Replace(Locks, await _lfs.GetLocksAsync(ActiveRepository.LocalPath));
    }

    private async Task LoadAdvancedAsync()
    {
        if (ActiveRepository is null)
        {
            return;
        }

        Replace(Stashes, await _git.GetStashesAsync(ActiveRepository.LocalPath));
        Replace(Worktrees, await _git.GetWorktreesAsync(ActiveRepository.LocalPath));
        Replace(Reflog, await _git.GetReflogAsync(ActiveRepository.LocalPath));
        await LoadJournalAsync();
    }

    private async Task LoadJournalAsync()
    {
        Replace(OperationJournal, await _journal.ReadRecentAsync(100));
    }

    private async Task RefreshGitHubCoreAsync()
    {
        var connection = await _forge.GetConnectionAsync();
        GitHubStatus = connection.IsConnected
            ? $"Connected as {connection.Login ?? "GitHub user"}"
            : connection.Message;

        Notifications.Clear();
        if (connection.IsConnected)
        {
            Replace(Notifications, await _forge.GetNotificationsAsync());
        }

        await LoadForgeForActiveRepositoryAsync();
    }

    private async Task LoadForgeForActiveRepositoryAsync()
    {
        PullRequests.Clear();
        WorkflowRuns.Clear();
        Activity.Clear();

        if (ActiveRepository is null)
        {
            return;
        }

        var connection = await _forge.GetConnectionAsync();
        if (!connection.IsConnected)
        {
            GitHubStatus = connection.Message;
            return;
        }

        var repository = _forge.ResolveRepository(ActiveRepository.RemoteUrl);
        if (repository is null)
        {
            return;
        }

        Replace(PullRequests, await _forge.GetPullRequestsAsync(repository));
        Replace(WorkflowRuns, await _forge.GetWorkflowRunsAsync(repository));
        Replace(Activity, await _forge.GetActivityAsync(repository));
    }

    private async Task LoadMyWorkAsync()
    {
        MyWorkItems.Clear();
        var connection = await _forge.GetConnectionAsync();
        if (!connection.IsConnected)
        {
            MyWorkItems.Add("Connect GitHub to build My Work.");
            return;
        }

        var notifications = await _forge.GetNotificationsAsync();
        foreach (var notification in notifications.Where(item => item.Unread).Take(100))
        {
            MyWorkItems.Add($"Notification · {notification.Repository} · {notification.Subject}");
        }

        foreach (var repository in Repositories.Where(item => item.Exists && item.RemoteUrl is not null).Take(25))
        {
            var forgeRepository = _forge.ResolveRepository(repository.RemoteUrl);
            if (forgeRepository is null)
            {
                continue;
            }

            try
            {
                var pullRequests = await _forge.GetPullRequestsAsync(forgeRepository);
                foreach (var pullRequest in pullRequests.Where(item =>
                             string.Equals(item.Author, connection.Login, StringComparison.OrdinalIgnoreCase) ||
                             !string.IsNullOrWhiteSpace(item.ReviewDecision)))
                {
                    MyWorkItems.Add(
                        $"PR · {forgeRepository.FullName} · #{pullRequest.Number} {pullRequest.Title} · {pullRequest.ReviewDecision ?? "open"}");
                }
            }
            catch (InvalidOperationException exception)
            {
                MyWorkItems.Add($"{forgeRepository.FullName} · refresh failed · {exception.Message}");
            }
        }

        if (MyWorkItems.Count == 0)
        {
            MyWorkItems.Add("Nothing currently needs your attention.");
        }
    }

    private Task ResolveConflictAsync(GitConflictResolution resolution) =>
        ActiveRepository is null || string.IsNullOrWhiteSpace(SelectedConflict)
            ? Task.CompletedTask
            : RunAsync("Resolving conflict…", async () =>
            {
                await _git.ResolveConflictAsync(
                    ActiveRepository.LocalPath,
                    SelectedConflict,
                    resolution);
                await LoadActiveRepositoryAsync();
            });

    private Task RunResetAsync(GitResetMode mode) =>
        RunAdvancedHistoryChangeAsync(
            $"Resetting ({mode})…",
            $"RESET {mode.ToString().ToUpperInvariant()} {AdvancedTarget.Trim()}",
            path => _git.ResetAsync(path, AdvancedTarget.Trim(), mode));

    private Task RunAdvancedHistoryChangeAsync(
        string activity,
        string confirmation,
        Func<string, Task> action) =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync(activity, async () =>
            {
                if (string.IsNullOrWhiteSpace(AdvancedTarget))
                {
                    throw new InvalidOperationException("A target commit or reference is required.");
                }

                if (!string.Equals(
                        AdvancedConfirmation.Trim(),
                        confirmation,
                        StringComparison.Ordinal))
                {
                    throw new InvalidOperationException(
                        $"Type '{confirmation}' exactly to confirm this history-changing operation.");
                }

                await action(ActiveRepository.LocalPath);
                AdvancedConfirmation = string.Empty;
                await LoadActiveRepositoryAsync();
                await LoadAdvancedAsync();
            });

    private Task ReviewPullRequestAsync(PullRequestReviewAction action) =>
        SelectedPullRequest is null
            ? Task.CompletedTask
            : RunAsync("Submitting review…", async () =>
            {
                await _forge.ReviewPullRequestAsync(
                    RequireForgeRepository(),
                    SelectedPullRequest.Number,
                    action,
                    PullRequestComment);
                PullRequestComment = string.Empty;
                await LoadForgeForActiveRepositoryAsync();
            });

    private ForgeRepository RequireForgeRepository()
    {
        if (ActiveRepository is null)
        {
            throw new InvalidOperationException("Open a repository first.");
        }

        return _forge.ResolveRepository(ActiveRepository.RemoteUrl)
            ?? throw new InvalidOperationException("The repository origin is not a recognized GitHub remote.");
    }

    private Task RunGitMutationAsync(string activity, Func<string, Task> action) =>
        ActiveRepository is null
            ? Task.CompletedTask
            : RunAsync(activity, async () =>
            {
                await action(ActiveRepository.LocalPath);
                await LoadActiveRepositoryAsync();
            });

    private Task MutateSelectedChangeAsync(
        string activity,
        Func<string, string, Task> action) =>
        ActiveRepository is null || SelectedChange is null
            ? Task.CompletedTask
            : RunAsync(activity, async () =>
            {
                await action(ActiveRepository.LocalPath, SelectedChange.Path);
                await LoadActiveRepositoryAsync();
            });

    private async Task RunAsync(string activity, Func<Task> action)
    {
        if (IsBusy)
        {
            return;
        }

        IsBusy = true;
        OperationStatus = activity;

        try
        {
            await action();
            if (OperationStatus == activity)
            {
                OperationStatus = "Ready";
            }
        }
        catch (OperationCanceledException)
        {
            OperationStatus = "Operation cancelled.";
        }
        catch (Exception exception)
        {
            OperationStatus = exception.Message;
        }
        finally
        {
            IsBusy = false;
        }
    }

    private void ClearRepositoryState()
    {
        CurrentRepositoryStatus = null;
        StatusSummary = "No repository selected.";
        ClearRepositoryCollections();
    }

    private void ClearRepositoryCollections()
    {
        Changes.Clear();
        Branches.Clear();
        History.Clear();
        Locks.Clear();
        PullRequests.Clear();
        WorkflowRuns.Clear();
        Activity.Clear();
        Conflicts.Clear();
        Stashes.Clear();
        Worktrees.Clear();
        Reflog.Clear();
        DiffText = "Select a changed file to inspect its diff.";
        CommitDetailText = string.Empty;
        PullRequestDetailText = string.Empty;
        WorkflowLogText = string.Empty;
    }

    private static void Replace<T>(ObservableCollection<T> target, IEnumerable<T> values)
    {
        target.Clear();
        foreach (var value in values)
        {
            target.Add(value);
        }
    }
}
