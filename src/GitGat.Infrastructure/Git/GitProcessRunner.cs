using System.Diagnostics;

namespace GitGat.Infrastructure.Git;

internal sealed class GitProcessRunner
{
    public async Task<GitCommandResult> RunAsync(
        string workingDirectory,
        IEnumerable<string> arguments,
        CancellationToken cancellationToken = default)
    {
        var startInfo = new ProcessStartInfo
        {
            FileName = "git",
            WorkingDirectory = workingDirectory,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true
        };

        foreach (var argument in arguments)
        {
            startInfo.ArgumentList.Add(argument);
        }

        using var process = new Process { StartInfo = startInfo };

        try
        {
            if (!process.Start())
            {
                throw new InvalidOperationException("Git process could not be started.");
            }
        }
        catch (Exception exception) when (exception is System.ComponentModel.Win32Exception or InvalidOperationException)
        {
            throw new InvalidOperationException(
                "Git is not available. Install Git and ensure it is available on PATH.",
                exception);
        }

        var outputTask = process.StandardOutput.ReadToEndAsync(cancellationToken);
        var errorTask = process.StandardError.ReadToEndAsync(cancellationToken);

        await process.WaitForExitAsync(cancellationToken);

        return new GitCommandResult(
            process.ExitCode,
            await outputTask,
            await errorTask);
    }
}
