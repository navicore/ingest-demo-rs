#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use rand::thread_rng;
    use uuid::Uuid;

    use crate::model::Record;

    #[test]
    fn test_parse_timestamp() {
        let timestamp_str = "2025-04-16T07:18:45.592502Z";
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
            .unwrap()
            .with_timezone(&Utc);
        
        assert_eq!(timestamp.to_rfc3339(), "2025-04-16T07:18:45.592502+00:00");
    }

    #[test]
    fn test_record_methods() {
        let timestamp_str = "2022-03-15T07:18:45.592502Z";
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
            .unwrap()
            .with_timezone(&Utc);
        
        let record = Record {
            version: "1.0.0".to_string(),
            name: "Boat 1".to_string(),
            uuid: format!("urn:mrn:signalk:uuid:{}", Uuid::new_v4()),
            latitude: 37.78039400669318,
            longitude: -122.38526611923439,
            altitude: 0.0,
            course: 24.786300968644476,
            speed: 6,
            timestamp,
        };
        
        assert_eq!(record.year(), "2022");
        assert_eq!(record.month(), "03");
        assert_eq!(record.day(), "15");
    }
    
    #[test]
    fn test_generate_random() {
        let timestamp_str = "2025-04-16T07:18:45.592502Z";
        let timestamp = DateTime::parse_from_rfc3339(timestamp_str)
            .unwrap()
            .with_timezone(&Utc);
        
        let example = Record {
            version: "1.0.0".to_string(),
            name: "Boat 1".to_string(),
            uuid: format!("urn:mrn:signalk:uuid:{}", Uuid::new_v4()),
            latitude: 37.78039400669318,
            longitude: -122.38526611923439,
            altitude: 0.0,
            course: 24.786300968644476,
            speed: 6,
            timestamp,
        };
        
        let mut rng = thread_rng();
        let random_record = Record::generate_random(&example, &mut rng);
        
        assert_eq!(random_record.version, "1.0.0");
        assert!(random_record.name.starts_with("Boat "));
        assert!(random_record.uuid.starts_with("urn:mrn:signalk:uuid:"));
    }
}