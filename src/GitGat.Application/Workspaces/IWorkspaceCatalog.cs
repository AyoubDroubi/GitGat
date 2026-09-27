using GitGat.Domain.Workspaces;

namespace GitGat.Application.Workspaces;

public interface IWorkspaceCatalog
{
    Task<IReadOnlyList<WorkspaceIdentity>> GetAllAsync(CancellationToken cancellationToken = default);
    Task<WorkspaceIdentity> SaveAsync(WorkspaceIdentity workspace, CancellationToken cancellationToken = default);
    Task DeleteAsync(Guid workspaceId, CancellationToken cancellationToken = default);
}
