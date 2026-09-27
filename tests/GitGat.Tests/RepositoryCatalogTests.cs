using GitGat.Domain.Repositories;
using GitGat.Domain.Workspaces;
using GitGat.Infrastructure.Persistence;
using GitGat.Infrastructure.Repositories;
using GitGat.Infrastructure.Workspaces;

namespace GitGat.Tests;

public sealed class RepositoryCatalogTests : IDisposable
{
    private readonly string _root = Path.Combine(
        Path.GetTempPath(),
        "GitGat.Tests",
        Guid.NewGuid().ToString("N"));

    [Fact]
    public async Task RepositoryCatalog_PersistsAcrossFactoryInstances()
    {
        Directory.CreateDirectory(_root);
        var databasePath = Path.Combine(_root, "catalog.db");
        var repositoryPath = Path.Combine(_root, "repo");
        Directory.CreateDirectory(repositoryPath);

        var repository = RepositoryIdentity.Create(
            "repo",
            repositoryPath,
            "https://github.com/example/repo.git");

        var firstCatalog = new SqliteRepositoryCatalog(
            new GitGatDbContextFactory(databasePath));

        await firstCatalog.AddOrUpdateAsync(repository);
        await firstCatalog.SetFavoriteAsync(repository.Id, true);

        var secondCatalog = new SqliteRepositoryCatalog(
            new GitGatDbContextFactory(databasePath));

        var loaded = await secondCatalog.GetRecentAsync();

        var item = Assert.Single(loaded);
        Assert.Equal(repository.Id, item.Id);
        Assert.True(item.IsFavorite);
        Assert.True(item.Exists);
    }

    [Fact]
    public async Task WorkspaceCatalog_PersistsMembershipWithoutMutatingRepository()
    {
        Directory.CreateDirectory(_root);
        var databasePath = Path.Combine(_root, "workspace.db");
        var repositoryPath = Path.Combine(_root, "repo");
        Directory.CreateDirectory(repositoryPath);

        var factory = new GitGatDbContextFactory(databasePath);
        var repositories = new SqliteRepositoryCatalog(factory);
        var workspaces = new SqliteWorkspaceCatalog(factory);

        var repository = RepositoryIdentity.Create("repo", repositoryPath, null);
        await repositories.AddOrUpdateAsync(repository);

        var saved = await workspaces.SaveAsync(
            new WorkspaceIdentity(
                Guid.Empty,
                "Product",
                [repository.Id],
                DateTimeOffset.UtcNow));

        var loaded = Assert.Single(await workspaces.GetAllAsync());

        Assert.Equal(saved.Id, loaded.Id);
        Assert.Equal("Product", loaded.Name);
        Assert.Contains(repository.Id, loaded.RepositoryIds);
        Assert.True(Directory.Exists(repositoryPath));
    }

    public void Dispose()
    {
        if (!Directory.Exists(_root))
        {
            return;
        }

        try
        {
            Directory.Delete(_root, recursive: true);
        }
        catch (IOException)
        {
        }
        catch (UnauthorizedAccessException)
        {
        }
    }
}
