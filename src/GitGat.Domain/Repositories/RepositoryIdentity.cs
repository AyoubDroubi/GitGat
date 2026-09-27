namespace GitGat.Domain.Repositories;

public sealed record RepositoryIdentity(
    string Name,
    string LocalPath,
    string? RemoteUrl);
