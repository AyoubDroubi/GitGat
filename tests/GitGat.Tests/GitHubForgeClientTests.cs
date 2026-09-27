using GitGat.Infrastructure.Forge;

namespace GitGat.Tests;

public sealed class GitHubForgeClientTests
{
    private readonly GitHubForgeClient _client = new();

    [Theory]
    [InlineData("https://github.com/AyoubDroubi/GitGat.git", "AyoubDroubi", "GitGat")]
    [InlineData("https://github.com/AyoubDroubi/GitGat", "AyoubDroubi", "GitGat")]
    [InlineData("git@github.com:AyoubDroubi/GitGat.git", "AyoubDroubi", "GitGat")]
    [InlineData("ssh://git@github.com/AyoubDroubi/GitGat.git", "AyoubDroubi", "GitGat")]
    public void ResolveRepository_HandlesCommonGitHubRemotes(
        string remote,
        string expectedOwner,
        string expectedName)
    {
        var repository = _client.ResolveRepository(remote);

        Assert.NotNull(repository);
        Assert.Equal(expectedOwner, repository.Owner);
        Assert.Equal(expectedName, repository.Name);
    }

    [Fact]
    public void ResolveRepository_RejectsOtherHosts()
    {
        Assert.Null(_client.ResolveRepository("https://gitlab.com/example/project.git"));
    }
}
