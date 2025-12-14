# Palo Alto Launcher - Guía de Instalación para Windows

## 📋 Requisitos Previos

1. **Rust** instalado en Windows
2. **Firefox** y/o **Edge** y/o **Chrome**
3. **PowerShell** o **CMD**

## 🔧 Instalación de Rust en Windows

Si aún no tienes Rust instalado:

1. Descarga el instalador desde: https://rustup.rs/
2. Ejecuta `rustup-init.exe`
3. Sigue las instrucciones (opción por defecto está bien)
4. Reinicia tu terminal

Verifica la instalación:
```powershell
cargo --version
```

## 📦 Compilación del Proyecto

### Opción 1: Desde PowerShell

```powershell
# Navegar al directorio del proyecto
cd palo-launcher-windows

# Compilar
cargo build --release

# El ejecutable estará en:
# .\target\release\palo-launcher.exe
```

### Opción 2: Desde CMD

```cmd
cd palo-launcher-windows
cargo build --release
```

## 🚀 Instalación como Comando del Sistema

### Método 1: Añadir al PATH (Recomendado)

1. **Copiar el ejecutable a una ubicación permanente:**
```powershell
# Crear directorio para tus herramientas
mkdir C:\Tools
copy .\target\release\palo-launcher.exe C:\Tools\
```

2. **Añadir al PATH:**
   - Presiona `Win + X` → "Sistema"
   - Click en "Configuración avanzada del sistema"
   - Click en "Variables de entorno"
   - En "Variables del sistema", busca `Path` y haz doble click
   - Click en "Nuevo"
   - Añade: `C:\Tools`
   - Click OK en todas las ventanas

3. **Verificar:**
```powershell
# Cierra y abre una nueva terminal
palo-launcher --help
```

### Método 2: PowerShell Profile (Alias)

```powershell
# Crear/editar tu perfil de PowerShell
notepad $PROFILE

# Añadir esta línea:
New-Alias -Name palo -Value "C:\ruta\completa\a\palo-launcher.exe"

# Guardar y recargar:
. $PROFILE
```

### Método 3: Copiar a System32 (Requiere Admin)

```powershell
# Abrir PowerShell como Administrador
copy .\target\release\palo-launcher.exe C:\Windows\System32\
```

## 🎯 Configuración Inicial

### 1. Crear perfiles de Firefox (si usas Firefox)

```powershell
# Abrir el gestor de perfiles de Firefox
firefox -ProfileManager

# Crear los perfiles:
# - palo-firefox
# - palo-firefox-2 (opcional, para dos ventanas)
```

O desde línea de comandos:
```powershell
firefox -CreateProfile "palo-firefox"
firefox -CreateProfile "palo-firefox-2"
```

### 2. Configurar la herramienta

```powershell
palo-launcher config
```

Te preguntará:
- IP de tu Palo Alto
- Navegadores a usar (Firefox + Edge es lo recomendado en Windows)

### 3. Primera conexión

```powershell
palo-launcher
```

**IMPORTANTE - Primera vez:**
1. Se abrirán los navegadores
2. Loguéate manualmente en cada uno
3. **Marca "Recordar credenciales"**
4. Las siguientes veces ya estarás logueado

## 🔍 Solución de Problemas en Windows

### "El certificado SSL no es confiable"

**En Firefox:**
```
1. Acepta la excepción de seguridad
2. Click en "Avanzado" → "Aceptar el riesgo y continuar"
```

**En Edge/Chrome:**
```
1. Click en "Avanzado"
2. Click en "Continuar a [IP] (no seguro)"
```

### "Chrome/Edge rechaza las credenciales"

1. **Limpia el perfil y empieza de nuevo:**
```powershell
# Encontrar el directorio de perfiles
$env:LOCALAPPDATA\Google\Chrome\palo-chrome
$env:LOCALAPPDATA\Microsoft\Edge\palo-edge

# Eliminar el perfil
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Google\Chrome\palo-chrome"

# Vuelve a ejecutar palo-launcher
```

2. **Verifica que aceptaste el certificado SSL primero**

3. **Prueba con el flag de ignorar errores SSL (temporal):**
```powershell
# Para testing - NO recomendado para producción
chrome.exe --user-data-dir="$env:LOCALAPPDATA\Google\Chrome\palo-chrome" --ignore-certificate-errors https://TU_IP
```

### "No se encuentra el navegador"

Verifica que está instalado:
```powershell
# Firefox
Get-Command firefox

# Chrome
Test-Path "C:\Program Files\Google\Chrome\Application\chrome.exe"

# Edge (viene preinstalado en Windows 10/11)
Test-Path "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"
```

### "Error al compilar"

