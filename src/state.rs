use dashmap::DashMap;

use crate::anomaly::{detect_anomaly, AnomalyType};
use crate::telemetry::Telemetry;

#[derive(Debug, Default)]
pub struct WorldState {
    pub drones: DashMap<u16, Telemetry>,
}

impl WorldState {
    pub fn update(&self, telemetry: Telemetry) -> Option<AnomalyType> {
        let drone_id = telemetry.drone_id as u16;
        let anomaly = self
            .drones
            .get(&drone_id)
            .and_then(|prev| detect_anomaly(&prev, &telemetry));
        
        self.drones.insert(drone_id, telemetry);
        anomaly
    }

    pub fn len(&self) -> usize {
        self.drones.len()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::task::JoinSet;

    use super::WorldState;
    use crate::telemetry::Telemetry;

    #[tokio::test]
    async fn test_world_state_concurrent_updates() {
        let state = Arc::new(WorldState::default());
        let mut tasks = JoinSet::new();

        for drone_id in 0..=100u16 {
            let state = Arc::clone(&state);
            tasks.spawn(async move {
                for _ in 0..1000 {
                    let telemetry = Telemetry::random(drone_id as u32);
                    state.update(telemetry);
                }
            });
        }

        while tasks.join_next().await.is_some() {}

        assert_eq!(state.len(), 101);
    }
}
