# Palo Alto Launcher - Windows Edition 🪟

Herramienta CLI en Rust para gestionar conexiones al Firewall Palo Alto con múltiples navegadores y sesiones persistentes - **Compatible con Windows, Linux y macOS**.

## 🎯 Características

- ✅ Abre múltiples navegadores simultáneamente (Firefox, Chrome, Edge, Brave)
- ✅ Mantiene sesiones persistentes con perfiles separados
- ✅ Menú interactivo para acceder a diferentes secciones del firewall
- ✅ Configuración simple y reutilizable
- ✅ **Evita el tedioso proceso de login constante**
- ✅ **100% compatible con Windows y Linux**

## 📦 Instalación Rápida

### Windows

```powershell
# 1. Extraer el proyecto
cd palo-launcher-windows

# 2. Ejecutar script de instalación
.\install.ps1
```

### Linux

```bash
# 1. Extraer el proyecto
cd palo-launcher-windows

# 2. Compilar e instalar
cargo build --release
sudo cp target/release/palo-launcher /usr/local/bin/
```

## 🔧 Configuración Inicial

### 1. Crear perfiles de Firefox

**Windows:**
```powershell
firefox -CreateProfile "palo-firefox"
firefox -CreateProfile "palo-firefox-2"
```

**Linux:**
```bash
firefox -CreateProfile "palo-firefox"
firefox -CreateProfile "palo-firefox-2"
```

### 2. Configurar la herramienta

```bash
palo-launcher config
```

### 3. Primera conexión

```bash
palo-launcher
```

**⚠️ IMPORTANTE - Primera vez:**
1. Se abrirán los navegadores con perfiles nuevos
2. **Acepta el certificado SSL** en cada navegador
3. Loguéate manualmente con tus credenciales
4. **Marca "Recordar credenciales"**
5. ¡Las siguientes veces ya estarás logueado!

## 🚨 Problema: Chrome/Edge Rechaza Credenciales

Si Chrome o Edge dicen que tus credenciales son incorrectas (pero funcionan en Firefox):

**ES PROBLEMA DEL CERTIFICADO SSL, NO DE LAS CREDENCIALES**

### Solución rápida:

**Windows:**
```powershell
# Limpia el perfil de Chrome
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Google\Chrome\palo-chrome"

# Abre Chrome manualmente
& "C:\Program Files\Google\Chrome\Application\chrome.exe" --user-data-dir="$env:LOCALAPPDATA\Google\Chrome\palo-chrome" https://TU_IP_PALO_ALTO

# En el navegador:
# 1. Click "Avanzado"
# 2. Click "Continuar a [IP] (no seguro)"
# 3. AHORA loguéate (funcionará)
# 4. Marca "Recordar credenciales"
```

**Linux:**
```bash
# Limpia el perfil de Chrome
rm -rf ~/.config/google-chrome/palo-chrome

# Abre Chrome manualmente
google-chrome --user-data-dir=~/.config/google-chrome/palo-chrome https://TU_IP_PALO_ALTO

# En el navegador:
# 1. Click "Avanzado"
# 2. Click "Continuar a [IP] (no seguro)"
# 3. Loguéate
# 4. Marca "Recordar credenciales"
```

**📖 Guía completa:** Lee `CREDENTIALS_FIX.md` para más detalles

## 🎮 Uso

```bash
# Conectar (modo interactivo)
palo-launcher

# Ver configuración
palo-launcher show

# Reconfigurar
palo-launcher config
```

## 💡 Ventajas

| Antes | Ahora |
|-------|-------|
| Abrir navegadores manualmente | `palo-launcher` |
| Login cada vez | Una sola vez |
| Saltar entre pestañas | Dos navegadores organizados |
| Sin menú | Menú interactivo |

## 📂 Estructura del Proyecto

```
palo-launcher-windows/
├── src/
│   └── main.rs              # Código principal (cross-platform)
├── Cargo.toml               # Configuración de Rust
├── install.ps1              # Script de instalación Windows
├── README.md                # Este archivo
├── WINDOWS_INSTALL.md       # Guía detallada Windows
└── CREDENTIALS_FIX.md       # Solución problemas de login
```

## 🪟 Específico de Windows

### Navegadores recomendados en Windows
- **Firefox + Edge** (recomendado, Edge viene preinstalado)
- Firefox + Chrome
- Edge + Chrome

### Ubicación de archivos

