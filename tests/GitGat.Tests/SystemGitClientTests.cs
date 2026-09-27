using System.Diagnostics;
using GitGat.Application.Git;
using GitGat.Infrastructure.Git;

namespace GitGat.Tests;

public sealed class SystemGitClientTests
{
    [Fact]
    public async Task DailyWorkflow_StatusStageCommitBranchAndHistory_AreConsistent()
    {
        using var repository = new TemporaryGitRepository();
        repository.Write("README.md", "# GitGat");
        repository.Run("add", "README.md");
        repository.Run("commit", "-m", "initial");

        var client = new SystemGitClient();

        repository.Write("README.md", "# GitGat\n\nChanged");

        var changedStatus = await client.GetStatusAsync(repository.Path);
        Assert.Equal(1, changedStatus.ChangedFiles);
        Assert.False(changedStatus.IsClean);

        var changes = await client.GetChangesAsync(repository.Path);
        var readme = Assert.Single(changes);
        Assert.Equal("README.md", readme.Path);
        Assert.True(readme.IsUnstaged);

        await client.StageAsync(repository.Path, ["README.md"]);
        changes = await client.GetChangesAsync(repository.Path);
        readme = Assert.Single(changes);
        Assert.True(readme.IsStaged);

        await client.CommitAsync(repository.Path, "update readme");
        Assert.True((await client.GetStatusAsync(repository.Path)).IsClean);

        await client.CreateBranchAsync(repository.Path, "feature/test", checkout: true);
        Assert.Equal("feature/test", (await client.GetStatusAsync(repository.Path)).Branch);

        var history = await client.GetHistoryAsync(repository.Path);
        Assert.Contains(history, item => item.Subject == "update readme");
        Assert.Contains(history, item => item.Subject == "initial");
    }

    [Fact]
    public async Task BranchSwitch_IsBlockedWhenWorkingTreeIsDirty()
    {
        using var repository = new TemporaryGitRepository();
        repository.Write("file.txt", "base");
        repository.Run("add", "file.txt");
        repository.Run("commit", "-m", "initial");
        repository.Run("branch", "other");
        repository.Write("file.txt", "dirty");

        var client = new SystemGitClient();

        var exception = await Assert.ThrowsAsync<InvalidOperationException>(
            () => client.CheckoutBranchAsync(repository.Path, "other"));

        Assert.Contains("Commit or stash", exception.Message);
    }

    [Fact]
    public async Task DiscardTracked_DoesNotDeleteUntrackedFiles()
    {
        using var repository = new TemporaryGitRepository();
        repository.Write("tracked.txt", "base");
        repository.Run("add", "tracked.txt");
        repository.Run("commit", "-m", "initial");
        repository.Write("untracked.txt", "keep me");

        var client = new SystemGitClient();

        var exception = await Assert.ThrowsAsync<InvalidOperationException>(
            () => client.DiscardTrackedAsync(repository.Path, "untracked.txt"));

        Assert.Contains("never discards untracked files", exception.Message);
        Assert.True(File.Exists(Path.Combine(repository.Path, "untracked.txt")));
    }

    private sealed class TemporaryGitRepository : IDisposable
    {
        public TemporaryGitRepository()
        {
            Path = System.IO.Path.Combine(
                System.IO.Path.GetTempPath(),
                "GitGat.Tests",
                Guid.NewGuid().ToString("N"));

            Directory.CreateDirectory(Path);
            Run("init", "-b", "main");
            Run("config", "user.name", "GitGat Tests");
            Run("config", "user.email", "gitgat-tests@example.invalid");
        }

        public string Path { get; }

        public void Write(string relativePath, string content)
        {
            var fullPath = System.IO.Path.Combine(Path, relativePath);
            var directory = System.IO.Path.GetDirectoryName(fullPath);
            if (!string.IsNullOrWhiteSpace(directory))
            {
                Directory.CreateDirectory(directory);
            }

            File.WriteAllText(fullPath, content);
        }

        public string Run(params string[] arguments)
        {
            var startInfo = new ProcessStartInfo
            {
                FileName = "git",
                WorkingDirectory = Path,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
                CreateNoWindow = true
            };

            foreach (var argument in arguments)
            {
                startInfo.ArgumentList.Add(argument);
            }

            using var process = Process.Start(startInfo)
                ?? throw new InvalidOperationException("Could not start Git.");

            var output = process.StandardOutput.ReadToEnd();
            var error = process.StandardError.ReadToEnd();
            process.WaitForExit();

            if (process.ExitCode != 0)
            {
                throw new InvalidOperationException(
                    $"git {string.Join(' ', arguments)} failed: {error}");
            }

            return output;
        }

        public void Dispose()
        {
            if (!Directory.Exists(Path))
            {
                return;
            }

            try
            {
                Directory.Delete(Path, recursive: true);
            }
            catch (IOException)
            {
            }
            catch (UnauthorizedAccessException)
            {
            }
        }
    }
}
