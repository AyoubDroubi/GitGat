using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using GitGat.Application.Forge;
using GitGat.Application.Git;
using GitGat.Application.Operations;
using GitGat.Application.Repositories;
using GitGat.Application.Workspaces;
using GitGat.Desktop.ViewModels;
using GitGat.Desktop.Views;
using GitGat.Infrastructure.Forge;
using GitGat.Infrastructure.Git;
using GitGat.Infrastructure.Operations;
using GitGat.Infrastructure.Persistence;
using GitGat.Infrastructure.Repositories;
using GitGat.Infrastructure.Workspaces;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

namespace GitGat.Desktop;

public partial class App : Avalonia.Application
{
    private IHost? _host;

    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }

    public override void OnFrameworkInitializationCompleted()
    {
        var builder = Host.CreateApplicationBuilder();

        builder.Services.AddSingleton<GitGatDbContextFactory>();
        builder.Services.AddSingleton<IOperationJournal, FileOperationJournal>();
        builder.Services.AddSingleton<IGitClient, SystemGitClient>();
        builder.Services.AddSingleton<IGitLfsClient, SystemGitLfsClient>();
        builder.Services.AddSingleton<IForgeClient, GitHubForgeClient>();
        builder.Services.AddSingleton<IRepositoryCatalog, SqliteRepositoryCatalog>();
        builder.Services.AddSingleton<IWorkspaceCatalog, SqliteWorkspaceCatalog>();
        builder.Services.AddSingleton<MainWindowViewModel>();
        builder.Services.AddSingleton<MainWindow>();

        _host = builder.Build();
        _host.Start();

        if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        {
            var viewModel = _host.Services.GetRequiredService<MainWindowViewModel>();
            var window = _host.Services.GetRequiredService<MainWindow>();

            window.DataContext = viewModel;
            desktop.MainWindow = window;
            desktop.Exit += (_, _) => _host.Dispose();

            _ = viewModel.InitializeAsync();
        }

        base.OnFrameworkInitializationCompleted();
    }
}
