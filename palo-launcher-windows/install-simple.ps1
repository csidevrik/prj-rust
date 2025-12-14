# Palo Alto Launcher - Windows Installer
# Simple version without special characters

Write-Host "=============================================================" -ForegroundColor Cyan
Write-Host "  Palo Alto Launcher Installer v0.1.0 - Windows" -ForegroundColor Cyan
Write-Host "=============================================================" -ForegroundColor Cyan
Write-Host ""

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "[ERROR] Run this script from palo-launcher-windows directory" -ForegroundColor Red
    exit 1
}

# Check if Rust is installed
Write-Host "Checking Rust installation..." -NoNewline
$cargoCheck = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargoCheck) {
    Write-Host " [OK]" -ForegroundColor Green
} else {
    Write-Host " [FAILED]" -ForegroundColor Red
    Write-Host ""
    Write-Host "Rust is not installed." -ForegroundColor Red
    Write-Host "Please install from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "After installing, restart PowerShell and run this script again." -ForegroundColor Yellow
    exit 1
}

# Build the project
Write-Host ""
Write-Host "Building palo-launcher..." -ForegroundColor Blue
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Build failed" -ForegroundColor Red
    exit 1
}

Write-Host "[OK] Build successful" -ForegroundColor Green

# Installation options
Write-Host ""
Write-Host "=============================================================" -ForegroundColor Cyan
Write-Host "  Installation Options" -ForegroundColor Cyan
Write-Host "=============================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Install to C:\Tools (recommended)"
Write-Host "2. Install to your user bin directory"
Write-Host "3. Just build (run from .\target\release\)"
Write-Host ""

$choice = Read-Host "Select option [1-3]"

switch ($choice) {
    "1" {
        Write-Host ""
        Write-Host "Installing to C:\Tools..." -ForegroundColor Blue
        
        if (-not (Test-Path "C:\Tools")) {
            New-Item -ItemType Directory -Path "C:\Tools" -Force | Out-Null
        }
        
        Copy-Item ".\target\release\palo-launcher.exe" "C:\Tools\" -Force
        Write-Host "[OK] Installed to C:\Tools\palo-launcher.exe" -ForegroundColor Green
        
        Write-Host ""
        Write-Host "[IMPORTANT] Add C:\Tools to your PATH:" -ForegroundColor Yellow
        Write-Host "  1. Press Win + X -> System"
        Write-Host "  2. Advanced system settings"
        Write-Host "  3. Environment Variables"
        Write-Host "  4. Edit 'Path' in User variables"
        Write-Host "  5. Add: C:\Tools"
        Write-Host ""
    }
    "2" {
        Write-Host ""
        $binDir = Join-Path $env:USERPROFILE "bin"
        Write-Host "Installing to $binDir..." -ForegroundColor Blue
        
        if (-not (Test-Path $binDir)) {
            New-Item -ItemType Directory -Path $binDir -Force | Out-Null
        }
        
        Copy-Item ".\target\release\palo-launcher.exe" "$binDir\" -Force
        Write-Host "[OK] Installed to $binDir\palo-launcher.exe" -ForegroundColor Green
        
        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($userPath -notlike "*$binDir*") {
            Write-Host ""
            Write-Host "[WARNING] $binDir is not in your PATH" -ForegroundColor Yellow
            Write-Host "Add it manually or run (as Admin):" -ForegroundColor Yellow
            Write-Host "  [Environment]::SetEnvironmentVariable('Path', `"`$env:Path;$binDir`", 'User')"
            Write-Host ""
        }
    }
    "3" {
        Write-Host ""
        Write-Host "[OK] Build complete" -ForegroundColor Green
        Write-Host "Run with: .\target\release\palo-launcher.exe"
    }
    default {
        Write-Host "Invalid option" -ForegroundColor Red
        exit 1
    }
}

# Setup Firefox profiles
Write-Host ""
Write-Host "=============================================================" -ForegroundColor Cyan
Write-Host "  Browser Setup" -ForegroundColor Cyan
Write-Host "=============================================================" -ForegroundColor Cyan
Write-Host ""

$setupFirefox = Read-Host "Setup Firefox profiles now? (y/n)"
if ($setupFirefox -eq "y" -or $setupFirefox -eq "Y") {
    Write-Host ""
    Write-Host "Setting up Firefox profiles..." -ForegroundColor Blue
    
    $firefoxPaths = @(
        "C:\Program Files\Mozilla Firefox\firefox.exe",
        "C:\Program Files (x86)\Mozilla Firefox\firefox.exe"
    )
    
    $firefoxExe = $null
    foreach ($path in $firefoxPaths) {
        if (Test-Path $path) {
            $firefoxExe = $path
            break
        }
    }
    
    if ($firefoxExe) {
        Write-Host "Firefox found: $firefoxExe" -ForegroundColor Green
        
        Start-Process -FilePath $firefoxExe -ArgumentList "-CreateProfile","palo-firefox" -Wait -NoNewWindow -ErrorAction SilentlyContinue
        Write-Host "[OK] Profile 'palo-firefox' created" -ForegroundColor Green
        
        Start-Process -FilePath $firefoxExe -ArgumentList "-CreateProfile","palo-firefox-2" -Wait -NoNewWindow -ErrorAction SilentlyContinue
        Write-Host "[OK] Profile 'palo-firefox-2' created" -ForegroundColor Green
    } else {
        Write-Host "[WARNING] Firefox not found" -ForegroundColor Yellow
        Write-Host "Install from: https://www.mozilla.org/firefox/" -ForegroundColor Yellow
    }
}

# Final instructions
Write-Host ""
Write-Host "=============================================================" -ForegroundColor Green
Write-Host "  Installation Complete!" -ForegroundColor Green
Write-Host "=============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. If you added to PATH, restart PowerShell"
Write-Host "2. Configure your firewall IP:"
Write-Host "   palo-launcher config" -ForegroundColor Yellow
Write-Host "3. Connect to your firewall:"
Write-Host "   palo-launcher" -ForegroundColor Yellow
Write-Host "4. First time: login and check 'Remember credentials'"
Write-Host ""
Write-Host "For more info, read WINDOWS_INSTALL.md"
Write-Host ""
Write-Host "Enjoy easier Palo Alto management!" -ForegroundColor Green
Write-Host ""
