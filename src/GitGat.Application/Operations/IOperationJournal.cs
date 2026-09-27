namespace GitGat.Application.Operations;

public sealed record OperationRecord(
    DateTimeOffset TimestampUtc,
    string RepositoryPath,
    string Operation,
    string Detail,
    bool Destructive);

public interface IOperationJournal
{
    Task RecordAsync(OperationRecord record, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<OperationRecord>> ReadRecentAsync(int take = 100, CancellationToken cancellationToken = default);
}
