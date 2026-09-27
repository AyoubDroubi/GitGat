namespace GitGat.Application.Git;

public sealed record RepositoryStatus(
    string Branch,
    int ChangedFiles,
    int UntrackedFiles,
    int Ahead,
    int Behind)
{
    public bool IsClean => ChangedFiles == 0 && UntrackedFiles == 0;
}
