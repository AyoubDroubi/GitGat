namespace GitGat.Application.Git;

public sealed record GitLfsLock(
    string Id,
    string Path,
    string Owner,
    DateTimeOffset? LockedAt,
    bool IsMine);

public interface IGitLfsClient
{
    Task<string?> GetVersionAsync(CancellationToken cancellationToken = default);
    Task<IReadOnlyList<GitLfsLock>> GetLocksAsync(string repositoryPath, CancellationToken cancellationToken = default);
    Task LockAsync(string repositoryPath, string path, CancellationToken cancellationToken = default);
    Task UnlockAsync(string repositoryPath, string path, bool force, CancellationToken cancellationToken = default);
    Task ConfigureLockablePatternAsync(string repositoryPath, string pattern, CancellationToken cancellationToken = default);
}
