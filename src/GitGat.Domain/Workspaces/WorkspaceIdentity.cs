namespace GitGat.Domain.Workspaces;

public sealed record WorkspaceIdentity(
    Guid Id,
    string Name,
    IReadOnlyList<Guid> RepositoryIds,
    DateTimeOffset UpdatedUtc);
