namespace GitGat.Application.Git;

public interface IGitClient
{
    Task<string> GetVersionAsync(CancellationToken cancellationToken = default);
    Task<bool> IsRepositoryAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task<string?> GetRemoteUrlAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task CloneAsync(string remoteUrl, string destinationPath, CancellationToken cancellationToken = default);
    Task<RepositoryStatus> GetStatusAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitChange>> GetChangesAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task<string> GetDiffAsync(string repositoryPath, string path, bool staged, CancellationToken cancellationToken = default);
    Task StageAsync(string repositoryPath, IEnumerable<string> paths, CancellationToken cancellationToken = default);
    Task UnstageAsync(string repositoryPath, IEnumerable<string> paths, CancellationToken cancellationToken = default);
    Task CommitAsync(string repositoryPath, string message, CancellationToken cancellationToken = default);
    Task DiscardTrackedAsync(string repositoryPath, string path, CancellationToken cancellationToken = default);
    Task FetchAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task PullAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task PushAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitBranch>> GetBranchesAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task CheckoutBranchAsync(string repositoryPath, string branch, CancellationToken cancellationToken = default);
    Task CreateBranchAsync(string repositoryPath, string branch, bool checkout, CancellationToken cancellationToken = default);
    Task DeleteBranchAsync(string repositoryPath, string branch, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitCommit>> GetHistoryAsync(string repositoryPath, int skip = 0, int take = 100, CancellationToken cancellationToken = default);
    Task<GitCommitDetail> GetCommitDetailAsync(string repositoryPath, string sha, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<string>> GetConflictsAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task ResolveConflictAsync(string repositoryPath, string path, GitConflictResolution resolution, CancellationToken cancellationToken = default);
    Task MarkConflictResolvedAsync(string repositoryPath, string path, CancellationToken cancellationToken = default);
    Task RebaseAsync(string repositoryPath, string target, CancellationToken cancellationToken = default);
    Task CherryPickAsync(string repositoryPath, string commit, CancellationToken cancellationToken = default);
    Task ResetAsync(string repositoryPath, string target, GitResetMode mode, CancellationToken cancellationToken = default);
    Task CreateRecoveryBranchAsync(string repositoryPath, string reference, string branchName, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitStash>> GetStashesAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task StashPushAsync(string repositoryPath, string? message, bool includeUntracked, CancellationToken cancellationToken = default);
    Task StashApplyAsync(string repositoryPath, string reference, bool pop, CancellationToken cancellationToken = default);
    Task StashDropAsync(string repositoryPath, string reference, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitWorktree>> GetWorktreesAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task CreateWorktreeAsync(string repositoryPath, string path, string branch, CancellationToken cancellationToken = default);
    Task RemoveWorktreeAsync(string repositoryPath, string path, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitReflogEntry>> GetReflogAsync(string repositoryPath, int take = 100, CancellationToken cancellationToken = default);
    Task AbortCurrentOperationAsync(string repositoryPath, CancellationToken cancellationToken = default);
}
