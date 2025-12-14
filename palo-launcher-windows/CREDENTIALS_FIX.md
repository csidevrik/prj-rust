# Solución al Problema de Credenciales Incorrectas en Chrome/Edge

## 🔍 Diagnóstico del Problema

Si Firefox acepta tus credenciales pero Chrome o Edge las rechaza, el problema suele ser:

1. **Certificado SSL no aceptado** (90% de los casos)
2. **Caché de navegador con sesión antigua**
3. **Cookies bloqueadas**
4. **Configuración de seguridad diferente**

---

## ✅ SOLUCIÓN COMPLETA (Paso a Paso)

### 🔧 Para Linux

#### Paso 1: Limpia el perfil de Chrome

```bash
# Elimina el perfil existente
rm -rf ~/.config/google-chrome/palo-chrome

# Si usas Brave:
rm -rf ~/.config/BraveSoftware/Brave-Browser/palo-brave
```

#### Paso 2: Abre Chrome manualmente y acepta el certificado

```bash
# Abre Chrome con el perfil específico
google-chrome --user-data-dir=~/.config/google-chrome/palo-chrome https://TU_IP_PALO_ALTO

# Para Brave:
brave-browser --user-data-dir=~/.config/BraveSoftware/Brave-Browser/palo-brave https://TU_IP_PALO_ALTO
```

**En la ventana que se abre:**

1. Verás una advertencia: "Su conexión no es privada" o "Your connection is not private"
2. Click en **"Avanzado"** o **"Advanced"**
3. Click en **"Continuar a [IP] (no seguro)"** o **"Proceed to [IP] (unsafe)"**
4. **AHORA sí** ingresa tus credenciales
5. **IMPORTANTE**: Marca la casilla **"Recordar mis credenciales"** o similar
6. Loguéate

#### Paso 3: Verifica que funcionó

```bash
# Cierra Chrome
# Ejecuta palo-launcher de nuevo
palo-launcher
```

Ahora Chrome debería abrirse ya logueado.

---

### 🪟 Para Windows

#### Paso 1: Limpia el perfil de Chrome/Edge

```powershell
# Chrome
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Google\Chrome\palo-chrome" -ErrorAction SilentlyContinue

# Edge
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\Microsoft\Edge\palo-edge" -ErrorAction SilentlyContinue
```

#### Paso 2: Abre el navegador manualmente

**Para Chrome:**
```powershell
& "C:\Program Files\Google\Chrome\Application\chrome.exe" --user-data-dir="$env:LOCALAPPDATA\Google\Chrome\palo-chrome" https://TU_IP_PALO_ALTO
```

**Para Edge:**
```powershell
& "C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe" --user-data-dir="$env:LOCALAPPDATA\Microsoft\Edge\palo-edge" https://TU_IP_PALO_ALTO
```

**En la ventana que se abre:**

1. Verás advertencia de certificado
2. Click en **"Avanzado"**
3. Click en **"Continuar a [IP] (no seguro)"**
4. Ingresa tus credenciales (las mismas que funcionan en Firefox)
5. **Marca "Recordar mis credenciales"**
6. Loguéate

#### Paso 3: Ejecuta palo-launcher

```powershell
palo-launcher
```

---

## 🔐 SOLUCIÓN ALTERNATIVA: Importar Certificado

Si los pasos anteriores no funcionan, importa el certificado de Palo Alto:

### En Linux

#### Paso 1: Exportar certificado desde Firefox (donde funciona)

1. Abre Firefox con tu perfil de Palo Alto
2. Ve a la IP del firewall
3. Click en el **candado** (barra de direcciones)
4. Click en **"Conexión segura"** → **"Más información"**
5. Tab **"Seguridad"** → **"Ver certificado"**
6. Click en **"Descargar"** → **"PEM (cert)"**
7. Guarda como: `palo-alto-cert.pem`

#### Paso 2: Importar en Chrome

```bash
# Opción A: Usar certutil
sudo apt install libnss3-tools

# Crear directorio de certificados de Chrome si no existe
mkdir -p ~/.pki/nssdb

# Importar el certificado
certutil -d sql:$HOME/.pki/nssdb -A -t "C,," -n "Palo Alto Firewall" -i palo-alto-cert.pem

# Opción B: Importar desde la GUI de Chrome
google-chrome
# Ve a: chrome://settings/certificates
# Tab "Autoridades" → "Importar" → Selecciona palo-alto-cert.pem
# Marca: "Confiar en este certificado para identificar sitios web"
```

