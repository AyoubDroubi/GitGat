using GitGat.Application.Workspaces;
using GitGat.Domain.Workspaces;
using GitGat.Infrastructure.Persistence;
using Microsoft.EntityFrameworkCore;

namespace GitGat.Infrastructure.Workspaces;

public sealed class SqliteWorkspaceCatalog(GitGatDbContextFactory factory) : IWorkspaceCatalog
{
    public async Task<IReadOnlyList<WorkspaceIdentity>> GetAllAsync(
        CancellationToken cancellationToken = default)
    {
        await using var database = factory.Create();
        var workspaces = await database.Workspaces
            .AsNoTracking()
            .OrderBy(item => item.Name)
            .ToListAsync(cancellationToken);

        var memberships = await database.WorkspaceRepositories
            .AsNoTracking()
            .ToListAsync(cancellationToken);

        return workspaces
            .Select(workspace => new WorkspaceIdentity(
                workspace.Id,
                workspace.Name,
                memberships
                    .Where(item => item.WorkspaceId == workspace.Id)
                    .Select(item => item.RepositoryId)
                    .ToArray(),
                workspace.UpdatedUtc))
            .ToArray();
    }

    public async Task<WorkspaceIdentity> SaveAsync(
        WorkspaceIdentity workspace,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(workspace.Name))
        {
            throw new InvalidOperationException("Workspace name is required.");
        }

        await using var database = factory.Create();
        var id = workspace.Id == Guid.Empty ? Guid.NewGuid() : workspace.Id;
        var row = await database.Workspaces.FindAsync([id], cancellationToken);

        if (row is null)
        {
            row = new WorkspaceRow { Id = id };
            database.Workspaces.Add(row);
        }

        row.Name = workspace.Name.Trim();
        row.UpdatedUtc = DateTimeOffset.UtcNow;

        var oldMemberships = database.WorkspaceRepositories.Where(item => item.WorkspaceId == id);
        database.WorkspaceRepositories.RemoveRange(oldMemberships);

        foreach (var repositoryId in workspace.RepositoryIds.Distinct())
        {
            database.WorkspaceRepositories.Add(new WorkspaceRepositoryRow
            {
                WorkspaceId = id,
                RepositoryId = repositoryId
            });
        }

        await database.SaveChangesAsync(cancellationToken);
        return workspace with { Id = id, Name = row.Name, UpdatedUtc = row.UpdatedUtc };
    }

    public async Task DeleteAsync(Guid workspaceId, CancellationToken cancellationToken = default)
    {
        await using var database = factory.Create();
        var row = await database.Workspaces.FindAsync([workspaceId], cancellationToken);
        if (row is null)
        {
            return;
        }

        database.Workspaces.Remove(row);
        await database.SaveChangesAsync(cancellationToken);
    }
}
