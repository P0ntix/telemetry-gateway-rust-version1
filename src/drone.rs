use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Result;
use tokio::net::UdpSocket;

use crate::telemetry::Telemetry;

pub async fn spawn_drone(drone_id: u32) -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0").await?;
    let target: SocketAddr = "127.0.0.1:8080".parse()?;

    loop {
        let telemetry = Telemetry::random(drone_id);
        let payload = telemetry.to_bytes()?;
        socket.send_to(&payload, target).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

pub async fn start_simulation() -> Result<()> {
    let mut handles = Vec::with_capacity(101);

    for drone_id in 0..=100u32 {
        let handle = tokio::spawn(async move {
            if let Err(err) = spawn_drone(drone_id).await {
                tracing::warn!(drone_id, error = ?err, "drone task exited");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
