use codetether_goal_core::TaskLog;

#[tokio::test]
async fn malformed_and_incomplete_events_are_errors_in_both_read_paths() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("journal"));
    for bytes in [b"{broken}\n".as_slice(), b"{incomplete".as_slice()] {
        std::fs::write(log.path(), bytes).unwrap();
        assert!(log.read_all().await.is_err());
        assert!(log.read_all_blocking().is_err());
    }
    std::fs::write(log.path(), b"\n \n").unwrap();
    assert!(log.read_all().await.unwrap().is_empty());
    assert!(log.read_all_blocking().unwrap().is_empty());
}
