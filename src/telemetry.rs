use anyhow::Result;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub drone_id: u32,
    pub timestamp: u64,
    pub lat: f64,
    pub lon: f64,
    pub altitude: f64,
    pub speed: f64,
    pub heading: f64,
    pub battery_pct: f64,
    pub is_armed: bool,
}

impl Telemetry {
    pub fn random(drone_id: u32) -> Self {
        let mut rng = thread_rng();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_millis() as u64;

        Self {
            drone_id,
            timestamp: now,
            lat: rng.gen_range(32.0..36.0),
            lon: rng.gen_range(-122.0..-116.0),
            altitude: rng.gen_range(10.0..220.0),
            speed: rng.gen_range(0.0..55.0),
            heading: rng.gen_range(0.0..360.0),
            battery_pct: rng.gen_range(15.0..100.0),
            is_armed: rng.gen_bool(0.75),
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        postcard::to_allocvec(self).map_err(Into::into)
    }
}
