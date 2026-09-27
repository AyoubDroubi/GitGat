using Microsoft.EntityFrameworkCore;

namespace GitGat.Infrastructure.Persistence;

public sealed class GitGatDbContextFactory
{
    private readonly DbContextOptions<GitGatDbContext> _options;

    public GitGatDbContextFactory()
    {
        var root = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
            "GitGat");

        Directory.CreateDirectory(root);
        var databasePath = Path.Combine(root, "gitgat.db");

        _options = new DbContextOptionsBuilder<GitGatDbContext>()
            .UseSqlite($"Data Source={databasePath}")
            .Options;

        using var database = Create();
        database.Database.EnsureCreated();
    }

    internal GitGatDbContext Create() => new(_options);
}