```
Configuración:
%APPDATA%\palo-launcher\config.json

Perfiles:
Firefox:  %APPDATA%\Mozilla\Firefox\Profiles\
Chrome:   %LOCALAPPDATA%\Google\Chrome\palo-chrome\
Edge:     %LOCALAPPDATA%\Microsoft\Edge\palo-edge\
```

### Añadir al PATH en Windows

**PowerShell (como Administrador):**
```powershell
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
[Environment]::SetEnvironmentVariable("Path", "$currentPath;C:\Tools", "User")
```

**O manualmente:**
1. Win + X → Sistema
2. Configuración avanzada del sistema
3. Variables de entorno
4. Editar "Path" → Añadir `C:\Tools`

## 🐧 Específico de Linux

### Navegadores recomendados en Linux
- **Firefox + Brave** (recomendado)
- Firefox + Chrome

### Ubicación de archivos

```
Configuración:
~/.config/palo-launcher/config.json

Perfiles:
Firefox: ~/.mozilla/firefox/
Brave:   ~/.config/BraveSoftware/Brave-Browser/palo-brave/
Chrome:  ~/.config/google-chrome/palo-chrome/
```

## 🔍 Solución de Problemas

### "No se encuentra el navegador"

**Windows:**
```powershell
# Verificar Firefox
Get-Command firefox

# Verificar Chrome
Test-Path "C:\Program Files\Google\Chrome\Application\chrome.exe"

# Verificar Edge (preinstalado en Windows 10/11)
Test-Path "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"
```

**Linux:**
```bash
which firefox
which google-chrome
which brave-browser
```

### "Las sesiones no persisten"

1. Verifica que creaste los perfiles
2. No uses modo incógnito/privado
3. Acepta el certificado SSL la primera vez
4. Marca "Recordar credenciales" al loguear

### "Error al compilar"

```bash
# Actualizar Rust
rustup update

# Limpiar y recompilar
cargo clean
cargo build --release
```

## 🎨 Personalización

### Cambiar navegadores por defecto

```bash
palo-launcher config
```

### Editar configuración manualmente

**Windows:**
```powershell
notepad %APPDATA%\palo-launcher\config.json
```

**Linux:**
```bash
nano ~/.config/palo-launcher/config.json
```

Ejemplo:
```json
{
  "firewall_ip": "192.168.1.1",
  "firefox_profile": "palo-firefox",
  "brave_profile": "palo-brave",
  "chrome_profile": "palo-chrome",
  "edge_profile": "palo-edge",
  "default_browsers": ["firefox", "edge"]
}
```

## 🚀 Casos de Uso

### Día típico

```bash
# Llegar a la oficina
palo-launcher

# Seleccionar "Policies"
# Firefox se abre en Policies
# Edge/Chrome se abre listo para navegar a Monitor
# Trabajar eficientemente entre ambos
```

### Revisar logs mientras configuras

```bash
palo-launcher
# Selecciona "Logs de Tráfico"
# Un navegador muestra logs
# Otro navegador para buscar detalles
```

## 💼 Integración con Tu Workflow

### PowerShell (Windows)

Añade a tu `$PROFILE`:
```powershell
# Alias rápido
function palo { palo-launcher }

# Funciones helper
function palo-config { palo-launcher config }
function palo-show { palo-launcher show }
```

### Bash/Zsh (Linux)

Añade a tu `.bashrc` o `.zshrc`:
```bash
alias palo='palo-launcher'
alias palo-config='palo-launcher config'
alias palo-show='palo-launcher show'
```

## 📚 Documentación Adicional

- **WINDOWS_INSTALL.md** - Guía detallada de instalación en Windows
- **CREDENTIALS_FIX.md** - Solución al problema de credenciales incorrectas
- Los mismos archivos funcionan en Linux

## 🤝 Contribuciones

¿Ideas para mejorar? ¡Las sugerencias son bienvenidas!

## 📄 Licencia

MIT License - Úsalo libremente

---

## 🎯 TL;DR - Inicio Súper Rápido

```bash
# 1. Instalar
./install.ps1        # Windows
# o
cargo build --release && sudo cp target/release/palo-launcher /usr/local/bin/  # Linux

# 2. Crear perfiles
firefox -CreateProfile "palo-firefox"

# 3. Configurar IP
palo-launcher config

# 4. Conectar (primera vez acepta certificado SSL)
palo-launcher

# 5. ¡Listo! Las siguientes veces ya estarás logueado
```

---

**Desarrollado con ❤️ para administradores de Palo Alto que valoran su tiempo**

**Compatible con Windows 🪟 | Linux 🐧 | macOS 🍎**
