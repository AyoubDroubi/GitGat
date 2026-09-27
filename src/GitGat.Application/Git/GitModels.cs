namespace GitGat.Application.Git;

public enum GitChangeKind
{
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Conflicted,
    Changed
}

public sealed record GitChange(
    string Path,
    string? OriginalPath,
    GitChangeKind Kind,
    bool IsStaged,
    bool IsUnstaged,
    bool IsUntracked,
    bool IsConflicted);

public sealed record GitBranch(
    string Name,
    bool IsCurrent,
    bool IsRemote,
    string? Upstream,
    int Ahead,
    int Behind);

public sealed record GitCommit(
    string Sha,
    string ShortSha,
    string Author,
    DateTimeOffset AuthoredAt,
    string Subject);

public sealed record GitCommitDetail(
    GitCommit Commit,
    string Body,
    string Statistics);

public sealed record GitStash(
    string Reference,
    string Subject,
    DateTimeOffset? CreatedAt);

public sealed record GitWorktree(
    string Path,
    string? Head,
    string? Branch,
    bool IsLocked,
    bool IsPrunable);

public sealed record GitReflogEntry(
    string Selector,
    string Sha,
    string Subject,
    DateTimeOffset? CreatedAt);

public enum GitConflictResolution
{
    KeepOurs,
    KeepTheirs
}

public enum GitResetMode
{
    Soft,
    Mixed,
    Hard
}
