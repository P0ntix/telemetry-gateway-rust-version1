mod anomaly;
mod drone;
mod ingest;
mod state;
mod telemetry;
mod ui;

use std::sync::Arc;

use tokio::sync::mpsc;
use tracing_subscriber::fmt;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    fmt()
        .with_env_filter("info")
        .with_thread_ids(false)
        .try_init()
        .ok();

    let (tx, _rx) = mpsc::unbounded_channel();
    let world = Arc::new(state::WorldState::default());

    tracing::info!("starting telemetry gateway simulator");

    tokio::spawn(async {
        if let Err(err) = drone::start_simulation().await {
            tracing::error!(error = ?err, "simulation failed");
        }
    });

    tokio::spawn({
        let world = Arc::clone(&world);
        async move {
            if let Err(err) = ingest::start_receiver(tx, world).await {
                tracing::error!(error = ?err, "receiver failed");
            }
        }
    });

    let dashboard_world = Arc::clone(&world);
    tokio::spawn(async move {
        ui::run_dashboard(dashboard_world).await;
    });

    tracing::info!("waiting for Ctrl+C");
    tokio::signal::ctrl_c()
        .await
        .expect("failed to wait for Ctrl+C");

    ui::shutdown_terminal();
    tracing::info!("shutdown requested");
}
