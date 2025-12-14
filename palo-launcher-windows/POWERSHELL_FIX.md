# Solucion a Errores de PowerShell - install.ps1

## El Problema

Los errores que viste eran causados por:

1. **Caracteres UTF-8 especiales** (como ═ ✓ ✗ ⚠) que PowerShell no maneja bien en algunos sistemas
2. **Bloques try-catch** con sintaxis incorrecta
3. **Here-strings** (@" ... "@) mal formateados

## La Solucion

He creado **DOS versiones** del script de instalacion:

### 1. install.ps1 (Corregido)
- Version corregida del script original
- Elimine caracteres UTF-8 especiales
- Simplifique el manejo de errores
- Usa solo ASCII basico

### 2. install-simple.ps1 (RECOMENDADO)
- Version completamente nueva y simplificada
- Sin caracteres especiales en absoluto
- Manejo de errores mas robusto
- Mas facil de debuggear

## Como Usar

### Opcion A: Script Simple (Recomendado)

```powershell
cd palo-launcher-windows
.\install-simple.ps1
```

### Opcion B: Script Original Corregido

```powershell
cd palo-launcher-windows
.\install.ps1
```

## Si Aun Tienes Problemas con PowerShell

### Solucion 1: Cambiar la politica de ejecucion

```powershell
# Ejecuta PowerShell como Administrador
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Luego ejecuta el script
.\install-simple.ps1
```

### Solucion 2: Bypass temporal

```powershell
powershell -ExecutionPolicy Bypass -File .\install-simple.ps1
```

### Solucion 3: Instalacion completamente manual

Si los scripts no funcionan, puedes instalar manualmente:

```powershell
# 1. Compilar
cargo build --release

# 2. Crear directorio
New-Item -ItemType Directory -Path "C:\Tools" -Force

# 3. Copiar ejecutable
Copy-Item .\target\release\palo-launcher.exe C:\Tools\

# 4. Aniadir al PATH manualmente (ver instrucciones abajo)
```

## Como Aniadir al PATH Manualmente

### Metodo Grafico (Mas Facil)

1. Presiona `Win + X`
2. Selecciona "Sistema"
3. Click en "Configuracion avanzada del sistema"
4. Click en "Variables de entorno"
5. En "Variables del usuario", busca y selecciona "Path"
6. Click en "Editar"
7. Click en "Nuevo"
8. Aniade: `C:\Tools`
9. Click "OK" en todas las ventanas
10. **REINICIA PowerShell**

### Metodo PowerShell (Rapido)

```powershell
# Ejecuta PowerShell como Administrador
$currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
[Environment]::SetEnvironmentVariable("Path", "$currentPath;C:\Tools", "User")

# Reinicia PowerShell para aplicar cambios
```

## Verificar que Funciona

Despues de instalar y aniadir al PATH:

```powershell
# Cierra y abre una nueva ventana de PowerShell
# Luego verifica:
palo-launcher --help

# Si funciona, veras la ayuda del comando
```

## Configurar Firefox

Crea los perfiles manualmente si el script no lo hizo:

```powershell
# Buscar donde esta instalado Firefox
$firefox = "C:\Program Files\Mozilla Firefox\firefox.exe"

# Crear perfiles
& $firefox -CreateProfile "palo-firefox"
& $firefox -CreateProfile "palo-firefox-2"
```

## Proximos Pasos

Una vez instalado:

```powershell
# 1. Configurar IP del Palo Alto
palo-launcher config

# 2. Conectar
palo-launcher

# 3. Primera vez: acepta certificado SSL y loguea
# 4. Marca "Recordar credenciales"
# 5. Las siguientes veces ya estaras logueado!
```

## Diferencias Entre los Scripts

| Caracteristica | install.ps1 | install-simple.ps1 |
|----------------|-------------|-------------------|
| Caracteres especiales | No | No |
| Mensajes en espaniol | Si | Si (simplificados) |
| Manejo de errores | Basico | Mejorado |
| Compatibilidad | Alta | Muy alta |
| Recomendado | OK | **SI** |

## Problemas Comunes

### "No se puede ejecutar scripts en este sistema"

**Causa:** Politica de ejecucion restrictiva

**Solucion:**
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### "cargo no se reconoce"

**Causa:** Rust no esta instalado o no esta en el PATH

**Solucion:**
1. Instala Rust desde https://rustup.rs/
2. Reinicia PowerShell
3. Verifica: `cargo --version`

### "No se puede crear el directorio C:\Tools"

**Causa:** Permisos insuficientes

**Solucion:**
1. Ejecuta PowerShell como Administrador, o
2. Usa la opcion 2 (instalar en tu directorio de usuario)

## Archivos Incluidos

```
palo-launcher-windows/
├── install.ps1              <- Corregido, version 1
├── install-simple.ps1       <- RECOMENDADO, version 2
├── src/
│   └── main.rs             <- Codigo fuente
├── Cargo.toml              <- Config de Rust
├── README.md               <- Documentacion general
├── WINDOWS_INSTALL.md      <- Guia detallada Windows
└── CREDENTIALS_FIX.md      <- Solucion problema Chrome
```

## Resumen Ultra-Rapido

```powershell
# Opcion 1: Script automatico
cd palo-launcher-windows
.\install-simple.ps1

# Opcion 2: Manual
cargo build --release
Copy-Item .\target\release\palo-launcher.exe C:\Tools\
# Aniadir C:\Tools al PATH manualmente

# Despues:
palo-launcher config
palo-launcher
```

## Si Nada Funciona

Como ultima alternativa, puedes:

1. Compilar el proyecto: `cargo build --release`
2. Ejecutarlo directamente desde el directorio:
   ```powershell
   .\target\release\palo-launcher.exe config
   .\target\release\palo-launcher.exe
   ```
3. No necesitas aniadirlo al PATH, solo ejecutalo con la ruta completa

---

**¡El script simple (install-simple.ps1) deberia funcionar sin problemas!**
