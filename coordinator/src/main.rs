use tonic::{Request, transport::Channel};

pub mod agent {
    tonic::include_proto!("agent");
}

use agent::{agent_service_client::AgentServiceClient, HostInfoRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AgentServiceClient::connect("http://[::1]:8000").await?;

    let request = Request::new(HostInfoRequest {});
    let response = client.get_host_info(request).await?;

    println!("Información del host:");
    println!("Hostname: {}", response.get_ref().hostname);
    println!("Arquitectura: {}", response.get_ref().architecture);
    println!("Dirección IPv4: {}", response.get_ref().ipv4_address);
    println!("Dirección MAC: {}", response.get_ref().mac_address);

    Ok(())
} 