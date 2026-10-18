//! Listing cache is written and reused across scans.

use super::super::scan::scan;
use super::write_session;

#[tokio::test]
async fn scan_writes_and_reuses_listing_cache() {
    let temp = tempfile::tempdir().unwrap();
    let sessions = temp.path().join("sessions");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&sessions).unwrap();
    std::fs::create_dir_all(&workspace).unwrap();
    write_session(&sessions, "keep", &workspace, 3);

    let first = scan(sessions.clone(), Some(workspace.clone()))
        .await
        .unwrap();
    assert_eq!(first.len(), 1);
    assert!(
        sessions.join(".listing_cache.json").exists(),
        "cache file should be written"
    );

    let second = scan(sessions, Some(workspace)).await.unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(second[0].id, "keep");
    assert_eq!(second[0].message_count, 3);
}
