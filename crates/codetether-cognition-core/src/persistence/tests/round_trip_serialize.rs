//! round trip serialize.
use super::*;
#[test]
fn round_trip_serialize() {
    let state = PersistedCognitionState {
        schema_version: SCHEMA_VERSION,
        persisted_at: Utc::now(),
        personas: HashMap::new(),
        proposals: HashMap::new(),
        beliefs: HashMap::new(),
        attention_queue: Vec::new(),
        workspace: GlobalWorkspace::default(),
        evidence_events: Vec::new(),
        recent_snapshots: Vec::new(),
    };

    let json = serde_json::to_string(&state).expect("should serialize");
    let loaded: PersistedCognitionState = serde_json::from_str(&json).expect("should deserialize");
    assert_eq!(loaded.schema_version, SCHEMA_VERSION);
}
