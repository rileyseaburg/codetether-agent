//! serialization contract.
//! Deterministic signing and serialized configuration contracts.
use super::*;

#[test]
fn config_defaults_and_archive_wire() {
    let config: S3SinkConfig =
        serde_json::from_str(r#"{"bucket":"test","prefix":"events/"}"#).unwrap();
    assert_eq!(config.region, "us-east-1");
    assert!(!config.stub_mode);
    assert!(config.access_key.is_none());
    let wire = serde_json::to_value(config).unwrap();
    assert!(wire.get("endpoint").is_none());
    let archive = ArchivedEventFile {
        local_path: "events.jsonl".into(),
        s3_url: "s3://test/key".into(),
        session_id: "run".into(),
        start_offset: 2,
        end_offset: 8,
        archived_at: "2026-10-17T12:00:00Z".parse().unwrap(),
    };
    let wire = serde_json::to_string(&archive).unwrap();
    let decoded: ArchivedEventFile = serde_json::from_str(&wire).unwrap();
    assert_eq!(decoded.end_offset, 8);
    assert_eq!(decoded.local_path, archive.local_path);
}
