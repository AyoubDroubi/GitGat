using System.Diagnostics;
using System.Text.Json;
using System.Text.RegularExpressions;
using GitGat.Application.Forge;

namespace GitGat.Infrastructure.Forge;

public sealed class GitHubForgeClient : IForgeClient
{
    private readonly GhProcessRunner _runner = new();

    public async Task<ForgeConnection> GetConnectionAsync(CancellationToken cancellationToken = default)
    {
        var status = await _runner.RunAsync(["auth", "status", "--hostname", "github.com"], cancellationToken);
        if (status.ExitCode != 0)
        {
            return new ForgeConnection(false, null, string.IsNullOrWhiteSpace(status.Error) ? "GitHub is not connected." : status.Error.Trim());
        }

        var user = await _runner.RunAsync(["api", "user", "--jq", ".login"], cancellationToken);
        return user.ExitCode == 0
            ? new ForgeConnection(true, user.Output.Trim(), "Connected through GitHub CLI secure credential storage.")
            : new ForgeConnection(true, null, "GitHub CLI is authenticated, but the account could not be read.");
    }

    public Task StartInteractiveLoginAsync(CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        LaunchInteractive("gh auth login --hostname github.com --git-protocol https --web");
        return Task.CompletedTask;
    }

    public Task StartInteractiveLogoutAsync(CancellationToken cancellationToken = default)
    {
        cancellationToken.ThrowIfCancellationRequested();
        LaunchInteractive("gh auth logout --hostname github.com");
        return Task.CompletedTask;
    }

    public ForgeRepository? ResolveRepository(string? remoteUrl)
    {
        if (string.IsNullOrWhiteSpace(remoteUrl))
        {
            return null;
        }

        var value = remoteUrl.Trim();

        var sshMatch = Regex.Match(
            value,
            @"^(?:ssh://)?git@github\.com[:/](?<owner>[^/]+)/(?<name>[^/]+?)(?:\.git)?$",
            RegexOptions.IgnoreCase | RegexOptions.CultureInvariant);

        if (sshMatch.Success)
        {
            return new ForgeRepository(sshMatch.Groups["owner"].Value, sshMatch.Groups["name"].Value);
        }

        if (Uri.TryCreate(value, UriKind.Absolute, out var uri) &&
            string.Equals(uri.Host, "github.com", StringComparison.OrdinalIgnoreCase))
        {
            var parts = uri.AbsolutePath.Trim('/').Split('/', StringSplitOptions.RemoveEmptyEntries);
            if (parts.Length >= 2)
            {
                return new ForgeRepository(parts[0], parts[1].EndsWith(".git", StringComparison.OrdinalIgnoreCase)
                    ? parts[1][..^4]
                    : parts[1]);
            }
        }

        return null;
    }

    public async Task<IReadOnlyList<PullRequestSummary>> GetPullRequestsAsync(
        ForgeRepository repository,
        CancellationToken cancellationToken = default)
    {
        var result = await RunRequiredAsync(
            ["pr", "list", "--repo", repository.FullName, "--limit", "100", "--state", "open",
             "--json", "number,title,state,author,headRefName,baseRefName,isDraft,reviewDecision,updatedAt,url"],
            cancellationToken);

        using var document = JsonDocument.Parse(result);
        return document.RootElement.EnumerateArray().Select(ParsePullRequestSummary).ToArray();
    }

    public async Task<PullRequestDetail> GetPullRequestAsync(
        ForgeRepository repository,
        int number,
        CancellationToken cancellationToken = default)
    {
        var result = await RunRequiredAsync(
            ["pr", "view", number.ToString(), "--repo", repository.FullName,
             "--json", "number,title,state,author,headRefName,baseRefName,isDraft,reviewDecision,updatedAt,url,body,mergeable,mergeStateStatus,files"],
            cancellationToken);

        using var document = JsonDocument.Parse(result);
        var root = document.RootElement;

        var files = root.TryGetProperty("files", out var filesElement)
            ? filesElement.EnumerateArray()
                .Select(item => item.TryGetProperty("path", out var path) ? path.GetString() ?? string.Empty : string.Empty)
                .Where(path => path.Length > 0)
                .ToArray()
            : [];

        return new PullRequestDetail(
            ParsePullRequestSummary(root),
            ReadString(root, "body"),
            ReadString(root, "mergeable"),
            ReadNullableString(root, "mergeStateStatus"),
            files);
    }

    public async Task CommentOnPullRequestAsync(
        ForgeRepository repository,
        int number,
        string body,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(body))
        {
            throw new InvalidOperationException("Comment cannot be empty.");
        }

