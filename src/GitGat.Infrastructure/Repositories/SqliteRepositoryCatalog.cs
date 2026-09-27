using GitGat.Application.Repositories;
using GitGat.Domain.Repositories;
using GitGat.Infrastructure.Persistence;
using Microsoft.EntityFrameworkCore;

namespace GitGat.Infrastructure.Repositories;

public sealed class SqliteRepositoryCatalog(GitGatDbContextFactory factory) : IRepositoryCatalog
{
    public async Task<IReadOnlyList<RepositoryIdentity>> GetRecentAsync(
        string? search = null,
        CancellationToken cancellationToken = default)
    {
        await using var database = factory.Create();
        var query = database.Repositories.AsNoTracking();

        if (!string.IsNullOrWhiteSpace(search))
        {
            var value = search.Trim();
            query = query.Where(item =>
                EF.Functions.Like(item.Name, $"%{value}%") ||
                EF.Functions.Like(item.LocalPath, $"%{value}%"));
        }

        var rows = await query
            .OrderByDescending(item => item.IsFavorite)
            .ThenByDescending(item => item.LastOpenedUtc)
            .Take(250)
            .ToListAsync(cancellationToken);

        return rows.Select(ToDomain).ToArray();
    }

    public async Task AddOrUpdateAsync(
        RepositoryIdentity repository,
        CancellationToken cancellationToken = default)
    {
        await using var database = factory.Create();
        var row = await database.Repositories
            .SingleOrDefaultAsync(item => item.LocalPath == repository.LocalPath, cancellationToken);

        if (row is null)
        {
            row = new RepositoryRow
            {
                Id = repository.Id == Guid.Empty ? Guid.NewGuid() : repository.Id,
                LocalPath = repository.LocalPath
            };
            database.Repositories.Add(row);
        }

        row.Name = repository.Name;
        row.RemoteUrl = repository.RemoteUrl;
        row.IsFavorite = repository.IsFavorite;
        row.LastOpenedUtc = repository.LastOpenedUtc == default
            ? DateTimeOffset.UtcNow
            : repository.LastOpenedUtc;

        await database.SaveChangesAsync(cancellationToken);
    }

    public async Task RemoveAsync(Guid repositoryId, CancellationToken cancellationToken = default)
    {
        await using var database = factory.Create();
        var row = await database.Repositories.FindAsync([repositoryId], cancellationToken);
        if (row is null)
        {
            return;
        }

        database.Repositories.Remove(row);
        await database.SaveChangesAsync(cancellationToken);
    }

    public async Task SetFavoriteAsync(
        Guid repositoryId,
        bool isFavorite,
        CancellationToken cancellationToken = default)
    {
        await using var database = factory.Create();
        var row = await database.Repositories.FindAsync([repositoryId], cancellationToken)
            ?? throw new InvalidOperationException("Repository was not found in the local catalog.");

        row.IsFavorite = isFavorite;
        await database.SaveChangesAsync(cancellationToken);
    }

    public async Task UpdatePathAsync(
        Guid repositoryId,
        string newPath,
        CancellationToken cancellationToken = default)
    {
        var fullPath = Path.GetFullPath(newPath);
        await using var database = factory.Create();
        var row = await database.Repositories.FindAsync([repositoryId], cancellationToken)
            ?? throw new InvalidOperationException("Repository was not found in the local catalog.");

        row.LocalPath = fullPath;
        row.Name = new DirectoryInfo(fullPath).Name;
        row.LastOpenedUtc = DateTimeOffset.UtcNow;
        await database.SaveChangesAsync(cancellationToken);
    }

    private static RepositoryIdentity ToDomain(RepositoryRow row) =>
        new(
            row.Id,
            row.Name,
            row.LocalPath,
            row.RemoteUrl,
            row.IsFavorite,
            row.LastOpenedUtc,
            Directory.Exists(row.LocalPath));
}
