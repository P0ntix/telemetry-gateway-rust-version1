use anyhow::{Context, Result};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::UnboundedSender;

use crate::state::WorldState;
use crate::telemetry::Telemetry;

pub async fn start_receiver(tx: UnboundedSender<Telemetry>, state: std::sync::Arc<WorldState>) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0u8; 2048];

    loop {
        let (len, _) = socket.recv_from(&mut buf).await?;
        let packet = &buf[..len];

        let telemetry: Telemetry = postcard::from_bytes(packet)
            .with_context(|| "failed to deserialize telemetry packet")?;

        state.update(telemetry.clone());
        tx.send(telemetry)
            .with_context(|| "failed to forward telemetry packet")?;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::start_receiver;
    use crate::state::WorldState;
    use crate::telemetry::Telemetry;
    use tokio::net::UdpSocket;
    use tokio::sync::mpsc;
    use tokio::time::{timeout, Duration as TokioDuration};

    #[tokio::test]
    async fn test_ingest_receives_packet() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let state = Arc::new(WorldState::default());
        let receiver = tokio::spawn(async move {
            if let Err(err) = start_receiver(tx, state).await {
                panic!("receiver failed: {err:?}");
            }
        });

        let expected = Telemetry {
            drone_id: 42,
            timestamp: 1_700_000_000,
            lat: 37.7749,
            lon: -122.4194,
            altitude: 150.0,
            speed: 31.5,
            heading: 90.0,
            battery_pct: 96.0,
            is_armed: true,
        };

        let packet = expected.to_bytes().unwrap();
        let sender = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        sender.send_to(&packet, "127.0.0.1:8080").await.unwrap();

        let telemetry = timeout(TokioDuration::from_secs(2), rx.recv())
            .await
            .expect("timed out waiting for telemetry packet")
            .expect("channel closed before receiving telemetry");

        assert_eq!(telemetry.drone_id, expected.drone_id);
        assert_eq!(telemetry.timestamp, expected.timestamp);
        assert_eq!(telemetry.speed, expected.speed);

        receiver.abort();
    }
}