### En Windows

#### Paso 1: Exportar certificado desde Firefox

1. En Firefox, ve a tu Palo Alto
2. Click en el **candado** → **"Conexión segura"** → **"Más información"**
3. Tab **"Seguridad"** → **"Ver certificado"**
4. Click en **"Descargar"** → **"PEM (cert)"**
5. Guarda en: `C:\Temp\palo-alto-cert.cer`

#### Paso 2: Importar en Chrome/Edge

**Para Chrome:**
```
1. Abre Chrome
2. chrome://settings/certificates
3. Tab "Autoridades"
4. Click "Importar"
5. Selecciona C:\Temp\palo-alto-cert.cer
6. Marca todas las opciones de confianza
7. OK
```

**Para Edge:**
```
1. Abre Edge
2. edge://settings/privacy
3. "Administrar certificados"
4. Tab "Entidades de certificación raíz de confianza"
5. "Importar"
6. Selecciona el certificado
7. Siguiente → Finalizar
```

---

## 🧪 PRUEBA RÁPIDA: ¿Es problema de SSL?

### Linux
```bash
# Abre Chrome ignorando errores de certificado (SOLO PARA TESTING)
google-chrome --user-data-dir=~/.config/google-chrome/palo-chrome-test --ignore-certificate-errors https://TU_IP_PALO_ALTO
```

### Windows
```powershell
& "C:\Program Files\Google\Chrome\Application\chrome.exe" --user-data-dir="$env:TEMP\chrome-test" --ignore-certificate-errors https://TU_IP_PALO_ALTO
```

**Si con este comando SÍ puedes loguear**, entonces el problema definitivamente es el certificado SSL.

---

## 📋 Checklist de Verificación

Marca cada paso:

- [ ] Eliminé el perfil de Chrome/Edge completamente
- [ ] Abrí el navegador manualmente con el perfil específico
- [ ] Acepté el certificado SSL en la advertencia
- [ ] Ingresé las mismas credenciales que funcionan en Firefox
- [ ] Marqué "Recordar credenciales"
- [ ] Cerré el navegador después de loguear exitosamente
- [ ] Ejecuté `palo-launcher` de nuevo

---

## 🎯 ¿Por qué Firefox funciona y Chrome no?

**Razón técnica:**
- Firefox usa su propio almacén de certificados
- Chrome/Edge usan el almacén del sistema operativo
- Cuando abres diferentes perfiles, cada uno tiene su propia lista de certificados aceptados
- El perfil nuevo de Chrome NO tiene el certificado de Palo Alto en su lista de confianza

**Por eso necesitas aceptar manualmente el certificado en cada perfil nuevo.**

---

## 💡 Prevención para el Futuro

Para evitar este problema en instalaciones futuras:

1. **Siempre abre el navegador manualmente la primera vez**
2. **Acepta el certificado antes de usar palo-launcher**
3. **Considera instalar un certificado SSL válido en tu Palo Alto** (solución definitiva)

---

## 🆘 Si NADA Funciona

Como último recurso, usa dos instancias de Firefox:

```bash
# En Linux - Edita tu config
nano ~/.config/palo-launcher/config.json

# Cambia a:
"default_browsers": ["firefox", "firefox2"]

# En Windows - Edita:
# %APPDATA%\palo-launcher\config.json
```

Firefox con dos perfiles diferentes siempre funciona porque usa su propio motor de certificados.

---

## 📞 Debug Adicional

Si el problema persiste, ejecuta esto para debug:

### Linux
```bash
# Ver si el certificado está en el almacén
certutil -d sql:$HOME/.pki/nssdb -L

# Ver logs de Chrome
google-chrome --enable-logging --v=1 --user-data-dir=~/.config/google-chrome/palo-chrome 2>&1 | grep -i cert
```

### Windows
```powershell
# Ver certificados instalados
Get-ChildItem -Path Cert:\CurrentUser\Root

# Ejecutar Chrome con logs
& "C:\Program Files\Google\Chrome\Application\chrome.exe" --enable-logging --v=1 --user-data-dir="$env:LOCALAPPDATA\Google\Chrome\palo-chrome"
# Los logs estarán en: %LOCALAPPDATA%\Google\Chrome\palo-chrome\chrome_debug.log
```

---

**Esta guía debería resolver el 99% de los problemas de credenciales incorrectas. ¡Buena suerte! 🚀**
