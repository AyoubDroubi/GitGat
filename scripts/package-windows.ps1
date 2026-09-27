param(
    [string]$Version = "0.1.0",
    [switch]$SkipTests
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$Project = Join-Path $Root "src/GitGat.Desktop/GitGat.Desktop.csproj"
$PublishDir = Join-Path $Root "artifacts/publish/win-x64"
$InstallerDir = Join-Path $Root "artifacts/installer"
$ChecksumsDir = Join-Path $Root "artifacts/checksums"
$InstallerScript = Join-Path $Root "packaging/windows/GitGat.iss"

Remove-Item $PublishDir -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item $InstallerDir -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item $ChecksumsDir -Recurse -Force -ErrorAction SilentlyContinue
New-Item $PublishDir -ItemType Directory -Force | Out-Null
New-Item $InstallerDir -ItemType Directory -Force | Out-Null
New-Item $ChecksumsDir -ItemType Directory -Force | Out-Null

dotnet restore (Join-Path $Root "GitGat.slnx")
dotnet build (Join-Path $Root "GitGat.slnx") --configuration Release --no-restore

if (-not $SkipTests) {
    dotnet test (Join-Path $Root "GitGat.slnx") --configuration Release --no-build
}

dotnet publish $Project --configuration Release --runtime win-x64 --self-contained true --output $PublishDir /p:PublishProfile=win-x64 /p:Version=$Version /p:ContinuousIntegrationBuild=true

$Exe = Join-Path $PublishDir "GitGat.exe"
if (-not (Test-Path $Exe)) { throw "GitGat.exe was not produced." }

$Unexpected = Get-ChildItem $PublishDir -File | Where-Object { $_.Name -ne "GitGat.exe" }
if ($Unexpected) {
    $names = ($Unexpected | Select-Object -ExpandProperty Name) -join ", "
    throw "Single-file verification failed. Unexpected publish files: $names"
}

function Test-GitGatLaunch([string]$Executable, [string]$Label) {
    $process = Start-Process -FilePath $Executable -PassThru
    Start-Sleep -Seconds 5
    $process.Refresh()

    if ($process.HasExited) {
        if ($process.ExitCode -ne 0) {
            throw "$Label exited with code $($process.ExitCode)."
        }

        Write-Host "$Label started and exited cleanly."
        return
    }

    Stop-Process -Id $process.Id -Force
    Write-Host "$Label launch succeeded."
}

Test-GitGatLaunch $Exe "Portable GitGat"

$ProgramFilesX86 = [Environment]::GetFolderPath("ProgramFilesX86")
$IsccCandidates = @(
    (Join-Path $ProgramFilesX86 "Inno Setup 6\ISCC.exe"),
    (Join-Path $env:ProgramFiles "Inno Setup 6\ISCC.exe")
) | Where-Object { $_ -and (Test-Path $_) }
$Iscc = $IsccCandidates | Select-Object -First 1
if (-not $Iscc) { throw "Inno Setup 6 was not found." }

& $Iscc "/DMyAppVersion=$Version" "/DPublishDir=$PublishDir" "/DInstallerDir=$InstallerDir" $InstallerScript

$Installer = Get-ChildItem $InstallerDir -Filter "*.exe" -File | Select-Object -First 1
if (-not $Installer) { throw "Installer was not produced." }

$SmokeRoot = if ($env:RUNNER_TEMP) { $env:RUNNER_TEMP } else { [IO.Path]::GetTempPath() }
$InstallDir = Join-Path $SmokeRoot "GitGat-Installer-Smoke"
Remove-Item $InstallDir -Recurse -Force -ErrorAction SilentlyContinue

& $Installer.FullName /VERYSILENT /SUPPRESSMSGBOXES /NORESTART "/DIR=$InstallDir"

$InstalledExe = Join-Path $InstallDir "GitGat.exe"
$Uninstaller = Join-Path $InstallDir "unins000.exe"
$StartMenuShortcut = Join-Path $env:APPDATA "Microsoft\Windows\Start Menu\Programs\GitGat.lnk"

if (-not (Test-Path $InstalledExe)) { throw "Installer smoke test: installed GitGat.exe is missing." }
if (-not (Test-Path $Uninstaller)) { throw "Installer smoke test: uninstaller is missing." }
if (-not (Test-Path $StartMenuShortcut)) { throw "Installer smoke test: Start Menu shortcut is missing." }

Test-GitGatLaunch $InstalledExe "Installed GitGat"

& $Uninstaller /VERYSILENT /SUPPRESSMSGBOXES /NORESTART
Start-Sleep -Seconds 2

if (Test-Path $InstalledExe) {
    throw "Installer smoke test: uninstall did not remove GitGat.exe."
}

Remove-Item $StartMenuShortcut -Force -ErrorAction SilentlyContinue
Write-Host "Installer install/start-menu/launch/uninstall smoke test succeeded."

$Hashes = @(
    Get-FileHash $Exe -Algorithm SHA256
    Get-FileHash $Installer.FullName -Algorithm SHA256
)
$ChecksumFile = Join-Path $ChecksumsDir "SHA256SUMS.txt"
$Hashes | ForEach-Object { "$($_.Hash.ToLowerInvariant())  $(Split-Path $_.Path -Leaf)" } | Set-Content -Path $ChecksumFile -Encoding utf8

Write-Host "Portable executable: $Exe"
Write-Host "Installer: $($Installer.FullName)"
Write-Host "Checksums: $ChecksumFile"
