using GitGat.Application.Repositories;
using GitGat.Domain.Repositories;

namespace GitGat.Infrastructure.Repositories;

public sealed class InMemoryRepositoryCatalog : IRepositoryCatalog
{
    private readonly List<RepositoryIdentity> _repositories = [];

    public IReadOnlyList<RepositoryIdentity> GetRecent() => _repositories.AsReadOnly();

    public void AddOrUpdate(RepositoryIdentity repository)
    {
        var index = _repositories.FindIndex(
            item => string.Equals(
                item.LocalPath,
                repository.LocalPath,
                StringComparison.OrdinalIgnoreCase));

        if (index >= 0)
        {
            _repositories[index] = repository;
            return;
        }

        _repositories.Insert(0, repository);
    }
}
