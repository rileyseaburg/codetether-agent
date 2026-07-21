use tempfile::tempdir;

#[test]
fn finds_session_in_sibling_mux_worktree() {
    let root = tempdir().unwrap();
    let old = root.path().join(".codetether-worktrees/old");
    let session = old.join(".codetether-agent/sessions/session-1234.json");
    std::fs::create_dir_all(session.parent().unwrap()).unwrap();
    std::fs::write(&session, b"{}").unwrap();

    assert_eq!(super::under(root.path(), "session-1234"), Some(session));
}
