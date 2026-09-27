using System.Text.Json;
using GitGat.Application.Git;
using GitGat.Application.Operations;

namespace GitGat.Infrastructure.Git;

public sealed class SystemGitLfsClient : IGitLfsClient
{
    private readonly GitProcessRunner _runner = new();
    private readonly IOperationJournal? _journal;

    public SystemGitLfsClient(IOperationJournal? journal = null)
    {
        _journal = journal;
    }

    public async Task<string?> GetVersionAsync(CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(Environment.CurrentDirectory, ["lfs", "version"], cancellationToken);
        return result.ExitCode == 0 ? result.StandardOutput.Trim() : null;
    }

    public async Task<IReadOnlyList<GitLfsLock>> GetLocksAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        var version = await GetVersionAsync(cancellationToken);
        if (version is null)
        {
            return [];
        }

        var result = await _runner.RunAsync(repositoryPath, ["lfs", "locks", "--json"], cancellationToken);
        EnsureSuccess(result);

        using var document = JsonDocument.Parse(result.StandardOutput);
        if (!document.RootElement.TryGetProperty("locks", out var locksElement))
        {
            return [];
        }

        var localUser = await GetLocalUserAsync(repositoryPath, cancellationToken);
        var locks = new List<GitLfsLock>();

        foreach (var item in locksElement.EnumerateArray())
        {
            var owner = ReadOwner(item);
            locks.Add(new GitLfsLock(
                item.TryGetProperty("id", out var id) ? id.GetString() ?? string.Empty : string.Empty,
                item.TryGetProperty("path", out var path) ? path.GetString() ?? string.Empty : string.Empty,
                owner,
                item.TryGetProperty("locked_at", out var lockedAt) &&
                DateTimeOffset.TryParse(lockedAt.GetString(), out var parsedAt)
                    ? parsedAt
                    : null,
                !string.IsNullOrWhiteSpace(localUser) &&
                string.Equals(localUser, owner, StringComparison.OrdinalIgnoreCase)));
        }

        return locks;
    }

    public async Task LockAsync(
        string repositoryPath,
        string path,
        CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "lfs-lock", path, false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["lfs", "lock", path], cancellationToken));
    }

    public async Task UnlockAsync(
        string repositoryPath,
        string path,
        bool force,
        CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, force ? "lfs-force-unlock" : "lfs-unlock", path, force, cancellationToken);
        var arguments = force
            ? new[] { "lfs", "unlock", "--force", path }
            : new[] { "lfs", "unlock", path };
        EnsureSuccess(await _runner.RunAsync(repositoryPath, arguments, cancellationToken));
    }

    public async Task ConfigureLockablePatternAsync(
        string repositoryPath,
        string pattern,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(pattern))
        {
            throw new InvalidOperationException("A lockable file pattern is required.");
        }

        await JournalAsync(repositoryPath, "lfs-track-lockable", pattern, false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["lfs", "track", "--lockable", pattern.Trim()], cancellationToken));
    }

    private async Task<string?> GetLocalUserAsync(string repositoryPath, CancellationToken cancellationToken)
    {
        var result = await _runner.RunAsync(repositoryPath, ["config", "user.name"], cancellationToken);
        return result.ExitCode == 0 ? result.StandardOutput.Trim() : null;
    }

    private async Task JournalAsync(
        string repositoryPath,
        string operation,
        string detail,
        bool destructive,
        CancellationToken cancellationToken)
    {
        if (_journal is null)
        {
            return;
        }

        await _journal.RecordAsync(
            new OperationRecord(DateTimeOffset.UtcNow, repositoryPath, operation, detail, destructive),
            cancellationToken);
    }

    private static string ReadOwner(JsonElement item)
    {
        if (!item.TryGetProperty("owner", out var owner))
        {
            return "unknown";
        }

        if (owner.ValueKind == JsonValueKind.String)
        {
            return owner.GetString() ?? "unknown";
        }

        if (owner.TryGetProperty("name", out var name))
        {
            return name.GetString() ?? "unknown";
        }

        if (owner.TryGetProperty("login", out var login))
        {
            return login.GetString() ?? "unknown";
        }

        return "unknown";
    }

    private static void EnsureSuccess(GitCommandResult result)
    {
        if (result.ExitCode != 0)
        {
            throw new InvalidOperationException(
                string.IsNullOrWhiteSpace(result.StandardError)
                    ? "Git LFS command failed."
                    : result.StandardError.Trim());
        }
    }
}
