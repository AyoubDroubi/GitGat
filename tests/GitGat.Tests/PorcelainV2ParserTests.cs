using GitGat.Application.Git;
using GitGat.Infrastructure.Git;

namespace GitGat.Tests;

public sealed class PorcelainV2ParserTests
{
    [Fact]
    public void Parse_ReadsBranchCountersAndChanges()
    {
        const string output = """
            # branch.oid 0123456789abcdef
            # branch.head main
            # branch.upstream origin/main
            # branch.ab +2 -1
            1 .M N... 100644 100644 100644 0123456 0123456 src/file.cs
            ? notes.txt
            """;

        var (status, changes) = PorcelainV2Parser.Parse(output);

        Assert.Equal("main", status.Branch);
        Assert.Equal("origin/main", status.Upstream);
        Assert.Equal(2, status.Ahead);
        Assert.Equal(1, status.Behind);
        Assert.Equal(1, status.ChangedFiles);
        Assert.Equal(1, status.UntrackedFiles);
        Assert.False(status.IsClean);

        Assert.Contains(changes, item =>
            item.Path == "src/file.cs" &&
            item.Kind == GitChangeKind.Modified &&
            item.IsUnstaged);

        Assert.Contains(changes, item =>
            item.Path == "notes.txt" &&
            item.IsUntracked);
    }

    [Fact]
    public void Parse_RecognizesConflicts()
    {
        const string output = """
            # branch.head feature
            u UU N... 100644 100644 100644 100644 aaaaaaa bbbbbbb ccccccc conflict.txt
            """;

        var (status, changes) = PorcelainV2Parser.Parse(output);

        Assert.True(status.HasConflicts);
        Assert.Single(changes);
        Assert.Equal(GitChangeKind.Conflicted, changes[0].Kind);
    }
}
