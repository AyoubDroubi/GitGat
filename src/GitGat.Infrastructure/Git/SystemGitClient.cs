using System.Text.RegularExpressions;
using GitGat.Application.Git;

namespace GitGat.Infrastructure.Git;

public sealed partial class SystemGitClient : IGitClient
{
    private readonly GitProcessRunner _runner = new();

    public async Task<string> GetVersionAsync(CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(
            Environment.CurrentDirectory,
            ["--version"],
            cancellationToken);

        EnsureSuccess(result);
        return result.StandardOutput.Trim();
    }

    public async Task<RepositoryStatus> GetStatusAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        if (!Directory.Exists(repositoryPath))
        {
            throw new DirectoryNotFoundException(repositoryPath);
        }

        var result = await _runner.RunAsync(
            repositoryPath,
            ["status", "--porcelain=v1", "--branch"],
            cancellationToken);

        EnsureSuccess(result);

        var lines = result.StandardOutput
            .Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries);

        if (lines.Length == 0)
        {
            return new RepositoryStatus("(detached)", 0, 0, 0, 0);
        }

        var header = lines[0];
        var branch = ParseBranch(header);
        var changedFiles = 0;
        var untrackedFiles = 0;

        foreach (var line in lines.Skip(1))
        {
            if (line.StartsWith("??", StringComparison.Ordinal))
            {
                untrackedFiles++;
            }
            else
            {
                changedFiles++;
            }
        }

        return new RepositoryStatus(
            branch,
            changedFiles,
            untrackedFiles,
            ParseCount(AheadRegex(), header),
            ParseCount(BehindRegex(), header));
    }

    private static string ParseBranch(string header)
    {
        var value = header.StartsWith("## ", StringComparison.Ordinal)
            ? header[3..]
            : header;

        var separator = value.IndexOf("...", StringComparison.Ordinal);
        if (separator >= 0)
        {
            value = value[..separator];
        }

        var metadata = value.IndexOf(' ');
        if (metadata >= 0)
        {
            value = value[..metadata];
        }

        return string.IsNullOrWhiteSpace(value) ? "(detached)" : value.Trim();
    }

    private static int ParseCount(Regex regex, string value)
    {
        var match = regex.Match(value);
        return match.Success && int.TryParse(match.Groups[1].Value, out var count)
            ? count
            : 0;
    }

    private static void EnsureSuccess(GitCommandResult result)
    {
        if (result.ExitCode == 0)
        {
            return;
        }

        var message = string.IsNullOrWhiteSpace(result.StandardError)
            ? "Git command failed."
            : result.StandardError.Trim();

        throw new InvalidOperationException(message);
    }

    [GeneratedRegex(@"ahead (\d+)", RegexOptions.CultureInvariant)]
    private static partial Regex AheadRegex();

    [GeneratedRegex(@"behind (\d+)", RegexOptions.CultureInvariant)]
    private static partial Regex BehindRegex();
}
