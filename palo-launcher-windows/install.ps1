# Script de Instalacion para Palo Alto Launcher - Windows
# Ejecuta este script en PowerShell

Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host "       Instalador de Palo Alto Launcher v0.1.0" -ForegroundColor Cyan
Write-Host "                    Windows Edition" -ForegroundColor Cyan
Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host ""

# Verificar que estamos en el directorio correcto
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "X Error: Debes ejecutar este script desde el directorio palo-launcher-windows" -ForegroundColor Red
    exit 1
}

# Verificar si Rust esta instalado
Write-Host "Verificando Rust..." -NoNewline
$cargoVersion = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargoVersion) {
    Write-Host " OK" -ForegroundColor Green
} else {
    Write-Host " X" -ForegroundColor Red
    Write-Host ""
    Write-Host "Rust/Cargo no esta instalado." -ForegroundColor Red
    Write-Host ""
    Write-Host "Por favor instala Rust primero desde: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "Despues de instalar, reinicia PowerShell y ejecuta este script de nuevo." -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "Compilando palo-launcher..." -ForegroundColor Blue
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "OK Compilacion exitosa" -ForegroundColor Green
} else {
    Write-Host "X Error en la compilacion" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host "           Opciones de instalacion" -ForegroundColor Cyan
Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Instalar en C:\Tools (recomendado, aniadir al PATH manualmente)"
Write-Host "2. Instalar en `$env:USERPROFILE\bin (solo tu usuario)"
Write-Host "3. Solo compilar (no instalar, ejecutar desde .\target\release\)"
Write-Host ""

$option = Read-Host "Selecciona una opcion [1-3]"

$needsPath = $false
$installedPath = ""

switch ($option) {
    "1" {
        Write-Host ""
        Write-Host "Instalando en C:\Tools..." -ForegroundColor Blue
        
        # Crear directorio si no existe
        if (-not (Test-Path "C:\Tools")) {
            New-Item -ItemType Directory -Path "C:\Tools" -Force | Out-Null
        }
        
        # Copiar ejecutable
        Copy-Item ".\target\release\palo-launcher.exe" "C:\Tools\" -Force
        Write-Host "OK Instalado en C:\Tools\palo-launcher.exe" -ForegroundColor Green
        
        $installedPath = "C:\Tools\palo-launcher.exe"
        $needsPath = $true
    }
    "2" {
        Write-Host ""
        Write-Host "Instalando en $env:USERPROFILE\bin..." -ForegroundColor Blue
        
        # Crear directorio si no existe
        $binDir = Join-Path $env:USERPROFILE "bin"
        if (-not (Test-Path $binDir)) {
            New-Item -ItemType Directory -Path $binDir -Force | Out-Null
        }
        
        # Copiar ejecutable
        Copy-Item ".\target\release\palo-launcher.exe" "$binDir\" -Force
        Write-Host "OK Instalado en $binDir\palo-launcher.exe" -ForegroundColor Green
        
        $installedPath = "$binDir\palo-launcher.exe"
        
        # Verificar si esta en el PATH
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        if ($currentPath -notlike "*$binDir*") {
            Write-Host ""
            Write-Host "! Advertencia: $binDir no esta en tu PATH" -ForegroundColor Yellow
            $needsPath = $true
        }
    }
    "3" {
        Write-Host ""
        Write-Host "OK Compilacion completada" -ForegroundColor Green
        Write-Host "El ejecutable esta en: $(Get-Location)\target\release\palo-launcher.exe"
        Write-Host ""
        Write-Host "Para ejecutarlo:" -ForegroundColor Yellow
        Write-Host "  .\target\release\palo-launcher.exe"
        $installedPath = ".\target\release\palo-launcher.exe"
    }
    default {
        Write-Host "Opcion invalida" -ForegroundColor Red
        exit 1
    }
}

# Instrucciones para aniadir al PATH si es necesario
if ($needsPath) {
    Write-Host ""
    Write-Host "===============================================================" -ForegroundColor Yellow
    Write-Host "           IMPORTANTE: Aniadir al PATH" -ForegroundColor Yellow
    Write-Host "===============================================================" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Para usar 'palo-launcher' desde cualquier lugar, aniade el directorio al PATH:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Opcion A - Manualmente:" -ForegroundColor Cyan
    Write-Host "  1. Presiona Win + X -> Sistema"
    Write-Host "  2. Configuracion avanzada del sistema"
    Write-Host "  3. Variables de entorno"
    Write-Host "  4. En 'Variables del usuario', edita 'Path'"
    Write-Host "  5. Aniade: $(Split-Path $installedPath)"
    Write-Host ""
    Write-Host "Opcion B - Con PowerShell (Administrador):" -ForegroundColor Cyan
    
    $pathToAdd = Split-Path $installedPath
    Write-Host "  `$currentPath = [Environment]::GetEnvironmentVariable('Path', 'User')"
    Write-Host "  [Environment]::SetEnvironmentVariable('Path', `"`$currentPath;$pathToAdd`", 'User')"
    Write-Host ""
    Write-Host "Despues, reinicia PowerShell para que los cambios surtan efecto." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host "        Configuracion de Navegadores" -ForegroundColor Cyan
Write-Host "===============================================================" -ForegroundColor Cyan
Write-Host ""

$setupFirefox = Read-Host "Deseas configurar los perfiles de Firefox ahora? (s/n)"
if ($setupFirefox -eq "s" -or $setupFirefox -eq "S") {
    Write-Host ""
    Write-Host "Configurando perfiles de Firefox..." -ForegroundColor Blue
    
    # Buscar Firefox
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
        Write-Host "Firefox encontrado en: $firefoxExe" -ForegroundColor Green
        
        # Crear perfiles
        Start-Process -FilePath $firefoxExe -ArgumentList "-CreateProfile palo-firefox" -Wait -NoNewWindow
        Write-Host "OK Perfil 'palo-firefox' creado" -ForegroundColor Green
        
        Start-Process -FilePath $firefoxExe -ArgumentList "-CreateProfile palo-firefox-2" -Wait -NoNewWindow
        Write-Host "OK Perfil 'palo-firefox-2' creado" -ForegroundColor Green
    } else {
        Write-Host "! Firefox no encontrado. Instalalo desde: https://www.mozilla.org/firefox/" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "===============================================================" -ForegroundColor Green
Write-Host "OK Instalacion completada" -ForegroundColor Green
Write-Host "===============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "              Proximos pasos" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Si aniadiste al PATH, reinicia PowerShell" -ForegroundColor White
Write-Host ""
Write-Host "2. Configura la IP de tu Palo Alto:" -ForegroundColor White
Write-Host "   palo-launcher config" -ForegroundColor Yellow
Write-Host ""
Write-Host "3. Conecta al firewall:" -ForegroundColor White
Write-Host "   palo-launcher" -ForegroundColor Yellow
Write-Host ""
Write-Host "4. La primera vez, logueate en cada navegador y marca" -ForegroundColor White
Write-Host "   'Recordar credenciales' para mantener la sesion" -ForegroundColor White
Write-Host ""
Write-Host "5. (Opcional) Lee WINDOWS_INSTALL.md para mas detalles" -ForegroundColor White
Write-Host ""
Write-Host "Disfruta de una administracion mas eficiente de Palo Alto!" -ForegroundColor Green
Write-Host ""
