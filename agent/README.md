# DMIG Agent

Agente de sistema que recopila y expone información del host a través de gRPC, con una interfaz de terminal interactiva.

## Estructura del Proyecto

```bash
agent/
├── proto/
│   └── agent.proto      # Definición del servicio gRPC
├── src/
│   ├── generated/       # Código generado por tonic
│   │   └── agent.rs    # Implementaciones del cliente y servidor gRPC
│   ├── main.rs         # Punto de entrada y lógica principal
│   └── tui.rs          # Interfaz de usuario en terminal
├── build.rs            # Script de construcción para protobuf
└── Cargo.toml          # Manifiesto y dependencias del proyecto
```

## Componentes Principales

### 1. Servicio gRPC (proto/agent.proto)
Define el servicio y mensajes para la comunicación:
- `AgentService`: Servicio principal
  - `GetHostInfo`: Obtiene información del host
- Mensajes:
  - `HostInfoRequest`: Mensaje de solicitud vacío
  - `HostInfoResponse`: Contiene hostname, arquitectura, IP y MAC

### 2. Interfaz de Terminal (src/tui.rs)
Implementa una TUI interactiva usando ratatui con:
- Panel de información del sistema
- Indicador de estado del servidor
- Actualización en tiempo real
- Controles:
  - `q`: Salir de la aplicación

### 3. Servidor Principal (src/main.rs)
Implementa:
- `AgentServiceImpl`: Servicio gRPC
  - Recopilación de información del sistema
  - Detección automática de interfaces de red
- Servidor concurrente usando tokio
- Integración con la interfaz TUI

## Requisitos del Sistema

- Sistema operativo: Linux, Windows o macOS
- Rust 1.70 o superior
- Acceso a información del sistema (permisos adecuados)

## Instalación

1. Clonar el repositorio:

```bash
git clone [URL_DEL_REPOSITORIO]
cd agent
```

2. Compilar el proyecto:
```bash
cargo build --release
```

## Uso

1. Ejecutar el agente:
```bash
cargo run --release
```

2. La interfaz mostrará:
   - Información del sistema
   - Estado del servidor gRPC
   - Dirección IP y MAC de la interfaz principal

## Características Técnicas

### Networking
- Puerto por defecto: 8000
- Escucha en todas las interfaces ([::0])
- Detección automática de la interfaz de red principal
- Soporte para IPv4

### Monitoreo del Sistema
- Hostname del sistema
- Arquitectura del procesador
- Dirección IP principal
- Dirección MAC correspondiente

### Interfaz de Usuario
- Diseño en paneles
- Actualización en tiempo real
- Indicadores de estado
- Manejo de eventos del teclado

## Desarrollo

### Requisitos para Desarrollo
- Rust y Cargo
- Protobuf compiler
- Dependencias de desarrollo:
  ```toml
  tonic = "0.9"
  prost = "0.11"
  tokio = "1.0"
  ratatui = "0.24"
  ```

### Comandos Útiles

```bash
# Compilar en modo desarrollo
cargo build

# Ejecutar tests
cargo test

# Generar documentación
cargo doc --open

# Verificar formato
cargo fmt

# Verificar con clippy
cargo clippy
```

## Solución de Problemas

### Problemas Comunes
1. Error de permisos al obtener información del sistema
   - Solución: Ejecutar con permisos adecuados

2. Puerto 8000 en uso
   - Solución: Modificar el puerto en el código

3. Interfaz de red no detectada
   - Solución: Verificar conexión de red activa

## Contribuir

1. Fork del repositorio
2. Crear rama de características
3. Commit de cambios
4. Push a la rama
5. Crear Pull Request

## Licencia

[Especificar la licencia aquí]

## Contacto

[Información de contacto del equipo] 