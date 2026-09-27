using System.Globalization;
using System.Text.RegularExpressions;
using GitGat.Application.Git;
using GitGat.Application.Operations;

namespace GitGat.Infrastructure.Git;

public sealed partial class SystemGitClient : IGitClient
{
    private readonly GitProcessRunner _runner = new();
    private readonly IOperationJournal? _journal;

    public SystemGitClient(IOperationJournal? journal = null)
    {
        _journal = journal;
    }

    public async Task<string> GetVersionAsync(CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(Environment.CurrentDirectory, ["--version"], cancellationToken);
        EnsureSuccess(result);
        return result.StandardOutput.Trim();
    }

    public async Task<bool> IsRepositoryAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        if (!Directory.Exists(repositoryPath))
        {
            return false;
        }

        var result = await _runner.RunAsync(
            repositoryPath,
            ["rev-parse", "--is-inside-work-tree"],
            cancellationToken);

        return result.ExitCode == 0 &&
               string.Equals(result.StandardOutput.Trim(), "true", StringComparison.OrdinalIgnoreCase);
    }

    public async Task<string?> GetRemoteUrlAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        EnsureDirectory(repositoryPath);
        var result = await _runner.RunAsync(repositoryPath, ["remote", "get-url", "origin"], cancellationToken);
        return result.ExitCode == 0 ? result.StandardOutput.Trim() : null;
    }

    public async Task CloneAsync(string remoteUrl, string destinationPath, CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(remoteUrl))
        {
            throw new ArgumentException("Remote URL is required.", nameof(remoteUrl));
        }

        var fullDestination = Path.GetFullPath(destinationPath);
        if (Directory.Exists(fullDestination) && Directory.EnumerateFileSystemEntries(fullDestination).Any())
        {
            throw new InvalidOperationException("Clone destination must be empty.");
        }

        var parent = Path.GetDirectoryName(fullDestination)
            ?? throw new InvalidOperationException("Clone destination has no parent directory.");

        Directory.CreateDirectory(parent);
        await JournalAsync(parent, "clone", remoteUrl, false, cancellationToken);
        var result = await _runner.RunAsync(parent, ["clone", "--progress", remoteUrl, fullDestination], cancellationToken);
        EnsureSuccess(result);
    }

    public async Task<RepositoryStatus> GetStatusAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        var parsed = await ReadStatusAsync(repositoryPath, cancellationToken);
        return parsed.Status with { Operation = DetectOperation(repositoryPath) };
    }

    public async Task<IReadOnlyList<GitChange>> GetChangesAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        var parsed = await ReadStatusAsync(repositoryPath, cancellationToken);
        return parsed.Changes;
    }

    public async Task<string> GetDiffAsync(
        string repositoryPath,
        string path,
        bool staged,
        CancellationToken cancellationToken = default)
    {
        EnsureDirectory(repositoryPath);
        var numStatArguments = new List<string> { "diff" };
        if (staged)
        {
            numStatArguments.Add("--cached");
        }

        numStatArguments.AddRange(["--numstat", "--", path]);
        var numStat = await _runner.RunAsync(repositoryPath, numStatArguments, cancellationToken);
        EnsureSuccess(numStat);

        if (numStat.StandardOutput
            .Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries)
            .Any(line => line.StartsWith("-\t-\t", StringComparison.Ordinal)))
        {
            return "Binary file — preview unavailable.";
        }

        var arguments = new List<string> { "diff" };
        if (staged)
        {
            arguments.Add("--cached");
        }

        arguments.AddRange(["--no-ext-diff", "--unified=3", "--", path]);
        var result = await _runner.RunAsync(repositoryPath, arguments, cancellationToken);
        EnsureSuccess(result);
        return result.StandardOutput;
    }

    public async Task StageAsync(
        string repositoryPath,
        IEnumerable<string> paths,
        CancellationToken cancellationToken = default)
    {
        var list = paths.Where(path => !string.IsNullOrWhiteSpace(path)).Distinct(StringComparer.Ordinal).ToArray();
        if (list.Length == 0)
        {
            return;
        }

        await JournalAsync(repositoryPath, "stage", string.Join(", ", list), false, cancellationToken);
        var arguments = new List<string> { "add", "--" };
        arguments.AddRange(list);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, arguments, cancellationToken));
    }

    public async Task UnstageAsync(
        string repositoryPath,
        IEnumerable<string> paths,
        CancellationToken cancellationToken = default)
    {
        var list = paths.Where(path => !string.IsNullOrWhiteSpace(path)).Distinct(StringComparer.Ordinal).ToArray();
        if (list.Length == 0)
        {
            return;
        }

        await JournalAsync(repositoryPath, "unstage", string.Join(", ", list), false, cancellationToken);
        var arguments = new List<string> { "restore", "--staged", "--" };
        arguments.AddRange(list);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, arguments, cancellationToken));
    }

    public async Task CommitAsync(string repositoryPath, string message, CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(message))
        {
            throw new InvalidOperationException("Commit message is required.");
        }

        var staged = await _runner.RunAsync(repositoryPath, ["diff", "--cached", "--quiet"], cancellationToken);
        if (staged.ExitCode == 0)
        {
            throw new InvalidOperationException("There are no staged changes to commit.");
        }

        if (staged.ExitCode != 1)
        {
            EnsureSuccess(staged);
        }

        await JournalAsync(repositoryPath, "commit", message.Trim(), false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["commit", "-m", message.Trim()], cancellationToken));
    }

    public async Task DiscardTrackedAsync(
        string repositoryPath,
        string path,
        CancellationToken cancellationToken = default)
    {
        var tracked = await _runner.RunAsync(
            repositoryPath,
            ["ls-files", "--error-unmatch", "--", path],
            cancellationToken);

        if (tracked.ExitCode != 0)
        {
            throw new InvalidOperationException("GitGat never discards untracked files. Remove the file manually if that is intended.");
        }

        await JournalAsync(repositoryPath, "discard-tracked", path, true, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["restore", "--worktree", "--", path], cancellationToken));
    }

    public async Task FetchAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "fetch", "all remotes", false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["fetch", "--prune"], cancellationToken));
    }

    public async Task PullAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "pull", "fast-forward only", false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["pull", "--ff-only"], cancellationToken));
    }

    public async Task PushAsync(string repositoryPath, CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "push", "normal push; force disabled", false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["push"], cancellationToken));
    }

    public async Task<IReadOnlyList<GitBranch>> GetBranchesAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(
            repositoryPath,
            ["for-each-ref", "--format=%(refname:short)%09%(HEAD)%09%(upstream:short)%09%(upstream:track)", "refs/heads", "refs/remotes"],
            cancellationToken);

        EnsureSuccess(result);
        var branches = new List<GitBranch>();

        foreach (var line in result.StandardOutput.Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries))
        {
            var parts = line.Split('\t');
            if (parts.Length < 4)
            {
                continue;
            }

            var track = parts[3];
            branches.Add(new GitBranch(
                parts[0],
                string.Equals(parts[1].Trim(), "*", StringComparison.Ordinal),
                parts[0].StartsWith("origin/", StringComparison.Ordinal),
                string.IsNullOrWhiteSpace(parts[2]) ? null : parts[2],
                ParseTrackCount(AheadRegex(), track),
                ParseTrackCount(BehindRegex(), track)));
        }

        return branches;
    }

    public async Task CheckoutBranchAsync(
        string repositoryPath,
        string branch,
        CancellationToken cancellationToken = default)
    {
        var status = await GetStatusAsync(repositoryPath, cancellationToken);
        if (!status.IsClean)
        {
            throw new InvalidOperationException("Commit or stash changes before switching branches.");
        }

        await JournalAsync(repositoryPath, "checkout", branch, false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["switch", branch], cancellationToken));
    }

    public async Task CreateBranchAsync(
        string repositoryPath,
        string branch,
        bool checkout,
        CancellationToken cancellationToken = default)
    {
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["check-ref-format", "--branch", branch], cancellationToken));
        await JournalAsync(repositoryPath, "create-branch", branch, false, cancellationToken);

        var arguments = checkout
            ? new[] { "switch", "-c", branch }
            : new[] { "branch", branch };

        EnsureSuccess(await _runner.RunAsync(repositoryPath, arguments, cancellationToken));
    }

    public async Task DeleteBranchAsync(
        string repositoryPath,
        string branch,
        CancellationToken cancellationToken = default)
    {
        var status = await GetStatusAsync(repositoryPath, cancellationToken);
        if (string.Equals(status.Branch, branch, StringComparison.Ordinal))
        {
            throw new InvalidOperationException("The currently checked out branch cannot be deleted.");
        }

        await JournalAsync(repositoryPath, "delete-branch", branch, true, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["branch", "-d", branch], cancellationToken));
    }

    public async Task<IReadOnlyList<GitCommit>> GetHistoryAsync(
        string repositoryPath,
        int skip = 0,
        int take = 100,
        CancellationToken cancellationToken = default)
    {
        skip = Math.Max(0, skip);
        take = Math.Clamp(take, 1, 500);

        var result = await _runner.RunAsync(
            repositoryPath,
            ["log", $"--skip={skip}", $"-n{take}", "--date=iso-strict", "--pretty=format:%H%x1f%h%x1f%an%x1f%aI%x1f%s"],
            cancellationToken);

        EnsureSuccess(result);
        return result.StandardOutput
            .Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries)
            .Select(ParseCommit)
            .ToArray();
    }

    public async Task<GitCommitDetail> GetCommitDetailAsync(
        string repositoryPath,
        string sha,
        CancellationToken cancellationToken = default)
    {
        var metadata = await _runner.RunAsync(
            repositoryPath,
            ["show", "-s", "--date=iso-strict", "--pretty=format:%H%x1f%h%x1f%an%x1f%aI%x1f%s%x1f%b", sha],
            cancellationToken);
        EnsureSuccess(metadata);

        var parts = metadata.StandardOutput.Split((char)0x1f);
        if (parts.Length < 6)
        {
            throw new InvalidOperationException("Git returned an unexpected commit format.");
        }

        var commit = new GitCommit(
            parts[0],
            parts[1],
            parts[2],
            ParseDate(parts[3]),
            parts[4]);

        var stats = await _runner.RunAsync(
            repositoryPath,
            ["show", "--stat", "--format=", "--no-renames", sha],
            cancellationToken);
        EnsureSuccess(stats);

        return new GitCommitDetail(commit, parts[5].Trim(), stats.StandardOutput.Trim());
    }

    public async Task<IReadOnlyList<string>> GetConflictsAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(
            repositoryPath,
            ["diff", "--name-only", "--diff-filter=U"],
            cancellationToken);
        EnsureSuccess(result);

        return result.StandardOutput
            .Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries)
            .ToArray();
    }

    public async Task ResolveConflictAsync(
        string repositoryPath,
        string path,
        GitConflictResolution resolution,
        CancellationToken cancellationToken = default)
    {
        var conflicts = await GetConflictsAsync(repositoryPath, cancellationToken);
        if (!conflicts.Contains(path, StringComparer.Ordinal))
        {
            throw new InvalidOperationException("The selected file is not currently conflicted.");
        }

        var option = resolution == GitConflictResolution.KeepOurs ? "--ours" : "--theirs";
        await JournalAsync(
            repositoryPath,
            "resolve-conflict",
            $"{path} using {resolution}",
            false,
            cancellationToken);

        EnsureSuccess(await _runner.RunAsync(
            repositoryPath,
            ["checkout", option, "--", path],
            cancellationToken));

        EnsureSuccess(await _runner.RunAsync(
            repositoryPath,
            ["add", "--", path],
            cancellationToken));
    }

    public async Task MarkConflictResolvedAsync(
        string repositoryPath,
        string path,
        CancellationToken cancellationToken = default)
    {
        var conflicts = await GetConflictsAsync(repositoryPath, cancellationToken);
        if (!conflicts.Contains(path, StringComparer.Ordinal))
        {
            throw new InvalidOperationException("The selected file is not currently conflicted.");
        }

        var fullPath = Path.GetFullPath(Path.Combine(repositoryPath, path));
        var root = Path.GetFullPath(repositoryPath) + Path.DirectorySeparatorChar;
        if (!fullPath.StartsWith(root, StringComparison.OrdinalIgnoreCase))
        {
            throw new InvalidOperationException("Conflict path escapes the repository root.");
        }

        if (File.Exists(fullPath))
        {
            var info = new FileInfo(fullPath);
            if (info.Length > 5 * 1024 * 1024)
            {
                throw new InvalidOperationException(
                    "Large conflicted files must be resolved with Keep ours/Keep theirs or staged explicitly after external verification.");
            }

            var text = await File.ReadAllTextAsync(fullPath, cancellationToken);
            var hasMarkers = text
                .Split(['\r', '\n'], StringSplitOptions.None)
                .Any(line =>
                    line.StartsWith("<<<<<<<", StringComparison.Ordinal) ||
                    line.StartsWith(">>>>>>>", StringComparison.Ordinal) ||
                    string.Equals(line.Trim(), "=======", StringComparison.Ordinal));

            if (hasMarkers)
            {
                throw new InvalidOperationException(
                    "Conflict markers are still present. Resolve them before marking the file resolved.");
            }
        }

        await JournalAsync(repositoryPath, "mark-conflict-resolved", path, false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["add", "--", path], cancellationToken));
    }

    public async Task RebaseAsync(
        string repositoryPath,
        string target,
        CancellationToken cancellationToken = default)
    {
        await EnsureCleanForHistoryChangeAsync(repositoryPath, cancellationToken);
        await VerifyCommitAsync(repositoryPath, target, cancellationToken);
        var head = await GetHeadAsync(repositoryPath, cancellationToken);

        await JournalAsync(
            repositoryPath,
            "rebase",
            $"from {head} onto {target}",
            true,
            cancellationToken);

        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["rebase", target], cancellationToken));
    }

    public async Task CherryPickAsync(
        string repositoryPath,
        string commit,
        CancellationToken cancellationToken = default)
    {
        await EnsureCleanForHistoryChangeAsync(repositoryPath, cancellationToken);
        await VerifyCommitAsync(repositoryPath, commit, cancellationToken);
        var head = await GetHeadAsync(repositoryPath, cancellationToken);

        await JournalAsync(
            repositoryPath,
            "cherry-pick",
            $"from {head}; commit {commit}",
            true,
            cancellationToken);

        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["cherry-pick", commit], cancellationToken));
    }

    public async Task ResetAsync(
        string repositoryPath,
        string target,
        GitResetMode mode,
        CancellationToken cancellationToken = default)
    {
        var status = await GetStatusAsync(repositoryPath, cancellationToken);
        if (status.Operation is not null)
        {
            throw new InvalidOperationException(
                "Finish or abort the current Git operation before resetting.");
        }

        if (mode == GitResetMode.Hard && !status.IsClean)
        {
            throw new InvalidOperationException(
                "Hard reset is blocked while the working tree is dirty. Commit or stash first.");
        }

        await VerifyCommitAsync(repositoryPath, target, cancellationToken);
        var head = await GetHeadAsync(repositoryPath, cancellationToken);
        var modeArgument = mode switch
        {
            GitResetMode.Soft => "--soft",
            GitResetMode.Mixed => "--mixed",
            GitResetMode.Hard => "--hard",
            _ => throw new ArgumentOutOfRangeException(nameof(mode))
        };

        await JournalAsync(
            repositoryPath,
            "reset",
            $"{mode} from {head} to {target}",
            true,
            cancellationToken);

        EnsureSuccess(await _runner.RunAsync(
            repositoryPath,
            ["reset", modeArgument, target],
            cancellationToken));
    }

    public async Task CreateRecoveryBranchAsync(
        string repositoryPath,
        string reference,
        string branchName,
        CancellationToken cancellationToken = default)
    {
        EnsureSuccess(await _runner.RunAsync(
            repositoryPath,
            ["check-ref-format", "--branch", branchName],
            cancellationToken));
        await VerifyCommitAsync(repositoryPath, reference, cancellationToken);

        await JournalAsync(
            repositoryPath,
            "recovery-branch",
            $"{branchName} from {reference}",
            false,
            cancellationToken);

        EnsureSuccess(await _runner.RunAsync(
            repositoryPath,
            ["branch", branchName, reference],
            cancellationToken));
    }

    public async Task<IReadOnlyList<GitStash>> GetStashesAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(
            repositoryPath,
            ["stash", "list", "--date=iso-strict", "--pretty=format:%gd%x1f%gs%x1f%ci"],
            cancellationToken);
        EnsureSuccess(result);

        return result.StandardOutput
            .Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries)
            .Select(line =>
            {
                var parts = line.Split((char)0x1f);
                return new GitStash(
                    parts.ElementAtOrDefault(0) ?? string.Empty,
                    parts.ElementAtOrDefault(1) ?? string.Empty,
                    TryParseDate(parts.ElementAtOrDefault(2)));
            })
            .ToArray();
    }

    public async Task StashPushAsync(
        string repositoryPath,
        string? message,
        bool includeUntracked,
        CancellationToken cancellationToken = default)
    {
        var arguments = new List<string> { "stash", "push" };
        if (includeUntracked)
        {
            arguments.Add("--include-untracked");
        }

        if (!string.IsNullOrWhiteSpace(message))
        {
            arguments.AddRange(["-m", message.Trim()]);
        }

        await JournalAsync(repositoryPath, "stash-push", message ?? "stash", false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, arguments, cancellationToken));
    }

    public async Task StashApplyAsync(
        string repositoryPath,
        string reference,
        bool pop,
        CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, pop ? "stash-pop" : "stash-apply", reference, false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["stash", pop ? "pop" : "apply", reference], cancellationToken));
    }

    public async Task StashDropAsync(
        string repositoryPath,
        string reference,
        CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "stash-drop", reference, true, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["stash", "drop", reference], cancellationToken));
    }

    public async Task<IReadOnlyList<GitWorktree>> GetWorktreesAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        var result = await _runner.RunAsync(repositoryPath, ["worktree", "list", "--porcelain"], cancellationToken);
        EnsureSuccess(result);

        var items = new List<GitWorktree>();
        string? path = null;
        string? head = null;
        string? branch = null;
        var locked = false;
        var prunable = false;

        void Flush()
        {
            if (path is null)
            {
                return;
            }

            items.Add(new GitWorktree(path, head, branch, locked, prunable));
            path = null;
            head = null;
            branch = null;
            locked = false;
            prunable = false;
        }

        foreach (var line in result.StandardOutput.Split(['\r', '\n']))
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                Flush();
                continue;
            }

            if (line.StartsWith("worktree ", StringComparison.Ordinal))
            {
                Flush();
                path = line["worktree ".Length..];
            }
            else if (line.StartsWith("HEAD ", StringComparison.Ordinal))
            {
                head = line["HEAD ".Length..];
            }
            else if (line.StartsWith("branch ", StringComparison.Ordinal))
            {
                branch = line["branch ".Length..].Replace("refs/heads/", string.Empty, StringComparison.Ordinal);
            }
            else if (line.StartsWith("locked", StringComparison.Ordinal))
            {
                locked = true;
            }
            else if (line.StartsWith("prunable", StringComparison.Ordinal))
            {
                prunable = true;
            }
        }

        Flush();
        return items;
    }

    public async Task CreateWorktreeAsync(
        string repositoryPath,
        string path,
        string branch,
        CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "worktree-add", $"{path} -> {branch}", false, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["worktree", "add", path, branch], cancellationToken));
    }

    public async Task RemoveWorktreeAsync(
        string repositoryPath,
        string path,
        CancellationToken cancellationToken = default)
    {
        await JournalAsync(repositoryPath, "worktree-remove", path, true, cancellationToken);
        EnsureSuccess(await _runner.RunAsync(repositoryPath, ["worktree", "remove", path], cancellationToken));
    }

    public async Task<IReadOnlyList<GitReflogEntry>> GetReflogAsync(
        string repositoryPath,
        int take = 100,
        CancellationToken cancellationToken = default)
    {
        take = Math.Clamp(take, 1, 500);
        var result = await _runner.RunAsync(
            repositoryPath,
            ["reflog", $"-n{take}", "--date=iso-strict", "--format=%gd%x1f%H%x1f%gs%x1f%ci"],
            cancellationToken);
        EnsureSuccess(result);

        return result.StandardOutput
            .Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries)
            .Select(line =>
            {
                var parts = line.Split((char)0x1f);
                return new GitReflogEntry(
                    parts.ElementAtOrDefault(0) ?? string.Empty,
                    parts.ElementAtOrDefault(1) ?? string.Empty,
                    parts.ElementAtOrDefault(2) ?? string.Empty,
                    TryParseDate(parts.ElementAtOrDefault(3)));
            })
            .ToArray();
    }

    public async Task AbortCurrentOperationAsync(
        string repositoryPath,
        CancellationToken cancellationToken = default)
    {
        var gitDirectory = ResolveGitDirectory(repositoryPath);

        if (File.Exists(Path.Combine(gitDirectory, "MERGE_HEAD")))
        {
            await JournalAsync(repositoryPath, "merge-abort", "abort in-progress merge", false, cancellationToken);
            EnsureSuccess(await _runner.RunAsync(repositoryPath, ["merge", "--abort"], cancellationToken));
            return;
        }

        if (Directory.Exists(Path.Combine(gitDirectory, "rebase-merge")) ||
            Directory.Exists(Path.Combine(gitDirectory, "rebase-apply")))
        {
            await JournalAsync(repositoryPath, "rebase-abort", "abort in-progress rebase", false, cancellationToken);
            EnsureSuccess(await _runner.RunAsync(repositoryPath, ["rebase", "--abort"], cancellationToken));
            return;
        }

        if (File.Exists(Path.Combine(gitDirectory, "CHERRY_PICK_HEAD")))
        {
            await JournalAsync(repositoryPath, "cherry-pick-abort", "abort in-progress cherry-pick", false, cancellationToken);
            EnsureSuccess(await _runner.RunAsync(repositoryPath, ["cherry-pick", "--abort"], cancellationToken));
            return;
        }

        if (File.Exists(Path.Combine(gitDirectory, "REVERT_HEAD")))
        {
            await JournalAsync(repositoryPath, "revert-abort", "abort in-progress revert", false, cancellationToken);
            EnsureSuccess(await _runner.RunAsync(repositoryPath, ["revert", "--abort"], cancellationToken));
            return;
        }

        throw new InvalidOperationException("No supported Git operation is currently in progress.");
    }

    private async Task EnsureCleanForHistoryChangeAsync(
        string repositoryPath,
        CancellationToken cancellationToken)
    {
        var status = await GetStatusAsync(repositoryPath, cancellationToken);
        if (status.Operation is not null)
        {
            throw new InvalidOperationException(
                "Finish or abort the current Git operation before changing history.");
        }

        if (!status.IsClean)
        {
            throw new InvalidOperationException(
                "Commit or stash changes before changing history.");
        }
    }

    private async Task VerifyCommitAsync(
        string repositoryPath,
        string reference,
        CancellationToken cancellationToken)
    {
        if (string.IsNullOrWhiteSpace(reference))
        {
            throw new InvalidOperationException("A Git commit or reference is required.");
        }

        var result = await _runner.RunAsync(
            repositoryPath,
            ["rev-parse", "--verify", "--quiet", $"{reference}^{{commit}}"],
            cancellationToken);

        if (result.ExitCode != 0)
        {
            throw new InvalidOperationException(
                $"Git reference '{reference}' does not resolve to a commit.");
        }
    }

    private async Task<string> GetHeadAsync(
        string repositoryPath,
        CancellationToken cancellationToken)
    {
        var result = await _runner.RunAsync(
            repositoryPath,
            ["rev-parse", "HEAD"],
            cancellationToken);
        EnsureSuccess(result);
        return result.StandardOutput.Trim();
    }

    private async Task<(RepositoryStatus Status, IReadOnlyList<GitChange> Changes)> ReadStatusAsync(
        string repositoryPath,
        CancellationToken cancellationToken)
    {
        EnsureDirectory(repositoryPath);
        var result = await _runner.RunAsync(
            repositoryPath,
            ["status", "--porcelain=v2", "--branch"],
            cancellationToken);
        EnsureSuccess(result);
        return PorcelainV2Parser.Parse(result.StandardOutput);
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

    private static string ResolveGitDirectory(string repositoryPath)
    {
        var dotGit = Path.Combine(repositoryPath, ".git");
        if (Directory.Exists(dotGit))
        {
            return dotGit;
        }

        if (File.Exists(dotGit))
        {
            var text = File.ReadAllText(dotGit).Trim();
            const string prefix = "gitdir:";
            if (text.StartsWith(prefix, StringComparison.OrdinalIgnoreCase))
            {
                var raw = text[prefix.Length..].Trim();
                return Path.GetFullPath(Path.Combine(repositoryPath, raw));
            }
        }

        return dotGit;
    }

    private static string? DetectOperation(string repositoryPath)
    {
        var gitDirectory = ResolveGitDirectory(repositoryPath);
        if (File.Exists(Path.Combine(gitDirectory, "MERGE_HEAD")))
        {
            return "merge";
        }

        if (Directory.Exists(Path.Combine(gitDirectory, "rebase-merge")) ||
            Directory.Exists(Path.Combine(gitDirectory, "rebase-apply")))
        {
            return "rebase";
        }

        if (File.Exists(Path.Combine(gitDirectory, "CHERRY_PICK_HEAD")))
        {
            return "cherry-pick";
        }

        if (File.Exists(Path.Combine(gitDirectory, "REVERT_HEAD")))
        {
            return "revert";
        }

        return null;
    }

    private static GitCommit ParseCommit(string line)
    {
        var parts = line.Split((char)0x1f);
        if (parts.Length < 5)
        {
            throw new InvalidOperationException("Git returned an unexpected history format.");
        }

        return new GitCommit(parts[0], parts[1], parts[2], ParseDate(parts[3]), parts[4]);
    }

    private static DateTimeOffset ParseDate(string value) =>
        DateTimeOffset.TryParse(
            value,
            CultureInfo.InvariantCulture,
            DateTimeStyles.RoundtripKind,
            out var parsed)
            ? parsed
            : DateTimeOffset.MinValue;

    private static DateTimeOffset? TryParseDate(string? value) =>
        string.IsNullOrWhiteSpace(value) ? null : ParseDate(value);

    private static int ParseTrackCount(Regex regex, string value)
    {
        var match = regex.Match(value);
        return match.Success && int.TryParse(match.Groups[1].Value, out var count) ? count : 0;
    }

    private static void EnsureDirectory(string repositoryPath)
    {
        if (!Directory.Exists(repositoryPath))
        {
            throw new DirectoryNotFoundException(repositoryPath);
        }
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
