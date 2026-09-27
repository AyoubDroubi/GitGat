using System.Text.Json;
using GitGat.Application.Operations;

namespace GitGat.Infrastructure.Operations;

public sealed class FileOperationJournal : IOperationJournal, IDisposable
{
    private readonly SemaphoreSlim _gate = new(1, 1);
    private readonly string _filePath;

    public FileOperationJournal()
    {
        var root = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),
            "GitGat");

        Directory.CreateDirectory(root);
        _filePath = Path.Combine(root, "operations.jsonl");
    }

    public async Task RecordAsync(
        OperationRecord record,
        CancellationToken cancellationToken = default)
    {
        var line = JsonSerializer.Serialize(record) + Environment.NewLine;
        await _gate.WaitAsync(cancellationToken);
        try
        {
            await File.AppendAllTextAsync(_filePath, line, cancellationToken);
        }
        finally
        {
            _gate.Release();
        }
    }

    public async Task<IReadOnlyList<OperationRecord>> ReadRecentAsync(
        int take = 100,
        CancellationToken cancellationToken = default)
    {
        take = Math.Clamp(take, 1, 1000);
        if (!File.Exists(_filePath))
        {
            return [];
        }

        await _gate.WaitAsync(cancellationToken);
        try
        {
            var lines = await File.ReadAllLinesAsync(_filePath, cancellationToken);
            return lines
                .Reverse()
                .Take(take)
                .Select(line => JsonSerializer.Deserialize<OperationRecord>(line))
                .Where(record => record is not null)
                .Select(record => record!)
                .ToArray();
        }
        finally
        {
            _gate.Release();
        }
    }

    public void Dispose()
    {
        _gate.Dispose();
    }
}
