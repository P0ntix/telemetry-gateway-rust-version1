use crate::telemetry::Telemetry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyType {
    Speed,
    BatteryDrain,
    GpsSpoof,
}

pub fn detect_anomaly(old: &Telemetry, new: &Telemetry) -> Option<AnomalyType> {
    if new.speed > 150.0 {
        return Some(AnomalyType::Speed);
    }

    if old.battery_pct - new.battery_pct > 20.0 {
        return Some(AnomalyType::BatteryDrain);
    }

    let lat_delta = (new.lat - old.lat).abs();
    let lon_delta = (new.lon - old.lon).abs();
    if lat_delta > 0.5 || lon_delta > 0.5 {
        return Some(AnomalyType::GpsSpoof);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{detect_anomaly, AnomalyType};
    use crate::telemetry::Telemetry;

    #[test]
    fn test_detects_spoof_packet() {
        let old = Telemetry {
            drone_id: 7,
            timestamp: 1_700_000_000,
            lat: 40.7128,
            lon: -74.0060,
            altitude: 120.0,
            speed: 45.0,
            heading: 90.0,
            battery_pct: 98.0,
            is_armed: true,
        };

        let new = Telemetry {
            drone_id: 7,
            timestamp: 1_700_000_100,
            lat: 41.5000,
            lon: -73.8000,
            altitude: 122.0,
            speed: 42.0,
            heading: 100.0,
            battery_pct: 90.0,
            is_armed: true,
        };

        assert_eq!(detect_anomaly(&old, &new), Some(AnomalyType::GpsSpoof));
    }
}
