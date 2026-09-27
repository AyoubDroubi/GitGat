namespace GitGat.Infrastructure.Git;

internal sealed record GitCommandResult(
    int ExitCode,
    string StandardOutput,
    string StandardError);
