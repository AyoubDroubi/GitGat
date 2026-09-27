using System.Diagnostics;
using System.Text.RegularExpressions;

namespace GitGat.Infrastructure.Forge;

internal sealed class GhProcessRunner
{
    public async Task<(int ExitCode, string Output, string Error)> RunAsync(
        IEnumerable<string> arguments,
        CancellationToken cancellationToken = default)
    {
        var startInfo = new ProcessStartInfo
        {
            FileName = "gh",
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true
        };

        startInfo.Environment["GH_PROMPT_DISABLED"] = "1";

        foreach (var argument in arguments)
        {
            startInfo.ArgumentList.Add(argument);
        }

        using var process = new Process { StartInfo = startInfo };

        try
        {
            if (!process.Start())
            {
                return (-1, string.Empty, "GitHub CLI could not be started.");
            }
        }
        catch (System.ComponentModel.Win32Exception)
        {
            return (-1, string.Empty, "GitHub CLI (gh) is not installed or is not available on PATH.");
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

        return (
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
