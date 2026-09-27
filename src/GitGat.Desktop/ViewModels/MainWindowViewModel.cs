using System.Collections.ObjectModel;
using CommunityToolkit.Mvvm.ComponentModel;
using GitGat.Application.Git;

namespace GitGat.Desktop.ViewModels;

public partial class MainWindowViewModel : ObservableObject
{
    private readonly IGitClient _gitClient;

    [ObservableProperty]
    private string _gitVersion = "Checking Git…";

    [ObservableProperty]
    private string _activeSection = "Overview";

    public MainWindowViewModel(IGitClient gitClient)
    {
        _gitClient = gitClient;

        PrimaryNavigation =
        [
            new("Overview"),
            new("My Work"),
            new("Pull Requests", "0"),
            new("Reviews", "0"),
            new("Notifications", "0")
        ];

        RepositoryNavigation =
        [
            new("Files"),
            new("Changes", "0"),
            new("Branches"),
            new("History"),
            new("Locks", "0"),
            new("Actions")
        ];
    }

    public ObservableCollection<NavigationItemViewModel> PrimaryNavigation { get; }

    public ObservableCollection<NavigationItemViewModel> RepositoryNavigation { get; }

    public async Task InitializeAsync()
    {
        try
        {
            GitVersion = await _gitClient.GetVersionAsync();
        }
        catch (Exception exception)
        {
            GitVersion = exception.Message;
        }
    }
}
