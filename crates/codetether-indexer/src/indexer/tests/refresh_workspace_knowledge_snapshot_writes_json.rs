//! Indexer behavior regression.

use super::*;

#[tokio::test]
async fn refresh_workspace_knowledge_snapshot_writes_json() {
    let temp = tempdir().expect("tempdir");
    let data_dir = temp.path().join("data");
    std::fs::write(temp.path().join("lib.rs"), "pub fn run() {}\n").expect("write");

    unsafe {
        std::env::set_var("CODETETHER_DATA_DIR", data_dir.display().to_string());
    }

    let output_path = refresh_workspace_knowledge_snapshot(temp.path())
        .await
        .expect("snapshot should write");
    let payload = std::fs::read_to_string(&output_path).expect("snapshot payload");

    unsafe {
        std::env::remove_var("CODETETHER_DATA_DIR");
    }

    assert_eq!(
        output_path.extension().and_then(|ext| ext.to_str()),
        Some("json")
    );
    assert!(payload.contains("\"knowledge_graph\""));
    assert!(payload.contains("\"symbol\""));
}
