import { useState, useEffect } from 'react'
import './App.css'

interface HostInfo {
  hostname: string;
  architecture: string;
  ipv4_address: string;
  mac_address: string;
}

function App() {
  const [hostInfo, setHostInfo] = useState<HostInfo | null>(null);

  useEffect(() => {
    // En un caso real, esto se conectaría con el coordinador
    // Por ahora es solo un ejemplo
    const fetchHostInfo = async () => {
      try {
        const response = await fetch('http://localhost:8000/host-info');
        const data = await response.json();
        setHostInfo(data);
      } catch (error) {
        console.error('Error fetching host info:', error);
      }
    };

    fetchHostInfo();
  }, []);

  return (
    <div className="container">
      <header className="header">
        <h1>DMIG - Sistema de Migración de Red</h1>
      </header>

      <div className="warning-box">
        <span className="warning-icon">⚠️</span>
        <p className="warning-text">
          Este agente debe ser ejecutado con privilegios de administrador para poder realizar cambios en la
          configuración de red.
        </p>
      </div>

      <section className="download-section">
        <h2>Descarga del Agente</h2>
        <p>Seleccione la versión correspondiente a su sistema:</p>
        
        <div className="download-buttons">
          <button className="download-button">
            Descargar para Windows x64
          </button>
          <button className="download-button">
            Descargar para Windows x86
          </button>
        </div>
      </section>

      <section className="instructions-section">
        <h2>Instrucciones de Instalación</h2>
        <ol>
          <li>Descargue el agente correspondiente a su sistema operativo</li>
          <li>Ejecute el archivo con privilegios de administrador</li>
          <li>El agente se instalará automáticamente y comenzará a ejecutarse como servicio</li>
          <li>Una vez instalado, el coordinador podrá detectar y gestionar el equipo</li>
        </ol>
      </section>
    </div>
  )
}

export default App 