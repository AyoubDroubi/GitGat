param(
    [Parameter(Mandatory = $false)]
    [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$portable = Join-Path $root "dist\portable"
$installer = Join-Path $root "dist\installer"
$checksums = Join-Path $root "dist\checksums"

Remove-Item (Join-Path $root "dist") -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $portable, $installer, $checksums | Out-Null

cargo test --all-targets
cargo build --release

$builtExe = Join-Path $root "target\release\gitgat.exe"
if (-not (Test-Path $builtExe)) {
    throw "Expected release executable was not produced: $builtExe"
}

$portableExe = Join-Path $portable "GitGat.exe"
Copy-Item $builtExe $portableExe
& $portableExe --self-check

$iscc = Join-Path ${env:ProgramFiles(x86)} "Inno Setup 6\ISCC.exe"
if (-not (Test-Path $iscc)) {
    throw "Inno Setup compiler not found: $iscc"
}

& $iscc "/DMyAppVersion=$Version" (Join-Path $root "packaging\windows\GitGat.iss")

$installerExe = Join-Path $installer "GitGat-Setup-x64.exe"
if (-not (Test-Path $installerExe)) {
    throw "Installer was not produced."
}

$installRoot = Join-Path $env:TEMP ("GitGat-Smoke-" + [guid]::NewGuid().ToString("N"))
$installLog = Join-Path $env:TEMP ("GitGat-Install-" + [guid]::NewGuid().ToString("N") + ".log")
$installProcess = Start-Process -FilePath $installerExe -ArgumentList @(
    "/VERYSILENT",
    "/SUPPRESSMSGBOXES",
    "/NORESTART",
    "/SP-",
    "/DIR=$installRoot",
    "/LOG=$installLog"
) -Wait -PassThru
if ($installProcess.ExitCode -ne 0) {
    if (Test-Path $installLog) {
        Get-Content $installLog | Write-Host
    }
    throw "Silent installer exited with code $($installProcess.ExitCode)"
}

$installedExe = Join-Path $installRoot "GitGat.exe"
if (-not (Test-Path $installedExe)) {
    if (Test-Path $installLog) {
        Get-Content $installLog | Write-Host
    }
    throw "Silent install did not produce GitGat.exe"
}
& $installedExe --self-check

$uninstaller = Join-Path $installRoot "unins000.exe"
if (-not (Test-Path $uninstaller)) {
    throw "Uninstaller was not created."
}
$uninstallProcess = Start-Process -FilePath $uninstaller -ArgumentList @(
    "/VERYSILENT",
    "/SUPPRESSMSGBOXES",
    "/NORESTART"
) -Wait -PassThru
if ($uninstallProcess.ExitCode -ne 0) {
    throw "Silent uninstaller exited with code $($uninstallProcess.ExitCode)"
}
if (Test-Path $installedExe) {
    throw "Silent uninstall did not remove GitGat.exe"
}
Remove-Item $installLog -Force -ErrorAction SilentlyContinue

$portableHash = (Get-FileHash $portableExe -Algorithm SHA256).Hash.ToLowerInvariant()
$installerHash = (Get-FileHash $installerExe -Algorithm SHA256).Hash.ToLowerInvariant()
@(
    "$portableHash  GitGat.exe",
    "$installerHash  GitGat-Setup-x64.exe"
) | Set-Content (Join-Path $checksums "SHA256SUMS.txt")

Write-Host "GitGat $Version packaged successfully."
