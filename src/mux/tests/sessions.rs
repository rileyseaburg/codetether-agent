//! Session lifecycle on one workspace server.

use std::path::PathBuf;

use crate::mux::model::MuxRuntimeStatus;

#[test]
fn sessions_on_one_server_are_isolated_and_window_ids_are_server_wide() {
    let mut state = super::model::server();
    let first = state
        .create_session("stripe".into(), PathBuf::from("/repo"))
        .unwrap();
    let second = state
        .create_session("twilio".into(), PathBuf::from("/repo"))
        .unwrap();
    assert_eq!((first, second), (1, 2));
    assert!(
        state
            .create_session("stripe".into(), PathBuf::from("/repo"))
            .is_err()
    );

    state.session_mut("stripe").unwrap().runtime = Some(runtime("s1"));
    assert!(state.session("twilio").unwrap().runtime.is_none());
    assert!(state.session("work").unwrap().runtime.is_none());

    assert_eq!(state.close_session("stripe").unwrap(), vec![1]);
    assert_eq!(state.sessions.len(), 2);
    assert!(state.close_session("stripe").is_err());
}

fn runtime(id: &str) -> MuxRuntimeStatus {
    MuxRuntimeStatus {
        session_id: id.into(),
        session_title: id.into(),
        processing: false,
        message_count: 0,
        current_tool: None,
        needs_interaction: false,
        lagging: false,
        principal: Default::default(),
    }
}
