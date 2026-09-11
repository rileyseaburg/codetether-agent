use std::path::PathBuf;

use crate::mux::isolation::Isolation;
use crate::mux::model::MuxSnapshot;

pub(super) fn server() -> MuxSnapshot {
    MuxSnapshot::new("work".into(), PathBuf::from("/repo"), Isolation::Shared)
}

#[test]
fn windows_keep_independent_workspaces() {
    let mut state = server();
    let id = state.next_window_id();
    let session = state.session_mut("work").unwrap();
    session.create_window(id, PathBuf::from("/repo/frontend"));

    assert_eq!(session.active_window, 1);
    assert_eq!(session.windows[0].workspace, PathBuf::from("/repo"));
    assert_eq!(
        session.windows[1].workspace,
        PathBuf::from("/repo/frontend")
    );
}

#[test]
fn last_window_cannot_be_closed() {
    let mut state = server();
    assert!(state.session_mut("work").unwrap().close_window(0).is_err());
}

#[test]
fn shared_isolation_survives_snapshot_round_trip() {
    let encoded = serde_json::to_string(&server()).unwrap();
    let decoded: MuxSnapshot = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, server());
}

#[test]
fn legacy_single_session_snapshot_becomes_one_session_server() {
    let legacy = r#"{"name":"work","active_window":0,"windows":[{"id":0,"title":"repo","workspace":"/repo"}],"runtime":null}"#;
    let decoded: MuxSnapshot = serde_json::from_str(legacy).unwrap();
    assert_eq!(decoded.isolation, Isolation::Worktree);
    assert_eq!(decoded.workspace, PathBuf::from("/repo"));
    assert_eq!(decoded.sessions[0].name, "work");
}