```powershell
# Actualizar Rust
rustup update

# Limpiar y recompilar
cargo clean
cargo build --release
```

## 💡 Uso en Windows

### Desde PowerShell
```powershell
# Conectar
palo-launcher

# Ver configuración
palo-launcher show

# Reconfigurar
palo-launcher config
```

### Desde CMD
```cmd
palo-launcher
```

### Crear un atajo de escritorio

1. Click derecho en el escritorio → Nuevo → Acceso directo
2. Ubicación: `C:\Tools\palo-launcher.exe`
3. Nombre: "Palo Alto"
4. Cambiar el ícono si quieres

## 🎨 Configuración Avanzada en Windows

### Script de inicio automático

Crea un archivo `palo-start.ps1`:
```powershell
# palo-start.ps1
Write-Host "Iniciando Palo Alto Launcher..." -ForegroundColor Green
Start-Process palo-launcher.exe -NoNewWindow
```

### Tarea programada (Inicio automático)

```powershell
# Crear tarea para ejecutar al login
$action = New-ScheduledTaskAction -Execute "C:\Tools\palo-launcher.exe"
$trigger = New-ScheduledTaskTrigger -AtLogOn
Register-ScheduledTask -TaskName "PaloLauncher" -Action $action -Trigger $trigger
```

### Integración con Windows Terminal

Añade un perfil en `settings.json`:
```json
{
    "name": "Palo Alto",
    "commandline": "powershell.exe -NoExit -Command palo-launcher",
    "icon": "🔥"
}
```

## 📂 Ubicaciones de Archivos en Windows

```
Configuración:
C:\Users\TU_USUARIO\AppData\Roaming\palo-launcher\config.json

Perfiles de navegador:
Firefox:  C:\Users\TU_USUARIO\AppData\Roaming\Mozilla\Firefox\Profiles\
Chrome:   C:\Users\TU_USUARIO\AppData\Local\Google\Chrome\palo-chrome\
Edge:     C:\Users\TU_USUARIO\AppData\Local\Microsoft\Edge\palo-edge\
```

## 🔐 Solución al Problema de Credenciales Incorrectas

Si Chrome o Edge rechaza tus credenciales:

### Paso 1: Acepta el certificado SSL manualmente

```powershell
# Abre Chrome con el perfil específico
& "C:\Program Files\Google\Chrome\Application\chrome.exe" --user-data-dir="$env:LOCALAPPDATA\Google\Chrome\palo-chrome" https://TU_IP_PALO_ALTO
```

1. Verás advertencia de certificado
2. Click en "Avanzado"
3. Click en "Continuar a [IP] (no seguro)"
4. **AHORA loguéate con tus credenciales**
5. Marca "Recordar credenciales"

### Paso 2: Exporta el certificado desde Firefox (si funciona ahí)

1. En Firefox, ve a tu Palo Alto
2. Click en el candado → "Conexión segura" → "Más información"
3. Tab "Seguridad" → "Ver certificado"
4. Descarga el certificado

5. En Chrome:
   - `chrome://settings/certificates`
   - Tab "Autoridades"
   - "Importar"
   - Selecciona el certificado descargado
   - Marca todas las opciones de confianza

### Paso 3: Verifica las cookies

```powershell
# En Chrome, ve a:
chrome://settings/cookies/detail?site=TU_IP_PALO_ALTO

# Asegúrate que las cookies están permitidas
```

## 🚀 Mejoras para Windows

### PowerShell Helper Functions

Añade esto a tu `$PROFILE`:

```powershell
# Funciones helper para Palo Alto
function palo { palo-launcher }
function palo-config { palo-launcher config }
function palo-show { palo-launcher show }

# Función para limpiar perfiles si hay problemas
function palo-reset {
    Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Google\Chrome\palo-chrome" -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Microsoft\Edge\palo-edge" -ErrorAction SilentlyContinue
    Write-Host "Perfiles limpiados. Ejecuta 'palo' y loguéate de nuevo." -ForegroundColor Yellow
}
```

## 📝 Notas Importantes

1. **Edge es excelente en Windows**: Viene preinstalado y funciona muy bien
2. **Firefox + Edge**: Combinación recomendada para Windows
3. **Perfiles persistentes**: Una vez configurados, las sesiones se mantienen
4. **Certificados SSL**: El primer login puede requerir aceptar certificados

## 🆘 Ayuda Adicional

Si tienes problemas:

1. Verifica que los navegadores están instalados correctamente
2. Ejecuta como Administrador si hay problemas de permisos
3. Revisa el archivo de configuración: `%APPDATA%\palo-launcher\config.json`
4. Prueba con otro navegador para descartar problemas específicos

---

**¡Disfruta de una administración más eficiente de Palo Alto en Windows! 🪟**
