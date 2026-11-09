# Hit Official Windows Installer
# https://hit-scm.com
#
# Usage:
#   irm https://hit-scm.com/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host @"
   __    __   __  .___________.
  |  |  |  | |  | |           |
  |  |__|  | |  | `---|  |----`
  |   __   | |  |     |  |     
  |  |  |  | |  |     |  |     
  |__|  |__| |__|     |__|     
"@ -ForegroundColor Cyan

Write-Host "Hit - High-Performance Source Control System" -ForegroundColor White
Write-Host "https://hit-scm.com`n" -ForegroundColor DarkGray

$Arch = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($Arch -ne "x86_64") {
    Write-Error "Hit currently requires a 64-bit Windows environment."
    exit 1
}

$InstallDir = if ($env:HIT_INSTALL_DIR) { $env:HIT_INSTALL_DIR } else { "$env:LOCALAPPDATA\Hit\bin" }
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$DownloadUrl = "https://github.com/hit-scm/hit/releases/latest/download/hit-x86_64-windows.zip"
$ZipPath = Join-Path $env:TEMP "hit-x86_64-windows.zip"
$ExePath = Join-Path $InstallDir "hit.exe"

# If local build exists, install directly
$LocalRelease = ".\target\release\hit.exe"
if (Test-Path $LocalRelease) {
    Write-Host "Found local release binary, installing directly..." -ForegroundColor DarkGray
    Copy-Item $LocalRelease -Destination $ExePath -Force
} else {
    Write-Host "Downloading Hit for Windows..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath
    Expand-Archive -Path $ZipPath -DestinationPath $env:TEMP\hit-extracted -Force
    Copy-Item "$env:TEMP\hit-extracted\hit.exe" -Destination $ExePath -Force
    Remove-Item $ZipPath -Force -ErrorAction SilentlyContinue
    Remove-Item "$env:TEMP\hit-extracted" -Recurse -Force -ErrorAction SilentlyContinue
}

# Add to user PATH if not present
$UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path = "$env:Path;$InstallDir"
    Write-Host "`n✔ Added $InstallDir to user PATH environment variable." -ForegroundColor Green
}

Write-Host "`n✔ Hit installed successfully at $ExePath" -ForegroundColor Green
Write-Host "`nGet started:"
Write-Host "  hit init" -ForegroundColor Yellow
Write-Host "  hit --help" -ForegroundColor Yellow
Write-Host "`nDocumentation & guides: https://hit-scm.com" -ForegroundColor Cyan

