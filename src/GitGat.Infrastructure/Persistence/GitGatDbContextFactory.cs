using Microsoft.EntityFrameworkCore;

namespace GitGat.Infrastructure.Persistence;

public sealed class GitGatDbContextFactory
{
    private readonly DbContextOptions<GitGatDbContext> _options;

    public GitGatDbContextFactory(string? databasePath = null)
    {
        if (string.IsNullOrWhiteSpace(databasePath))
        {
            var root = Path.Combine(
                Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
                "GitGat");

            Directory.CreateDirectory(root);
            databasePath = Path.Combine(root, "gitgat.db");
        }
        else
        {
            var directory = Path.GetDirectoryName(Path.GetFullPath(databasePath));
            if (!string.IsNullOrWhiteSpace(directory))
            {
                Directory.CreateDirectory(directory);
            }
        }

        _options = new DbContextOptionsBuilder<GitGatDbContext>()
            .UseSqlite($"Data Source={databasePath}")
            .Options;

        using var database = Create();
        database.Database.EnsureCreated();
    }

    internal GitGatDbContext Create() => new(_options);
}
