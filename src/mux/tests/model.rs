use std::path::PathBuf;

use crate::mux::isolation::Isolation;
use crate::mux::model::MuxSnapshot;

#[test]
fn windows_keep_independent_workspaces() {
    let mut state = MuxSnapshot::new(
        "work".into(),
        PathBuf::from("/backend"),
        Isolation::Worktree,
    );
    state.create_window(PathBuf::from("/frontend"));

    assert_eq!(state.active_window, 1);
    assert_eq!(state.windows[0].workspace, PathBuf::from("/backend"));
    assert_eq!(state.windows[1].workspace, PathBuf::from("/frontend"));
}

#[test]
fn last_window_cannot_be_closed() {
    let mut state = MuxSnapshot::new(
        "work".into(),
        PathBuf::from("/workspace"),
        Isolation::Worktree,
    );
    assert!(state.close_window(0).is_err());
}

#[test]
fn shared_isolation_survives_snapshot_round_trip() {
    let state = MuxSnapshot::new("work".into(), PathBuf::from("/repo"), Isolation::Shared);
    let encoded = serde_json::to_string(&state).unwrap();
    let decoded: MuxSnapshot = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.isolation, Isolation::Shared);
}

#[test]
fn legacy_snapshot_without_isolation_defaults_to_worktree() {
    let legacy = r#"{"name":"work","active_window":0,"windows":[],"runtime":null}"#;
    let decoded: MuxSnapshot = serde_json::from_str(legacy).unwrap();
    assert_eq!(decoded.isolation, Isolation::Worktree);
}
