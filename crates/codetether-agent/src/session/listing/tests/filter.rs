//! Workspace filtering + message counting.

use super::super::scan::scan;
use super::write_session;

#[tokio::test]
async fn scan_counts_messages_and_filters_workspace() {
    let temp = tempfile::tempdir().unwrap();
    let sessions = temp.path().join("sessions");
    let workspace = temp.path().join("workspace");
    let other = temp.path().join("other");
    std::fs::create_dir_all(&sessions).unwrap();
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::create_dir_all(&other).unwrap();

    write_session(&sessions, "keep", &workspace, 3);
    write_session(&sessions, "skip", &other, 2);

    let summaries = scan(sessions, Some(workspace)).await.unwrap();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].id, "keep");
    assert_eq!(summaries[0].message_count, 3);
}
