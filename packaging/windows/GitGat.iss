#ifndef MyAppVersion
  #define MyAppVersion "0.1.0"
#endif

#ifndef PublishDir
  #define PublishDir "..\..\artifacts\publish\win-x64"
#endif

#ifndef InstallerDir
  #define InstallerDir "..\..\artifacts\installer"
#endif

#define MyAppName "GitGat"
#define MyAppPublisher "GitGat"
#define MyAppExeName "GitGat.exe"
#define IconFile "..\..\src\GitGat.Desktop\Assets\gitgat.ico"

[Setup]
AppId={{C6A1265E-7351-4E9E-81F7-A0D43BA3B235}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={localappdata}\Programs\GitGat
DefaultGroupName=GitGat
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
OutputDir={#InstallerDir}
OutputBaseFilename=GitGat-Setup-x64
SetupIconFile={#IconFile}
UninstallDisplayIcon={app}\{#MyAppExeName}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
CloseApplications=yes
RestartApplications=no
VersionInfoVersion={#MyAppVersion}.0
VersionInfoProductName={#MyAppName}
VersionInfoCompany={#MyAppPublisher}

[Files]
Source: "{#PublishDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\GitGat"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\GitGat"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional shortcuts:"

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch GitGat"; Flags: nowait postinstall skipifsilent
