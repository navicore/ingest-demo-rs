use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub version: String,
    pub name: String,
    pub uuid: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub course: f64,
    pub speed: i32,
    pub timestamp: DateTime<Utc>,
}

impl Record {
    pub fn load_example(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let record: Record = serde_json::from_str(&contents)?;
        Ok(record)
    }

    pub fn generate_random(example: &Record, rng: &mut impl Rng) -> Self {
        let year = rng.random_range(2020..=2025);
        let month = rng.random_range(1..=12);
        let day = rng.random_range(1..=28);
        let hour = rng.random_range(0..=23);
        let minute = rng.random_range(0..=59);
        let second = rng.random_range(0..=59);

        let timestamp_str =
            format!("{year}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z");
        let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
            .unwrap()
            .with_timezone(&Utc);

        let boat_num = rng.random_range(1..=100);

        Self {
            version: example.version.clone(),
            name: format!("Boat {}", boat_num),
            uuid: format!("urn:mrn:signalk:uuid:{}", uuid::Uuid::new_v4()),
            latitude: example.latitude + rng.random_range(-1.0..1.0),
            longitude: example.longitude + rng.random_range(-1.0..1.0),
            altitude: rng.random_range(-10.0..100.0),
            course: rng.random_range(0.0..360.0),
            speed: rng.random_range(0..25),
            timestamp,
        }
    }

    pub fn year(&self) -> String {
        self.timestamp.format("%Y").to_string()
    }

    pub fn month(&self) -> String {
        self.timestamp.format("%m").to_string()
    }

    pub fn day(&self) -> String {
        self.timestamp.format("%d").to_string()
    }
}
