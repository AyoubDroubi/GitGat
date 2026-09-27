using GitGat.Application.Repositories;
using GitGat.Domain.Repositories;

namespace GitGat.Infrastructure.Repositories;

[Obsolete("Use SqliteRepositoryCatalog. Kept only for lightweight test scenarios.")]
public sealed class InMemoryRepositoryCatalog : IRepositoryCatalog
{
    private readonly List<RepositoryIdentity> _repositories = [];

    public Task<IReadOnlyList<RepositoryIdentity>> GetRecentAsync(
        string? search = null,
        CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        IEnumerable<RepositoryIdentity> query = _repositories;

        if (!string.IsNullOrWhiteSpace(search))
        {
            query = query.Where(item =>
                item.Name.Contains(search, StringComparison.OrdinalIgnoreCase) ||
                item.LocalPath.Contains(search, StringComparison.OrdinalIgnoreCase));
        }

        return Task.FromResult<IReadOnlyList<RepositoryIdentity>>(
            query.OrderByDescending(item => item.IsFavorite)
                .ThenByDescending(item => item.LastOpenedUtc)
                .ToArray());
    }

    public Task AddOrUpdateAsync(
        RepositoryIdentity repository,
        CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        var index = _repositories.FindIndex(
            item => string.Equals(item.LocalPath, repository.LocalPath, StringComparison.OrdinalIgnoreCase));

        if (index >= 0)
        {
            _repositories[index] = repository;
        }
        else
        {
            _repositories.Add(repository);
        }

        return Task.CompletedTask;
    }

    public Task RemoveAsync(Guid repositoryId, CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        _repositories.RemoveAll(item => item.Id == repositoryId);
        return Task.CompletedTask;
    }

    public Task SetFavoriteAsync(
        Guid repositoryId,
        bool isFavorite,
        CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        var index = _repositories.FindIndex(item => item.Id == repositoryId);
        if (index >= 0)
        {
            _repositories[index] = _repositories[index] with { IsFavorite = isFavorite };
        }

        return Task.CompletedTask;
    }

    public Task UpdatePathAsync(
        Guid repositoryId,
        string newPath,
        CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        var index = _repositories.FindIndex(item => item.Id == repositoryId);
        if (index >= 0)
        {
            _repositories[index] = _repositories[index] with
            {
                LocalPath = Path.GetFullPath(newPath),
                Exists = Directory.Exists(newPath),
                LastOpenedUtc = DateTimeOffset.UtcNow
            };
        }

        return Task.CompletedTask;
    }
}
