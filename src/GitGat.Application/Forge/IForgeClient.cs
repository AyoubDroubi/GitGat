namespace GitGat.Application.Forge;

public interface IForgeClient
{
    Task<ForgeConnection> GetConnectionAsync(CancellationToken cancellationToken = default);
    Task StartInteractiveLoginAsync(CancellationToken cancellationToken = default);
    Task StartInteractiveLogoutAsync(CancellationToken cancellationToken = default);
    ForgeRepository? ResolveRepository(string? remoteUrl);
    Task<IReadOnlyList<PullRequestSummary>> GetPullRequestsAsync(ForgeRepository repository, CancellationToken cancellationToken = default);
    Task<PullRequestDetail> GetPullRequestAsync(ForgeRepository repository, int number, CancellationToken cancellationToken = default);
    Task CommentOnPullRequestAsync(ForgeRepository repository, int number, string body, CancellationToken cancellationToken = default);
    Task ReviewPullRequestAsync(ForgeRepository repository, int number, PullRequestReviewAction action, string? body, CancellationToken cancellationToken = default);
    Task<string> CreatePullRequestAsync(ForgeRepository repository, string head, string @base, string title, string body, CancellationToken cancellationToken = default);
    Task MergePullRequestAsync(ForgeRepository repository, int number, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<WorkflowRunSummary>> GetWorkflowRunsAsync(ForgeRepository repository, CancellationToken cancellationToken = default);
    Task<string> GetFailedWorkflowLogAsync(ForgeRepository repository, long runId, CancellationToken cancellationToken = default);
    Task RerunWorkflowAsync(ForgeRepository repository, long runId, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<ForgeNotification>> GetNotificationsAsync(CancellationToken cancellationToken = default);
    Task<IReadOnlyList<TeamActivityItem>> GetActivityAsync(ForgeRepository repository, CancellationToken cancellationToken = default);
}
