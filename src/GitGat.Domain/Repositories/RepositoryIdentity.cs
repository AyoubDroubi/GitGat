namespace GitGat.Domain.Repositories;

public sealed record RepositoryIdentity(
    Guid Id,
    string Name,
    string LocalPath,
    string? RemoteUrl,
    bool IsFavorite,
    DateTimeOffset LastOpenedUtc,
    bool Exists)
{
    public static RepositoryIdentity Create(string name, string localPath, string? remoteUrl) =>
        new(Guid.NewGuid(), name, localPath, remoteUrl, false, DateTimeOffset.UtcNow, true);
}
