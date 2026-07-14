//! Internal metadata exclusion from session discovery.

use super::super::scan::scan;
use super::write_session;

#[tokio::test]
async fn scan_excludes_workspace_index_from_results() {
    let temp = tempfile::tempdir().unwrap();
    let sessions = temp.path().join("sessions");
    let workspace = temp.path().join("workspace");
    std::fs::create_dir_all(&sessions).unwrap();
    std::fs::create_dir_all(&workspace).unwrap();
    write_session(&sessions, ".workspace_index", &workspace, 1);
    write_session(&sessions, "session-id", &workspace, 1);

    let summaries = scan(sessions, Some(workspace)).await.unwrap();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].id, "session-id");
}
