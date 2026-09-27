using Microsoft.EntityFrameworkCore;

namespace GitGat.Infrastructure.Persistence;

internal sealed class GitGatDbContext(DbContextOptions<GitGatDbContext> options) : DbContext(options)
{
    public DbSet<RepositoryRow> Repositories => Set<RepositoryRow>();
    public DbSet<WorkspaceRow> Workspaces => Set<WorkspaceRow>();
    public DbSet<WorkspaceRepositoryRow> WorkspaceRepositories => Set<WorkspaceRepositoryRow>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<RepositoryRow>(entity =>
        {
            entity.HasKey(item => item.Id);
            entity.Property(item => item.Name).IsRequired().HasMaxLength(260);
            entity.Property(item => item.LocalPath).IsRequired().UseCollation("NOCASE");
            entity.HasIndex(item => item.LocalPath).IsUnique();
            entity.Property(item => item.RemoteUrl).HasMaxLength(2048);
        });

        modelBuilder.Entity<WorkspaceRow>(entity =>
        {
            entity.HasKey(item => item.Id);
            entity.Property(item => item.Name).IsRequired().HasMaxLength(160);
        });

        modelBuilder.Entity<WorkspaceRepositoryRow>(entity =>
        {
            entity.HasKey(item => new { item.WorkspaceId, item.RepositoryId });
            entity.HasOne<WorkspaceRow>()
                .WithMany()
                .HasForeignKey(item => item.WorkspaceId)
                .OnDelete(DeleteBehavior.Cascade);
            entity.HasOne<RepositoryRow>()
                .WithMany()
                .HasForeignKey(item => item.RepositoryId)
                .OnDelete(DeleteBehavior.Cascade);
        });
    }
}

internal sealed class RepositoryRow
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string LocalPath { get; set; } = string.Empty;
    public string? RemoteUrl { get; set; }
    public bool IsFavorite { get; set; }
    public DateTimeOffset LastOpenedUtc { get; set; }
}

internal sealed class WorkspaceRow
{
    public Guid Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public DateTimeOffset UpdatedUtc { get; set; }
}

internal sealed class WorkspaceRepositoryRow
{
    public Guid WorkspaceId { get; set; }
    public Guid RepositoryId { get; set; }
}
