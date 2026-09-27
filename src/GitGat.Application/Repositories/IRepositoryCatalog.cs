using GitGat.Domain.Repositories;

namespace GitGat.Application.Repositories;

public interface IRepositoryCatalog
{
    Task<IReadOnlyList<RepositoryIdentity>> GetRecentAsync(string? search = null, CancellationToken cancellationToken = default);
    Task AddOrUpdateAsync(RepositoryIdentity repository, CancellationToken cancellationToken = default);
    Task RemoveAsync(Guid repositoryId, CancellationToken cancellationToken = default);
    Task SetFavoriteAsync(Guid repositoryId, bool isFavorite, CancellationToken cancellationToken = default);
    Task UpdatePathAsync(Guid repositoryId, string newPath, CancellationToken cancellationToken = default);
}
