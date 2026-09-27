param(
    [string]$Version = "0.1.0",
    [switch]$SkipTests
)

$ErrorActionPreference = "Stop"
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
    Write-Host "Publish directory also contains supporting files:"
    $Unexpected | ForEach-Object { Write-Host " - $($_.Name)" }
}

$Process = Start-Process -FilePath $Exe -PassThru
Start-Sleep -Seconds 5
$Process.Refresh()
if ($Process.HasExited) {
    if ($Process.ExitCode -ne 0) { throw "GitGat smoke launch exited with code $($Process.ExitCode)." }
    Write-Host "GitGat started and exited cleanly during smoke launch."
} else {
    Stop-Process -Id $Process.Id -Force
    Write-Host "GitGat smoke launch succeeded."
}

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

$Hashes = @(
    Get-FileHash $Exe -Algorithm SHA256
    Get-FileHash $Installer.FullName -Algorithm SHA256
)
$ChecksumFile = Join-Path $ChecksumsDir "SHA256SUMS.txt"
$Hashes | ForEach-Object { "$($_.Hash.ToLowerInvariant())  $(Split-Path $_.Path -Leaf)" } | Set-Content -Path $ChecksumFile -Encoding utf8

Write-Host "Portable executable: $Exe"
Write-Host "Installer: $($Installer.FullName)"
Write-Host "Checksums: $ChecksumFile"
