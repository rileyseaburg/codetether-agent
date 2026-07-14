use std::path::PathBuf;

use crate::mux::model::MuxSnapshot;

#[test]
fn windows_keep_independent_workspaces() {
    let mut state = MuxSnapshot::new("work".into(), PathBuf::from("/backend"));
    state.create_window(PathBuf::from("/frontend"));

    assert_eq!(state.active_window, 1);
    assert_eq!(state.windows[0].workspace, PathBuf::from("/backend"));
    assert_eq!(state.windows[1].workspace, PathBuf::from("/frontend"));
}

#[test]
fn last_window_cannot_be_closed() {
    let mut state = MuxSnapshot::new("work".into(), PathBuf::from("/workspace"));
    assert!(state.close_window(0).is_err());
}
