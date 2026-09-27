#ifndef MyAppVersion
  #define MyAppVersion "0.1.0"
#endif

[Setup]
AppId={{B2288A6E-58D6-4B89-84EC-8E1CC899C766}
AppName=GitGat
AppVersion={#MyAppVersion}
AppPublisher=GitGat
DefaultDirName={autopf}\GitGat
DefaultGroupName=GitGat
OutputDir=..\..\dist\installer
OutputBaseFilename=GitGat-Setup-x64
Compression=lzma2
SolidCompression=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
WizardStyle=modern
UninstallDisplayName=GitGat
SetupLogging=yes

[Files]
Source: "..\..\dist\portable\GitGat.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\GitGat"; Filename: "{app}\GitGat.exe"
Name: "{userdesktop}\GitGat"; Filename: "{app}\GitGat.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"

[Run]
Filename: "{app}\GitGat.exe"; Description: "Launch GitGat"; Flags: nowait postinstall skipifsilent
