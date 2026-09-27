namespace GitGat.Application.Git;

public interface IGitClient
{
    Task<string> GetVersionAsync(CancellationToken cancellationToken = default);

    Task<RepositoryStatus> GetStatusAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default);
}
