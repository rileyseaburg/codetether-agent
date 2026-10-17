//! JSONL bucket serialization contract.
use super::fixtures::sample;

#[test]
fn split_bucket_wire_contract() {
    let dir = tempfile::tempdir().unwrap();
    let record = sample("golden");
    let batch = crate::oracle::BatchValidationStats {
        golden: vec![record.trace.clone()],
        consensus: vec![],
        failed: vec![(record.trace.clone(), "failed reason".into())],
        unverified: vec![],
    };
    let stats = batch
        .write_jsonl_split(dir.path().to_str().unwrap(), "sample")
        .unwrap();
    assert_eq!(stats.golden_count, 1);
    assert_eq!(stats.failed_count, 1);
    let text = std::fs::read_to_string(stats.failed_path).unwrap();
    let actual: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(actual["verdict"], "failed");
    assert_eq!(actual["reason"], "failed reason");
    assert_eq!(actual["trace"]["trace_id"], record.trace.trace_id);
    assert!(
        std::fs::read_to_string(stats.consensus_path)
            .unwrap()
            .is_empty()
    );
}