        await RunRequiredAsync(
            ["pr", "comment", number.ToString(), "--repo", repository.FullName, "--body", body],
            cancellationToken);
    }

    public async Task ReviewPullRequestAsync(
        ForgeRepository repository,
        int number,
        PullRequestReviewAction action,
        string? body,
        CancellationToken cancellationToken = default)
    {
        var arguments = new List<string> { "pr", "review", number.ToString(), "--repo", repository.FullName };
        arguments.Add(action switch
        {
            PullRequestReviewAction.Approve => "--approve",
            PullRequestReviewAction.RequestChanges => "--request-changes",
            _ => "--comment"
        });

        if (!string.IsNullOrWhiteSpace(body))
        {
            arguments.AddRange(["--body", body.Trim()]);
        }

        await RunRequiredAsync(arguments, cancellationToken);
    }

    public async Task<string> CreatePullRequestAsync(
        ForgeRepository repository,
        string head,
        string @base,
        string title,
        string body,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(head) || string.IsNullOrWhiteSpace(@base) || string.IsNullOrWhiteSpace(title))
        {
            throw new InvalidOperationException("Head, base and title are required.");
        }

        return (await RunRequiredAsync(
            ["pr", "create", "--repo", repository.FullName, "--head", head, "--base", @base,
             "--title", title, "--body", body ?? string.Empty],
            cancellationToken)).Trim();
    }

    public async Task EditPullRequestAsync(
        ForgeRepository repository,
        int number,
        string title,
        string body,
        CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(title))
        {
            throw new InvalidOperationException("Pull request title is required.");
        }

        await RunRequiredAsync(
            ["pr", "edit", number.ToString(), "--repo", repository.FullName,
             "--title", title.Trim(), "--body", body ?? string.Empty],
            cancellationToken);
    }

    public async Task MergePullRequestAsync(
        ForgeRepository repository,
        int number,
        CancellationToken cancellationToken = default)
    {
        var detail = await GetPullRequestAsync(repository, number, cancellationToken);
        if (!string.Equals(detail.Mergeable, "MERGEABLE", StringComparison.OrdinalIgnoreCase))
        {
            throw new InvalidOperationException($"Pull request #{number} is not mergeable ({detail.Mergeable}).");
        }

        if (detail.MergeStateStatus is "BLOCKED" or "DIRTY" or "UNKNOWN")
        {
            throw new InvalidOperationException(
                $"Pull request #{number} cannot be merged while GitHub reports {detail.MergeStateStatus}.");
        }

        await RunRequiredAsync(
            ["pr", "merge", number.ToString(), "--repo", repository.FullName, "--merge"],
            cancellationToken);
    }

    public async Task<IReadOnlyList<WorkflowRunSummary>> GetWorkflowRunsAsync(
        ForgeRepository repository,
        CancellationToken cancellationToken = default)
    {
        var result = await RunRequiredAsync(
            ["run", "list", "--repo", repository.FullName, "--limit", "50",
             "--json", "databaseId,name,displayTitle,status,conclusion,headBranch,createdAt,url"],
            cancellationToken);

        using var document = JsonDocument.Parse(result);
        return document.RootElement.EnumerateArray()
            .Select(item => new WorkflowRunSummary(
                ReadLong(item, "databaseId"),
                string.IsNullOrWhiteSpace(ReadString(item, "displayTitle"))
                    ? ReadString(item, "name")
                    : ReadString(item, "displayTitle"),
                ReadString(item, "status"),
                ReadNullableString(item, "conclusion"),
                ReadString(item, "headBranch"),
                ReadDate(item, "createdAt"),
                ReadString(item, "url")))
            .ToArray();
    }

    public Task<string> GetFailedWorkflowLogAsync(
        ForgeRepository repository,
        long runId,
        CancellationToken cancellationToken = default) =>
        RunRequiredAsync(
            ["run", "view", runId.ToString(), "--repo", repository.FullName, "--log-failed"],
            cancellationToken);

    public async Task RerunWorkflowAsync(
        ForgeRepository repository,
        long runId,
        CancellationToken cancellationToken = default)
    {
        await RunRequiredAsync(
            ["run", "rerun", runId.ToString(), "--repo", repository.FullName, "--failed"],
            cancellationToken);
    }

    public async Task<IReadOnlyList<ForgeNotification>> GetNotificationsAsync(
        CancellationToken cancellationToken = default)
    {
        var result = await RunRequiredAsync(
            ["api", "--paginate", "--slurp", "notifications"],
            cancellationToken);

        using var document = JsonDocument.Parse(result);
        var notifications = new List<ForgeNotification>();

        if (document.RootElement.ValueKind != JsonValueKind.Array)
        {
            return notifications;
        }

        foreach (var page in document.RootElement.EnumerateArray())
        {
            if (page.ValueKind != JsonValueKind.Array)
            {
                continue;
            }

            foreach (var item in page.EnumerateArray())
            {
                var repository = item.TryGetProperty("repository", out var repositoryElement)
                    ? ReadString(repositoryElement, "full_name")
                    : string.Empty;
                var subject = item.TryGetProperty("subject", out var subjectElement)
                    ? subjectElement
                    : default;

                notifications.Add(new ForgeNotification(
                    ReadString(item, "id"),
                    repository,
                    subject.ValueKind == JsonValueKind.Object ? ReadString(subject, "title") : string.Empty,
                    subject.ValueKind == JsonValueKind.Object ? ReadString(subject, "type") : string.Empty,
                    ReadString(item, "reason"),
                    item.TryGetProperty("unread", out var unread) && unread.GetBoolean(),
                    ReadDate(item, "updated_at"),
                    subject.ValueKind == JsonValueKind.Object ? ReadNullableString(subject, "url") : null));
            }
        }

        return notifications;
    }

    public async Task<IReadOnlyList<TeamActivityItem>> GetActivityAsync(
        ForgeRepository repository,
        CancellationToken cancellationToken = default)
    {
        var result = await RunRequiredAsync(
            ["api", $"repos/{repository.FullName}/events?per_page=50"],
            cancellationToken);

        using var document = JsonDocument.Parse(result);
        return document.RootElement.EnumerateArray()
            .Select(item =>
            {
                var actor = item.TryGetProperty("actor", out var actorElement)
                    ? ReadString(actorElement, "login")
                    : string.Empty;
                var repo = item.TryGetProperty("repo", out var repoElement)
                    ? ReadString(repoElement, "name")
                    : repository.FullName;

                return new TeamActivityItem(
                    ReadString(item, "id"),
                    ReadString(item, "type"),
                    actor,
                    repo,
                    ReadDate(item, "created_at"),
                    null);
            })
            .ToArray();
    }

    private async Task<string> RunRequiredAsync(
        IEnumerable<string> arguments,
        CancellationToken cancellationToken)
    {
        var result = await _runner.RunAsync(arguments, cancellationToken);
        if (result.ExitCode != 0)
        {
            throw new InvalidOperationException(
                string.IsNullOrWhiteSpace(result.Error)
                    ? "GitHub CLI command failed."
                    : result.Error.Trim());
        }

        return result.Output;
    }

    private static PullRequestSummary ParsePullRequestSummary(JsonElement item)
    {
        var author = item.TryGetProperty("author", out var authorElement)
            ? ReadString(authorElement, "login")
            : string.Empty;

        return new PullRequestSummary(
            ReadInt(item, "number"),
            ReadString(item, "title"),
            ReadString(item, "state"),
            author,
            ReadString(item, "headRefName"),
            ReadString(item, "baseRefName"),
            item.TryGetProperty("isDraft", out var draft) && draft.GetBoolean(),
            ReadNullableString(item, "reviewDecision"),
            ReadDate(item, "updatedAt"),
            ReadString(item, "url"));
    }

    private static void LaunchInteractive(string command)
    {
        ProcessStartInfo startInfo;

        if (OperatingSystem.IsWindows())
        {
            startInfo = new ProcessStartInfo("cmd.exe", $"/k {command}") { UseShellExecute = true };
        }
        else if (OperatingSystem.IsMacOS())
        {
            var escaped = command
                .Replace("\\", "\\\\", StringComparison.Ordinal)
                .Replace("\"", "\\\"", StringComparison.Ordinal);
            var script = $"tell application \"Terminal\" to do script \"{escaped}\"";
            startInfo = new ProcessStartInfo("osascript")
            {
                UseShellExecute = false
            };
            startInfo.ArgumentList.Add("-e");
            startInfo.ArgumentList.Add(script);
        }
        else
        {
            startInfo = new ProcessStartInfo("x-terminal-emulator", $"-e sh -lc \"{command}; exec sh\"")
            {
                UseShellExecute = true
            };
        }

        Process.Start(startInfo);
    }

    private static string ReadString(JsonElement element, string name) =>
        element.TryGetProperty(name, out var property) && property.ValueKind == JsonValueKind.String
            ? property.GetString() ?? string.Empty
            : string.Empty;

    private static string? ReadNullableString(JsonElement element, string name)
    {
        if (!element.TryGetProperty(name, out var property) || property.ValueKind == JsonValueKind.Null)
        {
            return null;
        }

        return property.ValueKind == JsonValueKind.String ? property.GetString() : property.ToString();
    }

    private static int ReadInt(JsonElement element, string name) =>
        element.TryGetProperty(name, out var property) && property.TryGetInt32(out var value) ? value : 0;

    private static long ReadLong(JsonElement element, string name) =>
        element.TryGetProperty(name, out var property) && property.TryGetInt64(out var value) ? value : 0;

    private static DateTimeOffset ReadDate(JsonElement element, string name) =>
        element.TryGetProperty(name, out var property) &&
        DateTimeOffset.TryParse(property.GetString(), out var value)
            ? value
            : DateTimeOffset.MinValue;
}
