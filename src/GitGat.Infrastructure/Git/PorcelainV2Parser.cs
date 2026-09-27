using GitGat.Application.Git;

namespace GitGat.Infrastructure.Git;

public static class PorcelainV2Parser
{
    public static (RepositoryStatus Status, IReadOnlyList<GitChange> Changes) Parse(string output)
    {
        var branch = "(detached)";
        string? upstream = null;
        var ahead = 0;
        var behind = 0;
        var changes = new List<GitChange>();

        foreach (var rawLine in output.Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries))
        {
            var line = rawLine.TrimEnd();
            if (line.StartsWith("# branch.head ", StringComparison.Ordinal))
            {
                branch = line["# branch.head ".Length..].Trim();
                continue;
            }

            if (line.StartsWith("# branch.upstream ", StringComparison.Ordinal))
            {
                upstream = line["# branch.upstream ".Length..].Trim();
                continue;
            }

            if (line.StartsWith("# branch.ab ", StringComparison.Ordinal))
            {
                var parts = line["# branch.ab ".Length..].Split(' ', StringSplitOptions.RemoveEmptyEntries);
                foreach (var part in parts)
                {
                    if (part.StartsWith('+') && int.TryParse(part.AsSpan(1), out var parsedAhead))
                    {
                        ahead = parsedAhead;
                    }
                    else if (part.StartsWith('-') && int.TryParse(part.AsSpan(1), out var parsedBehind))
                    {
                        behind = parsedBehind;
                    }
                }

                continue;
            }

            if (line.StartsWith("? ", StringComparison.Ordinal))
            {
                changes.Add(new GitChange(
                    line[2..],
                    null,
                    GitChangeKind.Untracked,
                    false,
                    true,
                    true,
                    false));
                continue;
            }

            if (line.Length < 4 || line[1] != ' ')
            {
                continue;
            }

            var recordType = line[0];
            var maxParts = recordType == '2' ? 10 : recordType == 'u' ? 11 : 9;
            var partsForChange = line.Split(' ', maxParts, StringSplitOptions.None);
            if (partsForChange.Length < maxParts)
            {
                continue;
            }

            var xy = partsForChange[1];
            var pathField = partsForChange[^1];
            string path;
            string? originalPath = null;

            if (recordType == '2')
            {
                var tab = pathField.IndexOf('\t');
                if (tab >= 0)
                {
                    path = pathField[..tab];
                    originalPath = pathField[(tab + 1)..];
                }
                else
                {
                    path = pathField;
                }
            }
            else
            {
                path = pathField;
            }

            var conflicted = recordType == 'u' || xy.Contains('U');
            var staged = xy.Length > 0 && xy[0] is not '.' and not '?';
            var unstaged = xy.Length > 1 && xy[1] is not '.';

            changes.Add(new GitChange(
                path,
                originalPath,
                ToKind(xy, recordType),
                staged,
                unstaged,
                false,
                conflicted));
        }

        var untracked = changes.Count(item => item.IsUntracked);
        var changed = changes.Count - untracked;
        var status = new RepositoryStatus(
            branch,
            changed,
            untracked,
            ahead,
            behind,
            upstream,
            changes.Any(item => item.IsConflicted));

        return (status, changes);
    }

    private static GitChangeKind ToKind(string xy, char recordType)
    {
        if (recordType == 'u' || xy.Contains('U'))
        {
            return GitChangeKind.Conflicted;
        }

        if (xy.Contains('R'))
        {
            return GitChangeKind.Renamed;
        }

        if (xy.Contains('C'))
        {
            return GitChangeKind.Copied;
        }

        if (xy.Contains('A'))
        {
            return GitChangeKind.Added;
        }

        if (xy.Contains('D'))
        {
            return GitChangeKind.Deleted;
        }

        if (xy.Contains('M'))
        {
            return GitChangeKind.Modified;
        }

        return GitChangeKind.Changed;
    }
}
