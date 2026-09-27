using System.Diagnostics;
using System.Text.RegularExpressions;

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

        startInfo.Environment["GIT_TERMINAL_PROMPT"] = "0";

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

        try
        {
            await process.WaitForExitAsync(cancellationToken);
        }
        catch (OperationCanceledException)
        {
            if (!process.HasExited)
            {
                process.Kill(entireProcessTree: true);
            }

            throw;
        }

        return new GitCommandResult(
            process.ExitCode,
            await outputTask,
            Sanitize(await errorTask));
    }

    private static string Sanitize(string value)
    {
        if (string.IsNullOrWhiteSpace(value))
        {
            return value;
        }

        var sanitized = Regex.Replace(
            value,
            @"https://[^\s/@:]+:[^\s/@]+@",
            "https://[REDACTED]@",
            RegexOptions.CultureInvariant);

        return Regex.Replace(
            sanitized,
            @"(?:ghp|github_pat)_[A-Za-z0-9_]+",
            "[REDACTED]",
            RegexOptions.CultureInvariant);
    }
}
