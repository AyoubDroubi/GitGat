namespace GitGat.Application.Forge;

public sealed record ForgeConnection(bool IsConnected, string? Login, string Message);

public sealed record ForgeRepository(string Owner, string Name)
{
    public string FullName => $"{Owner}/{Name}";
}

public enum PullRequestReviewAction
{
    Approve,
    RequestChanges,
    Comment
}

public sealed record PullRequestSummary(
    int Number,
    string Title,
    string State,
    string Author,
    string HeadBranch,
    string BaseBranch,
    bool IsDraft,
    string? ReviewDecision,
    DateTimeOffset UpdatedAt,
    string Url);

public sealed record PullRequestDetail(
    PullRequestSummary Summary,
    string Body,
    string Mergeable,
    string? MergeStateStatus,
    IReadOnlyList<string> ChangedFiles);

public sealed record WorkflowRunSummary(
    long Id,
    string Name,
    string Status,
    string? Conclusion,
    string Branch,
    DateTimeOffset CreatedAt,
    string Url);

public sealed record ForgeNotification(
    string Id,
    string Repository,
    string Subject,
    string Type,
    string Reason,
    bool Unread,
    DateTimeOffset UpdatedAt,
    string? ApiUrl);

public sealed record TeamActivityItem(
    string Id,
    string Type,
    string Actor,
    string Repository,
    DateTimeOffset CreatedAt,
    string? ApiUrl);
