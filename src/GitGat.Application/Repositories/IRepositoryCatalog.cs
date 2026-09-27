using GitGat.Domain.Repositories;

namespace GitGat.Application.Repositories;

public interface IRepositoryCatalog
{
    IReadOnlyList<RepositoryIdentity> GetRecent();

    void AddOrUpdate(RepositoryIdentity repository);
}
