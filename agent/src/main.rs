mod tui;

use tonic::{transport::Server, Request, Response, Status};
use sysinfo::{System, SystemExt};
use local_ip_address::local_ip;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use std::net::IpAddr;

// Generamos el módulo agent
pub mod agent {
    tonic::include_proto!("agent");
}

use agent::{
    agent_service_server::{AgentService, AgentServiceServer},
    HostInfoRequest, HostInfoResponse,
};
use tui::AgentTui;

#[derive(Default)]
pub struct AgentServiceImpl {}

impl AgentServiceImpl {
    fn get_correct_mac(target_ip: IpAddr) -> String {
        if let Ok(interfaces) = NetworkInterface::show() {
            for interface in interfaces {
                // Buscar la IP en las direcciones de la interfaz
                if interface.addr.iter().any(|addr| addr.ip() == target_ip) {
                    // Si encontramos la interfaz, devolver su MAC
                    if let Some(mac) = interface.mac_addr {
                        return mac;
                    }
                }
            }
        }
        "No encontrada".to_string()
    }
}

#[tonic::async_trait]
impl AgentService for AgentServiceImpl {
    async fn get_host_info(
        &self,
        _request: Request<HostInfoRequest>,
    ) -> Result<Response<HostInfoResponse>, Status> {
        let sys = System::new_all();
        let hostname = sys.host_name().unwrap_or_default();
        let architecture = std::env::consts::ARCH.to_string();
        let ipv4 = local_ip().unwrap();
        let mac = Self::get_correct_mac(ipv4);

        Ok(Response::new(HostInfoResponse {
            hostname,
            architecture,
            ipv4_address: ipv4.to_string(),
            mac_address: mac,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tui = AgentTui::new();
    
    // Obtener información inicial
    let sys = System::new_all();
    let hostname = sys.host_name().unwrap_or_default();
    let architecture = std::env::consts::ARCH.to_string();
    let ipv4 = local_ip().unwrap();
    let mac = AgentServiceImpl::get_correct_mac(ipv4);
    
    tui.update_info(hostname, architecture, ipv4.to_string(), mac);
    
    // Iniciar el servidor en un task separado
    let addr = "[::0]:8000".parse()?;
    tokio::spawn(async move {
        let agent = AgentServiceImpl::default();
        Server::builder()
            .add_service(AgentServiceServer::new(agent))
            .serve(addr)
            .await
            .unwrap();
    });

    tui.update_status(format!("Servidor escuchando en {}", addr));
    tui.run().await?;

    Ok(())
} 